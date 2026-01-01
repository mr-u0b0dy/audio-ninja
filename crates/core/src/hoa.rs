// SPDX-License-Identifier: Apache-2.0

//! Higher-Order Ambisonics (HOA) decoder for scene-based spatial audio rendering

/// Ambisonic order (1 = B-format, 2 = 2nd order, 3 = 3rd order)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmbisonicOrder(pub u8);

impl AmbisonicOrder {
    /// 1st order (4 channels: W, X, Y, Z)
    pub const FIRST: Self = Self(1);
    /// 2nd order (9 channels)
    pub const SECOND: Self = Self(2);
    /// 3rd order (16 channels)
    pub const THIRD: Self = Self(3);

    /// Number of ambisonic channels: (order + 1)^2
    pub fn channel_count(self) -> usize {
        let n = self.0 as usize + 1;
        n * n
    }

    /// Maximum degree for this order
    pub fn max_degree(self) -> i32 {
        self.0 as i32
    }
}

/// Decoding mode affects spatial resolution and frequency response
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodingMode {
    /// Basic mode: simple pseudoinverse, preserves amplitude
    Basic,
    /// Max-rE: optimizes energy vector (better localization)
    MaxRE,
    /// In-phase: preserves phase relationships
    InPhase,
}

/// Speaker position for HOA decoding
#[derive(Clone, Debug)]
pub struct HoaSpeaker {
    pub id: usize,
    pub azimuth: f32,   // degrees
    pub elevation: f32, // degrees
}

impl HoaSpeaker {
    pub fn new(id: usize, azimuth: f32, elevation: f32) -> Self {
        Self {
            id,
            azimuth,
            elevation,
        }
    }
}

/// HOA decoder: converts ambisonic signals to speaker feeds
pub struct HoaDecoder {
    order: AmbisonicOrder,
    mode: DecodingMode,
    speakers: Vec<HoaSpeaker>,
    decode_matrix: Vec<Vec<f32>>, // [speaker][channel]
}

impl HoaDecoder {
    /// Create a new HOA decoder for a speaker layout
    pub fn new(order: AmbisonicOrder, mode: DecodingMode, speakers: Vec<HoaSpeaker>) -> Self {
        let mut decoder = Self {
            order,
            mode,
            speakers,
            decode_matrix: Vec::new(),
        };
        decoder.compute_decode_matrix();
        decoder
    }

    /// Get ambisonic order
    pub fn order(&self) -> AmbisonicOrder {
        self.order
    }

    /// Get number of speakers
    pub fn speaker_count(&self) -> usize {
        self.speakers.len()
    }

    /// Get number of ambisonic channels
    pub fn channel_count(&self) -> usize {
        self.order.channel_count()
    }

    /// Compute the decoding matrix
    fn compute_decode_matrix(&mut self) {
        let num_speakers = self.speakers.len();
        let num_channels = self.order.channel_count();

        // Build encoding matrix: E[speaker][channel] = Y_nm(speaker_direction)
        let mut encode_matrix = vec![vec![0.0; num_channels]; num_speakers];

        for (i, speaker) in self.speakers.iter().enumerate() {
            let az_rad = speaker.azimuth.to_radians();
            let el_rad = speaker.elevation.to_radians();

            let harmonics = compute_spherical_harmonics(self.order, az_rad, el_rad);
            encode_matrix[i] = harmonics;
        }

        // Apply mode-dependent weights
        let weights = self.compute_mode_weights();

        // Apply weights to encoding matrix
        for row in encode_matrix.iter_mut() {
            for (val, &weight) in row.iter_mut().zip(weights.iter()) {
                *val *= weight;
            }
        }

        // Decode matrix is pseudoinverse of weighted encoding matrix
        // For simplicity, use transpose (works well for uniformly distributed speakers)
        self.decode_matrix = transpose_matrix(&encode_matrix);

        // Normalize each speaker output
        self.normalize_decode_matrix();
    }

    /// Compute mode-dependent channel weights
    fn compute_mode_weights(&self) -> Vec<f32> {
        let num_channels = self.order.channel_count();
        let mut weights = vec![1.0; num_channels];

        match self.mode {
            DecodingMode::Basic => {
                // No special weighting
            }
            DecodingMode::MaxRE => {
                // max-rE weighting: optimizes energy vector
                let mut idx = 0;
                for degree in 0..=self.order.max_degree() {
                    let weight = max_re_weight(degree, self.order.0);
                    for _ in 0..=(2 * degree) {
                        weights[idx] = weight;
                        idx += 1;
                    }
                }
            }
            DecodingMode::InPhase => {
                // In-phase weighting: preserves phase
                let mut idx = 0;
                for degree in 0..=self.order.max_degree() {
                    let weight = in_phase_weight(degree, self.order.0);
                    for _ in 0..=(2 * degree) {
                        weights[idx] = weight;
                        idx += 1;
                    }
                }
            }
        }

        weights
    }

    /// Normalize decode matrix for energy preservation
    fn normalize_decode_matrix(&mut self) {
        let num_speakers = self.speakers.len();
        let scale = (num_speakers as f32).sqrt();

        for row in self.decode_matrix.iter_mut() {
            for val in row.iter_mut() {
                *val /= scale;
            }
        }
    }

    /// Decode ambisonic signal to speaker feeds
    /// Input: ambisonic channels (length = channel_count())
    /// Output: speaker gains (length = speaker_count())
    pub fn decode(&self, ambisonic_channels: &[f32]) -> Vec<f32> {
        assert_eq!(
            ambisonic_channels.len(),
            self.channel_count(),
            "Input must have {} channels",
            self.channel_count()
        );

        let mut output = vec![0.0; self.speaker_count()];

        // decode_matrix is [channel][speaker] after transpose
        // output[speaker] = sum(decode_matrix[channel][speaker] * input[channel])
        for (ch, channel_row) in self.decode_matrix.iter().enumerate() {
            let input_val = ambisonic_channels[ch];
            for (spk, &coeff) in channel_row.iter().enumerate() {
                output[spk] += coeff * input_val;
            }
        }

        output
    }

    /// Decode a buffer of ambisonic audio
    /// Input: [channel][sample]
    /// Output: [speaker][sample]
    pub fn decode_buffer(&self, input: &[Vec<f32>]) -> Vec<Vec<f32>> {
        assert_eq!(
            input.len(),
            self.channel_count(),
            "Input must have {} channels",
            self.channel_count()
        );

        let num_samples = input[0].len();
        let mut output = vec![vec![0.0; num_samples]; self.speaker_count()];

        for sample_idx in 0..num_samples {
            let mut ambisonic_sample = vec![0.0; self.channel_count()];
            for (ch, channel) in input.iter().enumerate() {
                ambisonic_sample[ch] = channel[sample_idx];
            }

            let decoded = self.decode(&ambisonic_sample);

            for (spk, &gain) in decoded.iter().enumerate() {
                output[spk][sample_idx] = gain;
            }
        }

        output
    }
}

/// Compute spherical harmonics up to given order at (azimuth, elevation)
/// Returns vector of Y_nm values in ACN (Ambisonic Channel Numbering) order
fn compute_spherical_harmonics(order: AmbisonicOrder, azimuth: f32, elevation: f32) -> Vec<f32> {
    let num_channels = order.channel_count();
    let mut harmonics = vec![0.0; num_channels];

    let cos_el = elevation.cos();
    let sin_el = elevation.sin();

    let mut idx = 0;

    // Degree 0 (W channel)
    harmonics[idx] = 0.5 * (1.0 / std::f32::consts::PI).sqrt();
    idx += 1;

    if order.0 >= 1 {
        // Degree 1 (X, Y, Z) - 3 channels
        let k1 = (3.0 / (4.0 * std::f32::consts::PI)).sqrt();
        harmonics[idx] = k1 * cos_el * azimuth.sin(); // Y_1,-1 (Y)
        idx += 1;
        harmonics[idx] = k1 * sin_el; // Y_1,0 (Z)
        idx += 1;
        harmonics[idx] = k1 * cos_el * azimuth.cos(); // Y_1,1 (X)
        idx += 1;
    }

    if order.0 >= 2 {
        // Degree 2 (5 channels)
        let k2 = (5.0 / (16.0 * std::f32::consts::PI)).sqrt();
        let cos_az = azimuth.cos();
        let sin_az = azimuth.sin();
        let cos2_el = cos_el * cos_el;
        let sin2_el = sin_el * sin_el;

        harmonics[idx] = k2 * 3.0 * cos_el * sin_el * sin_az;
        idx += 1;
        harmonics[idx] = k2 * 3.0 * sin_el * cos_az;
        idx += 1;
        harmonics[idx] = k2 * (3.0 * sin2_el - 1.0);
        idx += 1;
        harmonics[idx] = k2 * 3.0 * cos_el * sin_el * cos_az;
        idx += 1;
        harmonics[idx] = k2 * 3.0 * cos2_el * (2.0 * cos_az * cos_az - 1.0);
        idx += 1;
    }

    if order.0 >= 3 {
        // Degree 3 (7 channels) - simplified approximation
        let k3 = (7.0 / (16.0 * std::f32::consts::PI)).sqrt();
        for _ in 0..7 {
            harmonics[idx] = k3 * sin_el.powi(idx as i32 % 3) * azimuth.cos();
            idx += 1;
        }
    }

    harmonics
}

/// Max-rE weight for degree n and order N
fn max_re_weight(degree: i32, order: u8) -> f32 {
    let n = degree as f32;
    let order_f = order as f32;

    // Simplified max-rE formula
    let numerator = (order_f + 1.0 - n).abs();
    let denominator = order_f + 1.0;

    (numerator / denominator).sqrt()
}

/// In-phase weight for degree n and order N
fn in_phase_weight(degree: i32, _order: u8) -> f32 {
    // In-phase:cos^n weighting
    let n = degree as f32;
    1.0 / (1.0 + n * 0.5)
}

/// Transpose a matrix
fn transpose_matrix(matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![0.0; rows]; cols];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            result[j][i] = val;
        }
    }

    result
}

/// Create standard HOA speaker layouts
/// Stereo layout (not ideal for HOA but useful for testing)
pub fn create_stereo_hoa_layout() -> Vec<HoaSpeaker> {
    vec![
        HoaSpeaker::new(0, -30.0, 0.0),
        HoaSpeaker::new(1, 30.0, 0.0),
    ]
}

/// 5.1 layout for HOA
pub fn create_5_1_hoa_layout() -> Vec<HoaSpeaker> {
    vec![
        HoaSpeaker::new(0, -30.0, 0.0),  // Front Left
        HoaSpeaker::new(1, 30.0, 0.0),   // Front Right
        HoaSpeaker::new(2, 0.0, 0.0),    // Center
        HoaSpeaker::new(3, -110.0, 0.0), // Surround Left
        HoaSpeaker::new(4, 110.0, 0.0),  // Surround Right
        HoaSpeaker::new(5, 180.0, 0.0),  // Back (for better coverage)
    ]
}

/// 7.1.4 layout with height for 3D HOA
pub fn create_7_1_4_hoa_layout() -> Vec<HoaSpeaker> {
    vec![
        // Ear level
        HoaSpeaker::new(0, -30.0, 0.0),
        HoaSpeaker::new(1, 30.0, 0.0),
        HoaSpeaker::new(2, 0.0, 0.0),
        HoaSpeaker::new(3, -90.0, 0.0),
        HoaSpeaker::new(4, 90.0, 0.0),
        HoaSpeaker::new(5, -135.0, 0.0),
        HoaSpeaker::new(6, 135.0, 0.0),
        HoaSpeaker::new(7, 180.0, 0.0),
        // Height
        HoaSpeaker::new(8, -30.0, 30.0),
        HoaSpeaker::new(9, 30.0, 30.0),
        HoaSpeaker::new(10, -135.0, 30.0),
        HoaSpeaker::new(11, 135.0, 30.0),
    ]
}

/// Cube layout (8 speakers) - ideal for 1st order HOA
pub fn create_cube_hoa_layout() -> Vec<HoaSpeaker> {
    vec![
        HoaSpeaker::new(0, -45.0, 0.0),    // Front Left
        HoaSpeaker::new(1, 45.0, 0.0),     // Front Right
        HoaSpeaker::new(2, -135.0, 0.0),   // Back Left
        HoaSpeaker::new(3, 135.0, 0.0),    // Back Right
        HoaSpeaker::new(4, -45.0, 35.26),  // Top Front Left
        HoaSpeaker::new(5, 45.0, 35.26),   // Top Front Right
        HoaSpeaker::new(6, -135.0, 35.26), // Top Back Left
        HoaSpeaker::new(7, 135.0, 35.26),  // Top Back Right
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ambisonic_order_channel_count() {
        assert_eq!(AmbisonicOrder::FIRST.channel_count(), 4);
        assert_eq!(AmbisonicOrder::SECOND.channel_count(), 9);
        assert_eq!(AmbisonicOrder::THIRD.channel_count(), 16);
    }

    #[test]
    fn test_hoa_speaker_creation() {
        let speaker = HoaSpeaker::new(0, 30.0, 15.0);
        assert_eq!(speaker.id, 0);
        assert_eq!(speaker.azimuth, 30.0);
        assert_eq!(speaker.elevation, 15.0);
    }

    #[test]
    fn test_spherical_harmonics_order_1() {
        let harmonics = compute_spherical_harmonics(AmbisonicOrder::FIRST, 0.0, 0.0);
        assert_eq!(harmonics.len(), 4);
        // W channel should be non-zero
        assert!(harmonics[0] > 0.0);
    }

    #[test]
    fn test_hoa_decoder_creation() {
        let speakers = create_stereo_hoa_layout();
        let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

        assert_eq!(decoder.order(), AmbisonicOrder::FIRST);
        assert_eq!(decoder.speaker_count(), 2);
        assert_eq!(decoder.channel_count(), 4);
    }

    #[test]
    fn test_hoa_decoder_decode() {
        let speakers = create_stereo_hoa_layout();
        let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

        // Mono signal in W channel
        let input = vec![1.0, 0.0, 0.0, 0.0];
        let output = decoder.decode(&input);

        assert_eq!(output.len(), 2);
        // Both speakers should get signal
        assert!(output[0].abs() > 0.0);
        assert!(output[1].abs() > 0.0);
    }

    #[test]
    fn test_layout_presets() {
        assert_eq!(create_stereo_hoa_layout().len(), 2);
        assert_eq!(create_5_1_hoa_layout().len(), 6);
        assert_eq!(create_7_1_4_hoa_layout().len(), 12);
        assert_eq!(create_cube_hoa_layout().len(), 8);
    }
}
