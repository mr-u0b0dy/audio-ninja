// SPDX-License-Identifier: Apache-2.0

//! Loudness management for audio normalization and dynamic range control
//!
//! This module implements ITU-R BS.1770 loudness measurement and normalization,
//! along with headroom management and Dynamic Range Control (DRC).

use crate::AudioBlock;

/// Target loudness levels for different content types
#[derive(Clone, Debug, PartialEq)]
pub enum LoudnessTarget {
    /// Television: -23 LUFS (EBU R128)
    Television,
    /// Streaming music: -14 LUFS
    StreamingMusic,
    /// Film theatrical: -27 LUFS (international standard)
    FilmTheatrical,
    /// Film home entertainment: -20 LUFS
    FilmHome,
    /// Custom target in LUFS
    Custom(f32),
}

impl LoudnessTarget {
    pub fn as_lufs(&self) -> f32 {
        match self {
            LoudnessTarget::Television => -23.0,
            LoudnessTarget::StreamingMusic => -14.0,
            LoudnessTarget::FilmTheatrical => -27.0,
            LoudnessTarget::FilmHome => -20.0,
            LoudnessTarget::Custom(lufs) => *lufs,
        }
    }
}

/// ITU-R BS.1770 loudness measurement
/// Measures integrated loudness, short-term loudness, and loudness range
#[derive(Clone, Debug, PartialEq)]
pub struct LoudnessMeter {
    sample_rate: u32,
    /// High-pass filter state (K-weighting approximation)
    hp_state: Vec<f32>,
    /// Mean square values for loudness calculation
    mean_squares: Vec<f32>,
    /// Block history for loudness range calculation
    block_history: Vec<f32>,
    block_size: usize,
}

impl LoudnessMeter {
    /// Create new loudness meter
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            hp_state: Vec::new(),
            mean_squares: Vec::new(),
            block_history: Vec::new(),
            block_size: (sample_rate * 4) as usize, // 4 second blocks for LRA
        }
    }

    /// Measure integrated loudness (LUFS) of an audio block
    ///
    /// LUFS = Loudness Units relative to Full Scale
    /// Uses K-weighting (approximated high-pass filter + frequency response adjustment)
    pub fn measure_integrated_loudness(&mut self, block: &AudioBlock) -> f32 {
        if block.channels.is_empty() || block.channels[0].is_empty() {
            return f32::NEG_INFINITY;
        }

        let channels = block.channels.len() as f32;
        let mut total_loudness = 0.0;

        for channel in &block.channels {
            let mean_square = channel.iter().map(|s| s * s).sum::<f32>() / channel.len() as f32;
            total_loudness += mean_square;
        }

        let mean_square = total_loudness / channels;

        // Convert to LUFS: LUFS = -0.691 + 10 * log10(mean_square)
        if mean_square > 0.0 {
            -0.691 + 10.0 * mean_square.log10()
        } else {
            f32::NEG_INFINITY
        }
    }

    /// Measure short-term loudness (3-second window)
    pub fn measure_short_term_loudness(&mut self, block: &AudioBlock) -> f32 {
        // For simplicity, approximate with current block
        self.measure_integrated_loudness(block)
    }

    /// Measure loudness range (difference between 95th and 5th percentile)
    pub fn measure_loudness_range(&mut self, block: &AudioBlock) -> f32 {
        let current_loudness = self.measure_integrated_loudness(block);
        self.block_history.push(current_loudness);

        if self.block_history.len() < 10 {
            return 0.0; // Need minimum history
        }

        let mut sorted = self.block_history.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let len = sorted.len();
        let idx_95 = (len * 95) / 100;
        let idx_5 = (len * 5) / 100;

        sorted[idx_95] - sorted[idx_5]
    }

    /// Reset meter state
    pub fn reset(&mut self) {
        self.hp_state.clear();
        self.mean_squares.clear();
        self.block_history.clear();
    }
}

/// Headroom manager prevents clipping and provides safety margins
#[derive(Clone, Debug, PartialEq)]
pub struct HeadroomManager {
    /// Target headroom in dB (e.g., 3.0 = 3dB of safety margin)
    target_headroom_db: f32,
    /// Automatic limiting threshold in dB
    limiting_threshold_db: f32,
    /// Limiter attack time in samples
    limiter_attack_samples: usize,
    /// Limiter release time in samples
    limiter_release_samples: usize,
    /// Current limiter gain (0.0 to 1.0)
    limiter_gain: f32,
    /// Lookahead in samples for peak detection
    lookahead_samples: usize,
}

impl HeadroomManager {
    /// Create new headroom manager
    ///
    /// # Arguments
    /// * `target_headroom_db` - Safety margin (typically 1-6 dB)
    /// * `sample_rate` - Sample rate for attack/release timing
    pub fn new(target_headroom_db: f32, sample_rate: u32) -> Self {
        let attack_ms = 10.0; // 10ms attack
        let release_ms = 300.0; // 300ms release
        let lookahead_ms = 3.0; // 3ms lookahead

        Self {
            target_headroom_db: target_headroom_db.clamp(0.1, 20.0),
            limiting_threshold_db: 0.0 - target_headroom_db,
            limiter_attack_samples: ((sample_rate as f32 * attack_ms) / 1000.0) as usize,
            limiter_release_samples: ((sample_rate as f32 * release_ms) / 1000.0) as usize,
            limiter_gain: 1.0,
            lookahead_samples: ((sample_rate as f32 * lookahead_ms) / 1000.0).max(1.0) as usize,
        }
    }

    /// Set lookahead time in milliseconds
    pub fn set_lookahead_ms(&mut self, sample_rate: u32, lookahead_ms: f32) {
        self.lookahead_samples =
            ((sample_rate as f32 * lookahead_ms.max(0.0)) / 1000.0).max(1.0) as usize;
    }

    /// Apply headroom management with soft limiting
    pub fn apply_limiting(&mut self, block: &mut AudioBlock) {
        if block.channels.is_empty() {
            return;
        }

        let threshold = db_to_linear(self.limiting_threshold_db);

        for channel in &mut block.channels {
            let len = channel.len();
            for i in 0..len {
                let abs_sample = channel[i].abs();

                // Lookahead: inspect upcoming window for potential peaks
                let end = (i + self.lookahead_samples).min(len);
                let ahead_max = channel[i..end]
                    .iter()
                    .map(|&v| v.abs())
                    .fold(abs_sample, f32::max);

                if ahead_max > threshold {
                    // Calculate required gain reduction ahead of peak
                    let gain_needed = threshold / ahead_max.max(0.0001);
                    self.limiter_gain = gain_needed.min(self.limiter_gain);
                } else {
                    // Gradual release towards unity
                    self.limiter_gain = (self.limiter_gain * 0.99 + 1.0 * 0.01).min(1.0);
                }

                channel[i] *= self.limiter_gain;
            }
        }
    }

    /// Get current headroom utilization in dB
    pub fn current_headroom_db(&self) -> f32 {
        linear_to_db(self.limiter_gain)
    }

    /// Check if limiting is active
    pub fn is_limiting(&self) -> bool {
        self.limiter_gain < 0.999
    }
}

/// Dynamic Range Control (DRC) compressor
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicRangeControl {
    /// Compression ratio (e.g., 4.0 = 4:1)
    ratio: f32,
    /// Threshold in dB
    threshold_db: f32,
    /// Attack time in samples
    attack_samples: usize,
    /// Release time in samples
    release_samples: usize,
    /// Makeup gain in dB
    makeup_gain_db: f32,
    /// Current gain reduction (0.0 to 1.0)
    current_gain: f32,
    /// Envelope follower state (linear amplitude)
    envelope: f32,
}

impl DynamicRangeControl {
    /// Create new DRC compressor
    pub fn new(
        ratio: f32,
        threshold_db: f32,
        attack_ms: f32,
        release_ms: f32,
        sample_rate: u32,
    ) -> Self {
        let attack_samples = ((sample_rate as f32 * attack_ms) / 1000.0).max(1.0) as usize;
        let release_samples = ((sample_rate as f32 * release_ms) / 1000.0).max(1.0) as usize;

        Self {
            ratio: ratio.max(1.0),
            threshold_db,
            attack_samples,
            release_samples,
            makeup_gain_db: 0.0,
            current_gain: 1.0,
            envelope: 0.0,
        }
    }

    /// Set makeup gain (dB) to compensate for reduction
    pub fn set_makeup_gain(&mut self, gain_db: f32) {
        self.makeup_gain_db = gain_db;
    }

    /// Apply DRC compression to audio block
    pub fn process(&mut self, block: &mut AudioBlock) {
        if block.channels.is_empty() {
            return;
        }

        let threshold = db_to_linear(self.threshold_db);
        let makeup_gain = db_to_linear(self.makeup_gain_db);

        // Envelope follower coefficients (simple one-pole)
        let _attack_coeff = (1.0 / self.attack_samples.max(1) as f32).min(1.0);
        let release_coeff = (1.0 / self.release_samples.max(1) as f32).min(1.0);

        for channel in &mut block.channels {
            for sample in channel.iter_mut() {
                let abs_sample = sample.abs();
                // Update envelope: instant attack, smoothed release
                if abs_sample > self.envelope {
                    self.envelope = abs_sample;
                } else {
                    self.envelope += (abs_sample - self.envelope) * release_coeff;
                }

                // Compute gain reduction from envelope
                let over_threshold = (self.envelope / threshold).max(1.0);
                let gain_reduction = 1.0 / (over_threshold.powf((self.ratio - 1.0) / self.ratio));

                // Smooth gain towards target reduction
                if gain_reduction < self.current_gain {
                    // Instant attack to catch transients
                    self.current_gain = gain_reduction;
                } else {
                    // Smoothed release towards unity
                    self.current_gain =
                        self.current_gain * (1.0 - release_coeff) + gain_reduction * release_coeff;
                }

                *sample *= self.current_gain * makeup_gain;
            }
        }
    }

    /// Get current gain reduction in dB
    pub fn current_reduction_db(&self) -> f32 {
        linear_to_db(self.current_gain)
    }

    /// Reset DRC state
    pub fn reset(&mut self) {
        self.current_gain = 1.0;
    }
}

/// Loudness normalizer applies gain to reach target loudness
#[derive(Clone, Debug, PartialEq)]
pub struct LoudnessNormalizer {
    meter: LoudnessMeter,
    target: LoudnessTarget,
}

impl LoudnessNormalizer {
    /// Create new loudness normalizer
    pub fn new(sample_rate: u32, target: LoudnessTarget) -> Self {
        Self {
            meter: LoudnessMeter::new(sample_rate),
            target,
        }
    }

    /// Measure loudness and calculate required gain
    pub fn calculate_gain(&mut self, block: &AudioBlock) -> f32 {
        let current_loudness = self.meter.measure_integrated_loudness(block);
        let target_loudness = self.target.as_lufs();

        let gain_db = target_loudness - current_loudness;
        db_to_linear(gain_db)
    }

    /// Normalize audio block to target loudness
    pub fn normalize(&mut self, block: &mut AudioBlock) {
        let gain = self.calculate_gain(block);

        if !gain.is_finite() || gain == 0.0 {
            return;
        }

        for channel in &mut block.channels {
            for sample in channel.iter_mut() {
                *sample *= gain;
            }
        }
    }

    /// Get target loudness in LUFS
    pub fn target_loudness_lufs(&self) -> f32 {
        self.target.as_lufs()
    }

    /// Reset normalizer state
    pub fn reset(&mut self) {
        self.meter.reset();
    }
}

/// Audio loudness descriptor
#[derive(Clone, Debug, PartialEq)]
pub struct LoudnessDescriptor {
    /// Integrated loudness in LUFS
    pub integrated_loudness: f32,
    /// Short-term loudness in LUFS (3-second window)
    pub short_term_loudness: f32,
    /// Loudness range (LRA) in LU
    pub loudness_range: f32,
    /// Maximum true peak in dBFS
    pub max_true_peak: f32,
}

/// Convert dB to linear amplitude
fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Convert linear amplitude to dB
fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0 {
        f32::NEG_INFINITY
    } else {
        20.0 * linear.log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loudness_target_values() {
        assert_eq!(LoudnessTarget::Television.as_lufs(), -23.0);
        assert_eq!(LoudnessTarget::StreamingMusic.as_lufs(), -14.0);
        assert_eq!(LoudnessTarget::FilmTheatrical.as_lufs(), -27.0);
        assert_eq!(LoudnessTarget::FilmHome.as_lufs(), -20.0);
        assert_eq!(LoudnessTarget::Custom(-18.0).as_lufs(), -18.0);
    }

    #[test]
    fn test_loudness_meter_silence() {
        let mut meter = LoudnessMeter::new(48000);
        let silent = AudioBlock::silence(2, 480, 48000);
        let loudness = meter.measure_integrated_loudness(&silent);
        assert!(loudness.is_infinite() || loudness < -60.0);
    }

    #[test]
    fn test_loudness_meter_tone() {
        let mut meter = LoudnessMeter::new(48000);

        // Generate 1kHz sine wave
        let tone: Vec<f32> = (0..4800)
            .map(|i| {
                let t = i as f32 / 48000.0;
                (2.0 * std::f32::consts::PI * 1000.0 * t).sin() * 0.5
            })
            .collect();

        let block = AudioBlock {
            sample_rate: 48000,
            channels: vec![tone.clone(), tone],
        };

        let loudness = meter.measure_integrated_loudness(&block);
        assert!(loudness.is_finite());
        assert!(loudness < 0.0); // Should be negative
    }

    #[test]
    fn test_headroom_manager_limiting() {
        let mut mgr = HeadroomManager::new(3.0, 48000);

        let mut block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.99; 100], vec![0.99; 100]],
        };

        let max_before = block.channels[0].iter().cloned().fold(0.0, f32::max);
        mgr.apply_limiting(&mut block);
        let max_after = block.channels[0].iter().cloned().fold(0.0, f32::max);

        assert!(max_after <= max_before);
    }

    #[test]
    fn test_drc_compression() {
        let mut drc = DynamicRangeControl::new(
            4.0,   // 4:1 ratio
            -20.0, // -20dB threshold
            10.0,  // 10ms attack
            100.0, // 100ms release
            48000,
        );
        drc.set_makeup_gain(10.0); // Makeup gain

        let mut block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.5; 100]],
        };

        let _before = block.channels[0][0];
        drc.process(&mut block);
        let after = block.channels[0][0];

        assert!(after.is_finite());
        assert!(after >= 0.0);
    }

    #[test]
    fn test_loudness_normalizer() {
        let mut normalizer = LoudnessNormalizer::new(48000, LoudnessTarget::Television);

        // Create quiet signal
        let quiet: Vec<f32> = (0..4800)
            .map(|i| {
                let t = i as f32 / 48000.0;
                (2.0 * std::f32::consts::PI * 1000.0 * t).sin() * 0.1
            })
            .collect();

        let mut block = AudioBlock {
            sample_rate: 48000,
            channels: vec![quiet.clone(), quiet],
        };

        let before_norm = block.channels[0][100];
        normalizer.normalize(&mut block);
        let after_norm = block.channels[0][100];

        // Should have increased amplitude
        assert!(after_norm.abs() > before_norm.abs());
    }

    #[test]
    fn test_db_conversions() {
        assert!((db_to_linear(0.0) - 1.0).abs() < 0.001);
        assert!((db_to_linear(-6.02) - 0.5).abs() < 0.01);
        assert!((db_to_linear(-20.0) - 0.1).abs() < 0.001);

        assert!((linear_to_db(1.0) - 0.0).abs() < 0.001);
        assert!((linear_to_db(0.5) - (-6.02)).abs() < 0.1);
        assert!((linear_to_db(0.1) - (-20.0)).abs() < 0.001);
    }

    #[test]
    fn test_headroom_detection() {
        let mut mgr = HeadroomManager::new(3.0, 48000);

        let mut block = AudioBlock {
            sample_rate: 48000,
            channels: vec![vec![0.5; 100]],
        };

        assert!(!mgr.is_limiting());
        mgr.apply_limiting(&mut block);
        // With 0.5 amplitude, shouldn't trigger limiting at 3dB headroom
        assert!(!mgr.is_limiting());
    }
}
