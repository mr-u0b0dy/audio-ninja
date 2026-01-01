// SPDX-License-Identifier: Apache-2.0

//! Audio input abstraction for capturing from multiple sources
//!
//! Supports three primary input sources:
//! 1. **System Audio**: Virtual loopback device capturing all system audio
//! 2. **Application Audio**: App-specific audio routing (deferred to Phase 2)
//! 3. **External Devices**: Microphones, line-in, USB devices, etc.
//!
//! Architecture:
//! - `InputDevice`: Device information and capabilities
//! - `InputSource`: Enum representing the three input source types
//! - `CaptureStream`: Trait for implementing capture backends (ALSA, PulseAudio)
//! - `InputManager`: Main interface for device enumeration and stream setup

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// Input device errors
#[derive(Debug, Error)]
pub enum InputError {
    #[error("device not found: {0}")]
    DeviceNotFound(String),

    #[error("device busy: {0}")]
    DeviceBusy(String),

    #[error("invalid format: {0}")]
    InvalidFormat(String),

    #[error("capture failed: {0}")]
    CaptureFailed(String),

    #[error("backend error: {0}")]
    BackendError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Input source for audio capture
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InputSource {
    /// System audio loopback device (captures all system audio)
    System {
        device_name: String,
    },

    /// Application-specific audio routing (Phase 2)
    Application {
        app_name: String,
        device_name: String,
    },

    /// External device (microphone, line-in, USB device, etc.)
    External {
        device_name: String,
        device_id: String,
    },
}

impl InputSource {
    pub fn device_name(&self) -> &str {
        match self {
            InputSource::System { device_name } => device_name,
            InputSource::Application { device_name, .. } => device_name,
            InputSource::External { device_name, .. } => device_name,
        }
    }

    pub fn source_type(&self) -> &str {
        match self {
            InputSource::System { .. } => "system",
            InputSource::Application { .. } => "application",
            InputSource::External { .. } => "external",
        }
    }
}

/// Audio input device information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDevice {
    /// Unique device identifier
    pub id: String,

    /// Human-readable device name
    pub name: String,

    /// Device type (e.g., "microphone", "line-in", "usb-audio", "loopback")
    pub device_type: String,

    /// Maximum input channels supported
    pub max_channels: u32,

    /// Supported sample rates (Hz)
    pub sample_rates: Vec<u32>,

    /// Is this device currently available/connected
    pub available: bool,

    /// Default sample rate for this device
    pub default_sample_rate: u32,
}

impl InputDevice {
    pub fn new(
        id: String,
        name: String,
        device_type: String,
        max_channels: u32,
        default_sample_rate: u32,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            max_channels,
            sample_rates: vec![48000, 44100, 96000],
            available: true,
            default_sample_rate,
        }
    }
}

/// Audio capture callback type: receives PCM frames [channel][sample]
pub type CaptureCallback = Arc<dyn Fn(&[Vec<f32>]) + Send + Sync>;

/// Trait for audio capture implementations (ALSA, PulseAudio backends)
pub trait CaptureStream: Send + Sync {
    /// Start capturing audio
    fn start(&mut self) -> Result<(), InputError>;

    /// Stop capturing audio
    fn stop(&mut self) -> Result<(), InputError>;

    /// Check if stream is currently active
    fn is_running(&self) -> bool;

    /// Get current stream sample rate
    fn sample_rate(&self) -> u32;

    /// Get number of input channels
    fn channels(&self) -> u32;

    /// Get latency in milliseconds
    fn latency_ms(&self) -> f32;
}

/// Capture stream status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptureStatus {
    pub is_running: bool,
    pub sample_rate: u32,
    pub channels: u32,
    pub latency_ms: f32,
    pub frames_captured: u64,
}

/// Input manager for device enumeration and stream setup
pub struct InputManager {
    devices: Vec<InputDevice>,
    active_source: Option<InputSource>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            active_source: None,
        }
    }

    /// Enumerate all available input devices
    pub fn enumerate_devices(&mut self) -> Result<Vec<InputDevice>, InputError> {
        // Placeholder: real implementation will use ALSA/PulseAudio APIs
        // For now, return mock devices
        self.devices = vec![
            InputDevice::new(
                "loopback".to_string(),
                "System Audio Loopback".to_string(),
                "loopback".to_string(),
                2,
                48000,
            ),
            InputDevice::new(
                "default".to_string(),
                "Default Microphone".to_string(),
                "microphone".to_string(),
                1,
                48000,
            ),
        ];
        Ok(self.devices.clone())
    }

    /// Get list of system loopback devices
    pub fn get_loopback_devices(&self) -> Vec<InputDevice> {
        self.devices
            .iter()
            .filter(|d| d.device_type == "loopback")
            .cloned()
            .collect()
    }

    /// Get list of external input devices
    pub fn get_external_devices(&self) -> Vec<InputDevice> {
        self.devices
            .iter()
            .filter(|d| d.device_type != "loopback")
            .cloned()
            .collect()
    }

    /// Find device by ID
    pub fn find_device(&self, device_id: &str) -> Option<InputDevice> {
        self.devices.iter().find(|d| d.id == device_id).cloned()
    }

    /// Select system audio loopback as input source
    pub fn select_system_audio(&mut self) -> Result<InputSource, InputError> {
        let loopback_devices = self.get_loopback_devices();
        if loopback_devices.is_empty() {
            return Err(InputError::DeviceNotFound(
                "No system loopback device found".to_string(),
            ));
        }

        let source = InputSource::System {
            device_name: loopback_devices[0].name.clone(),
        };
        self.active_source = Some(source.clone());
        Ok(source)
    }

    /// Select external device as input source
    pub fn select_external_device(&mut self, device_id: &str) -> Result<InputSource, InputError> {
        let device = self
            .find_device(device_id)
            .ok_or_else(|| InputError::DeviceNotFound(device_id.to_string()))?;

        let source = InputSource::External {
            device_name: device.name.clone(),
            device_id: device.id.clone(),
        };
        self.active_source = Some(source.clone());
        Ok(source)
    }

    /// Get currently active input source
    pub fn active_source(&self) -> Option<&InputSource> {
        self.active_source.as_ref()
    }

    /// Clear active input source
    pub fn clear_source(&mut self) {
        self.active_source = None;
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_device_creation() {
        let device = InputDevice::new(
            "mic1".to_string(),
            "USB Microphone".to_string(),
            "microphone".to_string(),
            1,
            48000,
        );

        assert_eq!(device.id, "mic1");
        assert_eq!(device.name, "USB Microphone");
        assert_eq!(device.max_channels, 1);
        assert_eq!(device.default_sample_rate, 48000);
        assert!(device.available);
    }

    #[test]
    fn test_input_source_system() {
        let source = InputSource::System {
            device_name: "loopback".to_string(),
        };

        assert_eq!(source.source_type(), "system");
        assert_eq!(source.device_name(), "loopback");
    }

    #[test]
    fn test_input_source_external() {
        let source = InputSource::External {
            device_name: "USB Audio Device".to_string(),
            device_id: "hw:1,0".to_string(),
        };

        assert_eq!(source.source_type(), "external");
        assert_eq!(source.device_name(), "USB Audio Device");
    }

    #[test]
    fn test_input_manager_enumerate() {
        let mut manager = InputManager::new();
        let devices = manager.enumerate_devices().unwrap();

        assert!(!devices.is_empty());
        assert!(devices.iter().any(|d| d.device_type == "loopback"));
    }

    #[test]
    fn test_input_manager_loopback_devices() {
        let mut manager = InputManager::new();
        manager.enumerate_devices().unwrap();

        let loopback = manager.get_loopback_devices();
        assert!(!loopback.is_empty());
    }

    #[test]
    fn test_input_manager_select_system() {
        let mut manager = InputManager::new();
        manager.enumerate_devices().unwrap();

        let source = manager.select_system_audio().unwrap();
        assert_eq!(source.source_type(), "system");
        assert!(manager.active_source().is_some());
    }

    #[test]
    fn test_input_manager_select_external() {
        let mut manager = InputManager::new();
        manager.enumerate_devices().unwrap();

        let external_devices = manager.get_external_devices();
        if !external_devices.is_empty() {
            let device_id = external_devices[0].id.clone();
            let source = manager.select_external_device(&device_id).unwrap();

            assert_eq!(source.source_type(), "external");
            assert!(manager.active_source().is_some());
        }
    }

    #[test]
    fn test_input_manager_clear_source() {
        let mut manager = InputManager::new();
        manager.enumerate_devices().unwrap();
        manager.select_system_audio().unwrap();

        assert!(manager.active_source().is_some());

        manager.clear_source();
        assert!(manager.active_source().is_none());
    }
}
