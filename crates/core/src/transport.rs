// SPDX-License-Identifier: Apache-2.0

use crate::AudioBlock;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RtpTimestamp(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RtpSequence(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ssrc(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum ClockSource {
    Ptp,
    Ntp,
    System,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClockTimestamp {
    pub seconds: u64,
    pub nanos: u32,
    pub source: ClockSource,
}

impl ClockTimestamp {
    pub fn now(source: ClockSource) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        Self {
            seconds: now.as_secs(),
            nanos: now.subsec_nanos(),
            source,
        }
    }

    pub fn to_duration(&self) -> Duration {
        Duration::new(self.seconds, self.nanos)
    }

    pub fn skew_from(&self, other: &ClockTimestamp) -> Duration {
        let self_dur = self.to_duration();
        let other_dur = other.to_duration();

        if self_dur > other_dur {
            self_dur - other_dur
        } else {
            other_dur - self_dur
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RtpHeader {
    pub version: u8,
    pub padding: bool,
    pub extension: bool,
    pub csrc_count: u8,
    pub marker: bool,
    pub payload_type: u8,
    pub sequence: RtpSequence,
    pub timestamp: RtpTimestamp,
    pub ssrc: Ssrc,
}

impl RtpHeader {
    pub fn new(sequence: u16, timestamp: u32, ssrc: u32) -> Self {
        Self {
            version: 2,
            padding: false,
            extension: false,
            csrc_count: 0,
            marker: false,
            payload_type: 96, // Dynamic
            sequence: RtpSequence(sequence),
            timestamp: RtpTimestamp(timestamp),
            ssrc: Ssrc(ssrc),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 12];

        buf[0] = (self.version << 6) | (self.csrc_count & 0x0f);
        if self.padding {
            buf[0] |= 0x20;
        }
        if self.extension {
            buf[0] |= 0x10;
        }

        buf[1] = self.payload_type & 0x7f;
        if self.marker {
            buf[1] |= 0x80;
        }

        buf[2..4].copy_from_slice(&self.sequence.0.to_be_bytes());
        buf[4..8].copy_from_slice(&self.timestamp.0.to_be_bytes());
        buf[8..12].copy_from_slice(&self.ssrc.0.to_be_bytes());

        buf
    }

    pub fn deserialize(buf: &[u8]) -> Option<Self> {
        if buf.len() < 12 {
            return None;
        }

        let version = (buf[0] >> 6) & 0x03;
        let padding = (buf[0] & 0x20) != 0;
        let extension = (buf[0] & 0x10) != 0;
        let csrc_count = buf[0] & 0x0f;

        let marker = (buf[1] & 0x80) != 0;
        let payload_type = buf[1] & 0x7f;

        let sequence = u16::from_be_bytes([buf[2], buf[3]]);
        let timestamp = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let ssrc = u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]);

        Some(Self {
            version,
            padding,
            extension,
            csrc_count,
            marker,
            payload_type,
            sequence: RtpSequence(sequence),
            timestamp: RtpTimestamp(timestamp),
            ssrc: Ssrc(ssrc),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RtpPacket {
    pub header: RtpHeader,
    pub clock: ClockTimestamp,
    pub payload: Vec<u8>,
}

impl RtpPacket {
    pub fn new(sequence: u16, timestamp: u32, ssrc: u32, payload: Vec<u8>) -> Self {
        Self {
            header: RtpHeader::new(sequence, timestamp, ssrc),
            clock: ClockTimestamp::now(ClockSource::System),
            payload,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = self.header.serialize();
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn deserialize(buf: &[u8]) -> Option<Self> {
        let header = RtpHeader::deserialize(buf)?;
        let payload = buf[12..].to_vec();

        Some(Self {
            header,
            clock: ClockTimestamp::now(ClockSource::System),
            payload,
        })
    }
}

pub trait TransportSender {
    fn send(&mut self, packet: RtpPacket) -> anyhow::Result<()>;
}

pub trait TransportReceiver {
    fn poll(&mut self) -> anyhow::Result<Option<RtpPacket>>;
}

pub trait SyncStrategy {
    fn anchor_time(&self) -> SystemTime;
    fn max_skew(&self) -> Duration;
}

pub struct LoopbackTransport {
    queue: Vec<RtpPacket>,
    sequence: u16,
    ssrc: u32,
}

impl LoopbackTransport {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            sequence: 0,
            ssrc: rand::random(),
        }
    }

    pub fn with_ssrc(ssrc: u32) -> Self {
        Self {
            queue: Vec::new(),
            sequence: 0,
            ssrc,
        }
    }
}

impl Default for LoopbackTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportSender for LoopbackTransport {
    fn send(&mut self, packet: RtpPacket) -> anyhow::Result<()> {
        self.queue.push(packet);
        self.sequence = self.sequence.wrapping_add(1);
        Ok(())
    }
}

impl TransportReceiver for LoopbackTransport {
    fn poll(&mut self) -> anyhow::Result<Option<RtpPacket>> {
        if self.queue.is_empty() {
            return Ok(None);
        }
        Ok(Some(self.queue.remove(0)))
    }
}

pub fn serialize_audio_block(block: &AudioBlock) -> Vec<u8> {
    bincode::serialize(block).unwrap_or_default()
}

pub fn deserialize_audio_block(bytes: &[u8]) -> anyhow::Result<AudioBlock> {
    let block: AudioBlock = bincode::deserialize(bytes)?;
    Ok(block)
}

pub fn audio_block_to_rtp(
    block: &AudioBlock,
    sequence: u16,
    timestamp: u32,
    ssrc: u32,
) -> RtpPacket {
    let payload = serialize_audio_block(block);
    RtpPacket::new(sequence, timestamp, ssrc, payload)
}

pub fn rtp_to_audio_block(packet: &RtpPacket) -> anyhow::Result<AudioBlock> {
    deserialize_audio_block(&packet.payload)
}
