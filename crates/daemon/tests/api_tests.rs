// SPDX-License-Identifier: Apache-2.0

//! Integration tests for daemon REST API endpoints

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::util::ServiceExt; // for `oneshot`
use uuid::Uuid;

use audio_ninja_daemon::AppState;
use std::time::Instant;

/// Helper to create app with test state
fn create_test_app() -> Router {
    use audio_ninja_daemon::EngineState;
    use axum::routing::{delete, get, post};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let engine_state = EngineState::new();
    let app_state = AppState {
        engine: Arc::new(RwLock::new(engine_state)),
        started_at: Instant::now(),
    };

    Router::new()
        .route("/api/v1/status", get(audio_ninja_daemon::api::status))
        .route("/api/v1/info", get(audio_ninja_daemon::api::info))
        .route(
            "/api/v1/speakers",
            get(audio_ninja_daemon::api::list_speakers),
        )
        .route(
            "/api/v1/speakers/discover",
            post(audio_ninja_daemon::api::discover_speakers),
        )
        .route(
            "/api/v1/speakers/{id}",
            get(audio_ninja_daemon::api::get_speaker),
        )
        .route(
            "/api/v1/speakers/{id}",
            delete(audio_ninja_daemon::api::remove_speaker),
        )
        .route("/api/v1/layout", get(audio_ninja_daemon::api::get_layout))
        .route("/api/v1/layout", post(audio_ninja_daemon::api::set_layout))
        .route(
            "/api/v1/transport/play",
            post(audio_ninja_daemon::api::transport_play),
        )
        .route(
            "/api/v1/transport/pause",
            post(audio_ninja_daemon::api::transport_pause),
        )
        .route(
            "/api/v1/transport/stop",
            post(audio_ninja_daemon::api::transport_stop),
        )
        .route(
            "/api/v1/transport/status",
            get(audio_ninja_daemon::api::transport_status),
        )
        .route(
            "/api/v1/transport/load-file",
            post(audio_ninja_daemon::api::load_audio_file),
        )
        .route(
            "/api/v1/transport/playback-status",
            get(audio_ninja_daemon::api::playback_status),
        )
        .route(
            "/api/v1/calibration/start",
            post(audio_ninja_daemon::api::calibration_start),
        )
        .route(
            "/api/v1/calibration/status",
            get(audio_ninja_daemon::api::calibration_status),
        )
        .route(
            "/api/v1/calibration/apply",
            post(audio_ninja_daemon::api::calibration_apply),
        )
        .route("/api/v1/stats", get(audio_ninja_daemon::api::stats))
        .route(
            "/api/v1/stats/network",
            get(audio_ninja_daemon::api::stats_network),
        )
        .route(
            "/api/v1/stats/latency",
            get(audio_ninja_daemon::api::stats_latency),
        )
        .route(
            "/api/v1/stats/daemon",
            get(audio_ninja_daemon::api::stats_daemon),
        )
        .route(
            "/api/v1/stats/sync",
            get(audio_ninja_daemon::api::stats_sync),
        )
        .route(
            "/api/v1/stats/audio-levels",
            get(audio_ninja_daemon::api::stats_audio_levels),
        )
        .route(
            "/api/v1/speakers/{id}/stats",
            get(audio_ninja_daemon::api::speaker_stats),
        )
        .with_state(app_state)
}

/// Helper to parse JSON response body
async fn json_body(body: Body) -> Value {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_status_endpoint() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/status")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert_eq!(body["status"], "running");
    assert!(body["version"].is_string());
    assert!(body["uptime_secs"].is_number());
}

#[tokio::test]
async fn test_info_endpoint() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/info")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert_eq!(body["name"], "Audio Ninja Engine");
    assert!(body["features"].is_array());
    assert!(body["features"].as_array().unwrap().len() >= 4);
}

#[tokio::test]
async fn test_list_speakers_empty() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/speakers")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_discover_speakers() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/speakers/discover")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}

#[tokio::test]
async fn test_get_speaker_not_found() {
    let app = create_test_app();
    let id = Uuid::new_v4();

    let request = Request::builder()
        .uri(format!("/api/v1/speakers/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_remove_speaker_not_found() {
    let app = create_test_app();
    let id = Uuid::new_v4();

    let request = Request::builder()
        .method("DELETE")
        .uri(format!("/api/v1/speakers/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_layout_not_found() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/layout")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_set_layout_stereo() {
    let app = create_test_app();

    let layout_request = json!({
        "preset": "stereo"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_set_layout_5_1() {
    let app = create_test_app();

    let layout_request = json!({
        "preset": "5.1"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_set_layout_invalid_preset() {
    let app = create_test_app();

    let layout_request = json!({
        "preset": "invalid"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_set_layout_custom() {
    let app = create_test_app();

    let speaker_id = Uuid::new_v4();
    let layout_request = json!({
        "speakers": [
            {
                "speaker_id": speaker_id,
                "azimuth": 0.0,
                "elevation": 0.0,
                "distance": 1.0
            }
        ]
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Fetch and verify layout exists
    let request = Request::builder()
        .uri("/api/v1/layout")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert_eq!(body["name"], "custom");
    assert_eq!(body["speakers"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_transport_play() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/play")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_transport_pause() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/pause")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_transport_stop() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/stop")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_transport_status() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/transport/status")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["state"].is_string());
}

#[tokio::test]
async fn test_calibration_start() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/calibration/start")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}

#[tokio::test]
async fn test_calibration_status() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/calibration/status")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["running"].is_boolean());
    assert!(body["progress"].is_number());
    assert!(body["measurements"].is_number());
}

#[tokio::test]
async fn test_calibration_apply_flow() {
    let app = create_test_app();

    // Begin calibration session
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/calibration/start")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    // Apply calibration
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/calibration/apply")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify status reflects completion
    let request = Request::builder()
        .uri("/api/v1/calibration/status")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(!body["running"].as_bool().unwrap());
    assert!((body["progress"].as_f64().unwrap() - 1.0).abs() < f64::EPSILON);
    assert!(body["measurements"].as_u64().unwrap() >= 1);
}

#[tokio::test]
async fn test_calibration_apply_without_session() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/calibration/apply")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_stats() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["total_speakers"].is_number());
    assert!(body["online_speakers"].is_number());
    assert!(body["transport_state"].is_string());
    assert!(body["has_layout"].is_boolean());
}

#[tokio::test]
async fn test_speaker_stats_not_found() {
    let app = create_test_app();
    let id = Uuid::new_v4();

    let request = Request::builder()
        .uri(format!("/api/v1/speakers/{}/stats", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_layout_workflow() {
    let app = create_test_app();

    // Initially no layout
    let request = Request::builder()
        .uri("/api/v1/layout")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Set stereo layout
    let layout_request = json!({
        "preset": "stereo"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Now layout should exist
    let request = Request::builder()
        .uri("/api/v1/layout")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["speakers"].is_array());
}

#[tokio::test]
async fn test_transport_state_workflow() {
    let app = create_test_app();

    // Check initial state
    let request = Request::builder()
        .uri("/api/v1/transport/status")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response.into_body()).await;
    assert_eq!(body["state"], "Stopped");

    // Play
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/play")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Pause
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/pause")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Stop
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/stop")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_load_file_not_found() {
    let app = create_test_app();

    let load_request = json!({
        "file_path": "/nonexistent/file.wav"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/load-file")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&load_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_load_file_success() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Create minimal WAV file (44 bytes header)
    let mut temp = NamedTempFile::new().unwrap();
    // RIFF header
    temp.write_all(b"RIFF").unwrap();
    temp.write_all(&(36u32).to_le_bytes()).unwrap(); // file size - 8
    // WAVE header
    temp.write_all(b"WAVE").unwrap();
    // fmt subchunk
    temp.write_all(b"fmt ").unwrap();
    temp.write_all(&(16u32).to_le_bytes()).unwrap(); // subchunk1 size
    temp.write_all(&(1u16).to_le_bytes()).unwrap(); // audio format (PCM)
    temp.write_all(&(2u16).to_le_bytes()).unwrap(); // channels (stereo)
    temp.write_all(&(48000u32).to_le_bytes()).unwrap(); // sample rate
    temp.write_all(&(192000u32).to_le_bytes()).unwrap(); // byte rate
    temp.write_all(&(4u16).to_le_bytes()).unwrap(); // block align
    temp.write_all(&(16u16).to_le_bytes()).unwrap(); // bits per sample
    // data chunk
    temp.write_all(b"data").unwrap();
    temp.write_all(&(0u32).to_le_bytes()).unwrap(); // data size
    temp.flush().unwrap();

    let app = create_test_app();
    let load_request = json!({
        "file_path": temp.path().to_string_lossy().to_string()
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/load-file")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&load_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert_eq!(body["success"], true);
    assert!(body["file"].is_string());
    assert_eq!(body["status"], "loaded");
}

#[tokio::test]
async fn test_playback_status_after_load() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Create minimal WAV file
    let mut temp = NamedTempFile::new().unwrap();
    temp.write_all(b"RIFF").unwrap();
    temp.write_all(&(36u32).to_le_bytes()).unwrap();
    temp.write_all(b"WAVE").unwrap();
    temp.write_all(b"fmt ").unwrap();
    temp.write_all(&(16u32).to_le_bytes()).unwrap();
    temp.write_all(&(1u16).to_le_bytes()).unwrap(); // PCM
    temp.write_all(&(2u16).to_le_bytes()).unwrap(); // stereo
    temp.write_all(&(48000u32).to_le_bytes()).unwrap(); // 48kHz
    temp.write_all(&(192000u32).to_le_bytes()).unwrap();
    temp.write_all(&(4u16).to_le_bytes()).unwrap();
    temp.write_all(&(16u16).to_le_bytes()).unwrap();
    // data chunk
    temp.write_all(b"data").unwrap();
    temp.write_all(&(0u32).to_le_bytes()).unwrap();
    temp.flush().unwrap();

    let app = create_test_app();

    // Load file
    let load_request = json!({
        "file_path": temp.path().to_string_lossy().to_string()
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/load-file")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&load_request).unwrap()))
        .unwrap();

    let _response = app.clone().oneshot(request).await.unwrap();

    // Get playback status
    let request = Request::builder()
        .uri("/api/v1/transport/playback-status")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["file"].is_string());
    assert_eq!(body["sample_rate"], 48000);
    assert!(body["total_samples"].is_number());
}

#[tokio::test]
async fn test_load_wav_file() {
    use std::io::Write;

    let app = create_test_app();

    // Create a minimal valid WAV file
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let wav_header = [
        // RIFF header
        b'R', b'I', b'F', b'F',
        0x24, 0x00, 0x00, 0x00, // file size - 8
        b'W', b'A', b'V', b'E',
        // fmt chunk
        b'f', b'm', b't', b' ',
        0x10, 0x00, 0x00, 0x00, // fmt chunk size
        0x01, 0x00,             // audio format (PCM)
        0x02, 0x00,             // channels (2)
        0x80, 0xBB, 0x00, 0x00, // sample rate (48000)
        0x00, 0xEE, 0x02, 0x00, // byte rate
        0x04, 0x00,             // block align
        0x10, 0x00,             // bits per sample (16)
        // data chunk
        b'd', b'a', b't', b'a',
        0x00, 0x00, 0x00, 0x00, // data size
    ];
    temp_file.write_all(&wav_header).unwrap();
    temp_file.flush().unwrap();

    let file_path = temp_file.path().to_str().unwrap();

    let load_request = json!({
        "file_path": file_path
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/load-file")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&load_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify playback status reflects parsed metadata
    let request = Request::builder()
        .uri("/api/v1/transport/playback-status")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = json_body(response.into_body()).await;
    assert_eq!(body["sample_rate"], 48000);
    assert!(body["total_samples"].is_number());
}

#[tokio::test]
async fn test_load_invalid_file() {
    let app = create_test_app();

    let load_request = json!({
        "file_path": "/nonexistent/file.wav"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/load-file")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&load_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_load_unsupported_format() {
    use std::io::Write;

    let app = create_test_app();

    // Create a file with unknown format
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    temp_file.write_all(b"JUNKDATAINVALIDFORMAT").unwrap();
    temp_file.flush().unwrap();

    let file_path = temp_file.path().to_str().unwrap();

    let load_request = json!({
        "file_path": file_path
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transport/load-file")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&load_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // For unrecognized formats, the daemon falls back to heuristic estimation
    // so it returns OK with estimated metadata
    assert_eq!(response.status(), StatusCode::OK);
}

// ===== New Stats Sub-Endpoint Tests =====

#[tokio::test]
async fn test_stats_network() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/stats/network")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["sent_kbps"].is_number());
    assert!(body["received_kbps"].is_number());
    assert!(body["peak_sent_kbps"].is_number());
    assert!(body["packets_sent"].is_number());
}

#[tokio::test]
async fn test_stats_latency() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/stats/latency")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["min_ms"].is_number());
    assert!(body["max_ms"].is_number());
    assert!(body["mean_ms"].is_number());
    assert!(body["stddev_ms"].is_number());
    assert!(body["samples"].is_array());
    assert!(body["speaker_count"].is_number());
}

#[tokio::test]
async fn test_stats_daemon() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/stats/daemon")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["cpu_percent"].is_number());
    assert!(body["memory_mb"].is_number());
    assert!(body["uptime_secs"].is_number());
    assert!(body["pid"].is_number());
    assert!(body["version"].is_string());
}

#[tokio::test]
async fn test_stats_sync() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/stats/sync")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["speakers"].is_array());
    assert!(body["overall_status"].is_string());
    assert_eq!(body["overall_status"], "no_speakers");
}

#[tokio::test]
async fn test_stats_audio_levels() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/api/v1/stats/audio-levels")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = json_body(response.into_body()).await;
    assert!(body["input_db_left"].is_number());
    assert!(body["input_db_right"].is_number());
    assert!(body["output_db_left"].is_number());
    assert!(body["output_db_right"].is_number());
    assert!(body["per_speaker_db"].is_array());
    assert!(body["clipping"].is_boolean());
}

#[tokio::test]
async fn test_set_layout_7_1() {
    let app = create_test_app();

    let layout_request = json!({
        "preset": "7.1"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_set_layout_7_1_4() {
    let app = create_test_app();

    let layout_request = json!({
        "preset": "7.1.4"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify layout has correct speaker count (7.1 = 8 + 4 height = 12)
    let request = Request::builder()
        .uri("/api/v1/layout")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response.into_body()).await;
    assert_eq!(body["speakers"].as_array().unwrap().len(), 12);
}

#[tokio::test]
async fn test_set_layout_9_1_6() {
    let app = create_test_app();

    let layout_request = json!({
        "preset": "9.1.6"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/layout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify layout: 7.1.4 = 12 + 2 wide + 2 top side = 16
    let request = Request::builder()
        .uri("/api/v1/layout")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response.into_body()).await;
    assert_eq!(body["speakers"].as_array().unwrap().len(), 16);
}

#[tokio::test]
async fn test_set_layout_all_presets() {
    // Verify all supported presets are accepted
    let presets = vec!["stereo", "2.0", "2.1", "3.1", "4.0", "5.1", "5.1.2", "7.1", "7.1.4", "9.1.6"];

    for preset in presets {
        let app = create_test_app();

        let layout_request = json!({ "preset": preset });

        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/layout")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&layout_request).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Layout preset '{}' should be accepted",
            preset
        );
    }
}
