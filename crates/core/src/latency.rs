// SPDX-License-Identifier: Apache-2.0

use crate::transport::ClockTimestamp;
use crate::{AudioBlock, SpeakerDescriptor};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct SpeakerLatency {
    pub speaker_id: String,
    pub network_latency: Duration,
    pub processing_latency: Duration,
    pub hardware_latency: Duration,
}

impl SpeakerLatency {
    pub fn total(&self) -> Duration {
        self.network_latency + self.processing_latency + self.hardware_latency
    }
}

pub struct LatencyCompensator {
    speaker_latencies: HashMap<String, SpeakerLatency>,
    max_latency: Duration,
}

impl LatencyCompensator {
    pub fn new() -> Self {
        Self {
            speaker_latencies: HashMap::new(),
            max_latency: Duration::ZERO,
        }
    }

    pub fn add_speaker(&mut self, latency: SpeakerLatency) {
        let total = latency.total();
        if total > self.max_latency {
            self.max_latency = total;
        }
        self.speaker_latencies
            .insert(latency.speaker_id.clone(), latency);
    }

    pub fn remove_speaker(&mut self, speaker_id: &str) {
        self.speaker_latencies.remove(speaker_id);
        self.recalculate_max();
    }

    pub fn update_speaker(&mut self, latency: SpeakerLatency) {
        self.speaker_latencies
            .insert(latency.speaker_id.clone(), latency);
        self.recalculate_max();
    }

    fn recalculate_max(&mut self) {
        self.max_latency = self
            .speaker_latencies
            .values()
            .map(|l| l.total())
            .max()
            .unwrap_or(Duration::ZERO);
    }

    pub fn delay_for_speaker(&self, speaker_id: &str) -> Option<Duration> {
        let speaker_lat = self.speaker_latencies.get(speaker_id)?;
        let total = speaker_lat.total();
        Some(self.max_latency.saturating_sub(total))
    }

    pub fn max_latency(&self) -> Duration {
        self.max_latency
    }

    pub fn speaker_count(&self) -> usize {
        self.speaker_latencies.len()
    }
}

impl Default for LatencyCompensator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct TimestampedAudioBlock {
    pub block: AudioBlock,
    pub timestamp: ClockTimestamp,
    pub presentation_time: Duration,
}

pub struct SpeakerBuffer {
    speaker_id: String,
    delay: Duration,
    buffer: Vec<TimestampedAudioBlock>,
}

impl SpeakerBuffer {
    pub fn new(speaker_id: String, delay: Duration) -> Self {
        Self {
            speaker_id,
            delay,
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, block: TimestampedAudioBlock) {
        self.buffer.push(block);
    }

    pub fn pop_ready(&mut self, now: &ClockTimestamp) -> Option<AudioBlock> {
        if self.buffer.is_empty() {
            return None;
        }

        let target_time = self.buffer[0].timestamp.to_duration() + self.delay;
        let current_time = now.to_duration();

        if current_time >= target_time {
            Some(self.buffer.remove(0).block)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

pub struct MultiSpeakerSync {
    compensator: LatencyCompensator,
    buffers: HashMap<String, SpeakerBuffer>,
}

impl MultiSpeakerSync {
    pub fn new(compensator: LatencyCompensator) -> Self {
        Self {
            compensator,
            buffers: HashMap::new(),
        }
    }

    pub fn add_speaker(&mut self, speaker: &SpeakerDescriptor, latency: SpeakerLatency) {
        self.compensator.add_speaker(latency.clone());

        let delay = self
            .compensator
            .delay_for_speaker(&speaker.id)
            .unwrap_or(Duration::ZERO);

        self.buffers.insert(
            speaker.id.clone(),
            SpeakerBuffer::new(speaker.id.clone(), delay),
        );
    }

    pub fn push_block(
        &mut self,
        speaker_id: &str,
        block: TimestampedAudioBlock,
    ) -> anyhow::Result<()> {
        let buffer = self
            .buffers
            .get_mut(speaker_id)
            .ok_or_else(|| anyhow::anyhow!("speaker not found: {}", speaker_id))?;

        buffer.push(block);
        Ok(())
    }

    pub fn pop_ready(&mut self, speaker_id: &str, now: &ClockTimestamp) -> Option<AudioBlock> {
        self.buffers.get_mut(speaker_id)?.pop_ready(now)
    }

    pub fn max_latency(&self) -> Duration {
        self.compensator.max_latency()
    }
}
