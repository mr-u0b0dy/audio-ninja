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
pub mod input;
pub mod jitter;
pub mod latency;
pub mod loudness;
pub mod mapping;
pub mod network;
pub mod output;
pub mod pipeline;
pub mod render;
pub mod sync;
pub mod transport;
pub mod vbap;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeakerDescriptor {
    pub id: String,
    pub role: SpeakerRole,
    pub position: Position3,
    pub max_spl_db: f32,
    #[serde(with = "duration_serde")]
    pub latency: Duration,
}

mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs_f64().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = f64::deserialize(deserializer)?;
        Ok(Duration::from_secs_f64(secs))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeakerLayout {
    pub name: String,
    pub speakers: Vec<SpeakerDescriptor>,
}

impl SpeakerLayout {
    pub fn by_id(&self, id: &str) -> Option<&SpeakerDescriptor> {
        self.speakers.iter().find(|s| s.id == id)
    }

    pub fn stereo() -> Self {
        crate::mapping::layout_from_name("stereo").expect("stereo layout")
    }

    pub fn surround_5_1() -> Self {
        crate::mapping::layout_from_name("5.1").expect("5.1 layout")
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
