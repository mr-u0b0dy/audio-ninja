// SPDX-License-Identifier: Apache-2.0

use audio_ninja::ble::*;
use std::time::Duration;

#[test]
fn test_speaker_role_channel_index() {
    assert_eq!(SpeakerRole::FrontLeft.to_channel_index(), 0);
    assert_eq!(SpeakerRole::FrontRight.to_channel_index(), 1);
    assert_eq!(SpeakerRole::Center.to_channel_index(), 2);
    assert_eq!(SpeakerRole::LFE.to_channel_index(), 3);
    assert_eq!(SpeakerRole::SurroundLeft.to_channel_index(), 4);
    assert_eq!(SpeakerRole::SurroundRight.to_channel_index(), 5);
}

#[test]
fn test_speaker_identity_creation() {
    let identity = SpeakerIdentity {
        id: "speaker1".into(),
        name: "Living Room Left".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    assert_eq!(identity.id, "speaker1");
    assert_eq!(identity.role, SpeakerRole::FrontLeft);
}

#[test]
fn test_layout_config_stereo() {
    let config = LayoutConfig {
        name: "2.0 Stereo".into(),
        channels: 2,
        speaker_positions: vec![
            SpeakerPosition {
                role: SpeakerRole::FrontLeft,
                azimuth: 30.0,
                elevation: 0.0,
                distance: 2.0,
            },
            SpeakerPosition {
                role: SpeakerRole::FrontRight,
                azimuth: -30.0,
                elevation: 0.0,
                distance: 2.0,
            },
        ],
    };

    assert_eq!(config.channels, 2);
    assert_eq!(config.speaker_positions.len(), 2);
}

#[test]
fn test_calibration_settings() {
    let settings = CalibrationSettings {
        trim_db: -2.5,
        delay_ms: 15.0,
        eq_enabled: true,
    };

    assert_eq!(settings.trim_db, -2.5);
    assert_eq!(settings.delay_ms, 15.0);
    assert!(settings.eq_enabled);
}

#[test]
fn test_gatt_characteristic_read_write() {
    let mut char =
        GattCharacteristic::new(characteristic_uuids::SPEAKER_IDENTITY, true, true, false);

    let data = vec![1, 2, 3, 4];
    char.write(data.clone()).unwrap();

    let read_data = char.read().unwrap();
    assert_eq!(read_data, data);
}

#[test]
fn test_gatt_characteristic_readonly() {
    let char = GattCharacteristic::new(characteristic_uuids::CAPABILITIES, true, false, false);

    let result = char.clone().write(vec![1, 2, 3]);
    assert!(result.is_err());
}

#[test]
fn test_gatt_service_add_characteristic() {
    let mut service = GattService::new(service_uuids::AUDIO_NINJA_SERVICE);

    let char = GattCharacteristic::new(characteristic_uuids::SPEAKER_IDENTITY, true, true, false);

    service.add_characteristic(char);

    let retrieved = service.get_characteristic(&characteristic_uuids::SPEAKER_IDENTITY);
    assert!(retrieved.is_some());
}

#[test]
fn test_ble_peripheral_creation() {
    let identity = SpeakerIdentity {
        id: "test_speaker".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::Center,
        mac_address: "11:22:33:44:55:66".into(),
    };

    let peripheral = BlePeripheral::new(identity.clone());

    assert_eq!(peripheral.identity().id, identity.id);
    assert!(matches!(
        peripheral.status(),
        ConnectionStatus::Disconnected
    ));
}

#[test]
fn test_ble_peripheral_connect() {
    let identity = SpeakerIdentity {
        id: "test_speaker".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    let mut peripheral = BlePeripheral::new(identity);

    peripheral.connect().unwrap();
    assert!(matches!(peripheral.status(), ConnectionStatus::Connected));

    peripheral.disconnect();
    assert!(matches!(
        peripheral.status(),
        ConnectionStatus::Disconnected
    ));
}

#[test]
fn test_ble_peripheral_pairing() {
    let identity = SpeakerIdentity {
        id: "test_speaker".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    let mut peripheral = BlePeripheral::new(identity);
    peripheral.connect().unwrap();

    let request = PairingRequest {
        pin_code: Some("1234".into()),
        master_id: "master001".into(),
    };

    peripheral.pair(request).unwrap();
    assert!(matches!(peripheral.status(), ConnectionStatus::Paired));
}

#[test]
fn test_ble_peripheral_pairing_no_pin() {
    let identity = SpeakerIdentity {
        id: "test_speaker".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    let mut peripheral = BlePeripheral::new(identity);

    let request = PairingRequest {
        pin_code: None,
        master_id: "master001".into(),
    };

    let result = peripheral.pair(request);
    assert!(result.is_err());
}

#[test]
fn test_ble_peripheral_read_write_characteristic() {
    let identity = SpeakerIdentity {
        id: "test_speaker".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    let mut peripheral = BlePeripheral::new(identity);

    let data = vec![1, 2, 3, 4, 5];
    peripheral
        .write_characteristic(&characteristic_uuids::VOLUME_TRIM, data.clone())
        .unwrap();

    let read_data = peripheral
        .read_characteristic(&characteristic_uuids::VOLUME_TRIM)
        .unwrap();
    assert_eq!(read_data, data);
}

#[test]
fn test_ble_central_creation() {
    let central = BleCentral::new();
    assert_eq!(central.list_devices().len(), 0);
}

#[test]
fn test_ble_central_scan() {
    let central = BleCentral::new();

    let devices = central.scan(Duration::from_secs(1)).unwrap();
    // Placeholder implementation returns empty
    assert_eq!(devices.len(), 0);
}

#[test]
fn test_ble_central_connect_disconnect() {
    let central = BleCentral::new();

    let identity = SpeakerIdentity {
        id: "speaker1".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    let peripheral = BlePeripheral::new(identity);

    central.connect("speaker1", peripheral).unwrap();
    assert_eq!(central.list_devices().len(), 1);

    central.disconnect("speaker1").unwrap();
    assert_eq!(central.list_devices().len(), 0);
}

#[test]
fn test_ble_central_read_write_identity() {
    let central = BleCentral::new();

    let identity = SpeakerIdentity {
        id: "speaker1".into(),
        name: "Living Room Left".into(),
        role: SpeakerRole::FrontLeft,
        mac_address: "AA:BB:CC:DD:EE:FF".into(),
    };

    let peripheral = BlePeripheral::new(identity.clone());
    central.connect("speaker1", peripheral).unwrap();

    // Write identity
    central
        .write_speaker_identity("speaker1", &identity)
        .unwrap();

    // Read it back
    let read_identity = central.read_speaker_identity("speaker1").unwrap();
    assert_eq!(read_identity.id, identity.id);
    assert_eq!(read_identity.name, identity.name);
}

#[test]
fn test_ble_central_write_calibration() {
    let central = BleCentral::new();

    let identity = SpeakerIdentity {
        id: "speaker1".into(),
        name: "Test Speaker".into(),
        role: SpeakerRole::Center,
        mac_address: "11:22:33:44:55:66".into(),
    };

    let peripheral = BlePeripheral::new(identity);
    central.connect("speaker1", peripheral).unwrap();

    let calibration = CalibrationSettings {
        trim_db: -3.0,
        delay_ms: 20.0,
        eq_enabled: true,
    };

    let result = central.write_calibration("speaker1", &calibration);
    assert!(result.is_ok());
}

#[test]
fn test_ble_central_device_not_found() {
    let central = BleCentral::new();

    let result = central.disconnect("nonexistent");
    assert!(result.is_err());

    match result {
        Err(BleError::DeviceNotFound(id)) => assert_eq!(id, "nonexistent"),
        _ => panic!("Expected DeviceNotFound error"),
    }
}

#[test]
fn test_connection_status_variants() {
    let status1 = ConnectionStatus::Disconnected;
    let status2 = ConnectionStatus::Connected;
    let status3 = ConnectionStatus::Paired;

    assert!(matches!(status1, ConnectionStatus::Disconnected));
    assert!(matches!(status2, ConnectionStatus::Connected));
    assert!(matches!(status3, ConnectionStatus::Paired));
}

#[test]
fn test_layout_config_5_1() {
    let config = LayoutConfig {
        name: "5.1 Surround".into(),
        channels: 6,
        speaker_positions: vec![
            SpeakerPosition {
                role: SpeakerRole::FrontLeft,
                azimuth: 30.0,
                elevation: 0.0,
                distance: 2.0,
            },
            SpeakerPosition {
                role: SpeakerRole::FrontRight,
                azimuth: -30.0,
                elevation: 0.0,
                distance: 2.0,
            },
            SpeakerPosition {
                role: SpeakerRole::Center,
                azimuth: 0.0,
                elevation: 0.0,
                distance: 2.0,
            },
            SpeakerPosition {
                role: SpeakerRole::LFE,
                azimuth: 0.0,
                elevation: -45.0,
                distance: 2.0,
            },
            SpeakerPosition {
                role: SpeakerRole::SurroundLeft,
                azimuth: 110.0,
                elevation: 0.0,
                distance: 2.0,
            },
            SpeakerPosition {
                role: SpeakerRole::SurroundRight,
                azimuth: -110.0,
                elevation: 0.0,
                distance: 2.0,
            },
        ],
    };

    assert_eq!(config.channels, 6);
    assert_eq!(config.speaker_positions.len(), 6);
    assert_eq!(config.speaker_positions[2].role, SpeakerRole::Center);
}
