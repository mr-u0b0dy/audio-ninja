// SPDX-License-Identifier: Apache-2.0

//! Forward Error Correction (FEC) for resilient audio streaming

use crate::AudioBlock;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FecError {
    #[error("Insufficient data for recovery")]
    InsufficientData,
    #[error("Invalid FEC packet")]
    InvalidPacket,
    #[error("Recovery failed")]
    RecoveryFailed,
}

/// Simple XOR-based FEC for audio packets
/// Groups N packets and generates 1 redundancy packet via XOR
#[derive(Clone, Debug)]
pub struct XorFec {
    group_size: usize,
    current_group: Vec<Vec<u8>>,
    group_index: usize,
}

impl XorFec {
    pub fn new(group_size: usize) -> Self {
        Self {
            group_size,
            current_group: Vec::new(),
            group_index: 0,
        }
    }

    /// Add a packet to the current group and return FEC packet if group complete
    pub fn encode(&mut self, packet: &[u8]) -> Option<Vec<u8>> {
        self.current_group.push(packet.to_vec());

        if self.current_group.len() >= self.group_size {
            let fec = self.generate_fec();
            self.current_group.clear();
            self.group_index += 1;
            Some(fec)
        } else {
            None
        }
    }

    fn generate_fec(&self) -> Vec<u8> {
        if self.current_group.is_empty() {
            return Vec::new();
        }

        let max_len = self
            .current_group
            .iter()
            .map(|p| p.len())
            .max()
            .unwrap_or(0);
        let mut fec = vec![0u8; max_len];

        for packet in &self.current_group {
            for (i, &byte) in packet.iter().enumerate() {
                fec[i] ^= byte;
            }
        }

        fec
    }

    /// Recover a missing packet from a group using FEC
    pub fn decode(&self, packets: &[Vec<u8>], fec: &[u8]) -> Result<Vec<u8>, FecError> {
        if packets.len() + 1 < self.group_size {
            return Err(FecError::InsufficientData);
        }

        // XOR all available packets with FEC to recover missing one
        let mut recovered = fec.to_vec();
        for packet in packets {
            for (i, &byte) in packet.iter().enumerate() {
                if i < recovered.len() {
                    recovered[i] ^= byte;
                }
            }
        }

        Ok(recovered)
    }
}

/// Packet loss statistics tracker
#[derive(Clone, Debug, Default)]
pub struct LossStatistics {
    pub total_expected: u64,
    pub total_received: u64,
    pub total_lost: u64,
    pub total_recovered: u64,
    pub consecutive_losses: u64,
    pub max_consecutive_losses: u64,
    last_sequence: Option<u16>,
}

impl LossStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, sequence: u16) {
        if let Some(last) = self.last_sequence {
            let expected_next = last.wrapping_add(1);
            let gap = sequence.wrapping_sub(expected_next) as u64;

            if gap > 0 {
                // Detected packet loss
                self.total_lost += gap;
                self.consecutive_losses += gap;
                self.max_consecutive_losses =
                    self.max_consecutive_losses.max(self.consecutive_losses);
            } else {
                self.consecutive_losses = 0;
            }

            self.total_expected += gap + 1;
        } else {
            self.total_expected = 1;
        }

        self.total_received += 1;
        self.last_sequence = Some(sequence);
    }

    pub fn record_recovery(&mut self) {
        self.total_recovered += 1;
        if self.consecutive_losses > 0 {
            self.consecutive_losses -= 1;
        }
    }

    pub fn loss_rate(&self) -> f64 {
        if self.total_expected == 0 {
            0.0
        } else {
            self.total_lost as f64 / self.total_expected as f64
        }
    }

    pub fn recovery_rate(&self) -> f64 {
        if self.total_lost == 0 {
            0.0
        } else {
            self.total_recovered as f64 / self.total_lost as f64
        }
    }
}

/// Packet loss concealment strategy
#[derive(Clone, Debug, PartialEq)]
pub enum ConcealmentStrategy {
    Silence,
    Repeat,
    Interpolate,
}

/// Packet loss concealer - generates replacement audio when packets are lost
pub struct LossConcealer {
    strategy: ConcealmentStrategy,
    last_block: Option<AudioBlock>,
}

impl LossConcealer {
    pub fn new(strategy: ConcealmentStrategy) -> Self {
        Self {
            strategy,
            last_block: None,
        }
    }

    pub fn update(&mut self, block: AudioBlock) {
        self.last_block = Some(block);
    }

    pub fn conceal(&self, sample_rate: u32, num_channels: usize, frames: usize) -> AudioBlock {
        match self.strategy {
            ConcealmentStrategy::Silence => AudioBlock::silence(num_channels, frames, sample_rate),
            ConcealmentStrategy::Repeat => {
                if let Some(ref last) = self.last_block {
                    last.clone()
                } else {
                    AudioBlock::silence(num_channels, frames, sample_rate)
                }
            }
            ConcealmentStrategy::Interpolate => {
                // Simple fade-out interpolation
                if let Some(ref last) = self.last_block {
                    let mut block = last.clone();
                    for channel in &mut block.channels {
                        let len = channel.len();
                        for (i, sample) in channel.iter_mut().enumerate() {
                            let fade = 1.0 - (i as f32 / len as f32);
                            *sample *= fade;
                        }
                    }
                    block
                } else {
                    AudioBlock::silence(num_channels, frames, sample_rate)
                }
            }
        }
    }
}

/// FEC-enhanced receiver with packet recovery
pub struct FecReceiver {
    fec: XorFec,
    concealer: LossConcealer,
    stats: LossStatistics,
    group_cache: HashMap<usize, Vec<Vec<u8>>>,
    fec_cache: HashMap<usize, Vec<u8>>,
}

impl FecReceiver {
    pub fn new(group_size: usize, concealment: ConcealmentStrategy) -> Self {
        Self {
            fec: XorFec::new(group_size),
            concealer: LossConcealer::new(concealment),
            stats: LossStatistics::new(),
            group_cache: HashMap::new(),
            fec_cache: HashMap::new(),
        }
    }

    pub fn process_packet(&mut self, sequence: u16, packet: Vec<u8>) {
        self.stats.update(sequence);

        let group_id = sequence as usize / self.fec.group_size;
        self.group_cache
            .entry(group_id)
            .or_default()
            .push(packet);
    }

    pub fn process_fec_packet(&mut self, group_id: usize, fec: Vec<u8>) {
        self.fec_cache.insert(group_id, fec);
        self.try_recover(group_id);
    }

    fn try_recover(&mut self, group_id: usize) {
        if let (Some(packets), Some(fec)) = (
            self.group_cache.get(&group_id),
            self.fec_cache.get(&group_id),
        ) {
            if packets.len() == self.fec.group_size - 1 {
                // Exactly one packet missing - can recover
                if let Ok(_recovered) = self.fec.decode(packets, fec) {
                    self.stats.record_recovery();
                }
            }
        }
    }

    pub fn statistics(&self) -> &LossStatistics {
        &self.stats
    }

    pub fn update_concealer(&mut self, block: AudioBlock) {
        self.concealer.update(block);
    }

    pub fn conceal_loss(&self, sample_rate: u32, num_channels: usize, frames: usize) -> AudioBlock {
        self.concealer.conceal(sample_rate, num_channels, frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_fec_encode() {
        let mut fec = XorFec::new(3);

        assert!(fec.encode(&[1, 2, 3]).is_none());
        assert!(fec.encode(&[4, 5, 6]).is_none());
        let result = fec.encode(&[7, 8, 9]);

        assert!(result.is_some());
        let fec_packet = result.unwrap();
        // XOR of [1,2,3], [4,5,6], [7,8,9] = [2,15,12]
        assert_eq!(fec_packet, vec![2, 15, 12]);
    }

    #[test]
    fn test_xor_fec_decode() {
        let fec = XorFec::new(3);

        let packets = vec![vec![1, 2, 3], vec![7, 8, 9]];
        let fec_packet = vec![2, 15, 12];

        let recovered = fec.decode(&packets, &fec_packet).unwrap();
        assert_eq!(recovered, vec![4, 5, 6]);
    }

    #[test]
    fn test_loss_statistics() {
        let mut stats = LossStatistics::new();

        stats.update(0);
        stats.update(1);
        stats.update(2);

        assert_eq!(stats.total_received, 3);
        assert_eq!(stats.total_lost, 0);
        assert_eq!(stats.loss_rate(), 0.0);

        // Skip sequence 3
        stats.update(4);

        assert_eq!(stats.total_lost, 1);
        assert_eq!(stats.consecutive_losses, 1);
    }

    #[test]
    fn test_loss_statistics_wraparound() {
        let mut stats = LossStatistics::new();

        stats.update(65534);
        stats.update(65535);
        stats.update(0); // Wraparound

        assert_eq!(stats.total_lost, 0);
        assert_eq!(stats.total_received, 3);
    }

    #[test]
    fn test_loss_concealment_silence() {
        let concealer = LossConcealer::new(ConcealmentStrategy::Silence);
        let block = concealer.conceal(48000, 2, 480);

        assert_eq!(block.channels.len(), 2);
        assert_eq!(block.channels[0].len(), 480);
        assert!(block.channels[0].iter().all(|&s| s == 0.0));
    }

    #[test]
    fn test_loss_concealment_repeat() {
        let mut concealer = LossConcealer::new(ConcealmentStrategy::Repeat);

        let original = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        };

        concealer.update(original.clone());
        let concealed = concealer.conceal(48000, 2, 2);

        assert_eq!(concealed.channels, original.channels);
    }
}
