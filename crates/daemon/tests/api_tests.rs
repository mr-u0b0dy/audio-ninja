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

/// Helper to create app with test state
fn create_test_app() -> Router {
    use audio_ninja_daemon::EngineState;
    use axum::routing::{delete, get, post};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let engine_state = EngineState::new();
    let app_state = AppState {
        engine: Arc::new(RwLock::new(engine_state)),
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
            "/api/v1/speakers/:id",
            get(audio_ninja_daemon::api::get_speaker),
        )
        .route(
            "/api/v1/speakers/:id",
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
            "/api/v1/speakers/:id/stats",
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
async fn test_calibration_apply_not_implemented() {
    let app = create_test_app();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/calibration/apply")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
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
