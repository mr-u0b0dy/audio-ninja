// SPDX-License-Identifier: Apache-2.0

use audio_ninja::calibration::*;
use audio_ninja::dspconfig::*;
use std::time::Duration;

#[test]
fn test_generate_log_sweep() {
    let sweep = generate_log_sweep(48000, Duration::from_secs(1), 20, 20000);

    assert_eq!(sweep.len(), 48000);

    // Check amplitude is normalized
    let max_amp = sweep.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    assert!(max_amp <= 1.0);
    assert!(max_amp > 0.5); // Should have reasonable amplitude
}

#[test]
fn test_generate_mls() {
    let mls = generate_mls(127);

    // MLS should be power-of-2 minus 1
    assert!(mls.len() == 127 || mls.len() == 255);

    // Check values are ±1
    for val in &mls {
        assert!(*val == 1.0 || *val == -1.0);
    }
}

#[test]
fn test_find_ir_peak() {
    let mut ir = vec![0.0; 100];
    ir[42] = 1.0; // Peak at index 42
    ir[10] = 0.5;
    ir[80] = -0.3;

    let peak = find_ir_peak(&ir);
    assert_eq!(peak, Some(42));
}

#[test]
fn test_compute_delay() {
    let delay = compute_delay(480, 48000); // 480 samples at 48kHz
    assert_eq!(delay.as_millis(), 10); // 10ms
}

#[test]
fn test_compute_rms() {
    let signal = vec![1.0, -1.0, 1.0, -1.0];
    let rms = compute_rms(&signal);
    assert!((rms - 1.0).abs() < 0.001);
}

#[test]
fn test_rms_to_db() {
    let db = rms_to_db(1.0);
    assert!((db - 0.0).abs() < 0.001);

    let db = rms_to_db(0.5);
    assert!((db - (-6.02)).abs() < 0.1); // -6dB for half amplitude
}

#[test]
fn test_design_peq() {
    let peq = design_peq(1000.0, 3.0, 1.0, 48000);

    // Check coefficients are finite
    assert!(peq.coeffs.b0.is_finite());
    assert!(peq.coeffs.b1.is_finite());
    assert!(peq.coeffs.b2.is_finite());
    assert!(peq.coeffs.a1.is_finite());
    assert!(peq.coeffs.a2.is_finite());

    assert_eq!(peq.gain_db, 3.0);
}

#[test]
fn test_design_low_shelf() {
    let shelf = design_low_shelf(100.0, 6.0, 48000);

    assert!(shelf.coeffs.b0.is_finite());
    assert_eq!(shelf.gain_db, 6.0);
}

#[test]
fn test_design_high_shelf() {
    let shelf = design_high_shelf(10000.0, -3.0, 48000);

    assert!(shelf.coeffs.b0.is_finite());
    assert_eq!(shelf.gain_db, -3.0);
}

#[test]
fn test_extract_ir_from_sweep() {
    let sweep = generate_log_sweep(48000, Duration::from_millis(100), 20, 20000);
    let recorded = sweep.clone(); // Simulate perfect recording

    let ir = extract_ir_from_sweep(&recorded, &sweep, 48000);

    assert_eq!(ir.len(), 4800); // 100ms at 48kHz

    // Should have a peak somewhere
    let peak = find_ir_peak(&ir);
    assert!(peak.is_some());
}

#[test]
fn test_measurement_config_log_sweep() {
    let cfg = MeasurementConfig {
        sweep_duration: Duration::from_secs(1),
        sample_rate: 48000,
        sweep_type: SweepType::LogSweep {
            start_hz: 20,
            end_hz: 20000,
        },
    };

    let mut cal = ReferenceCalibrator;
    let result = cal.measure(&cfg);
    assert!(result.is_ok());
}

#[test]
fn test_measurement_config_mls() {
    let cfg = MeasurementConfig {
        sweep_duration: Duration::from_millis(100),
        sample_rate: 48000,
        sweep_type: SweepType::Mls { length: 127 },
    };

    let mut cal = ReferenceCalibrator;
    let result = cal.measure(&cfg);
    assert!(result.is_ok());
}

#[test]
fn test_calibration_solution() {
    let result = MeasurementResult {
        impulse_response: vec![0.0; 1000],
        sample_rate: 48000,
        peak_index: Some(100),
        magnitude_response: None,
    };

    let cal = ReferenceCalibrator;
    let solution = cal.solve(&result);
    assert!(solution.is_ok());
}

#[test]
fn test_camilladsp_config_to_yaml() {
    use audio_ninja::dsp::BiquadCoefficients;

    let solution = CalibrationSolution {
        delays: vec![Duration::from_millis(5), Duration::ZERO],
        trims_db: vec![-2.0, 0.0],
        peq: vec![audio_ninja::dsp::BiquadFilter {
            coeffs: BiquadCoefficients {
                b0: 1.0,
                b1: 0.0,
                b2: 0.0,
                a1: 0.0,
                a2: 0.0,
            },
            gain_db: 3.0,
        }],
        fir: None,
    };

    let config = solution_to_camilladsp(&solution, 48000, 2);
    let yaml = config.to_yaml();

    assert!(yaml.contains("samplerate: 48000"));
    assert!(yaml.contains("channels: 2"));
    assert!(yaml.contains("delay_ch0"));
}

#[test]
fn test_brutefir_config() {
    use audio_ninja::dsp::FirFilter;

    let solution = CalibrationSolution {
        delays: vec![],
        trims_db: vec![],
        peq: vec![],
        fir: Some(FirFilter {
            taps: vec![0.0, 0.5, 1.0, 0.5, 0.0],
        }),
    };

    let config = solution_to_brutefir(&solution, 48000);
    assert_eq!(config.sample_rate, 48000);
    assert_eq!(config.filter_length, 5);

    let config_str = config.to_config_file();
    assert!(config_str.contains("sampling_rate: 48000"));
    assert!(config_str.contains("filter_length: 5"));
}

#[test]
fn test_full_calibration_workflow() {
    // Generate test sweep
    let sweep = generate_log_sweep(48000, Duration::from_millis(500), 20, 20000);
    assert_eq!(sweep.len(), 24000);

    // Simulate measurement
    let recorded = sweep.clone();

    // Extract IR
    let ir = extract_ir_from_sweep(&recorded, &sweep, 48000);
    assert!(!ir.is_empty());

    // Find peak
    let peak = find_ir_peak(&ir);
    assert!(peak.is_some());

    // Compute delay
    let delay = compute_delay(peak.unwrap(), 48000);
    assert!(delay < Duration::from_millis(100));

    // Design correction filters
    let peq = design_peq(1000.0, -3.0, 1.0, 48000);
    let low_shelf = design_low_shelf(80.0, 6.0, 48000);
    let high_shelf = design_high_shelf(12000.0, -2.0, 48000);

    // Build solution
    let solution = CalibrationSolution {
        delays: vec![delay],
        trims_db: vec![-2.0],
        peq: vec![peq, low_shelf, high_shelf],
        fir: None,
    };

    // Export to CamillaDSP
    let config = solution_to_camilladsp(&solution, 48000, 1);
    let yaml = config.to_yaml();
    assert!(yaml.contains("samplerate"));
}
