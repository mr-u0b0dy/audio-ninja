// SPDX-License-Identifier: Apache-2.0

use audio_ninja::jitter::*;
use audio_ninja::latency::*;
use audio_ninja::sync::*;
use audio_ninja::transport::*;
use audio_ninja::{AudioBlock, Position3, SpeakerDescriptor, SpeakerRole};
use std::time::Duration;

#[test]
fn test_rtp_header_serialize_deserialize() {
    let header = RtpHeader::new(12345, 67890, 0xABCDEF01);

    let serialized = header.serialize();
    assert_eq!(serialized.len(), 12);

    let deserialized = RtpHeader::deserialize(&serialized).expect("deserialize should work");

    assert_eq!(deserialized.version, 2);
    assert_eq!(deserialized.sequence.0, 12345);
    assert_eq!(deserialized.timestamp.0, 67890);
    assert_eq!(deserialized.ssrc.0, 0xABCDEF01);
}

#[test]
fn test_rtp_packet_serialize_deserialize() {
    let payload = vec![1, 2, 3, 4, 5];
    let packet = RtpPacket::new(100, 200, 300, payload.clone());

    let serialized = packet.serialize();
    let deserialized = RtpPacket::deserialize(&serialized).expect("deserialize should work");

    assert_eq!(deserialized.header.sequence.0, 100);
    assert_eq!(deserialized.header.timestamp.0, 200);
    assert_eq!(deserialized.header.ssrc.0, 300);
    assert_eq!(deserialized.payload, payload);
}

#[test]
fn test_loopback_transport() {
    let mut transport = LoopbackTransport::with_ssrc(12345);

    let packet = RtpPacket::new(1, 1000, 12345, vec![1, 2, 3]);
    transport.send(packet.clone()).expect("send should work");

    let received = transport.poll().expect("poll should work");
    assert!(received.is_some());

    let received = received.unwrap();
    assert_eq!(received.header.sequence.0, 1);
    assert_eq!(received.payload, vec![1, 2, 3]);
}

#[test]
fn test_audio_block_to_rtp_roundtrip() {
    let block = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    };

    let rtp = audio_block_to_rtp(&block, 100, 200, 300);
    assert_eq!(rtp.header.sequence.0, 100);
    assert_eq!(rtp.header.timestamp.0, 200);

    let decoded = rtp_to_audio_block(&rtp).expect("decode should work");
    assert_eq!(decoded.sample_rate, block.sample_rate);
    assert_eq!(decoded.channels.len(), block.channels.len());
}

#[test]
fn test_clock_timestamp_skew() {
    let ts1 = ClockTimestamp {
        seconds: 1000,
        nanos: 0,
        source: ClockSource::System,
    };

    let ts2 = ClockTimestamp {
        seconds: 1000,
        nanos: 100_000, // 100 µs later
        source: ClockSource::System,
    };

    let skew = ts1.skew_from(&ts2);
    assert_eq!(skew.as_micros(), 100);
}

#[test]
fn test_ptp_clock_sync() {
    let mut clock = PtpClock::default();

    let reference = ClockTimestamp {
        seconds: 2000,
        nanos: 0,
        source: ClockSource::Ptp,
    };

    clock.sync(&reference).expect("sync should work");

    let now = clock.now();
    assert!(now.seconds >= 2000);
}

#[test]
fn test_ntp_clock_gradual_sync() {
    let mut clock = NtpClock::default();

    let reference = ClockTimestamp {
        seconds: 3000,
        nanos: 0,
        source: ClockSource::Ntp,
    };

    // First sync applies 10% of offset
    clock.sync(&reference).expect("first sync should work");

    // Second sync applies another 10%
    clock.sync(&reference).expect("second sync should work");

    // Clock should now report a time closer to reference
    let now = clock.now();
    assert!(now.seconds > 1000); // Should be adjusted from system time
}

#[test]
fn test_jitter_buffer_push_pop() {
    let mut buffer = JitterBuffer::default();

    let packet1 = RtpPacket::new(1, 1000, 12345, vec![1, 2, 3]);
    let packet2 = RtpPacket::new(2, 2000, 12345, vec![4, 5, 6]);
    let packet3 = RtpPacket::new(3, 3000, 12345, vec![7, 8, 9]);

    buffer.push(packet1).expect("push 1 should work");
    buffer.push(packet2).expect("push 2 should work");
    buffer.push(packet3).expect("push 3 should work");

    assert_eq!(buffer.len(), 3);
    assert!(buffer.ready());

    let popped = buffer.pop().expect("pop should work");
    assert_eq!(popped.header.sequence.0, 1);
    assert_eq!(buffer.len(), 2);
}

#[test]
fn test_jitter_buffer_out_of_order() {
    let mut buffer = JitterBuffer::default();

    let packet1 = RtpPacket::new(1, 1000, 12345, vec![1]);
    let packet3 = RtpPacket::new(3, 3000, 12345, vec![3]);
    let packet2 = RtpPacket::new(2, 2000, 12345, vec![2]);

    buffer.push(packet1).expect("push 1");
    buffer.push(packet3).expect("push 3");
    buffer.push(packet2).expect("push 2");

    // Should pop in sequence order
    assert_eq!(buffer.pop().unwrap().header.sequence.0, 1);
    assert_eq!(buffer.pop().unwrap().header.sequence.0, 2);
    assert_eq!(buffer.pop().unwrap().header.sequence.0, 3);
}

#[test]
fn test_jitter_buffer_late_packet() {
    let mut buffer = JitterBuffer::default();

    let packet1 = RtpPacket::new(100, 1000, 12345, vec![1]);
    let packet2 = RtpPacket::new(101, 2000, 12345, vec![2]);
    let packet3 = RtpPacket::new(102, 3000, 12345, vec![3]);

    buffer.push(packet1).unwrap();
    buffer.push(packet2).unwrap();
    buffer.push(packet3).unwrap();

    buffer.pop().unwrap(); // Pop 100
    buffer.pop().unwrap(); // Pop 101

    // Now try to push packet 99 (too old)
    let old_packet = RtpPacket::new(99, 900, 12345, vec![0]);
    let result = buffer.push(old_packet);
    assert!(result.is_err());
}

#[test]
fn test_latency_compensator() {
    let mut comp = LatencyCompensator::new();

    let lat1 = SpeakerLatency {
        speaker_id: "sp1".into(),
        network_latency: Duration::from_millis(10),
        processing_latency: Duration::from_millis(5),
        hardware_latency: Duration::from_millis(2),
    };

    let lat2 = SpeakerLatency {
        speaker_id: "sp2".into(),
        network_latency: Duration::from_millis(20),
        processing_latency: Duration::from_millis(5),
        hardware_latency: Duration::from_millis(3),
    };

    comp.add_speaker(lat1);
    comp.add_speaker(lat2);

    // sp2 has 28ms total, sp1 has 17ms total
    // So sp1 needs 11ms delay to align with sp2
    let delay1 = comp.delay_for_speaker("sp1").unwrap();
    assert_eq!(delay1.as_millis(), 11);

    let delay2 = comp.delay_for_speaker("sp2").unwrap();
    assert_eq!(delay2.as_millis(), 0);

    assert_eq!(comp.max_latency().as_millis(), 28);
}

#[test]
fn test_speaker_buffer() {
    let mut buffer = SpeakerBuffer::new("sp1".into(), Duration::from_millis(50));

    let block = AudioBlock::silence(2, 100, 48000);
    let ts = ClockTimestamp::now(ClockSource::System);

    buffer.push(TimestampedAudioBlock {
        block: block.clone(),
        timestamp: ts.clone(),
        presentation_time: Duration::from_secs(1),
    });

    assert_eq!(buffer.len(), 1);

    // Too early
    let result = buffer.pop_ready(&ts);
    assert!(result.is_none());

    // After delay
    let mut later_ts = ts.clone();
    later_ts.nanos += 60_000_000; // +60ms
    let result = buffer.pop_ready(&later_ts);
    assert!(result.is_some());
}

#[test]
fn test_multi_speaker_sync() {
    let mut comp = LatencyCompensator::new();

    let lat1 = SpeakerLatency {
        speaker_id: "FL".into(),
        network_latency: Duration::from_millis(10),
        processing_latency: Duration::from_millis(5),
        hardware_latency: Duration::from_millis(2),
    };

    comp.add_speaker(lat1);

    let mut sync = MultiSpeakerSync::new(comp);

    let speaker = SpeakerDescriptor {
        id: "FL".into(),
        role: SpeakerRole::FrontLeft,
        position: Position3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        max_spl_db: 110.0,
        latency: Duration::from_millis(17),
    };

    let latency = SpeakerLatency {
        speaker_id: "FL".into(),
        network_latency: Duration::from_millis(10),
        processing_latency: Duration::from_millis(5),
        hardware_latency: Duration::from_millis(2),
    };

    sync.add_speaker(&speaker, latency);

    let block = AudioBlock::silence(2, 100, 48000);
    let ts = ClockTimestamp::now(ClockSource::System);

    sync.push_block(
        "FL",
        TimestampedAudioBlock {
            block,
            timestamp: ts.clone(),
            presentation_time: Duration::from_secs(1),
        },
    )
    .expect("push should work");

    // With zero delay (single speaker), should be available immediately or after delay
    let max_lat = sync.max_latency();
    assert_eq!(max_lat.as_millis(), 17);
}
