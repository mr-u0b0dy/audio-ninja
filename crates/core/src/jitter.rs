// SPDX-License-Identifier: Apache-2.0

use crate::transport::RtpPacket;
use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct JitterBufferConfig {
    pub target_delay: Duration,
    pub max_delay: Duration,
    pub max_packets: usize,
}

impl Default for JitterBufferConfig {
    fn default() -> Self {
        Self {
            target_delay: Duration::from_millis(50),
            max_delay: Duration::from_millis(200),
            max_packets: 100,
        }
    }
}

#[derive(Debug)]
pub enum JitterBufferError {
    Full,
    Underrun,
    TooOld,
}

pub struct JitterBuffer {
    config: JitterBufferConfig,
    buffer: BTreeMap<u16, RtpPacket>,
    last_popped: Option<u16>,
    packets_received: u64,
    packets_dropped: u64,
    packets_late: u64,
}

impl JitterBuffer {
    pub fn new(config: JitterBufferConfig) -> Self {
        Self {
            config,
            buffer: BTreeMap::new(),
            last_popped: None,
            packets_received: 0,
            packets_dropped: 0,
            packets_late: 0,
        }
    }

    pub fn push(&mut self, packet: RtpPacket) -> Result<(), JitterBufferError> {
        self.packets_received += 1;

        let seq = packet.header.sequence.0;

        // Check if packet is too old
        if let Some(last_seq) = self.last_popped {
            let seq_diff = seq.wrapping_sub(last_seq);
            if seq_diff > 32768 {
                // Packet is from the past (considering wraparound)
                self.packets_late += 1;
                return Err(JitterBufferError::TooOld);
            }
        }

        // Check if buffer is full
        if self.buffer.len() >= self.config.max_packets {
            self.packets_dropped += 1;
            return Err(JitterBufferError::Full);
        }

        self.buffer.insert(seq, packet);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<RtpPacket, JitterBufferError> {
        if self.buffer.is_empty() {
            return Err(JitterBufferError::Underrun);
        }

        // Get the oldest packet
        let (&seq, _) = self.buffer.iter().next().unwrap();
        let packet = self.buffer.remove(&seq).unwrap();

        self.last_popped = Some(seq);
        Ok(packet)
    }

    pub fn ready(&self) -> bool {
        if self.buffer.is_empty() {
            return false;
        }

        // Need at least target_delay worth of packets
        let target_packets = (self.config.target_delay.as_millis() / 20) as usize; // Assume 20ms packets
        self.buffer.len() >= target_packets.max(3)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn stats(&self) -> JitterBufferStats {
        JitterBufferStats {
            buffered: self.buffer.len(),
            received: self.packets_received,
            dropped: self.packets_dropped,
            late: self.packets_late,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.last_popped = None;
    }
}

impl Default for JitterBuffer {
    fn default() -> Self {
        Self::new(JitterBufferConfig::default())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JitterBufferStats {
    pub buffered: usize,
    pub received: u64,
    pub dropped: u64,
    pub late: u64,
}
