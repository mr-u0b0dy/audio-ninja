// SPDX-License-Identifier: Apache-2.0

//! REST API handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    engine::{SpeakerInfo, SpeakerStats},
    AppState,
};
use audio_ninja::{Position3, SpeakerDescriptor, SpeakerLayout, SpeakerRole};

#[derive(Serialize)]
pub struct StatusResponse {
    status: String,
    version: String,
    uptime_secs: u64,
}

#[derive(Serialize)]
pub struct InfoResponse {
    name: String,
    version: String,
    features: Vec<String>,
}

#[derive(Deserialize)]
pub struct LayoutRequest {
    preset: Option<String>,
    speakers: Option<Vec<SpeakerPositionRequest>>,
}

#[derive(Deserialize)]
pub struct SpeakerPositionRequest {
    speaker_id: Uuid,
    azimuth: f32,
    elevation: f32,
    distance: f32,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

/// GET /api/v1/status
pub async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_secs: state.started_at.elapsed().as_secs(),
    })
}

/// GET /api/v1/info
pub async fn info() -> Json<InfoResponse> {
    Json(InfoResponse {
        name: "Audio Ninja Engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        features: vec![
            "IAMF".to_string(),
            "VBAP".to_string(),
            "HOA".to_string(),
            "RTP Transport".to_string(),
            "Room Calibration".to_string(),
        ],
    })
}

/// GET /api/v1/speakers
pub async fn list_speakers(State(state): State<AppState>) -> Json<Vec<SpeakerInfo>> {
    let engine = state.engine.read().await;
    let speakers: Vec<SpeakerInfo> = engine.speakers.values().cloned().collect();
    Json(speakers)
}

/// POST /api/v1/speakers/discover
pub async fn discover_speakers(State(state): State<AppState>) -> StatusCode {
    let mut engine = state.engine.write().await;
    engine.start_discovery();
    StatusCode::ACCEPTED
}

/// GET /api/v1/speakers/:id
pub async fn get_speaker(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SpeakerInfo>, StatusCode> {
    let engine = state.engine.read().await;
    engine
        .speakers
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /api/v1/speakers/:id
pub async fn remove_speaker(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut engine = state.engine.write().await;
    if engine.remove_speaker(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

/// GET /api/v1/layout
pub async fn get_layout(State(state): State<AppState>) -> Result<Json<SpeakerLayout>, StatusCode> {
    let engine = state.engine.read().await;
    engine.layout.clone().map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// POST /api/v1/layout
pub async fn set_layout(
    State(state): State<AppState>,
    Json(request): Json<LayoutRequest>,
) -> StatusCode {
    let mut engine = state.engine.write().await;

    // Create layout from preset or custom positions
    let layout = if let Some(preset) = request.preset {
        match SpeakerLayout::from_preset(&preset) {
            Some(layout) => layout,
            None => return StatusCode::BAD_REQUEST,
        }
    } else if let Some(speakers) = request.speakers {
        if speakers.is_empty() {
            return StatusCode::BAD_REQUEST;
        }

        let descriptors: Vec<SpeakerDescriptor> = speakers
            .into_iter()
            .filter(|sp| sp.distance.is_finite() && sp.distance > 0.0)
            .map(|sp| {
                let az = sp.azimuth.to_radians();
                let el = sp.elevation.to_radians();
                let r = sp.distance;

                // Convert spherical (azimuth, elevation, radius) to Cartesian (x, y, z)
                // Azimuth rotates around Z with 0 at +Y (front), elevation from X-Y plane.
                let x = r * az.sin() * el.cos();
                let y = r * az.cos() * el.cos();
                let z = r * el.sin();

                SpeakerDescriptor {
                    id: sp.speaker_id.to_string(),
                    role: SpeakerRole::Custom("custom".to_string()),
                    position: Position3 { x, y, z },
                    max_spl_db: 110.0,
                    latency: std::time::Duration::ZERO,
                }
            })
            .collect();

        if descriptors.is_empty() {
            return StatusCode::BAD_REQUEST;
        }

        SpeakerLayout {
            name: "custom".to_string(),
            speakers: descriptors,
        }
    } else {
        return StatusCode::BAD_REQUEST;
    };

    engine.set_layout(layout);
    StatusCode::OK
}

/// POST /api/v1/transport/play
pub async fn transport_play(State(state): State<AppState>) -> StatusCode {
    let mut engine = state.engine.write().await;
    engine.play();
    StatusCode::OK
}

/// POST /api/v1/transport/pause
pub async fn transport_pause(State(state): State<AppState>) -> StatusCode {
    let mut engine = state.engine.write().await;
    engine.pause();
    StatusCode::OK
}

/// POST /api/v1/transport/stop
pub async fn transport_stop(State(state): State<AppState>) -> StatusCode {
    let mut engine = state.engine.write().await;
    engine.stop();
    StatusCode::OK
}

/// GET /api/v1/transport/status
pub async fn transport_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    Json(serde_json::json!({
        "state": format!("{:?}", engine.transport_state),
    }))
}

/// POST /api/v1/calibration/start
pub async fn calibration_start(State(state): State<AppState>) -> StatusCode {
    let mut engine = state.engine.write().await;
    engine.start_calibration();
    StatusCode::ACCEPTED
}

/// GET /api/v1/calibration/status
pub async fn calibration_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    Json(serde_json::json!({
        "running": engine.calibration.running,
        "progress": engine.calibration.progress,
        "measurements": engine.calibration.measurements.len(),
    }))
}

/// POST /api/v1/calibration/apply
pub async fn calibration_apply(State(state): State<AppState>) -> StatusCode {
    let mut engine = state.engine.write().await;
    match engine.apply_calibration() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

/// GET /api/v1/stats
pub async fn stats(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    let total_speakers = engine.speakers.len();
    let online_speakers = engine.speakers.values().filter(|s| s.online).count();

    Json(serde_json::json!({
        "total_speakers": total_speakers,
        "online_speakers": online_speakers,
        "transport_state": format!("{:?}", engine.transport_state),
        "has_layout": engine.layout.is_some(),
    }))
}

/// GET /api/v1/stats/network - Network bandwidth statistics
pub async fn stats_network(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    let active_speakers = engine.speakers.values().filter(|s| s.online).count();
    // Estimate bandwidth based on active speakers and sample rate
    let base_kbps = if matches!(engine.transport_state, crate::engine::TransportState::Playing) {
        (engine.playback.sample_rate as f64 * 2.0 * 16.0 / 8.0 / 1000.0) * active_speakers.max(1) as f64
    } else {
        0.0
    };
    Json(serde_json::json!({
        "sent_kbps": base_kbps,
        "received_kbps": base_kbps * 0.01,
        "peak_sent_kbps": base_kbps * 1.2,
        "peak_received_kbps": base_kbps * 0.015,
        "packets_sent": 0u64,
        "packets_received": 0u64,
    }))
}

/// GET /api/v1/stats/latency - Latency statistics across speakers
pub async fn stats_latency(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    let latencies: Vec<f64> = engine
        .speaker_stats
        .values()
        .map(|s| s.latency_ms as f64)
        .collect();

    let (min, max, mean, stddev) = if latencies.is_empty() {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        let min = latencies.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = latencies.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let variance = latencies.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
            / latencies.len() as f64;
        (min, max, mean, variance.sqrt())
    };

    Json(serde_json::json!({
        "min_ms": min,
        "max_ms": max,
        "mean_ms": mean,
        "stddev_ms": stddev,
        "samples": latencies,
        "speaker_count": engine.speakers.len(),
    }))
}

/// GET /api/v1/stats/daemon - Daemon process statistics
pub async fn stats_daemon(State(state): State<AppState>) -> Json<serde_json::Value> {
    let uptime = state.started_at.elapsed().as_secs();

    // Read process stats from /proc/self (Linux)
    let (cpu_percent, memory_mb) = read_proc_stats();

    Json(serde_json::json!({
        "cpu_percent": cpu_percent,
        "memory_mb": memory_mb,
        "uptime_secs": uptime,
        "pid": std::process::id(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Read process stats from /proc/self on Linux, fallback to zeros.
fn read_proc_stats() -> (f64, f64) {
    #[cfg(target_os = "linux")]
    {
        use std::sync::Mutex;

        // Track previous CPU tick values to compute delta-based CPU usage
        static PREV: Mutex<Option<(u64, std::time::Instant)>> = Mutex::new(None);

        let memory_mb = std::fs::read_to_string("/proc/self/statm")
            .ok()
            .and_then(|s| {
                s.split_whitespace()
                    .nth(1)
                    .and_then(|r| r.parse::<f64>().ok())
            })
            .map(|rss_pages| rss_pages * 4096.0 / (1024.0 * 1024.0))
            .unwrap_or(0.0);

        // CPU: read total process ticks from /proc/self/stat (fields 14+15 = utime+stime)
        let cpu_percent = std::fs::read_to_string("/proc/self/stat")
            .ok()
            .and_then(|stat| {
                // Fields after the (comm) block are space-separated; utime=field[13], stime=field[14]
                let after_paren = stat.rfind(')')?.checked_add(2)?;
                let fields: Vec<&str> = stat[after_paren..].split_whitespace().collect();
                let utime: u64 = fields.get(11)?.parse().ok()?;
                let stime: u64 = fields.get(12)?.parse().ok()?;
                let total_ticks = utime + stime;

                let mut prev = PREV.lock().ok()?;
                let ticks_per_sec = 100.0; // sysconf(_SC_CLK_TCK) is usually 100 on Linux

                let pct = if let Some((prev_ticks, prev_time)) = prev.as_ref() {
                    let dt = prev_time.elapsed().as_secs_f64();
                    if dt > 0.01 {
                        let dticks = total_ticks.saturating_sub(*prev_ticks) as f64;
                        (dticks / ticks_per_sec / dt * 100.0).min(100.0 * num_cpus())
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                *prev = Some((total_ticks, std::time::Instant::now()));
                Some(pct)
            })
            .unwrap_or(0.0);

        (cpu_percent, memory_mb)
    }
    #[cfg(not(target_os = "linux"))]
    {
        (0.0, 0.0)
    }
}

/// Return number of logical CPUs (for CPU% normalization)
fn num_cpus() -> f64 {
    std::thread::available_parallelism()
        .map(|n| n.get() as f64)
        .unwrap_or(1.0)
}

/// GET /api/v1/stats/sync - Speaker synchronization statistics
pub async fn stats_sync(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    let speakers: Vec<serde_json::Value> = engine
        .speakers
        .iter()
        .map(|(id, info)| {
            let stats = engine.speaker_stats.get(id);
            serde_json::json!({
                "id": id.to_string(),
                "name": info.name,
                "sync_error_ms": stats.map(|s| s.jitter_ms).unwrap_or(0.0),
                "status": if stats.map(|s| s.jitter_ms < 5.0).unwrap_or(true) {
                    "locked"
                } else if stats.map(|s| s.jitter_ms < 20.0).unwrap_or(false) {
                    "drift"
                } else {
                    "unlocked"
                },
            })
        })
        .collect();

    Json(serde_json::json!({
        "speakers": speakers,
        "overall_status": if speakers.is_empty() { "no_speakers" }
            else if speakers.iter().all(|s| s["status"] == "locked") { "locked" }
            else { "drift" },
    }))
}

/// GET /api/v1/stats/audio-levels - Real-time audio levels
pub async fn stats_audio_levels(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    let is_playing = matches!(engine.transport_state, crate::engine::TransportState::Playing);

    // Simulate dynamic levels with slight per-call jitter (will be replaced by
    // real metering once the cpal pipeline feeds peak values into shared state).
    let jitter = || -> f64 {
        // Simple pseudo-random based on nanosecond timestamp
        let ns = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as f64;
        (ns % 6.0) - 3.0 // ±3 dB jitter
    };

    let (input_db, output_db) = if is_playing {
        (-12.0 + jitter() * 0.5, -14.0 + jitter() * 0.5)
    } else {
        (-60.0, -60.0)
    };

    let per_speaker: Vec<f64> = engine
        .speakers
        .values()
        .map(|s| {
            if s.online && is_playing {
                -14.0 + jitter() * 0.4
            } else {
                -60.0
            }
        })
        .collect();

    Json(serde_json::json!({
        "input_db_left": input_db,
        "input_db_right": input_db + 0.5,
        "output_db_left": output_db,
        "output_db_right": output_db + 0.3,
        "per_speaker_db": per_speaker,
        "clipping": input_db > -1.0 || output_db > -1.0,
    }))
}

/// GET /api/v1/speakers/:id/stats
pub async fn speaker_stats(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SpeakerStats>, StatusCode> {
    let engine = state.engine.read().await;
    engine
        .speaker_stats
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
// ===== Audio Input/Output Endpoints =====

#[derive(Serialize)]
pub struct InputDeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub max_channels: u32,
    pub available: bool,
}

#[derive(Serialize)]
pub struct OutputDeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub max_channels: u32,
    pub available: bool,
    pub is_default: bool,
}

#[derive(Deserialize)]
pub struct LoadFileRequest {
    pub file_path: String,
}

#[derive(Deserialize)]
pub struct SelectInputRequest {
    pub source_id: String,
}

#[derive(Deserialize)]
pub struct SelectOutputRequest {
    pub device_id: String,
}

#[derive(Deserialize)]
pub struct SetTransportModeRequest {
    pub mode: String, // "file", "stream", or "mixed"
}

/// GET /api/v1/input/devices - List all input devices
pub async fn list_input_devices(State(state): State<AppState>) -> Json<Vec<InputDeviceInfo>> {
    let mut engine = state.engine.write().await;
    if let Ok(devices) = engine.enumerate_input_devices() {
        let infos: Vec<InputDeviceInfo> = devices
            .iter()
            .map(|d| {
                let parts: Vec<&str> = d.split(" (").collect();
                InputDeviceInfo {
                    id: d.clone(),
                    name: parts.first().map(|s| s.to_string()).unwrap_or_default(),
                    device_type: parts
                        .get(1)
                        .map(|s| s.trim_end_matches(')').to_string())
                        .unwrap_or_default(),
                    max_channels: 2,
                    available: true,
                }
            })
            .collect();
        Json(infos)
    } else {
        Json(vec![])
    }
}

/// GET /api/v1/output/devices - List all output devices
pub async fn list_output_devices(State(state): State<AppState>) -> Json<Vec<OutputDeviceInfo>> {
    let mut engine = state.engine.write().await;
    if let Ok(devices) = engine.enumerate_output_devices() {
        let infos: Vec<OutputDeviceInfo> = devices
            .iter()
            .enumerate()
            .map(|(idx, d)| {
                let parts: Vec<&str> = d.split(" (").collect();
                OutputDeviceInfo {
                    id: format!("output_{}", idx),
                    name: parts.first().map(|s| s.to_string()).unwrap_or_default(),
                    device_type: parts
                        .get(1)
                        .map(|s| s.trim_end_matches(')').to_string())
                        .unwrap_or_default(),
                    max_channels: 2,
                    available: true,
                    is_default: idx == 0,
                }
            })
            .collect();
        Json(infos)
    } else {
        Json(vec![])
    }
}

/// POST /api/v1/input/select - Select input source
pub async fn select_input_source(
    State(state): State<AppState>,
    Json(req): Json<SelectInputRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut engine = state.engine.write().await;
    match engine.select_input_source(&req.source_id) {
        Ok(source) => Ok(Json(serde_json::json!({
            "success": true,
            "source": source.source_type(),
            "device": source.device_name(),
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/v1/output/select - Select output device
pub async fn select_output_device(
    State(state): State<AppState>,
    Json(req): Json<SelectOutputRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut engine = state.engine.write().await;
    match engine.select_output_device(&req.device_id) {
        Ok(device) => Ok(Json(serde_json::json!({
            "success": true,
            "device": device.name,
            "device_type": device.device_type.to_string(),
            "channels": device.max_channels,
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/v1/transport/load-file - Load audio file for playback
pub async fn load_audio_file(
    State(state): State<AppState>,
    Json(req): Json<LoadFileRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut engine = state.engine.write().await;
    match engine.load_audio_file(&req.file_path) {
        Ok(_) => Ok(Json(serde_json::json!({
            "success": true,
            "file": req.file_path,
            "status": "loaded",
        }))),
        Err(e) => {
            eprintln!("Failed to load file: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// POST /api/v1/transport/mode - Set transport mode (file/stream/mixed)
pub async fn set_transport_mode(
    State(state): State<AppState>,
    Json(req): Json<SetTransportModeRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    use crate::engine::TransportMode;

    let mode = match req.mode.to_lowercase().as_str() {
        "file" => TransportMode::FilePlayback,
        "stream" => TransportMode::LiveStream,
        "mixed" => TransportMode::Mixed,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let mut engine = state.engine.write().await;
    engine.set_transport_mode(mode.clone());

    Ok(Json(serde_json::json!({
        "success": true,
        "mode": format!("{:?}", mode),
    })))
}

/// GET /api/v1/input/status - Get current input status
pub async fn input_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    if let Some(source) = engine.active_input_source() {
        Json(serde_json::json!({
            "active": true,
            "source_type": source.source_type(),
            "device": source.device_name(),
        }))
    } else {
        Json(serde_json::json!({
            "active": false,
            "source_type": null,
            "device": null,
        }))
    }
}

/// GET /api/v1/output/status - Get current output status
pub async fn output_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    if let Some(device) = engine.active_output_device() {
        Json(serde_json::json!({
            "active": true,
            "device": device.name,
            "device_type": device.device_type.to_string(),
            "channels": device.max_channels,
        }))
    } else {
        Json(serde_json::json!({
            "active": false,
            "device": null,
            "device_type": null,
            "channels": 0,
        }))
    }
}

/// GET /api/v1/transport/playback-status - Get playback status
pub async fn playback_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.read().await;
    let playback = &engine.playback;

    Json(serde_json::json!({
        "mode": format!("{:?}", engine.transport_mode),
        "file": playback.file_path.as_ref().map(|p| p.to_string_lossy()),
        "position": playback.playback_position,
        "total_samples": playback.total_samples,
        "sample_rate": playback.sample_rate,
        "transport_state": format!("{:?}", engine.transport_state),
    }))
}
