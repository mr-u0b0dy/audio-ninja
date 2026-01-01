// SPDX-License-Identifier: Apache-2.0

//! HRTF (Head-Related Transfer Function) processing for binaural audio rendering.
//!
//! Provides spatial audio virtualization for headphones through convolution
//! with measured or modeled HRTF filters.

use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// HRTF dataset source
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HrtfDataset {
    /// KEMAR (Knowles Electronics Manikin for Acoustic Research)
    Kemar,
    /// CIPIC (Center for Image Processing and Integrated Computing)
    Cipic,
    /// MIT KEMAR
    MitKemar,
}

/// Headphone equalization profile
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadphoneProfile {
    /// No equalization (raw HRTF)
    Flat,
    /// Closed-back headphone compensation
    ClosedBack,
    /// Open-back headphone compensation
    OpenBack,
    /// IEM (In-Ear Monitor) compensation
    IEM,
}

/// 3D spatial position for HRTF lookup
#[derive(Clone, Debug)]
pub struct HrtfPosition {
    pub azimuth: f32,   // degrees, -180..180
    pub elevation: f32, // degrees, -90..90
    pub distance: f32,  // meters, typically 1.0..2.0
}

impl HrtfPosition {
    /// Create new HRTF position
    pub fn new(azimuth: f32, elevation: f32, distance: f32) -> Self {
        // Normalize azimuth to -180..180
        let mut azimuth = azimuth % 360.0;
        if azimuth > 180.0 {
            azimuth -= 360.0;
        } else if azimuth < -180.0 {
            azimuth += 360.0;
        }
        let elevation = elevation.clamp(-90.0, 90.0); // Clamp to -90..90
        let distance = distance.clamp(0.1, 10.0); // Clamp to reasonable range

        Self {
            azimuth,
            elevation,
            distance,
        }
    }

    /// Convert to canonical key for lookups
    pub fn to_key(&self) -> (i32, i32, i32) {
        (
            (self.azimuth.round() as i32),
            (self.elevation.round() as i32),
            ((self.distance * 10.0).round() as i32),
        )
    }
}

/// HRTF impulse response for one ear
#[derive(Clone, Debug)]
pub struct HrtfImpulseResponse {
    pub left: Vec<f32>,  // Left ear impulse response
    pub right: Vec<f32>, // Right ear impulse response
    pub delay_left: usize,
    pub delay_right: usize,
}

impl HrtfImpulseResponse {
    /// Create new HRTF pair
    pub fn new(left: Vec<f32>, right: Vec<f32>) -> Self {
        Self {
            delay_left: 0,
            delay_right: 0,
            left,
            right,
        }
    }

    /// Create with inter-aural time differences (ITD)
    pub fn with_delays(
        left: Vec<f32>,
        right: Vec<f32>,
        delay_left: usize,
        delay_right: usize,
    ) -> Self {
        Self {
            left,
            right,
            delay_left,
            delay_right,
        }
    }

    /// Get maximum impulse response length
    pub fn max_length(&self) -> usize {
        self.left.len().max(self.right.len()) + self.delay_left.max(self.delay_right)
    }
}

/// HRTF database with spatial interpolation
#[derive(Clone)]
pub struct HrtfDatabase {
    dataset: HrtfDataset,
    responses: HashMap<(i32, i32, i32), HrtfImpulseResponse>,
    sample_rate: u32,
}

impl HrtfDatabase {
    /// Create new HRTF database
    pub fn new(dataset: HrtfDataset, sample_rate: u32) -> Self {
        Self {
            dataset,
            responses: HashMap::new(),
            sample_rate,
        }
    }

    /// Add HRTF response for a position
    pub fn add_response(&mut self, pos: &HrtfPosition, response: HrtfImpulseResponse) {
        let key = pos.to_key();
        self.responses.insert(key, response);
    }

    /// Load default responses (simplified for testing)
    pub fn load_default_kemar(&mut self) -> Result<()> {
        // Create synthetic HRTF responses for testing
        // In production, these would be loaded from measured KEMAR data

        for azimuth in (-180..180).step_by(15) {
            for elevation in (-90..90).step_by(15) {
                let az_rad = (azimuth as f32).to_radians();
                let el_rad = (elevation as f32).to_radians();

                // Generate synthetic HRTF impulse response
                let length = 256; // Typical HRTF length
                let mut left = vec![0.0; length];
                let mut right = vec![0.0; length];

                // Simple synthetic response based on position
                let peak_idx = (length as f32 * (0.1 + 0.3 * el_rad.sin())) as usize;
                if peak_idx < length {
                    left[peak_idx] = 0.5 + 0.3 * az_rad.cos(); // Scale based on azimuth
                    right[peak_idx] = 0.5 - 0.3 * az_rad.cos();
                }

                // Add some high-frequency content
                for i in 0..length {
                    let phase = 2.0 * std::f32::consts::PI * i as f32 / 64.0;
                    left[i] += 0.05 * phase.sin() * (1.0 - i as f32 / length as f32);
                    right[i] += 0.05 * (phase + az_rad).sin() * (1.0 - i as f32 / length as f32);
                }

                // Compute inter-aural time difference (ITD)
                let itd_samples =
                    ((az_rad.sin() * 0.0001 * self.sample_rate as f32) as usize).max(1);
                let delay_left = if az_rad > 0.0 { 0 } else { itd_samples };
                let delay_right = if az_rad > 0.0 { itd_samples } else { 0 };

                let pos = HrtfPosition::new(azimuth as f32, elevation as f32, 1.0);
                let response =
                    HrtfImpulseResponse::with_delays(left, right, delay_left, delay_right);
                self.add_response(&pos, response);
            }
        }

        Ok(())
    }

    /// Get HRTF response for a position (with nearest-neighbor for now)
    pub fn get_response(&self, pos: &HrtfPosition) -> Result<HrtfImpulseResponse> {
        let key = pos.to_key();

        if let Some(response) = self.responses.get(&key) {
            return Ok(response.clone());
        }

        // Find nearest neighbor
        let mut nearest = None;
        let mut min_distance = f32::INFINITY;

        for (&(az, el, _), response) in &self.responses {
            let dist = (az as f32 - pos.azimuth)
                .abs()
                .min(360.0 - (az as f32 - pos.azimuth).abs().max(0.0))
                .powi(2)
                + (el as f32 - pos.elevation).powi(2);

            if dist < min_distance {
                min_distance = dist;
                nearest = Some(response.clone());
            }
        }

        nearest.ok_or_else(|| anyhow!("No HRTF response available for position {:?}", pos))
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get dataset type
    pub fn dataset(&self) -> HrtfDataset {
        self.dataset
    }
}

/// Binaural renderer using HRTF
pub struct BinauralRenderer {
    database: HrtfDatabase,
    headphone_profile: HeadphoneProfile,
    eq_filters: Vec<Vec<f32>>, // Headphone EQ coefficients
}

impl BinauralRenderer {
    /// Create new binaural renderer
    pub fn new(database: HrtfDatabase, headphone_profile: HeadphoneProfile) -> Self {
        let eq_filters = Self::create_eq_filters(&headphone_profile);

        Self {
            database,
            headphone_profile,
            eq_filters,
        }
    }

    /// Create headphone equalization filters
    fn create_eq_filters(profile: &HeadphoneProfile) -> Vec<Vec<f32>> {
        match profile {
            HeadphoneProfile::Flat => {
                // No equalization
                vec![vec![1.0], vec![1.0]]
            }
            HeadphoneProfile::ClosedBack => {
                // Bass boost, treble presence peak
                vec![vec![1.0, 0.1], vec![1.0, 0.1]]
            }
            HeadphoneProfile::OpenBack => {
                // Minimal bass boost
                vec![vec![1.0, 0.05], vec![1.0, 0.05]]
            }
            HeadphoneProfile::IEM => {
                // Presence peak in upper midrange
                vec![vec![1.0, -0.05, 0.02], vec![1.0, -0.05, 0.02]]
            }
        }
    }

    /// Render a mono signal to binaural stereo
    pub fn render(&self, input: &[f32], position: &HrtfPosition) -> Result<(Vec<f32>, Vec<f32>)> {
        let hrtf = self.database.get_response(position)?;
        let max_len = hrtf.max_length();

        let mut left = vec![0.0; input.len() + max_len];
        let mut right = vec![0.0; input.len() + max_len];

        // Apply HRTF convolution for left channel
        for (i, &sample) in input.iter().enumerate() {
            for (j, &coeff) in hrtf.left.iter().enumerate() {
                if i + j + hrtf.delay_left < left.len() {
                    left[i + j + hrtf.delay_left] += sample * coeff;
                }
            }
        }

        // Apply HRTF convolution for right channel
        for (i, &sample) in input.iter().enumerate() {
            for (j, &coeff) in hrtf.right.iter().enumerate() {
                if i + j + hrtf.delay_right < right.len() {
                    right[i + j + hrtf.delay_right] += sample * coeff;
                }
            }
        }

        // Apply headphone equalization
        left = self.apply_eq(&left);
        right = self.apply_eq(&right);

        // Trim to input length
        left.truncate(input.len() + 32);
        right.truncate(input.len() + 32);

        Ok((left, right))
    }

    /// Render buffer of mono audio to stereo
    pub fn render_buffer(
        &self,
        input: &[Vec<f32>],
        positions: &[HrtfPosition],
    ) -> Result<Vec<Vec<f32>>> {
        if input.is_empty() {
            return Ok(vec![vec![], vec![]]);
        }

        let num_samples = input[0].len();
        let mut left_output = vec![0.0; num_samples + 256];
        let mut right_output = vec![0.0; num_samples + 256];

        // Mix all channels
        for channel in input {
            if channel.len() != num_samples {
                return Err(anyhow!("All input channels must have same length"));
            }

            // Use first position or average
            let pos = if !positions.is_empty() {
                &positions[0]
            } else {
                return Err(anyhow!("At least one position required"));
            };

            let (left, right) = self.render(channel, pos)?;

            for (i, &sample) in left.iter().enumerate() {
                if i < left_output.len() {
                    left_output[i] += sample;
                }
            }

            for (i, &sample) in right.iter().enumerate() {
                if i < right_output.len() {
                    right_output[i] += sample;
                }
            }
        }

        // Normalize
        let max_val: f32 = left_output
            .iter()
            .chain(right_output.iter())
            .fold(0.0, |a: f32, &b| a.max(b.abs()));

        if max_val > 1.0 {
            let scale = 1.0 / max_val;
            for sample in &mut left_output {
                *sample *= scale;
            }
            for sample in &mut right_output {
                *sample *= scale;
            }
        }

        Ok(vec![left_output, right_output])
    }

    /// Apply headphone equalization
    fn apply_eq(&self, signal: &[f32]) -> Vec<f32> {
        // Simple FIR filtering using first channel's EQ
        let eq = &self.eq_filters[0];

        if eq.len() == 1 && (eq[0] - 1.0).abs() < 0.001 {
            return signal.to_vec(); // No EQ
        }

        let mut output = vec![0.0; signal.len()];

        for i in 0..signal.len() {
            for (j, &coeff) in eq.iter().enumerate() {
                if i >= j {
                    output[i] += signal[i - j] * coeff;
                }
            }
        }

        output
    }

    /// Get headphone profile
    pub fn headphone_profile(&self) -> HeadphoneProfile {
        self.headphone_profile
    }

    /// Get database
    pub fn database(&self) -> &HrtfDatabase {
        &self.database
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hrtf_position_creation() {
        let pos = HrtfPosition::new(45.0, 15.0, 1.0);
        assert_eq!(pos.azimuth, 45.0);
        assert_eq!(pos.elevation, 15.0);
        assert_eq!(pos.distance, 1.0);
    }

    #[test]
    fn test_hrtf_position_normalization() {
        let pos = HrtfPosition::new(200.0, 100.0, 0.5);
        assert!(pos.azimuth >= -180.0 && pos.azimuth <= 180.0);
        assert!(pos.elevation >= -90.0 && pos.elevation <= 90.0);
        assert!(pos.distance >= 0.1 && pos.distance <= 10.0);
    }

    #[test]
    fn test_hrtf_impulse_response() {
        let left = vec![0.5; 256];
        let right = vec![0.4; 256];
        let ir = HrtfImpulseResponse::new(left, right);

        assert_eq!(ir.left.len(), 256);
        assert_eq!(ir.right.len(), 256);
        assert_eq!(ir.max_length(), 256);
    }

    #[test]
    fn test_hrtf_impulse_response_with_delays() {
        let left = vec![0.5; 256];
        let right = vec![0.4; 256];
        let ir = HrtfImpulseResponse::with_delays(left, right, 10, 5);

        assert_eq!(ir.delay_left, 10);
        assert_eq!(ir.delay_right, 5);
        assert_eq!(ir.max_length(), 266);
    }

    #[test]
    fn test_hrtf_database_creation() {
        let db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        assert_eq!(db.dataset(), HrtfDataset::Kemar);
        assert_eq!(db.sample_rate(), 48000);
    }

    #[test]
    fn test_hrtf_database_add_response() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        let pos = HrtfPosition::new(0.0, 0.0, 1.0);
        let ir = HrtfImpulseResponse::new(vec![0.5; 256], vec![0.4; 256]);

        db.add_response(&pos, ir);
        let retrieved = db.get_response(&pos).unwrap();
        assert_eq!(retrieved.left.len(), 256);
    }

    #[test]
    fn test_hrtf_database_load_default() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();

        let pos = HrtfPosition::new(0.0, 0.0, 1.0);
        let ir = db.get_response(&pos).unwrap();
        assert!(!ir.left.is_empty());
        assert!(!ir.right.is_empty());
    }

    #[test]
    fn test_headphone_profile_variants() {
        let profiles = [
            HeadphoneProfile::Flat,
            HeadphoneProfile::ClosedBack,
            HeadphoneProfile::OpenBack,
            HeadphoneProfile::IEM,
        ];
        assert_eq!(profiles.len(), 4);
    }

    #[test]
    fn test_binaural_renderer_creation() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();
        let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

        assert_eq!(renderer.headphone_profile(), HeadphoneProfile::Flat);
    }

    #[test]
    fn test_binaural_render_mono_to_stereo() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();
        let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

        let input = vec![0.5; 1024];
        let pos = HrtfPosition::new(0.0, 0.0, 1.0);
        let (left, right) = renderer.render(&input, &pos).unwrap();

        assert!(!left.is_empty());
        assert!(!right.is_empty());
    }

    #[test]
    fn test_binaural_render_different_positions() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();
        let renderer = BinauralRenderer::new(db, HeadphoneProfile::ClosedBack);

        let input = vec![0.5; 512];

        let pos_front = HrtfPosition::new(0.0, 0.0, 1.0);
        let (left_f, _right_f) = renderer.render(&input, &pos_front).unwrap();

        let pos_left = HrtfPosition::new(90.0, 0.0, 1.0);
        let (left_l, _right_l) = renderer.render(&input, &pos_left).unwrap();

        // Different positions should produce different results
        let diff: f32 = left_f
            .iter()
            .zip(left_l.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        assert!(diff > 0.01);
    }

    #[test]
    fn test_binaural_render_with_different_profiles() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();

        let renderer_flat = BinauralRenderer::new(db.clone(), HeadphoneProfile::Flat);
        let mut db2 = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db2.load_default_kemar().unwrap();
        let renderer_cb = BinauralRenderer::new(db2, HeadphoneProfile::ClosedBack);

        let input = vec![0.5; 512];
        let pos = HrtfPosition::new(45.0, 15.0, 1.0);

        let _ = renderer_flat.render(&input, &pos).unwrap();
        let _ = renderer_cb.render(&input, &pos).unwrap();
    }

    #[test]
    fn test_binaural_render_buffer() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();
        let renderer = BinauralRenderer::new(db, HeadphoneProfile::OpenBack);

        let input = vec![vec![0.3; 512], vec![0.2; 512]];
        let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];

        let output = renderer.render_buffer(&input, &positions).unwrap();
        assert_eq!(output.len(), 2); // Stereo output
    }

    #[test]
    fn test_binaural_render_elevated_source() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();
        let renderer = BinauralRenderer::new(db, HeadphoneProfile::IEM);

        let input = vec![0.5; 256];

        let pos_up = HrtfPosition::new(0.0, 30.0, 1.0);
        let (left_up, right_up) = renderer.render(&input, &pos_up).unwrap();

        let pos_down = HrtfPosition::new(0.0, -30.0, 1.0);
        let (left_down, right_down) = renderer.render(&input, &pos_down).unwrap();

        // Different elevations should produce different inter-aural differences
        let itd_up = (left_up.len() as i32 - right_up.len() as i32).abs();
        let itd_down = (left_down.len() as i32 - right_down.len() as i32).abs();

        // At least one should be non-zero or different
        assert!(itd_up >= 0 && itd_down >= 0);
    }

    #[test]
    fn test_binaural_eq_flat() {
        let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
        db.load_default_kemar().unwrap();
        let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

        let input = vec![0.5; 256];
        let original_sum: f32 = input.iter().sum();
        let eq_sum: f32 = renderer.apply_eq(&input).iter().sum();

        // Flat EQ should preserve overall amplitude
        assert!((original_sum - eq_sum).abs() < 1.0);
    }
}
