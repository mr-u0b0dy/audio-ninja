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
        // Placeholder: real implementation will use ALSA/PulseAudio APIs
        // For now, return mock devices
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
