// SPDX-License-Identifier: Apache-2.0

//! Audio output abstraction for playback to speakers and headphones
//!
//! Supports flexible output routing to:
//! - Built-in speaker(s)
//! - Headphones (3.5mm jack, Bluetooth, USB)
//! - External speaker systems (USB, HDMI, etc.)
//! - Network audio (future)
//!
//! Architecture:
//! - `OutputDevice`: Device information and capabilities
//! - `PlaybackStream`: Trait for implementing playback backends (ALSA, PulseAudio)
//! - `OutputManager`: Main interface for device enumeration and stream setup

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// Output device errors
#[derive(Debug, Error)]
pub enum OutputError {
    #[error("device not found: {0}")]
    DeviceNotFound(String),

    #[error("device busy: {0}")]
    DeviceBusy(String),

    #[error("invalid format: {0}")]
    InvalidFormat(String),

    #[error("playback failed: {0}")]
    PlaybackFailed(String),

    #[error("backend error: {0}")]
    BackendError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Audio device type classification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// Built-in speaker or laptop speaker
    Speaker,

    /// Headphones (wired or Bluetooth)
    Headphones,

    /// Line-out/analog audio output
    LineOut,

    /// USB audio device
    Usb,

    /// HDMI audio output
    Hdmi,

    /// Network audio device
    Network,

    /// Other/unknown device type
    Other,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Speaker => write!(f, "speaker"),
            DeviceType::Headphones => write!(f, "headphones"),
            DeviceType::LineOut => write!(f, "line-out"),
            DeviceType::Usb => write!(f, "usb"),
            DeviceType::Hdmi => write!(f, "hdmi"),
            DeviceType::Network => write!(f, "network"),
            DeviceType::Other => write!(f, "other"),
        }
    }
}

/// Audio output device information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputDevice {
    /// Unique device identifier
    pub id: String,

    /// Human-readable device name
    pub name: String,

    /// Device type (speaker, headphones, etc.)
    pub device_type: DeviceType,

    /// Maximum output channels supported
    pub max_channels: u32,

    /// Supported sample rates (Hz)
    pub sample_rates: Vec<u32>,

    /// Is this device currently available/connected
    pub available: bool,

    /// Default sample rate for this device
    pub default_sample_rate: u32,

    /// Whether this is the default output device
    pub is_default: bool,
}

impl OutputDevice {
    pub fn new(
        id: String,
        name: String,
        device_type: DeviceType,
        max_channels: u32,
        default_sample_rate: u32,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            max_channels,
            sample_rates: vec![48000, 44100, 96000, 192000],
            available: true,
            default_sample_rate,
            is_default: false,
        }
    }

    pub fn with_default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }
}

/// Playback callback type: requests PCM frames \[channel\]\[sample\]
pub type PlaybackCallback = Arc<dyn Fn(u32) -> Vec<Vec<f32>> + Send + Sync>;

/// Trait for audio playback implementations (ALSA, PulseAudio backends)
pub trait PlaybackStream: Send + Sync {
    /// Start playback
    fn start(&mut self) -> Result<(), OutputError>;

    /// Stop playback
    fn stop(&mut self) -> Result<(), OutputError>;

    /// Check if stream is currently active
    fn is_running(&self) -> bool;

    /// Get current stream sample rate
    fn sample_rate(&self) -> u32;

    /// Get number of output channels
    fn channels(&self) -> u32;

    /// Get latency in milliseconds
    fn latency_ms(&self) -> f32;

    /// Write audio frames to device \[channel\]\[sample\]
    fn write(&mut self, data: &[Vec<f32>]) -> Result<(), OutputError>;
}

/// Playback stream status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackStatus {
    pub is_running: bool,
    pub sample_rate: u32,
    pub channels: u32,
    pub latency_ms: f32,
    pub frames_written: u64,
}

/// Output manager for device enumeration and stream setup
pub struct OutputManager {
    devices: Vec<OutputDevice>,
    active_device: Option<OutputDevice>,
}

impl OutputManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            active_device: None,
        }
    }

    /// Enumerate all available output devices
    pub fn enumerate_devices(&mut self) -> Result<Vec<OutputDevice>, OutputError> {
        #[cfg(feature = "audio-backends")]
        {
            self.devices = cpal_enumerate_output_devices()?;
            return Ok(self.devices.clone());
        }
        #[cfg(not(feature = "audio-backends"))]
        {
            // Mock devices for testing and CI builds
            let devices = vec![
                OutputDevice::new(
                    "speaker".to_string(),
                    "Built-in Speaker".to_string(),
                    DeviceType::Speaker,
                    2,
                    48000,
                )
                .with_default(true),
                OutputDevice::new(
                    "headphones".to_string(),
                    "Headphone Jack".to_string(),
                    DeviceType::Headphones,
                    2,
                    48000,
                ),
                OutputDevice::new(
                    "hdmi".to_string(),
                    "HDMI Audio".to_string(),
                    DeviceType::Hdmi,
                    8,
                    48000,
                ),
            ];

            self.devices = devices;
            Ok(self.devices.clone())
        }
    }

    /// Get devices of a specific type
    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<OutputDevice> {
        self.devices
            .iter()
            .filter(|d| d.device_type == device_type)
            .cloned()
            .collect()
    }

    /// Get list of all available devices
    pub fn get_all_devices(&self) -> Vec<OutputDevice> {
        self.devices.clone()
    }

    /// Find device by ID
    pub fn find_device(&self, device_id: &str) -> Option<OutputDevice> {
        self.devices.iter().find(|d| d.id == device_id).cloned()
    }

    /// Get default output device
    pub fn get_default_device(&self) -> Option<OutputDevice> {
        self.devices
            .iter()
            .find(|d| d.is_default)
            .cloned()
            .or_else(|| self.devices.first().cloned())
    }

    /// Select output device
    pub fn select_device(&mut self, device_id: &str) -> Result<OutputDevice, OutputError> {
        let device = self
            .find_device(device_id)
            .ok_or_else(|| OutputError::DeviceNotFound(device_id.to_string()))?;

        if !device.available {
            return Err(OutputError::DeviceNotFound(format!(
                "Device {} is not available",
                device_id
            )));
        }

        self.active_device = Some(device.clone());
        Ok(device)
    }

    /// Get currently active output device
    pub fn active_device(&self) -> Option<&OutputDevice> {
        self.active_device.as_ref()
    }

    /// Clear active output device
    pub fn clear_device(&mut self) {
        self.active_device = None;
    }

    /// Check if headphones are connected
    pub fn has_headphones(&self) -> bool {
        self.get_devices_by_type(DeviceType::Headphones)
            .iter()
            .any(|d| d.available)
    }

    /// Check if speakers are available
    pub fn has_speakers(&self) -> bool {
        self.get_devices_by_type(DeviceType::Speaker)
            .iter()
            .any(|d| d.available)
    }
}

impl Default for OutputManager {
    fn default() -> Self {
        Self::new()
    }
}

// ===== cpal Backend for Real Device Enumeration =====
#[cfg(feature = "audio-backends")]
fn cpal_enumerate_output_devices() -> Result<Vec<OutputDevice>, OutputError> {
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();
    let mut devices = Vec::new();

    // Try to identify the default output device
    let default_name = host
        .default_output_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    let output_devices = host
        .output_devices()
        .map_err(|e| OutputError::BackendError(format!("Failed to enumerate devices: {}", e)))?;

    for (idx, device) in output_devices.enumerate() {
        let name = device
            .name()
            .unwrap_or_else(|_| format!("Output Device {}", idx));

        let config = device.default_output_config().ok();
        let (channels, sample_rate) = config
            .map(|c| (c.channels() as u32, c.sample_rate().0))
            .unwrap_or((2, 48000));

        let name_lower = name.to_lowercase();
        let device_type = if name_lower.contains("hdmi") {
            DeviceType::Hdmi
        } else if name_lower.contains("headphone") || name_lower.contains("headset") {
            DeviceType::Headphones
        } else if name_lower.contains("usb") {
            DeviceType::Usb
        } else if name_lower.contains("line") {
            DeviceType::LineOut
        } else if name_lower.contains("speaker") || name_lower.contains("analog") {
            DeviceType::Speaker
        } else {
            DeviceType::Other
        };

        let is_default = name == default_name;

        let mut supported_rates = vec![];
        if let Ok(configs) = device.supported_output_configs() {
            for cfg in configs {
                for rate in &[44100u32, 48000, 96000, 192000] {
                    if *rate >= cfg.min_sample_rate().0 && *rate <= cfg.max_sample_rate().0 {
                        if !supported_rates.contains(rate) {
                            supported_rates.push(*rate);
                        }
                    }
                }
            }
        }
        if supported_rates.is_empty() {
            supported_rates = vec![48000, 44100, 96000];
        }

        let mut output_device = OutputDevice::new(
            format!("output_{}", idx),
            name,
            device_type,
            channels,
            sample_rate,
        )
        .with_default(is_default);
        output_device.sample_rates = supported_rates;
        devices.push(output_device);
    }

    // If no device was marked as default, mark the first one
    if !devices.is_empty() && !devices.iter().any(|d| d.is_default) {
        devices[0].is_default = true;
    }

    Ok(devices)
}

/// Wrapper to make cpal::Stream usable across thread boundaries.
/// Safety: we only interact with the stream via play/pause/drop, all of which
/// are internally synchronized by cpal.
#[cfg(feature = "audio-backends")]
#[allow(dead_code)]
struct SendSyncStream(cpal::Stream);
#[cfg(feature = "audio-backends")]
unsafe impl Send for SendSyncStream {}
#[cfg(feature = "audio-backends")]
unsafe impl Sync for SendSyncStream {}

/// PlaybackStream implementation backed by cpal
#[cfg(feature = "audio-backends")]
pub struct CpalPlaybackStream {
    stream: Option<SendSyncStream>,
    sample_rate: u32,
    channels: u32,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// Ring buffer that `write()` pushes into and the audio callback drains
    ring: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<f32>>>,
    latency_ms: f32,
}

#[cfg(feature = "audio-backends")]
impl CpalPlaybackStream {
    /// Create a playback stream for the default output device
    pub fn new(sample_rate: u32, channels: u32) -> Result<Self, OutputError> {
        use cpal::traits::HostTrait;

        let host = cpal::default_host();
        let _device = host.default_output_device().ok_or_else(|| {
            OutputError::DeviceNotFound("no default output device".to_string())
        })?;

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ring = std::sync::Arc::new(std::sync::Mutex::new(
            std::collections::VecDeque::with_capacity(sample_rate as usize * channels as usize),
        ));

        Ok(Self {
            stream: None,
            sample_rate,
            channels,
            running,
            ring,
            latency_ms: 256.0 / sample_rate as f32 * 1000.0,
        })
    }

    /// Create a playback stream for a specific device by index
    pub fn for_device(
        device_index: usize,
        sample_rate: u32,
        channels: u32,
    ) -> Result<Self, OutputError> {
        use cpal::traits::HostTrait;

        let host = cpal::default_host();
        let _device = host
            .output_devices()
            .map_err(|e| OutputError::BackendError(format!("enumerate: {e}")))?
            .nth(device_index)
            .ok_or_else(|| {
                OutputError::DeviceNotFound(format!("device index {} not found", device_index))
            })?;

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ring = std::sync::Arc::new(std::sync::Mutex::new(
            std::collections::VecDeque::with_capacity(sample_rate as usize * channels as usize),
        ));

        Ok(Self {
            stream: None,
            sample_rate,
            channels,
            running,
            ring,
            latency_ms: 256.0 / sample_rate as f32 * 1000.0,
        })
    }
}

#[cfg(feature = "audio-backends")]
impl PlaybackStream for CpalPlaybackStream {
    fn start(&mut self) -> Result<(), OutputError> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use cpal::{SampleRate, StreamConfig};

        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| OutputError::DeviceNotFound("no default output device".to_string()))?;

        let config = StreamConfig {
            channels: self.channels as u16,
            sample_rate: SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let ring = self.ring.clone();
        let running = self.running.clone();

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if !running.load(std::sync::atomic::Ordering::Relaxed) {
                        // Fill with silence when not running
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                        return;
                    }

                    if let Ok(mut buf) = ring.lock() {
                        for sample in data.iter_mut() {
                            *sample = buf.pop_front().unwrap_or(0.0);
                        }
                    } else {
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                    }
                },
                |err| {
                    eprintln!("[audio-ninja] playback error: {}", err);
                },
                None,
            )
            .map_err(|e| OutputError::PlaybackFailed(format!("build stream: {e}")))?;

        stream
            .play()
            .map_err(|e| OutputError::PlaybackFailed(format!("play: {e}")))?;

        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.stream = Some(SendSyncStream(stream));
        Ok(())
    }

    fn stop(&mut self) -> Result<(), OutputError> {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.stream = None;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u32 {
        self.channels
    }

    fn latency_ms(&self) -> f32 {
        self.latency_ms
    }

    fn write(&mut self, data: &[Vec<f32>]) -> Result<(), OutputError> {
        if data.is_empty() {
            return Ok(());
        }

        let frame_count = data[0].len();
        let ch_count = data.len();

        // Interleave channels into the ring buffer
        let mut ring = self
            .ring
            .lock()
            .map_err(|_| OutputError::PlaybackFailed("lock poisoned".to_string()))?;

        // Cap ring buffer to ~1 second to avoid unbounded growth
        let max_samples = self.sample_rate as usize * self.channels as usize;
        let incoming = frame_count * ch_count;
        if ring.len() + incoming > max_samples {
            let drain = (ring.len() + incoming) - max_samples;
            ring.drain(..drain);
        }

        for frame in 0..frame_count {
            for ch in 0..ch_count {
                ring.push_back(data[ch].get(frame).copied().unwrap_or(0.0));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_device_creation() {
        let device = OutputDevice::new(
            "hw:0".to_string(),
            "Built-in Speaker".to_string(),
            DeviceType::Speaker,
            2,
            48000,
        );

        assert_eq!(device.id, "hw:0");
        assert_eq!(device.name, "Built-in Speaker");
        assert_eq!(device.device_type, DeviceType::Speaker);
        assert_eq!(device.max_channels, 2);
        assert!(device.available);
    }

    #[test]
    fn test_output_device_type_display() {
        assert_eq!(DeviceType::Speaker.to_string(), "speaker");
        assert_eq!(DeviceType::Headphones.to_string(), "headphones");
        assert_eq!(DeviceType::Hdmi.to_string(), "hdmi");
    }

    #[test]
    fn test_output_manager_enumerate() {
        let mut manager = OutputManager::new();
        let devices = manager.enumerate_devices().unwrap();

        assert!(!devices.is_empty());
        assert!(devices.iter().any(|d| d.device_type == DeviceType::Speaker));
    }

    #[test]
    fn test_output_manager_get_devices_by_type() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();

        let speakers = manager.get_devices_by_type(DeviceType::Speaker);
        assert!(!speakers.is_empty());

        let headphones = manager.get_devices_by_type(DeviceType::Headphones);
        assert!(!headphones.is_empty());
    }

    #[test]
    fn test_output_manager_default_device() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();

        let default = manager.get_default_device();
        assert!(default.is_some());
        assert!(default.unwrap().is_default);
    }

    #[test]
    fn test_output_manager_select_device() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();

        let device = manager.select_device("speaker").unwrap();
        assert_eq!(device.id, "speaker");
        assert!(manager.active_device().is_some());
    }

    #[test]
    fn test_output_manager_select_invalid() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();

        let result = manager.select_device("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_output_manager_has_headphones() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();

        assert!(manager.has_headphones());
    }

    #[test]
    fn test_output_manager_has_speakers() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();

        assert!(manager.has_speakers());
    }

    #[test]
    fn test_output_manager_clear_device() {
        let mut manager = OutputManager::new();
        manager.enumerate_devices().unwrap();
        manager.select_device("speaker").unwrap();

        assert!(manager.active_device().is_some());

        manager.clear_device();
        assert!(manager.active_device().is_none());
    }
}
