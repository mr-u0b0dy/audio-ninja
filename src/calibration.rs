// SPDX-License-Identifier: Apache-2.0

use crate::dsp::{BiquadCoefficients, BiquadFilter, FirFilter};
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct MeasurementConfig {
    pub sweep_duration: Duration,
    pub sample_rate: u32,
    pub sweep_type: SweepType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SweepType {
    LogSweep { start_hz: u32, end_hz: u32 },
    Mls { length: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeasurementResult {
    pub impulse_response: Vec<f32>,
    pub sample_rate: u32,
    pub peak_index: Option<usize>,
    pub magnitude_response: Option<Vec<f32>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalibrationSolution {
    pub delays: Vec<Duration>,
    pub trims_db: Vec<f32>,
    pub peq: Vec<BiquadFilter>,
    pub fir: Option<FirFilter>,
}

pub trait Calibrator {
    fn measure(&mut self, cfg: &MeasurementConfig) -> anyhow::Result<MeasurementResult>;
    fn solve(&self, measurement: &MeasurementResult) -> anyhow::Result<CalibrationSolution>;
}

pub struct ReferenceCalibrator;

impl Calibrator for ReferenceCalibrator {
    fn measure(&mut self, cfg: &MeasurementConfig) -> anyhow::Result<MeasurementResult> {
        let frames = (cfg.sample_rate as f32 * cfg.sweep_duration.as_secs_f32()) as usize;
        Ok(MeasurementResult {
            impulse_response: vec![0.0; frames],
            sample_rate: cfg.sample_rate,
            peak_index: None,
            magnitude_response: None,
        })
    }

    fn solve(&self, _measurement: &MeasurementResult) -> anyhow::Result<CalibrationSolution> {
        Ok(CalibrationSolution {
            delays: vec![],
            trims_db: vec![],
            peq: vec![],
            fir: None,
        })
    }
}

/// Generate logarithmic sweep from start_hz to end_hz
pub fn generate_log_sweep(
    sample_rate: u32,
    duration: Duration,
    start_hz: u32,
    end_hz: u32,
) -> Vec<f32> {
    let frames = (sample_rate as f32 * duration.as_secs_f32()) as usize;
    let mut sweep = Vec::with_capacity(frames);

    let f1 = start_hz as f32;
    let f2 = end_hz as f32;
    let t_end = duration.as_secs_f32();
    let k = t_end * f1 / (f2 - f1).ln();

    for i in 0..frames {
        let t = i as f32 / sample_rate as f32;
        let phase = 2.0 * PI * k * ((f2 - f1) * (t / t_end)).exp_m1();
        sweep.push(phase.sin());
    }

    sweep
}

/// Generate Maximum Length Sequence (MLS) of given length
pub fn generate_mls(length: u32) -> Vec<f32> {
    let n = length.next_power_of_two().trailing_zeros();
    let len = (1 << n) - 1;
    let mut sequence = Vec::with_capacity(len as usize);

    // Simple LFSR with taps for common lengths
    let taps = match n {
        7 => vec![7, 6],
        15 => vec![15, 14],
        17 => vec![17, 14],
        _ => vec![n, n - 1],
    };

    let mut state: u32 = 1;
    for _ in 0..len {
        let bit = state & 1;
        sequence.push(if bit == 1 { 1.0 } else { -1.0 });

        let mut feedback = 0;
        for &tap in &taps {
            feedback ^= (state >> (tap - 1)) & 1;
        }
        state = (state >> 1) | (feedback << (n - 1));
    }

    sequence
}

/// Compute impulse response from recorded sweep and reference sweep
pub fn extract_ir_from_sweep(
    recorded: &[f32],
    _reference_sweep: &[f32],
    sample_rate: u32,
) -> Vec<f32> {
    // Placeholder: real implementation uses FFT-based deconvolution
    // IR = IFFT(FFT(recorded) / FFT(reference))

    let ir_len = (sample_rate / 10) as usize; // 100ms IR
    let mut ir = vec![0.0; ir_len];

    // Simple peak detection for now
    if let Some((peak_idx, _)) = recorded.iter().enumerate().max_by(|(_, a), (_, b)| {
        a.abs()
            .partial_cmp(&b.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        if peak_idx < ir.len() {
            ir[peak_idx] = 1.0;
        }
    }

    ir
}

/// Find peak in impulse response (for delay detection)
pub fn find_ir_peak(ir: &[f32]) -> Option<usize> {
    ir.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap())
        .map(|(idx, _)| idx)
}

/// Compute delay from IR peak
pub fn compute_delay(peak_index: usize, sample_rate: u32) -> Duration {
    Duration::from_secs_f32(peak_index as f32 / sample_rate as f32)
}

/// Compute RMS level for trim calculation
pub fn compute_rms(signal: &[f32]) -> f32 {
    let sum_sq: f32 = signal.iter().map(|x| x * x).sum();
    (sum_sq / signal.len() as f32).sqrt()
}

/// Convert RMS to dB SPL (assuming reference level)
pub fn rms_to_db(rms: f32) -> f32 {
    20.0 * rms.log10()
}

/// Design parametric EQ biquad filter
pub fn design_peq(center_hz: f32, gain_db: f32, q: f32, sample_rate: u32) -> BiquadFilter {
    let a = 10_f32.powf(gain_db / 40.0);
    let omega = 2.0 * PI * center_hz / sample_rate as f32;
    let alpha = omega.sin() / (2.0 * q);

    let b0 = 1.0 + alpha * a;
    let b1 = -2.0 * omega.cos();
    let b2 = 1.0 - alpha * a;
    let a0 = 1.0 + alpha / a;
    let a1 = -2.0 * omega.cos();
    let a2 = 1.0 - alpha / a;

    BiquadFilter {
        coeffs: BiquadCoefficients {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        },
        gain_db,
    }
}

/// Design low-shelf filter
pub fn design_low_shelf(corner_hz: f32, gain_db: f32, sample_rate: u32) -> BiquadFilter {
    let a = 10_f32.powf(gain_db / 40.0);
    let omega = 2.0 * PI * corner_hz / sample_rate as f32;
    let alpha = omega.sin() / 2.0;

    let b0 = a * ((a + 1.0) - (a - 1.0) * omega.cos() + 2.0 * a.sqrt() * alpha);
    let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * omega.cos());
    let b2 = a * ((a + 1.0) - (a - 1.0) * omega.cos() - 2.0 * a.sqrt() * alpha);
    let a0 = (a + 1.0) + (a - 1.0) * omega.cos() + 2.0 * a.sqrt() * alpha;
    let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * omega.cos());
    let a2 = (a + 1.0) + (a - 1.0) * omega.cos() - 2.0 * a.sqrt() * alpha;

    BiquadFilter {
        coeffs: BiquadCoefficients {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        },
        gain_db,
    }
}

/// Design high-shelf filter
pub fn design_high_shelf(corner_hz: f32, gain_db: f32, sample_rate: u32) -> BiquadFilter {
    let a = 10_f32.powf(gain_db / 40.0);
    let omega = 2.0 * PI * corner_hz / sample_rate as f32;
    let alpha = omega.sin() / 2.0;

    let b0 = a * ((a + 1.0) + (a - 1.0) * omega.cos() + 2.0 * a.sqrt() * alpha);
    let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * omega.cos());
    let b2 = a * ((a + 1.0) + (a - 1.0) * omega.cos() - 2.0 * a.sqrt() * alpha);
    let a0 = (a + 1.0) - (a - 1.0) * omega.cos() + 2.0 * a.sqrt() * alpha;
    let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * omega.cos());
    let a2 = (a + 1.0) - (a - 1.0) * omega.cos() - 2.0 * a.sqrt() * alpha;

    BiquadFilter {
        coeffs: BiquadCoefficients {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        },
        gain_db,
    }
}
