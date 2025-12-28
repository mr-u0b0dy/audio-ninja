// SPDX-License-Identifier: Apache-2.0

//! Engine state management

use audio_ninja::{
    network::SpeakerDiscovery,
    SpeakerLayout,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerInfo {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub position: Option<SpeakerPosition>,
    pub online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerPosition {
    pub azimuth: f32,
    pub elevation: f32,
    pub distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerStats {
    pub packets_sent: u64,
    pub packets_lost: u64,
    pub latency_ms: f32,
    pub jitter_ms: f32,
    pub buffer_fill: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationState {
    pub running: bool,
    pub progress: f32,
    pub measurements: Vec<String>,
}

pub struct EngineState {
    pub speakers: HashMap<Uuid, SpeakerInfo>,
    pub layout: Option<SpeakerLayout>,
    pub transport_state: TransportState,
    pub speaker_stats: HashMap<Uuid, SpeakerStats>,
    pub calibration: CalibrationState,
    discovery: Option<SpeakerDiscovery>,
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            speakers: HashMap::new(),
            layout: None,
            transport_state: TransportState::Stopped,
            speaker_stats: HashMap::new(),
            calibration: CalibrationState {
                running: false,
                progress: 0.0,
                measurements: Vec::new(),
            },
            discovery: None,
        }
    }

    pub fn start_discovery(&mut self) {
        if self.discovery.is_none() {
            self.discovery = Some(SpeakerDiscovery::new());
        }
    }

    pub fn add_speaker(&mut self, speaker: SpeakerInfo) {
        self.speakers.insert(speaker.id, speaker);
    }

    pub fn remove_speaker(&mut self, id: &Uuid) -> Option<SpeakerInfo> {
        self.speakers.remove(id)
    }

    pub fn set_layout(&mut self, layout: SpeakerLayout) {
        self.layout = Some(layout);
    }

    pub fn play(&mut self) {
        self.transport_state = TransportState::Playing;
    }

    pub fn pause(&mut self) {
        self.transport_state = TransportState::Paused;
    }

    pub fn stop(&mut self) {
        self.transport_state = TransportState::Stopped;
    }

    pub fn start_calibration(&mut self) {
        self.calibration.running = true;
        self.calibration.progress = 0.0;
    }

    pub fn update_stats(&mut self, speaker_id: Uuid, stats: SpeakerStats) {
        self.speaker_stats.insert(speaker_id, stats);
    }
}
