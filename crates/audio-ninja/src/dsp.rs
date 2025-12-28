// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Debug, PartialEq)]
pub struct BiquadCoefficients {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiquadFilter {
    pub coeffs: BiquadCoefficients,
    pub gain_db: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FirFilter {
    pub taps: Vec<f32>,
}

impl FirFilter {
    pub fn impulse(len: usize) -> Self {
        let mut taps = vec![0.0; len];
        if len > 0 {
            taps[0] = 1.0;
        }
        Self { taps }
    }
}
