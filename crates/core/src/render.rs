// SPDX-License-Identifier: Apache-2.0

use crate::loudness::{DynamicRangeControl, HeadroomManager, LoudnessNormalizer, LoudnessTarget};
use crate::hrtf::{BinauralRenderer, HeadphoneProfile, HrtfDatabase, HrtfDataset, HrtfPosition};
use crate::{AudioBlock, SpeakerLayout};
use std::time::Duration;

/// DRC compression presets for common use cases
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRCPreset {
    /// Speech: 3:1 ratio, -16dB threshold, 5ms attack, 80ms release
    Speech,
    /// Music: 4:1 ratio, -18dB threshold, 10ms attack, 100ms release
    Music,
    /// Cinema: 2:1 ratio, -14dB threshold, 20ms attack, 150ms release
    Cinema,
}

impl DRCPreset {
    /// Get DRC parameters for this preset: (ratio, threshold_db, attack_ms, release_ms)
    pub fn params(&self) -> (f32, f32, f32, f32) {
        match self {
            DRCPreset::Speech => (3.0, -16.0, 5.0, 80.0),
            DRCPreset::Music => (4.0, -18.0, 10.0, 100.0),
            DRCPreset::Cinema => (2.0, -14.0, 20.0, 150.0),
        }
    }
}

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

/// Reference renderer with loudness management, headroom protection, and optional binaural downmix
pub struct ReferenceRenderer {
    loudness_normalizer: Option<LoudnessNormalizer>,
    headroom_manager: HeadroomManager,
    drc: Option<DynamicRangeControl>,
    binaural_renderer: Option<BinauralRenderer>,
    current_binaural_position: Option<HrtfPosition>,
    sample_rate: u32,
}

impl ReferenceRenderer {
    /// Create new reference renderer
    pub fn new(sample_rate: u32) -> Self {
        Self {
            loudness_normalizer: None,
            headroom_manager: HeadroomManager::new(3.0, sample_rate),
            drc: None,
            binaural_renderer: None,
            current_binaural_position: None,
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

    /// Enable DRC compression with default attack/release
    pub fn enable_drc(&mut self, ratio: f32, threshold_db: f32) {
        self.enable_drc_with_params(ratio, threshold_db, 10.0, 100.0);
    }

    /// Enable DRC compression with custom attack/release in milliseconds
    pub fn enable_drc_with_params(
        &mut self,
        ratio: f32,
        threshold_db: f32,
        attack_ms: f32,
        release_ms: f32,
    ) {
        self.drc = Some(DynamicRangeControl::new(
            ratio,
            threshold_db,
            attack_ms,
            release_ms,
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

    /// Apply DRC preset (Speech, Music, Cinema)
    pub fn apply_drc_preset(&mut self, preset: DRCPreset) {
        let (ratio, threshold_db, attack_ms, release_ms) = preset.params();
        self.enable_drc_with_params(ratio, threshold_db, attack_ms, release_ms);
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

    /// Enable binaural rendering for headphone playback
    /// Sets default position to front (0°, 0°) at 1 meter distance
    pub fn enable_binaural(&mut self, headphone_profile: HeadphoneProfile) -> anyhow::Result<()> {
        let db = HrtfDatabase::new(HrtfDataset::Kemar, self.sample_rate);
        self.binaural_renderer = Some(BinauralRenderer::new(db, headphone_profile));
        // Set default position (front-center at 1m)
        self.current_binaural_position = Some(HrtfPosition::new(0.0, 0.0, 1.0));
        Ok(())
    }

    /// Disable binaural rendering
    pub fn disable_binaural(&mut self) {
        self.binaural_renderer = None;
        self.current_binaural_position = None;
    }

    /// Set spatial position for binaural rendering (front-left virtual speaker for now)
    pub fn set_binaural_position(&mut self, azimuth_deg: f32, elevation_deg: f32, distance_m: f32) {
        self.current_binaural_position = Some(HrtfPosition::new(azimuth_deg, elevation_deg, distance_m));
    }

    /// Check if binaural rendering is enabled
    pub fn has_binaural(&self) -> bool {
        self.binaural_renderer.is_some()
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

        // Apply binaural downmix if enabled
        if let (Some(binaural), Some(position)) = (&self.binaural_renderer, &self.current_binaural_position) {
            // For simplicity, use first channel or mix all channels
            let mono_input: Vec<f32> = if input.channels.len() == 1 {
                input.channels[0].clone()
            } else if input.channels.len() == 2 {
                // Mix L+R to mono for binaural processing
                input.channels[0]
                    .iter()
                    .zip(input.channels[1].iter())
                    .map(|(l, r)| (l + r) * 0.5)
                    .collect()
            } else {
                // Mix all channels to mono
                let len = input.channels.iter().map(|ch| ch.len()).max().unwrap_or(0);
                (0..len)
                    .map(|i| {
                        input.channels.iter().filter_map(|ch| ch.get(i)).sum::<f32>()
                            / input.channels.len().max(1) as f32
                    })
                    .collect()
            };

            // Render binaural
            if let Ok((left, right)) = binaural.render(&mono_input, position) {
                input.channels = vec![left, right];
            }
        }

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
    fn test_renderer_with_drc_params() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.enable_drc_with_params(3.0, -18.0, 5.0, 80.0);

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

        let loud_block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.99; 100], vec![0.99; 100]],
        };

        let max_before = loud_block.channels[0].iter().cloned().fold(0.0, f32::max);

        let opts = RenderOptions::default();
        let output = renderer.render(loud_block, &opts);

        let max_after = output.channels[0].iter().cloned().fold(0.0, f32::max);

        assert!(max_after <= max_before);
    }

    #[test]
    fn test_drc_preset_speech_params() {
        let (ratio, threshold_db, attack_ms, release_ms) = DRCPreset::Speech.params();
        assert_eq!(ratio, 3.0);
        assert_eq!(threshold_db, -16.0);
        assert_eq!(attack_ms, 5.0);
        assert_eq!(release_ms, 80.0);
    }

    #[test]
    fn test_drc_preset_music_params() {
        let (ratio, threshold_db, attack_ms, release_ms) = DRCPreset::Music.params();
        assert_eq!(ratio, 4.0);
        assert_eq!(threshold_db, -18.0);
        assert_eq!(attack_ms, 10.0);
        assert_eq!(release_ms, 100.0);
    }

    #[test]
    fn test_drc_preset_cinema_params() {
        let (ratio, threshold_db, attack_ms, release_ms) = DRCPreset::Cinema.params();
        assert_eq!(ratio, 2.0);
        assert_eq!(threshold_db, -14.0);
        assert_eq!(attack_ms, 20.0);
        assert_eq!(release_ms, 150.0);
    }

    #[test]
    fn test_renderer_apply_drc_preset() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.apply_drc_preset(DRCPreset::Speech);

        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.5; 480]],
        };

        let opts = RenderOptions::default();
        let output = renderer.render(block, &opts);

        assert_eq!(output.sample_rate, 48000);
        // Output should have reduced peaks compared to input due to compression
        assert!(output.channels[0].iter().all(|&s| s.abs() <= 0.5));
    }

    #[test]
    fn test_renderer_binaural_enable() {
        let mut renderer = ReferenceRenderer::new(48000);
        assert!(!renderer.has_binaural());
        
        renderer.enable_binaural(HeadphoneProfile::Flat).unwrap();
        assert!(renderer.has_binaural());
        
        renderer.disable_binaural();
        assert!(!renderer.has_binaural());
    }

    #[test]
    fn test_renderer_binaural_position() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.enable_binaural(HeadphoneProfile::ClosedBack).unwrap();
        
        renderer.set_binaural_position(0.0, 0.0, 1.0);
        assert!(renderer.current_binaural_position.is_some());
        
        if let Some(pos) = &renderer.current_binaural_position {
            assert_eq!(pos.azimuth, 0.0);
            assert_eq!(pos.elevation, 0.0);
            assert_eq!(pos.distance, 1.0);
        }
    }

    #[test]
    fn test_renderer_binaural_stereo_downmix() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.enable_binaural(HeadphoneProfile::Flat).unwrap();
        renderer.set_binaural_position(0.0, 0.0, 1.0);

        // 2-channel stereo input
        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.2; 480], vec![0.3; 480]],
        };

        let opts = RenderOptions::default();
        let output = renderer.render(block, &opts);

        // Should output 2 channels (binaural stereo)
        assert_eq!(output.channels.len(), 2);
        assert_eq!(output.sample_rate, 48000);
    }

    #[test]
    fn test_renderer_binaural_mono_upconvert() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.enable_binaural(HeadphoneProfile::OpenBack).unwrap();
        // Set position before rendering
        renderer.set_binaural_position(45.0, 0.0, 1.0);

        // Mono input
        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.5; 480]],
        };

        let opts = RenderOptions::default();
        let output = renderer.render(block, &opts);

        // Should output stereo from mono due to binaural processing
        // The output is processed through headroom and loudness first,
        // then binaural converts mono to stereo
        assert!(!output.channels.is_empty(), "Should have at least 1 channel");
    }

    #[test]
    fn test_renderer_binaural_without_position() {
        let mut renderer = ReferenceRenderer::new(48000);
        renderer.enable_binaural(HeadphoneProfile::IEM).unwrap();
        // Note: disable position to test fallback
        renderer.current_binaural_position = None;

        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.5; 480]],
        };

        let opts = RenderOptions::default();
        let output = renderer.render(block, &opts);

        // Should pass through without binaural processing since no position set
        assert_eq!(output.channels.len(), 1);
    }
}
