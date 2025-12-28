// SPDX-License-Identifier: Apache-2.0

//! REST API handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, engine::{SpeakerInfo, SpeakerPosition, SpeakerStats}};
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
pub async fn remove_speaker(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
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
    engine
        .layout
        .clone()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
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
pub async fn calibration_apply(State(state): State<AppState>) -> StatusCode {
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
