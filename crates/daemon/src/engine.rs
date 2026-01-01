// SPDX-License-Identifier: Apache-2.0

//! Engine state management

use audio_ninja::{
    input::{InputManager, InputSource},
    network::SpeakerDiscovery,
    output::{OutputDevice, OutputManager},
    SpeakerLayout,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
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

/// Transport mode: what input source(s) are being used
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransportMode {
    /// Playing from loaded audio file
    FilePlayback,

    /// Streaming from live input source
    LiveStream,

    /// Both file playback and live stream active (mixed)
    Mixed,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    /// Currently loaded audio file path
    pub file_path: Option<PathBuf>,

    /// Current playback position in samples
    pub playback_position: u64,

    /// Total samples in current file
    pub total_samples: u64,

    /// Playback sample rate
    pub sample_rate: u32,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            file_path: None,
            playback_position: 0,
            total_samples: 0,
            sample_rate: 48000,
        }
    }
}

pub struct EngineState {
    pub speakers: HashMap<Uuid, SpeakerInfo>,
    pub layout: Option<SpeakerLayout>,
    pub transport_state: TransportState,
    pub transport_mode: TransportMode,
    pub playback: PlaybackState,
    pub speaker_stats: HashMap<Uuid, SpeakerStats>,
    pub calibration: CalibrationState,
    discovery: Option<SpeakerDiscovery>,

    // Audio I/O managers
    pub input_manager: InputManager,
    pub output_manager: OutputManager,
    pub active_input_source: Option<InputSource>,
    pub active_output_device: Option<OutputDevice>,
}

impl Default for EngineState {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            speakers: HashMap::new(),
            layout: None,
            transport_state: TransportState::Stopped,
            transport_mode: TransportMode::FilePlayback,
            playback: PlaybackState::default(),
            speaker_stats: HashMap::new(),
            calibration: CalibrationState {
                running: false,
                progress: 0.0,
                measurements: Vec::new(),
            },
            discovery: None,
            input_manager: InputManager::new(),
            output_manager: OutputManager::new(),
            active_input_source: None,
            active_output_device: None,
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

    // ===== Audio I/O Methods =====

    /// Load audio file for playback
    pub fn load_audio_file(&mut self, file_path: &str) -> Result<(), String> {
        let path = PathBuf::from(file_path);
        if !path.exists() {
            return Err(format!("File not found: {}", file_path));
        }

        self.playback.file_path = Some(path);
        self.playback.playback_position = 0;
        // Note: total_samples would be set after actual file parsing
        Ok(())
    }

    /// Set transport mode (file-only, stream-only, or mixed)
    pub fn set_transport_mode(&mut self, mode: TransportMode) {
        self.transport_mode = mode;
    }

    /// Get current transport mode
    pub fn transport_mode(&self) -> TransportMode {
        self.transport_mode.clone()
    }

    /// Enumerate all input devices
    pub fn enumerate_input_devices(&mut self) -> Result<Vec<String>, String> {
        self.input_manager
            .enumerate_devices()
            .map_err(|e| e.to_string())
            .map(|devices| {
                devices
                    .iter()
                    .map(|d| format!("{} ({})", d.name, d.device_type))
                    .collect()
            })
    }

    /// Enumerate all output devices
    pub fn enumerate_output_devices(&mut self) -> Result<Vec<String>, String> {
        self.output_manager
            .enumerate_devices()
            .map_err(|e| e.to_string())
            .map(|devices| {
                devices
                    .iter()
                    .map(|d| format!("{} ({})", d.name, d.device_type))
                    .collect()
            })
    }

    /// Select input source (system audio, external device, or app)
    pub fn select_input_source(&mut self, source_id: &str) -> Result<InputSource, String> {
        let source = match source_id {
            "system" => self
                .input_manager
                .select_system_audio()
                .map_err(|e| e.to_string())?,
            device_id => self
                .input_manager
                .select_external_device(device_id)
                .map_err(|e| e.to_string())?,
        };

        self.active_input_source = Some(source.clone());
        Ok(source)
    }

    /// Select output device (speaker or headphones)
    pub fn select_output_device(&mut self, device_id: &str) -> Result<OutputDevice, String> {
        let device = self
            .output_manager
            .select_device(device_id)
            .map_err(|e| e.to_string())?;

        self.active_output_device = Some(device.clone());
        Ok(device)
    }

    /// Get active input source
    pub fn active_input_source(&self) -> Option<&InputSource> {
        self.active_input_source.as_ref()
    }

    /// Get active output device
    pub fn active_output_device(&self) -> Option<&OutputDevice> {
        self.active_output_device.as_ref()
    }
}
