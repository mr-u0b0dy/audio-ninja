// SPDX-License-Identifier: Apache-2.0

//! Audio Ninja Daemon - Background service for audio engine
//!
//! This daemon runs the core audio engine as a system service and exposes
//! a REST API for control and monitoring by GUI clients or CLI tools.

use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use audio_ninja_daemon::{api, engine::EngineState, AppState};

#[derive(Parser, Debug)]
#[command(name = "audio-ninja-daemon")]
#[command(about = "Audio Ninja background engine daemon", long_about = None)]
struct Args {
    /// HTTP API port
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Bind address
    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    let level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!(
            "audio_ninja_daemon={},audio_ninja={}",
            level, level
        ))
        .init();

    info!("Audio Ninja Daemon starting...");

    // Initialize engine state
    let engine_state = EngineState::new();
    let app_state = AppState {
        engine: Arc::new(RwLock::new(engine_state)),
    };

    // Build REST API routes
    let app = Router::new()
        // Status and info
        .route("/api/v1/status", get(api::status))
        .route("/api/v1/info", get(api::info))
        // Speaker management
        .route("/api/v1/speakers", get(api::list_speakers))
        .route("/api/v1/speakers/discover", post(api::discover_speakers))
        .route("/api/v1/speakers/{id}", get(api::get_speaker))
        .route("/api/v1/speakers/{id}", delete(api::remove_speaker))
        // Layout configuration
        .route("/api/v1/layout", get(api::get_layout))
        .route("/api/v1/layout", post(api::set_layout))
        // Transport control
        .route("/api/v1/transport/play", post(api::transport_play))
        .route("/api/v1/transport/pause", post(api::transport_pause))
        .route("/api/v1/transport/stop", post(api::transport_stop))
        .route("/api/v1/transport/status", get(api::transport_status))
        // Calibration
        .route("/api/v1/calibration/start", post(api::calibration_start))
        .route("/api/v1/calibration/status", get(api::calibration_status))
        .route("/api/v1/calibration/apply", post(api::calibration_apply))
        // Statistics and monitoring
        .route("/api/v1/stats", get(api::stats))
        .route("/api/v1/speakers/{id}/stats", get(api::speaker_stats))
        // Add CORS middleware
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state);

    // Start server
    let addr: SocketAddr = format!("{}:{}", args.bind, args.port).parse()?;
    info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
