// SPDX-License-Identifier: Apache-2.0

//! UDP/RTP networking with mDNS discovery for wireless speaker transport

use crate::fec::{LossStatistics, XorFec};
use crate::transport::RtpPacket;
use crate::AudioBlock;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid packet")]
    InvalidPacket,
    #[error("Speaker not found: {0}")]
    SpeakerNotFound(String),
}

/// UDP-based RTP sender for wireless audio streaming
pub struct UdpRtpSender {
    socket: UdpSocket,
    target: SocketAddr,
    ssrc: u32,
    sequence: u16,
    timestamp: u32,
    fec: Option<XorFec>,
}

impl UdpRtpSender {
    pub fn new(bind_addr: &str, target: SocketAddr, ssrc: u32) -> Result<Self, NetworkError> {
        let socket = UdpSocket::bind(bind_addr)?;
        socket.set_nonblocking(false)?;

        Ok(Self {
            socket,
            target,
            ssrc,
            sequence: 0,
            timestamp: 0,
            fec: None,
        })
    }

    pub fn enable_fec(&mut self, group_size: usize) {
        self.fec = Some(XorFec::new(group_size));
    }

    pub fn send_block(&mut self, block: &AudioBlock) -> Result<(), NetworkError> {
        // Convert AudioBlock to RTP packet using existing transport functions
        let packet =
            crate::transport::audio_block_to_rtp(block, self.sequence, self.timestamp, self.ssrc);

        // Serialize and send
        let serialized = packet.serialize();

        self.socket.send_to(&serialized, self.target)?;

        // Generate FEC packet if enabled
        if let Some(ref mut fec) = self.fec {
            if let Some(fec_packet) = fec.encode(&serialized) {
                // Send FEC packet with marker bit set
                self.socket.send_to(&fec_packet, self.target)?;
            }
        }

        // Update sequence and timestamp
        self.sequence = self.sequence.wrapping_add(1);
        let frames = block.channels.first().map(|ch| ch.len()).unwrap_or(0);
        self.timestamp = self.timestamp.wrapping_add(frames as u32);

        Ok(())
    }

    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    pub fn sequence(&self) -> u16 {
        self.sequence
    }
}

/// UDP-based RTP receiver for wireless audio streaming
pub struct UdpRtpReceiver {
    socket: UdpSocket,
    buffer: Vec<u8>,
    stats: LossStatistics,
}

impl UdpRtpReceiver {
    pub fn new(bind_addr: &str) -> Result<Self, NetworkError> {
        let socket = UdpSocket::bind(bind_addr)?;
        socket.set_nonblocking(false)?;

        Ok(Self {
            socket,
            buffer: vec![0u8; 65536], // Max UDP packet size
            stats: LossStatistics::new(),
        })
    }

    pub fn local_addr(&self) -> Result<SocketAddr, NetworkError> {
        Ok(self.socket.local_addr()?)
    }

    pub fn set_timeout(&self, timeout: Option<Duration>) -> Result<(), NetworkError> {
        self.socket.set_read_timeout(timeout)?;
        Ok(())
    }

    pub fn recv_packet(&mut self) -> Result<(RtpPacket, SocketAddr), NetworkError> {
        let (len, addr) = self.socket.recv_from(&mut self.buffer)?;

        let packet =
            RtpPacket::deserialize(&self.buffer[..len]).ok_or(NetworkError::InvalidPacket)?;

        // Update loss statistics
        self.stats.update(packet.header.sequence.0);

        Ok((packet, addr))
    }

    pub fn recv_block(&mut self) -> Result<(AudioBlock, SocketAddr), NetworkError> {
        let (packet, addr) = self.recv_packet()?;

        let block = crate::transport::rtp_to_audio_block(&packet)
            .map_err(|_| NetworkError::InvalidPacket)?;

        Ok((block, addr))
    }

    pub fn statistics(&self) -> &LossStatistics {
        &self.stats
    }
}

/// Speaker information for discovery
#[derive(Clone, Debug, PartialEq)]
pub struct SpeakerInfo {
    pub id: String,
    pub name: String,
    pub address: SocketAddr,
    pub capabilities: SpeakerCapabilities,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpeakerCapabilities {
    pub max_sample_rate: u32,
    pub channels: u8,
    pub codecs: Vec<String>,
    pub layouts: Vec<String>,
}

impl Default for SpeakerCapabilities {
    fn default() -> Self {
        Self {
            max_sample_rate: 48000,
            channels: 2,
            codecs: vec!["PCM".into(), "Opus".into()],
            layouts: vec!["stereo".into(), "5.1".into()],
        }
    }
}

/// mDNS service discovery for speakers
pub struct SpeakerDiscovery {
    speakers: Arc<Mutex<Vec<SpeakerInfo>>>,
    _service_type: String,
}

impl SpeakerDiscovery {
    pub fn new() -> Self {
        Self {
            speakers: Arc::new(Mutex::new(Vec::new())),
            _service_type: "_audio-ninja._udp.local.".into(),
        }
    }

    pub fn start_discovery(&self) -> Result<(), NetworkError> {
        // Placeholder for mDNS discovery
        // Real implementation would use mdns-sd crate to browse for services
        Ok(())
    }

    pub fn register_speaker(&self, info: SpeakerInfo) -> Result<(), NetworkError> {
        let mut speakers = self.speakers.lock().unwrap();
        speakers.push(info);
        Ok(())
    }

    pub fn list_speakers(&self) -> Vec<SpeakerInfo> {
        self.speakers.lock().unwrap().clone()
    }

    pub fn find_speaker(&self, id: &str) -> Option<SpeakerInfo> {
        self.speakers
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.id == id)
            .cloned()
    }

    pub fn remove_speaker(&self, id: &str) {
        let mut speakers = self.speakers.lock().unwrap();
        speakers.retain(|s| s.id != id);
    }
}

impl Default for SpeakerDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-speaker UDP RTP broadcaster
pub struct MultiSpeakerSender {
    senders: Vec<UdpRtpSender>,
    discovery: Arc<SpeakerDiscovery>,
}

impl MultiSpeakerSender {
    pub fn new(_bind_addr: &str, discovery: Arc<SpeakerDiscovery>) -> Self {
        Self {
            senders: Vec::new(),
            discovery,
        }
    }

    pub fn add_speaker(&mut self, speaker_id: &str, bind_addr: &str) -> Result<(), NetworkError> {
        let speaker = self
            .discovery
            .find_speaker(speaker_id)
            .ok_or_else(|| NetworkError::SpeakerNotFound(speaker_id.into()))?;

        let ssrc = rand::random();
        let sender = UdpRtpSender::new(bind_addr, speaker.address, ssrc)?;
        self.senders.push(sender);

        Ok(())
    }

    pub fn broadcast_block(&mut self, block: &AudioBlock) -> Result<(), NetworkError> {
        for sender in &mut self.senders {
            sender.send_block(block)?;
        }
        Ok(())
    }

    pub fn speaker_count(&self) -> usize {
        self.senders.len()
    }
}
