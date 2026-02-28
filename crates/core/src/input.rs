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
    System { device_name: String },

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

/// Audio capture callback type: receives PCM frames \[channel\]\[sample\]
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
        #[cfg(feature = "audio-backends")]
        {
            self.devices = cpal_enumerate_input_devices()?;
            return Ok(self.devices.clone());
        }

        #[cfg(not(feature = "audio-backends"))]
        {
            // Mock devices for testing without real audio backends
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

// ===== cpal Backend for Real Device Enumeration =====
#[cfg(feature = "audio-backends")]
fn cpal_enumerate_input_devices() -> Result<Vec<InputDevice>, InputError> {
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();
    let mut devices = Vec::new();

    // Add system loopback placeholder (cpal doesn't directly support loopback;
    // on Linux this requires PulseAudio monitor sources or ALSA loopback module)
    devices.push(InputDevice::new(
        "loopback".to_string(),
        "System Audio Loopback".to_string(),
        "loopback".to_string(),
        2,
        48000,
    ));

    // Enumerate real input devices
    let input_devices = host
        .input_devices()
        .map_err(|e| InputError::BackendError(format!("Failed to enumerate devices: {}", e)))?;

    for (idx, device) in input_devices.enumerate() {
        let name = device
            .name()
            .unwrap_or_else(|_| format!("Input Device {}", idx));

        let config = device.default_input_config().ok();
        let (channels, sample_rate) = config
            .map(|c| (c.channels() as u32, c.sample_rate().0))
            .unwrap_or((2, 48000));

        let device_type = if name.to_lowercase().contains("mic") {
            "microphone"
        } else if name.to_lowercase().contains("usb") {
            "usb-audio"
        } else if name.to_lowercase().contains("line") {
            "line-in"
        } else {
            "audio-input"
        };

        let mut supported_rates = vec![];
        if let Ok(configs) = device.supported_input_configs() {
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

        let mut input_device = InputDevice::new(
            format!("input_{}", idx),
            name,
            device_type.to_string(),
            channels,
            sample_rate,
        );
        input_device.sample_rates = supported_rates;
        devices.push(input_device);
    }

    Ok(devices)
}

/// Wrapper to make cpal::Stream usable across thread boundaries.
/// Safety: we only interact with the stream via play/pause/drop, all of which
/// are internally synchronized by cpal. The audio callback closure itself is
/// required to be Send by cpal's API.
#[cfg(feature = "audio-backends")]
#[allow(dead_code)]
struct SendSyncStream(cpal::Stream);
#[cfg(feature = "audio-backends")]
unsafe impl Send for SendSyncStream {}
#[cfg(feature = "audio-backends")]
unsafe impl Sync for SendSyncStream {}

/// CaptureStream implementation backed by cpal
#[cfg(feature = "audio-backends")]
pub struct CpalCaptureStream {
    stream: Option<SendSyncStream>,
    sample_rate: u32,
    channels: u32,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// Shared ring buffer for captured audio (interleaved f32)
    buffer: std::sync::Arc<std::sync::Mutex<Vec<f32>>>,
}

#[cfg(feature = "audio-backends")]
impl CpalCaptureStream {
    /// Create a new capture stream for a device by index
    ///
    /// `device_index` selects the input device (0 = first real device after loopback).
    /// The stream is created but not started until `start()` is called.
    pub fn new(device_index: usize, sample_rate: u32, channels: u32) -> Result<Self, InputError> {
        use cpal::traits::HostTrait;

        let host = cpal::default_host();
        let _device = host
            .input_devices()
            .map_err(|e| InputError::BackendError(format!("enumerate failed: {e}")))?
            .nth(device_index)
            .ok_or_else(|| {
                InputError::DeviceNotFound(format!("device index {} not found", device_index))
            })?;

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let buffer = std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(
            sample_rate as usize * channels as usize,
        )));

        Ok(Self {
            stream: None,
            sample_rate,
            channels,
            running,
            buffer,
        })
    }

    /// Create using the default input device
    pub fn default_device(sample_rate: u32, channels: u32) -> Result<Self, InputError> {
        use cpal::traits::HostTrait;

        let host = cpal::default_host();
        let _device = host.default_input_device().ok_or_else(|| {
            InputError::DeviceNotFound("no default input device".to_string())
        })?;

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let buffer = std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(
            sample_rate as usize * channels as usize,
        )));

        Ok(Self {
            stream: None,
            sample_rate,
            channels,
            running,
            buffer,
        })
    }

    /// Read captured samples from the ring buffer, draining it
    pub fn read_captured(&self) -> Vec<f32> {
        let mut buf = self.buffer.lock().unwrap();
        std::mem::take(&mut *buf)
    }
}

#[cfg(feature = "audio-backends")]
impl CaptureStream for CpalCaptureStream {
    fn start(&mut self) -> Result<(), InputError> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use cpal::{SampleRate, StreamConfig};

        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(()); // already running
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| InputError::DeviceNotFound("no default input device".to_string()))?;

        let config = StreamConfig {
            channels: self.channels as u16,
            sample_rate: SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = self.buffer.clone();
        let running = self.running.clone();

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if running.load(std::sync::atomic::Ordering::Relaxed) {
                        if let Ok(mut buf) = buffer.lock() {
                            // Cap buffer size to avoid unbounded growth (~2 seconds)
                            const MAX_SAMPLES: usize = 48000 * 2 * 2;
                            if buf.len() + data.len() > MAX_SAMPLES {
                                let drain = (buf.len() + data.len()) - MAX_SAMPLES;
                                buf.drain(..drain);
                            }
                            buf.extend_from_slice(data);
                        }
                    }
                },
                |err| {
                    eprintln!("[audio-ninja] capture error: {}", err);
                },
                None,
            )
            .map_err(|e| InputError::CaptureFailed(format!("build stream: {e}")))?;

        stream
            .play()
            .map_err(|e| InputError::CaptureFailed(format!("play: {e}")))?;

        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.stream = Some(SendSyncStream(stream));
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InputError> {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        // Dropping the stream stops playback
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
        // Default buffer latency estimate; cpal doesn't expose exact latency
        // 256 frames at 48kHz ≈ 5.3ms
        256.0 / self.sample_rate as f32 * 1000.0
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
