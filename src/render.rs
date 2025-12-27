// SPDX-License-Identifier: Apache-2.0

use crate::loudness::{DynamicRangeControl, HeadroomManager, LoudnessNormalizer, LoudnessTarget};
use crate::{AudioBlock, SpeakerLayout};
use std::time::Duration;

/// Options for audio rendering
#[derive(Clone, Debug, PartialEq)]
pub struct RenderOptions {
    /// Target speaker layout
    pub target_layout: SpeakerLayout,
    /// Headroom in dB (safety margin for clipping prevention)
    pub headroom_db: f32,
    /// Maximum allowed latency
    pub max_latency: Duration,
    /// Target loudness for normalization
    pub target_loudness: Option<LoudnessTarget>,
    /// Enable dynamic range compression
    pub enable_drc: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            target_layout: SpeakerLayout {
                name: "stereo".into(),
                speakers: Vec::new(),
            },
            headroom_db: 3.0,
            max_latency: Duration::from_millis(100),
            target_loudness: Some(LoudnessTarget::StreamingMusic),
            enable_drc: false,
        }
    }
}

/// Trait for audio rendering engines
pub trait Renderer {
    /// Render audio block with specified options
    fn render(&mut self, input: AudioBlock, opts: &RenderOptions) -> AudioBlock;
}

/// Reference renderer with loudness management and headroom protection
pub struct ReferenceRenderer {
    loudness_normalizer: Option<LoudnessNormalizer>,
    headroom_manager: HeadroomManager,
    drc: Option<DynamicRangeControl>,
    sample_rate: u32,
}

impl ReferenceRenderer {
    /// Create new reference renderer
    pub fn new(sample_rate: u32) -> Self {
        Self {
            loudness_normalizer: None,
            headroom_manager: HeadroomManager::new(3.0, sample_rate),
            drc: None,
            sample_rate,
        }
    }

    /// Enable loudness normalization
    pub fn set_loudness_target(&mut self, target: LoudnessTarget) {
        self.loudness_normalizer = Some(LoudnessNormalizer::new(self.sample_rate, target));
    }

    /// Disable loudness normalization
    pub fn disable_loudness_normalization(&mut self) {
        self.loudness_normalizer = None;
    }

    /// Enable DRC compression
    pub fn enable_drc(&mut self, ratio: f32, threshold_db: f32) {
        self.drc = Some(DynamicRangeControl::new(
            ratio,
            threshold_db,
            10.0,  // 10ms attack
            100.0, // 100ms release
            self.sample_rate,
        ));

        // Set makeup gain to compensate for reduction
        if let Some(drc) = &mut self.drc {
            let makeup_gain = (threshold_db * (ratio - 1.0)) / ratio;
            drc.set_makeup_gain(makeup_gain);
        }
    }

    /// Disable DRC
    pub fn disable_drc(&mut self) {
        self.drc = None;
    }

    /// Set headroom target
    pub fn set_headroom_db(&mut self, headroom_db: f32) {
        self.headroom_manager = HeadroomManager::new(headroom_db, self.sample_rate);
    }

    /// Set limiter lookahead time in milliseconds
    pub fn set_headroom_lookahead_ms(&mut self, lookahead_ms: f32) {
        self.headroom_manager
            .set_lookahead_ms(self.sample_rate, lookahead_ms);
    }
}

impl Renderer for ReferenceRenderer {
    fn render(&mut self, mut input: AudioBlock, opts: &RenderOptions) -> AudioBlock {
        // Apply DRC if enabled
        if let Some(drc) = &mut self.drc {
            drc.process(&mut input);
        }

        // Apply loudness normalization if configured
        if let Some(loudness) = &mut self.loudness_normalizer {
            loudness.normalize(&mut input);
        } else if let Some(target) = &opts.target_loudness {
            // Use option if normalizer not explicitly configured
            let mut normalizer = LoudnessNormalizer::new(input.sample_rate, target.clone());
            normalizer.normalize(&mut input);
        }

        // Apply headroom protection (always enabled)
        self.headroom_manager.apply_limiting(&mut input);

        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_options_default() {
        let opts = RenderOptions::default();
        assert_eq!(opts.headroom_db, 3.0);
        assert!(opts.target_loudness.is_some());
        assert!(!opts.enable_drc);
    }

    #[test]
    fn test_reference_renderer_creation() {
        let renderer = ReferenceRenderer::new(48000);
        assert!(renderer.loudness_normalizer.is_none());
        assert!(renderer.drc.is_none());
    }

    #[test]
    fn test_renderer_with_loudness() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.set_loudness_target(LoudnessTarget::Television);

        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.1; 480], vec![0.1; 480]],
        };

        let opts = RenderOptions::default();
        let output = renderer.render(block, &opts);

        assert_eq!(output.channels.len(), 2);
        assert_eq!(output.sample_rate, 48000);
    }

    #[test]
    fn test_renderer_with_drc() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.enable_drc(4.0, -20.0);

        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.5; 480]],
        };

        let opts = RenderOptions::default();
        let output = renderer.render(block, &opts);

        assert_eq!(output.sample_rate, 48000);
    }

    #[test]
    fn test_renderer_headroom_protection() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.set_headroom_db(3.0);

        let mut loud_block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.99; 100], vec![0.99; 100]],
        };

        let max_before = loud_block.channels[0].iter().cloned().fold(0.0, f32::max);

        let opts = RenderOptions::default();
        let output = renderer.render(loud_block, &opts);

        let max_after = output.channels[0].iter().cloned().fold(0.0, f32::max);

        assert!(max_after <= max_before);
    }
}
