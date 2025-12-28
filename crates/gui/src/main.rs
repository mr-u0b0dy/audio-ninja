// SPDX-License-Identifier: Apache-2.0

//! Audio Ninja Desktop GUI - Tauri backend
//!
//! This GUI acts as a client to the audio-ninja-daemon service.
//! All audio processing happens in the daemon; the GUI provides
//! control and monitoring interfaces.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Daemon API client configuration
const DAEMON_URL: &str = "http://127.0.0.1:8080/api/v1";

// Application state
struct AppState {
    daemon_url: String,
    http_client: reqwest::Client,
}

impl AppState {
    fn new() -> Self {
        Self {
            daemon_url: DAEMON_URL.to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}

// Response types matching daemon API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeakerInfo {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub position: Option<SpeakerPosition>,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeakerPosition {
    pub azimuth: f32,
    pub elevation: f32,
    pub distance: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportStatus {
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationStatus {
    pub running: bool,
    pub progress: f32,
    pub measurements: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_speakers: usize,
    pub online_speakers: usize,
    pub transport_state: String,
    pub has_layout: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeakerStats {
    pub packets_sent: u64,
    pub packets_lost: u64,
    pub latency_ms: f32,
    pub jitter_ms: f32,
    pub buffer_fill: f32,
}

// Tauri commands - these call the daemon API

#[tauri::command]
async fn get_daemon_status(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<StatusResponse, String> {
    let app = state.read().await;
    let url = format!("{}/status", app.daemon_url);
    
    app.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_speakers(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<Vec<SpeakerInfo>, String> {
    let app = state.read().await;
    let url = format!("{}/speakers", app.daemon_url);
    
    app.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn discover_speakers(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app = state.read().await;
    let url = format!("{}/speakers/discover", app.daemon_url);
    
    app.http_client
        .post(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn set_layout(preset: String, state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app = state.read().await;
    let url = format!("{}/layout", app.daemon_url);
    
    let body = serde_json::json!({ "preset": preset });
    
    app.http_client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn transport_play(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app = state.read().await;
    let url = format!("{}/transport/play", app.daemon_url);
    
    app.http_client
        .post(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn transport_pause(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app = state.read().await;
    let url = format!("{}/transport/pause", app.daemon_url);
    
    app.http_client
        .post(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn transport_stop(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app = state.read().await;
    let url = format!("{}/transport/stop", app.daemon_url);
    
    app.http_client
        .post(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn get_transport_status(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<TransportStatus, String> {
    let app = state.read().await;
    let url = format!("{}/transport/status", app.daemon_url);
    
    app.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_calibration(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app = state.read().await;
    let url = format!("{}/calibration/start", app.daemon_url);
    
    app.http_client
        .post(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn get_calibration_status(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<CalibrationStatus, String> {
    let app = state.read().await;
    let url = format!("{}/calibration/status", app.daemon_url);
    
    app.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_stats(state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<StatsResponse, String> {
    let app = state.read().await;
    let url = format!("{}/stats", app.daemon_url);
    
    app.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_speaker_stats(id: String, state: tauri::State<'_, Arc<RwLock<AppState>>>) -> Result<SpeakerStats, String> {
    let app = state.read().await;
    let url = format!("{}/speakers/{}/stats", app.daemon_url, id);
    
    app.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(RwLock::new(AppState::new()));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_daemon_status,
            list_speakers,
            discover_speakers,
            set_layout,
            transport_play,
            transport_pause,
            transport_stop,
            get_transport_status,
            start_calibration,
            get_calibration_status,
            get_stats,
            get_speaker_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
