// SPDX-License-Identifier: Apache-2.0

use audio_ninja::network::*;
use audio_ninja::AudioBlock;
use std::net::SocketAddr;
use std::time::Duration;

#[test]
fn test_udp_sender_creation() {
    let target: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let sender = UdpRtpSender::new("127.0.0.1:0", target, 12345);

    assert!(sender.is_ok());
    let sender = sender.unwrap();
    assert_eq!(sender.ssrc(), 12345);
    assert_eq!(sender.sequence(), 0);
}

#[test]
fn test_udp_receiver_creation() {
    let receiver = UdpRtpReceiver::new("127.0.0.1:0");
    assert!(receiver.is_ok());
}

#[test]
fn test_udp_send_recv_roundtrip() {
    // Create receiver first to get its address
    let mut receiver = UdpRtpReceiver::new("127.0.0.1:0").unwrap();
    receiver
        .set_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    let recv_addr = receiver.local_addr().unwrap();

    // Create sender targeting the receiver
    let mut sender = UdpRtpSender::new("127.0.0.1:0", recv_addr, 54321).unwrap();

    // Send a block
    let block = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![0.1, 0.2], vec![0.3, 0.4]],
    };

    sender.send_block(&block).unwrap();

    // Receive the block
    let result = receiver.recv_block();
    assert!(result.is_ok());

    let (recv_block, _addr) = result.unwrap();
    assert_eq!(recv_block.channels, block.channels);
    assert_eq!(recv_block.sample_rate, 48000);
}

#[test]
fn test_udp_sequence_increment() {
    let target: SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let mut sender = UdpRtpSender::new("127.0.0.1:0", target, 99999).unwrap();

    assert_eq!(sender.sequence(), 0);

    let block = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![0.0; 240]; 2], // 2 channels, 240 frames
    };

    // Send multiple blocks
    for i in 0..5 {
        sender.send_block(&block).unwrap();
        assert_eq!(sender.sequence(), (i + 1) as u16);
    }
}

#[test]
fn test_speaker_discovery_register() {
    let discovery = SpeakerDiscovery::new();

    let speaker = SpeakerInfo {
        id: "speaker1".into(),
        name: "Living Room".into(),
        address: "192.168.1.100:8000".parse().unwrap(),
        capabilities: SpeakerCapabilities::default(),
    };

    discovery.register_speaker(speaker.clone()).unwrap();

    let speakers = discovery.list_speakers();
    assert_eq!(speakers.len(), 1);
    assert_eq!(speakers[0].id, "speaker1");
}

#[test]
fn test_speaker_discovery_find() {
    let discovery = SpeakerDiscovery::new();

    let speaker1 = SpeakerInfo {
        id: "speaker1".into(),
        name: "Living Room".into(),
        address: "192.168.1.100:8000".parse().unwrap(),
        capabilities: SpeakerCapabilities::default(),
    };

    let speaker2 = SpeakerInfo {
        id: "speaker2".into(),
        name: "Bedroom".into(),
        address: "192.168.1.101:8000".parse().unwrap(),
        capabilities: SpeakerCapabilities::default(),
    };

    discovery.register_speaker(speaker1).unwrap();
    discovery.register_speaker(speaker2).unwrap();

    let found = discovery.find_speaker("speaker2");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Bedroom");

    let not_found = discovery.find_speaker("speaker99");
    assert!(not_found.is_none());
}

#[test]
fn test_speaker_discovery_remove() {
    let discovery = SpeakerDiscovery::new();

    let speaker = SpeakerInfo {
        id: "speaker1".into(),
        name: "Test".into(),
        address: "192.168.1.100:8000".parse().unwrap(),
        capabilities: SpeakerCapabilities::default(),
    };

    discovery.register_speaker(speaker).unwrap();
    assert_eq!(discovery.list_speakers().len(), 1);

    discovery.remove_speaker("speaker1");
    assert_eq!(discovery.list_speakers().len(), 0);
}

#[test]
fn test_speaker_capabilities_default() {
    let caps = SpeakerCapabilities::default();

    assert_eq!(caps.max_sample_rate, 48000);
    assert_eq!(caps.channels, 2);
    assert!(caps.codecs.contains(&"PCM".into()));
    assert!(caps.layouts.contains(&"stereo".into()));
}

#[test]
fn test_multi_speaker_sender_creation() {
    let discovery = std::sync::Arc::new(SpeakerDiscovery::new());
    let sender = MultiSpeakerSender::new("127.0.0.1:0", discovery);

    assert_eq!(sender.speaker_count(), 0);
}

#[test]
fn test_multi_speaker_sender_add_speaker() {
    let discovery = std::sync::Arc::new(SpeakerDiscovery::new());

    let speaker = SpeakerInfo {
        id: "test_speaker".into(),
        name: "Test".into(),
        address: "127.0.0.1:9999".parse().unwrap(),
        capabilities: SpeakerCapabilities::default(),
    };

    discovery.register_speaker(speaker).unwrap();

    let mut sender = MultiSpeakerSender::new("127.0.0.1:0", discovery.clone());
    let result = sender.add_speaker("test_speaker", "127.0.0.1:0");

    assert!(result.is_ok());
    assert_eq!(sender.speaker_count(), 1);
}

#[test]
fn test_multi_speaker_sender_speaker_not_found() {
    let discovery = std::sync::Arc::new(SpeakerDiscovery::new());
    let mut sender = MultiSpeakerSender::new("127.0.0.1:0", discovery);

    let result = sender.add_speaker("nonexistent", "127.0.0.1:0");
    assert!(result.is_err());

    match result {
        Err(NetworkError::SpeakerNotFound(id)) => assert_eq!(id, "nonexistent"),
        _ => panic!("Expected SpeakerNotFound error"),
    }
}

#[test]
fn test_udp_timeout() {
    let mut receiver = UdpRtpReceiver::new("127.0.0.1:0").unwrap();
    receiver
        .set_timeout(Some(Duration::from_millis(10)))
        .unwrap();

    // Try to receive without any sender (should timeout)
    let result = receiver.recv_block();
    assert!(result.is_err());
}
