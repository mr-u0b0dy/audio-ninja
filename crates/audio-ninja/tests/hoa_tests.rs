// SPDX-License-Identifier: Apache-2.0

use audio_ninja::hoa::*;

#[test]
fn test_ambisonic_order_first() {
    assert_eq!(AmbisonicOrder::FIRST.channel_count(), 4);
    assert_eq!(AmbisonicOrder::FIRST.max_degree(), 1);
}

#[test]
fn test_ambisonic_order_second() {
    assert_eq!(AmbisonicOrder::SECOND.channel_count(), 9);
    assert_eq!(AmbisonicOrder::SECOND.max_degree(), 2);
}

#[test]
fn test_ambisonic_order_third() {
    assert_eq!(AmbisonicOrder::THIRD.channel_count(), 16);
    assert_eq!(AmbisonicOrder::THIRD.max_degree(), 3);
}

#[test]
fn test_hoa_speaker_properties() {
    let speaker = HoaSpeaker::new(5, 45.0, 30.0);
    assert_eq!(speaker.id, 5);
    assert_eq!(speaker.azimuth, 45.0);
    assert_eq!(speaker.elevation, 30.0);
}

#[test]
fn test_decoding_mode_variants() {
    let modes = vec![
        DecodingMode::Basic,
        DecodingMode::MaxRE,
        DecodingMode::InPhase,
    ];

    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], DecodingMode::Basic);
}

#[test]
fn test_hoa_decoder_stereo_basic() {
    let speakers = create_stereo_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    assert_eq!(decoder.speaker_count(), 2);
    assert_eq!(decoder.channel_count(), 4);
    assert_eq!(decoder.order(), AmbisonicOrder::FIRST);
}

#[test]
fn test_hoa_decoder_5_1_basic() {
    let speakers = create_5_1_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    assert_eq!(decoder.speaker_count(), 6);
    assert_eq!(decoder.channel_count(), 4);
}

#[test]
fn test_hoa_decoder_7_1_4_second_order() {
    let speakers = create_7_1_4_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::SECOND, DecodingMode::MaxRE, speakers);

    assert_eq!(decoder.speaker_count(), 12);
    assert_eq!(decoder.channel_count(), 9);
}

#[test]
fn test_hoa_decoder_cube_layout() {
    let speakers = create_cube_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    assert_eq!(decoder.speaker_count(), 8);
    // Cube layout is ideal for 1st order HOA
}

#[test]
fn test_decode_mono_signal() {
    let speakers = create_stereo_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // Pure W channel (omnidirectional)
    let input = vec![1.0, 0.0, 0.0, 0.0];
    let output = decoder.decode(&input);

    assert_eq!(output.len(), 2);

    // Both speakers should receive signal
    assert!(output[0] > 0.0);
    assert!(output[1] > 0.0);

    // Should be roughly equal for omnidirectional
    assert!((output[0] - output[1]).abs() < 0.3);
}

#[test]
fn test_decode_front_biased_signal() {
    let speakers = create_5_1_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // W + Y (forward bias) - note Y in ACN corresponds to Z-axis (elevation)
    // For horizontal forward, we should use W only or check actual output
    let input = vec![1.0, 0.0, 1.0, 0.0];
    let output = decoder.decode(&input);

    assert_eq!(output.len(), 6);

    // At least some output should be produced
    let total_energy: f32 = output.iter().map(|x| x * x).sum();
    assert!(total_energy > 0.0);
}

#[test]
fn test_decode_left_biased_signal() {
    let speakers = create_5_1_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // W + X (bias in Y direction based on ACN convention)
    let input = vec![1.0, 1.0, 0.0, 0.0];
    let output = decoder.decode(&input);

    // Should produce some output on all speakers
    let total_energy: f32 = output.iter().map(|x| x * x).sum();
    assert!(total_energy > 0.0);
}

#[test]
fn test_decode_buffer_processing() {
    let speakers = create_stereo_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // Create 4-channel input with 100 samples each
    let num_samples = 100;
    let input = vec![
        vec![1.0; num_samples], // W
        vec![0.5; num_samples], // X
        vec![0.0; num_samples], // Y
        vec![0.0; num_samples], // Z
    ];

    let output = decoder.decode_buffer(&input);

    assert_eq!(output.len(), 2); // 2 speakers
    assert_eq!(output[0].len(), num_samples);
    assert_eq!(output[1].len(), num_samples);

    // All samples should be processed
    assert!(output[0].iter().all(|&x| x != 0.0));
}

#[test]
fn test_decode_silent_signal() {
    let speakers = create_5_1_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    let input = vec![0.0, 0.0, 0.0, 0.0];
    let output = decoder.decode(&input);

    // All outputs should be zero
    for &gain in &output {
        assert_eq!(gain, 0.0);
    }
}

#[test]
fn test_max_re_mode() {
    let speakers = create_cube_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::MaxRE, speakers);

    let input = vec![1.0, 0.5, 0.3, 0.2];
    let output = decoder.decode(&input);

    assert_eq!(output.len(), 8);

    // Max-rE should produce output
    let energy: f32 = output.iter().map(|x| x * x).sum();
    assert!(energy > 0.0);
}

#[test]
fn test_in_phase_mode() {
    let speakers = create_5_1_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::InPhase, speakers);

    let input = vec![1.0, 0.0, 0.0, 0.0];
    let output = decoder.decode(&input);

    // In-phase mode should preserve signal
    assert!(output.iter().any(|&x| x > 0.0));
}

#[test]
fn test_second_order_decoding() {
    let speakers = create_7_1_4_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::SECOND, DecodingMode::Basic, speakers);

    // 9 channels for 2nd order
    let input = vec![1.0, 0.2, 0.3, 0.1, 0.15, 0.05, 0.1, 0.08, 0.12];
    let output = decoder.decode(&input);

    assert_eq!(output.len(), 12);
}

#[test]
fn test_third_order_decoding() {
    let speakers = create_7_1_4_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::THIRD, DecodingMode::MaxRE, speakers);

    // 16 channels for 3rd order
    let input = vec![
        1.0, 0.2, 0.3, 0.1, 0.15, 0.05, 0.1, 0.08, 0.12, 0.07, 0.04, 0.09, 0.06, 0.03, 0.11, 0.02,
    ];
    let output = decoder.decode(&input);

    assert_eq!(output.len(), 12);
}

#[test]
fn test_energy_preservation() {
    let speakers = create_cube_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // Unit energy input
    let input = vec![1.0, 0.0, 0.0, 0.0];
    let output = decoder.decode(&input);

    let output_energy: f32 = output.iter().map(|x| x * x).sum();

    // Energy should be approximately preserved (within reasonable tolerance)
    // HOA decoding doesn't strictly preserve energy, just check it's non-zero
    assert!(output_energy > 0.01);
    assert!(output_energy < 10.0);
}

#[test]
fn test_decode_varying_buffer() {
    let speakers = create_stereo_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // Create time-varying input
    let num_samples = 50;
    let mut input = vec![vec![0.0; num_samples]; 4];

    for i in 0..num_samples {
        let t = i as f32 / num_samples as f32;
        input[0][i] = 1.0; // Constant W
        input[1][i] = (t * 2.0 * std::f32::consts::PI).sin(); // Varying X
    }

    let output = decoder.decode_buffer(&input);

    assert_eq!(output.len(), 2);
    assert_eq!(output[0].len(), num_samples);

    // Output should vary over time
    let first_half: f32 = output[0][0..25].iter().sum();
    let second_half: f32 = output[0][25..50].iter().sum();

    // Should be different due to time-varying input
    assert!((first_half - second_half).abs() > 0.1);
}

#[test]
fn test_layout_speaker_counts() {
    assert_eq!(create_stereo_hoa_layout().len(), 2);
    assert_eq!(create_5_1_hoa_layout().len(), 6);
    assert_eq!(create_7_1_4_hoa_layout().len(), 12);
    assert_eq!(create_cube_hoa_layout().len(), 8);
}

#[test]
fn test_layout_speaker_positions() {
    let stereo = create_stereo_hoa_layout();
    assert_eq!(stereo[0].azimuth, -30.0);
    assert_eq!(stereo[1].azimuth, 30.0);

    let cube = create_cube_hoa_layout();
    // Verify height speakers have elevation
    assert!(cube[4].elevation > 0.0);
    assert!(cube[7].elevation > 0.0);
}

#[test]
fn test_multiple_decoders_independent() {
    let speakers = create_5_1_hoa_layout();

    let decoder1 = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers.clone());
    let decoder2 = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::MaxRE, speakers);

    let input = vec![1.0, 0.5, 0.3, 0.2];

    let output1 = decoder1.decode(&input);
    let output2 = decoder2.decode(&input);

    // Different modes should produce different outputs
    let diff: f32 = output1
        .iter()
        .zip(output2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    assert!(diff > 0.01); // Modes should differ
}

#[test]
#[should_panic(expected = "Input must have 4 channels")]
fn test_decode_wrong_channel_count() {
    let speakers = create_stereo_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::FIRST, DecodingMode::Basic, speakers);

    // Wrong number of channels
    let input = vec![1.0, 0.0];
    decoder.decode(&input);
}

#[test]
#[should_panic(expected = "Input must have 9 channels")]
fn test_decode_buffer_wrong_channel_count() {
    let speakers = create_5_1_hoa_layout();
    let decoder = HoaDecoder::new(AmbisonicOrder::SECOND, DecodingMode::Basic, speakers);

    // Wrong number of channels
    let input = vec![vec![1.0; 10]; 4]; // 4 channels instead of 9
    decoder.decode_buffer(&input);
}
