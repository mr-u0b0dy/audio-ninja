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
use audio_ninja::SpeakerLayout;

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
    #[allow(dead_code)]
    speakers: Option<Vec<SpeakerPositionRequest>>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
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
pub async fn status(State(_state): State<AppState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_secs: 0, // TODO: track actual uptime
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
        match preset.as_str() {
            "stereo" => SpeakerLayout::stereo(),
            "5.1" => SpeakerLayout::surround_5_1(),
            _ => return StatusCode::BAD_REQUEST,
        }
    } else {
        // Custom layout from speaker positions
        return StatusCode::NOT_IMPLEMENTED;
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
pub async fn calibration_apply(State(_state): State<AppState>) -> StatusCode {
    // TODO: Apply calibration results
    StatusCode::NOT_IMPLEMENTED
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
                    name: parts.get(0).map(|s| s.to_string()).unwrap_or_default(),
                    device_type: parts.get(1)
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
                    name: parts.get(0).map(|s| s.to_string()).unwrap_or_default(),
                    device_type: parts.get(1)
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