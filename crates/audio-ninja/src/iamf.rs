// SPDX-License-Identifier: Apache-2.0

use crate::{AudioBlock, SpeakerLayout, SpeakerRole};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum IamfError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error("unsupported feature: {0}")]
    Unsupported(String),
    #[error("decode error: {0}")]
    Decode(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioElementType {
    SceneBased,
    ChannelBased,
    ObjectBased,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelAudioElement {
    pub element_id: u32,
    pub channel_labels: Vec<SpeakerRole>,
    pub codec: CodecConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectAudioElement {
    pub element_id: u32,
    pub object_count: u16,
    pub codec: CodecConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SceneAudioElement {
    pub element_id: u32,
    pub ambisonics_order: u8,
    pub codec: CodecConfig,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodecConfig {
    Opus {
        sample_rate: u32,
        channels: u16,
    },
    Aac {
        sample_rate: u32,
        channels: u16,
    },
    Flac {
        sample_rate: u32,
        channels: u16,
    },
    Pcm {
        sample_rate: u32,
        bit_depth: u8,
        channels: u16,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixPresentationElement {
    pub element_id: u32,
    pub gain_db: f32,
    pub rendering_config: RenderingConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderingConfig {
    Binaural,
    Stereo,
    Surround { layout: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixPresentation {
    pub presentation_id: u32,
    pub elements: Vec<MixPresentationElement>,
    pub loudness_lufs: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IamfStreamConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub frame_duration: Duration,
    pub mix_presentations: Vec<MixPresentation>,
    pub channel_elements: Vec<ChannelAudioElement>,
    pub object_elements: Vec<ObjectAudioElement>,
    pub scene_elements: Vec<SceneAudioElement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IamfMetadata {
    pub presentation_id: u32,
    pub loudness_lufs: Option<f32>,
    pub dialog_gain_db: Option<f32>,
    pub personalization: Option<String>,
    pub element_gains: Vec<(u32, f32)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IamfRenderBlock {
    pub audio: AudioBlock,
    pub metadata: IamfMetadata,
}

pub trait IamfDecoder {
    fn configure(&mut self, cfg: IamfStreamConfig) -> Result<(), IamfError>;
    fn decode_block(&mut self, payload: &[u8]) -> Result<IamfRenderBlock, IamfError>;
}

pub struct ReferenceIamfDecoder {
    cfg: Option<IamfStreamConfig>,
}

impl ReferenceIamfDecoder {
    pub fn new() -> Self {
        Self { cfg: None }
    }
}

impl Default for ReferenceIamfDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl IamfDecoder for ReferenceIamfDecoder {
    fn configure(&mut self, cfg: IamfStreamConfig) -> Result<(), IamfError> {
        self.cfg = Some(cfg);
        Ok(())
    }

    fn decode_block(&mut self, _payload: &[u8]) -> Result<IamfRenderBlock, IamfError> {
        let cfg = self
            .cfg
            .clone()
            .ok_or_else(|| IamfError::Parse("decoder not configured".into()))?;

        let frames = (cfg.sample_rate as f32 * cfg.frame_duration.as_secs_f32()) as usize;
        let audio = AudioBlock::silence(cfg.channel_count as usize, frames, cfg.sample_rate);

        Ok(IamfRenderBlock {
            audio,
            metadata: IamfMetadata {
                presentation_id: 0,
                loudness_lufs: None,
                dialog_gain_db: None,
                personalization: None,
                element_gains: vec![],
            },
        })
    }
}

pub fn map_to_layout(block: &IamfRenderBlock, layout: &SpeakerLayout) -> AudioBlock {
    // Placeholder: map IAMF channels/objects to physical speaker positions.
    // Real implementation will:
    // 1. Extract channel-based elements and map labels to speaker roles
    // 2. Pan object-based elements using VBAP/vector panning to speaker positions
    // 3. Decode ambisonics scene-based elements to speaker array
    // 4. Apply downmix/upmix rules if channel count doesn't match layout
    // 5. Apply element gains from metadata

    let target_channels = layout.speakers.len();
    let source_channels = block.audio.channels.len();

    if source_channels == target_channels {
        return block.audio.clone();
    }

    // Simple passthrough or silence for now
    if source_channels > target_channels {
        // Downmix: take first N channels
        AudioBlock {
            sample_rate: block.audio.sample_rate,
            channels: block.audio.channels[..target_channels].to_vec(),
        }
    } else {
        // Upmix: pad with silence
        let mut channels = block.audio.channels.clone();
        let frame_len = block.audio.frame_len();
        while channels.len() < target_channels {
            channels.push(vec![0.0; frame_len]);
        }
        AudioBlock {
            sample_rate: block.audio.sample_rate,
            channels,
        }
    }
}

pub fn map_channel_element_to_layout(
    element: &ChannelAudioElement,
    audio: &AudioBlock,
    layout: &SpeakerLayout,
) -> AudioBlock {
    // Map channel labels to speaker roles in layout
    let frame_len = audio.frame_len();
    let mut output = vec![vec![0.0; frame_len]; layout.speakers.len()];

    for (ch_idx, label) in element.channel_labels.iter().enumerate() {
        if ch_idx >= audio.channels.len() {
            break;
        }

        // Find matching speaker in layout
        if let Some(spk_idx) = layout.speakers.iter().position(|s| &s.role == label) {
            output[spk_idx] = audio.channels[ch_idx].clone();
        }
    }

    AudioBlock {
        sample_rate: audio.sample_rate,
        channels: output,
    }
}

pub fn map_object_element_to_layout(
    _element: &ObjectAudioElement,
    audio: &AudioBlock,
    _layout: &SpeakerLayout,
) -> AudioBlock {
    // Placeholder: implement VBAP (Vector Base Amplitude Panning) or similar
    // to position audio objects at 3D coordinates and render to speaker array
    audio.clone()
}
