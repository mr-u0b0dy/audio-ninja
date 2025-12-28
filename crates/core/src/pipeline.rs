// SPDX-License-Identifier: Apache-2.0

use crate::ffmpeg::{Decoder, DemuxConfig, Demuxer};
use crate::iamf::{IamfMetadata, IamfRenderBlock, IamfStreamConfig};
use crate::AudioBlock;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("demux error: {0}")]
    Demux(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("not initialized")]
    NotInitialized,
}

pub struct IamfPipeline<D: Demuxer, C: Decoder> {
    demuxer: D,
    decoder: C,
    config: Option<IamfStreamConfig>,
}

impl<D: Demuxer, C: Decoder> IamfPipeline<D, C> {
    pub fn new(demuxer: D, decoder: C) -> Self {
        Self {
            demuxer,
            decoder,
            config: None,
        }
    }

    pub fn open(&mut self, input_path: &str) -> Result<IamfStreamConfig, PipelineError> {
        let cfg = DemuxConfig {
            input_path: input_path.into(),
            stream_index: None,
        };

        let stream_config = self
            .demuxer
            .open(cfg)
            .map_err(|e| PipelineError::Demux(e.to_string()))?;

        // Configure decoder with first codec from stream
        if let Some(elem) = stream_config.channel_elements.first() {
            self.decoder
                .configure(elem.codec.clone())
                .map_err(|e| PipelineError::Decode(e.to_string()))?;
        }

        self.config = Some(stream_config.clone());
        Ok(stream_config)
    }

    pub fn read_block(&mut self) -> Result<Option<IamfRenderBlock>, PipelineError> {
        let packet = self
            .demuxer
            .read_packet()
            .map_err(|e| PipelineError::Demux(e.to_string()))?;

        let packet = match packet {
            Some(p) => p,
            None => return Ok(None),
        };

        let frame = self
            .decoder
            .decode(&packet)
            .map_err(|e| PipelineError::Decode(e.to_string()))?;

        let frame = match frame {
            Some(f) => f,
            None => return Ok(None),
        };

        let audio = AudioBlock {
            sample_rate: frame.sample_rate,
            channels: frame.channels,
        };

        let metadata = IamfMetadata {
            presentation_id: 0,
            loudness_lufs: None,
            dialog_gain_db: None,
            personalization: None,
            element_gains: vec![],
        };

        Ok(Some(IamfRenderBlock { audio, metadata }))
    }

    pub fn config(&self) -> Option<&IamfStreamConfig> {
        self.config.as_ref()
    }
}
