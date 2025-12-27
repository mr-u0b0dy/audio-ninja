// SPDX-License-Identifier: Apache-2.0

pub mod ble;
pub mod calibration;
pub mod control;
pub mod dsp;
pub mod dspconfig;
pub mod fec;
pub mod ffmpeg;
pub mod hoa;
pub mod hrtf;
pub mod iamf;
pub mod jitter;
pub mod latency;
pub mod loudness;
pub mod mapping;
pub mod network;
pub mod pipeline;
pub mod render;
pub mod sync;
pub mod transport;
pub mod vbap;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum SpeakerRole {
    FrontLeft,
    FrontRight,
    Center,
    Subwoofer,
    SideLeft,
    SideRight,
    RearLeft,
    RearRight,
    FrontHeightLeft,
    FrontHeightRight,
    RearHeightLeft,
    RearHeightRight,
    TopFrontLeft,
    TopFrontRight,
    TopRearLeft,
    TopRearRight,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpeakerDescriptor {
    pub id: String,
    pub role: SpeakerRole,
    pub position: Position3,
    pub max_spl_db: f32,
    pub latency: Duration,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpeakerLayout {
    pub name: String,
    pub speakers: Vec<SpeakerDescriptor>,
}

impl SpeakerLayout {
    pub fn by_id(&self, id: &str) -> Option<&SpeakerDescriptor> {
        self.speakers.iter().find(|s| s.id == id)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioBlock {
    pub sample_rate: u32,
    pub channels: Vec<Vec<f32>>, // channel-major interleaving: channels[channel][frame]
}

impl AudioBlock {
    pub fn silence(channels: usize, frames: usize, sample_rate: u32) -> Self {
        Self {
            sample_rate,
            channels: vec![vec![0.0; frames]; channels],
        }
    }

    pub fn frame_len(&self) -> usize {
        self.channels.first().map(|c| c.len()).unwrap_or(0)
    }
}
