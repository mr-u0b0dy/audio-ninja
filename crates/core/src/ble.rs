// SPDX-License-Identifier: Apache-2.0

//! BLE GATT profiles for wireless speaker control and configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum BleError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Invalid characteristic")]
    InvalidCharacteristic,
    #[error("Write failed")]
    WriteFailed,
    #[error("Read failed")]
    ReadFailed,
    #[error("Pairing failed: {0}")]
    PairingFailed(String),
}

/// Audio Ninja BLE GATT Service UUIDs
pub mod service_uuids {
    use uuid::Uuid;

    /// Primary Audio Ninja service
    pub const AUDIO_NINJA_SERVICE: Uuid = Uuid::from_u128(0x0000FE59_0000_1000_8000_00805F9B34FB);

    /// Device information service
    pub const DEVICE_INFO_SERVICE: Uuid = Uuid::from_u128(0x0000180A_0000_1000_8000_00805F9B34FB);
}

/// Characteristic UUIDs for Audio Ninja service
pub mod characteristic_uuids {
    use uuid::Uuid;

    /// Speaker identity (read/write)
    pub const SPEAKER_IDENTITY: Uuid = Uuid::from_u128(0x0000FE5A_0000_1000_8000_00805F9B34FB);

    /// Layout configuration (read/write)
    pub const LAYOUT_CONFIG: Uuid = Uuid::from_u128(0x0000FE5B_0000_1000_8000_00805F9B34FB);

    /// Volume trim in dB (read/write)
    pub const VOLUME_TRIM: Uuid = Uuid::from_u128(0x0000FE5C_0000_1000_8000_00805F9B34FB);

    /// Delay compensation in ms (read/write)
    pub const DELAY_COMPENSATION: Uuid = Uuid::from_u128(0x0000FE5D_0000_1000_8000_00805F9B34FB);

    /// Speaker capabilities (read-only)
    pub const CAPABILITIES: Uuid = Uuid::from_u128(0x0000FE5E_0000_1000_8000_00805F9B34FB);

    /// Pairing control (write-only)
    pub const PAIRING_CONTROL: Uuid = Uuid::from_u128(0x0000FE5F_0000_1000_8000_00805F9B34FB);

    /// Connection status (notify)
    pub const CONNECTION_STATUS: Uuid = Uuid::from_u128(0x0000FE60_0000_1000_8000_00805F9B34FB);

    /// Firmware version (read-only)
    pub const FIRMWARE_VERSION: Uuid = Uuid::from_u128(0x0000FE61_0000_1000_8000_00805F9B34FB);
}

/// Speaker identity information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeakerIdentity {
    pub id: String,
    pub name: String,
    pub role: SpeakerRole,
    pub mac_address: String,
}

/// Speaker role in the audio system
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SpeakerRole {
    FrontLeft,
    FrontRight,
    Center,
    LFE,
    SurroundLeft,
    SurroundRight,
    BackLeft,
    BackRight,
    TopFrontLeft,
    TopFrontRight,
    TopBackLeft,
    TopBackRight,
    Custom(String),
}

impl SpeakerRole {
    pub fn to_channel_index(&self) -> usize {
        match self {
            SpeakerRole::FrontLeft => 0,
            SpeakerRole::FrontRight => 1,
            SpeakerRole::Center => 2,
            SpeakerRole::LFE => 3,
            SpeakerRole::SurroundLeft => 4,
            SpeakerRole::SurroundRight => 5,
            SpeakerRole::BackLeft => 6,
            SpeakerRole::BackRight => 7,
            SpeakerRole::TopFrontLeft => 8,
            SpeakerRole::TopFrontRight => 9,
            SpeakerRole::TopBackLeft => 10,
            SpeakerRole::TopBackRight => 11,
            SpeakerRole::Custom(_) => 99,
        }
    }
}

/// Layout configuration for speaker positioning
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub name: String,
    pub channels: u8,
    pub speaker_positions: Vec<SpeakerPosition>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeakerPosition {
    pub role: SpeakerRole,
    pub azimuth: f32,   // degrees: 0=front, 90=left, -90=right
    pub elevation: f32, // degrees: 0=horizon, 90=zenith
    pub distance: f32,  // meters from listener
}

/// Speaker calibration settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalibrationSettings {
    pub trim_db: f32,
    pub delay_ms: f32,
    pub eq_enabled: bool,
}

/// Connection status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Paired,
    Error(String),
}

/// Pairing request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairingRequest {
    pub pin_code: Option<String>,
    pub master_id: String,
}

/// BLE GATT characteristic
#[derive(Clone, Debug)]
pub struct GattCharacteristic {
    pub uuid: Uuid,
    pub value: Vec<u8>,
    pub readable: bool,
    pub writable: bool,
    pub notifiable: bool,
}

impl GattCharacteristic {
    pub fn new(uuid: Uuid, readable: bool, writable: bool, notifiable: bool) -> Self {
        Self {
            uuid,
            value: Vec::new(),
            readable,
            writable,
            notifiable,
        }
    }

    pub fn read(&self) -> Result<Vec<u8>, BleError> {
        if !self.readable {
            return Err(BleError::InvalidCharacteristic);
        }
        Ok(self.value.clone())
    }

    pub fn write(&mut self, data: Vec<u8>) -> Result<(), BleError> {
        if !self.writable {
            return Err(BleError::InvalidCharacteristic);
        }
        self.value = data;
        Ok(())
    }
}

/// BLE GATT service
pub struct GattService {
    _uuid: Uuid,
    characteristics: HashMap<Uuid, GattCharacteristic>,
}

impl GattService {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            _uuid: uuid,
            characteristics: HashMap::new(),
        }
    }

    pub fn add_characteristic(&mut self, char: GattCharacteristic) {
        self.characteristics.insert(char.uuid, char);
    }

    pub fn get_characteristic(&self, uuid: &Uuid) -> Option<&GattCharacteristic> {
        self.characteristics.get(uuid)
    }

    pub fn get_characteristic_mut(&mut self, uuid: &Uuid) -> Option<&mut GattCharacteristic> {
        self.characteristics.get_mut(uuid)
    }
}

/// BLE peripheral device (speaker)
pub struct BlePeripheral {
    identity: SpeakerIdentity,
    services: HashMap<Uuid, GattService>,
    connection_status: ConnectionStatus,
}

impl BlePeripheral {
    pub fn new(identity: SpeakerIdentity) -> Self {
        let mut peripheral = Self {
            identity,
            services: HashMap::new(),
            connection_status: ConnectionStatus::Disconnected,
        };

        peripheral.setup_services();
        peripheral
    }

    fn setup_services(&mut self) {
        // Audio Ninja service
        let mut service = GattService::new(service_uuids::AUDIO_NINJA_SERVICE);

        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::SPEAKER_IDENTITY,
            true,
            true,
            false,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::LAYOUT_CONFIG,
            true,
            true,
            false,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::VOLUME_TRIM,
            true,
            true,
            false,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::DELAY_COMPENSATION,
            true,
            true,
            false,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::CAPABILITIES,
            true,
            false,
            false,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::PAIRING_CONTROL,
            false,
            true,
            false,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::CONNECTION_STATUS,
            true,
            false,
            true,
        ));
        service.add_characteristic(GattCharacteristic::new(
            characteristic_uuids::FIRMWARE_VERSION,
            true,
            false,
            false,
        ));

        self.services
            .insert(service_uuids::AUDIO_NINJA_SERVICE, service);
    }

    pub fn connect(&mut self) -> Result<(), BleError> {
        self.connection_status = ConnectionStatus::Connected;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connection_status = ConnectionStatus::Disconnected;
    }

    pub fn pair(&mut self, request: PairingRequest) -> Result<(), BleError> {
        // Simplified pairing - real implementation would validate PIN
        if request.pin_code.is_some() {
            self.connection_status = ConnectionStatus::Paired;
            Ok(())
        } else {
            Err(BleError::PairingFailed("PIN required".into()))
        }
    }

    pub fn read_characteristic(&self, char_uuid: &Uuid) -> Result<Vec<u8>, BleError> {
        for service in self.services.values() {
            if let Some(char) = service.get_characteristic(char_uuid) {
                return char.read();
            }
        }
        Err(BleError::InvalidCharacteristic)
    }

    pub fn write_characteristic(
        &mut self,
        char_uuid: &Uuid,
        data: Vec<u8>,
    ) -> Result<(), BleError> {
        for service in self.services.values_mut() {
            if let Some(char) = service.get_characteristic_mut(char_uuid) {
                return char.write(data);
            }
        }
        Err(BleError::InvalidCharacteristic)
    }

    pub fn identity(&self) -> &SpeakerIdentity {
        &self.identity
    }

    pub fn status(&self) -> &ConnectionStatus {
        &self.connection_status
    }
}

/// BLE central device (controller/phone)
pub struct BleCentral {
    connected_devices: Arc<Mutex<HashMap<String, BlePeripheral>>>,
}

impl BleCentral {
    pub fn new() -> Self {
        Self {
            connected_devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn scan(&self, _timeout: Duration) -> Result<Vec<SpeakerIdentity>, BleError> {
        // Placeholder - real implementation would scan for BLE devices
        Ok(Vec::new())
    }

    pub fn connect(&self, device_id: &str, peripheral: BlePeripheral) -> Result<(), BleError> {
        let mut devices = self.connected_devices.lock().unwrap();
        devices.insert(device_id.to_string(), peripheral);
        Ok(())
    }

    pub fn disconnect(&self, device_id: &str) -> Result<(), BleError> {
        let mut devices = self.connected_devices.lock().unwrap();
        if let Some(mut device) = devices.remove(device_id) {
            device.disconnect();
            Ok(())
        } else {
            Err(BleError::DeviceNotFound(device_id.into()))
        }
    }

    pub fn read_speaker_identity(&self, device_id: &str) -> Result<SpeakerIdentity, BleError> {
        let devices = self.connected_devices.lock().unwrap();
        let device = devices
            .get(device_id)
            .ok_or_else(|| BleError::DeviceNotFound(device_id.into()))?;

        let data = device.read_characteristic(&characteristic_uuids::SPEAKER_IDENTITY)?;
        bincode::deserialize(&data).map_err(|_| BleError::ReadFailed)
    }

    pub fn write_speaker_identity(
        &self,
        device_id: &str,
        identity: &SpeakerIdentity,
    ) -> Result<(), BleError> {
        let mut devices = self.connected_devices.lock().unwrap();
        let device = devices
            .get_mut(device_id)
            .ok_or_else(|| BleError::DeviceNotFound(device_id.into()))?;

        let data = bincode::serialize(identity).map_err(|_| BleError::WriteFailed)?;
        device.write_characteristic(&characteristic_uuids::SPEAKER_IDENTITY, data)
    }

    pub fn write_calibration(
        &self,
        device_id: &str,
        cal: &CalibrationSettings,
    ) -> Result<(), BleError> {
        let mut devices = self.connected_devices.lock().unwrap();
        let device = devices
            .get_mut(device_id)
            .ok_or_else(|| BleError::DeviceNotFound(device_id.into()))?;

        // Write trim
        let trim_data = bincode::serialize(&cal.trim_db).map_err(|_| BleError::WriteFailed)?;
        device.write_characteristic(&characteristic_uuids::VOLUME_TRIM, trim_data)?;

        // Write delay
        let delay_data = bincode::serialize(&cal.delay_ms).map_err(|_| BleError::WriteFailed)?;
        device.write_characteristic(&characteristic_uuids::DELAY_COMPENSATION, delay_data)?;

        Ok(())
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.connected_devices
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }
}

impl Default for BleCentral {
    fn default() -> Self {
        Self::new()
    }
}
