// SPDX-License-Identifier: Apache-2.0

use crate::iamf::{CodecConfig, IamfStreamConfig};

#[derive(Debug, thiserror::Error)]
pub enum FfmpegError {
    #[error("initialization error: {0}")]
    Init(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("format error: {0}")]
    Format(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DemuxConfig {
    pub input_path: String,
    pub stream_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodecPacket {
    pub pts: i64,
    pub dts: i64,
    pub data: Vec<u8>,
    pub is_keyframe: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DecodedFrame {
    pub pts: i64,
    pub sample_rate: u32,
    pub channels: Vec<Vec<f32>>,
}

pub trait Demuxer {
    fn open(&mut self, cfg: DemuxConfig) -> Result<IamfStreamConfig, FfmpegError>;
    fn read_packet(&mut self) -> Result<Option<CodecPacket>, FfmpegError>;
}

pub trait Decoder {
    fn configure(&mut self, codec: CodecConfig) -> Result<(), FfmpegError>;
    fn decode(&mut self, packet: &CodecPacket) -> Result<Option<DecodedFrame>, FfmpegError>;
    fn flush(&mut self) -> Result<Vec<DecodedFrame>, FfmpegError>;
}

pub struct StubDemuxer {
    configured: bool,
}

impl StubDemuxer {
    pub fn new() -> Self {
        Self { configured: false }
    }
}

impl Default for StubDemuxer {
    fn default() -> Self {
        Self::new()
    }
}

impl Demuxer for StubDemuxer {
    fn open(&mut self, _cfg: DemuxConfig) -> Result<IamfStreamConfig, FfmpegError> {
        self.configured = true;
        Ok(IamfStreamConfig {
            sample_rate: 48000,
            channel_count: 6,
            frame_duration: std::time::Duration::from_millis(20),
            mix_presentations: vec![],
            channel_elements: vec![],
            object_elements: vec![],
            scene_elements: vec![],
        })
    }

    fn read_packet(&mut self) -> Result<Option<CodecPacket>, FfmpegError> {
        if !self.configured {
            return Err(FfmpegError::Format("demuxer not opened".into()));
        }
        Ok(None)
    }
}

pub struct StubDecoder {
    codec: Option<CodecConfig>,
}

impl StubDecoder {
    pub fn new() -> Self {
        Self { codec: None }
    }
}

impl Default for StubDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for StubDecoder {
    fn configure(&mut self, codec: CodecConfig) -> Result<(), FfmpegError> {
        self.codec = Some(codec);
        Ok(())
    }

    fn decode(&mut self, _packet: &CodecPacket) -> Result<Option<DecodedFrame>, FfmpegError> {
        let codec = self
            .codec
            .as_ref()
            .ok_or_else(|| FfmpegError::Decode("decoder not configured".into()))?;

        let (sample_rate, channels) = match codec {
            CodecConfig::Opus {
                sample_rate,
                channels,
            } => (*sample_rate, *channels as usize),
            CodecConfig::Aac {
                sample_rate,
                channels,
            } => (*sample_rate, *channels as usize),
            CodecConfig::Flac {
                sample_rate,
                channels,
            } => (*sample_rate, *channels as usize),
            CodecConfig::Pcm {
                sample_rate,
                channels,
                ..
            } => (*sample_rate, *channels as usize),
        };

        Ok(Some(DecodedFrame {
            pts: 0,
            sample_rate,
            channels: vec![vec![0.0; 960]; channels],
        }))
    }

    fn flush(&mut self) -> Result<Vec<DecodedFrame>, FfmpegError> {
        Ok(vec![])
    }
}
