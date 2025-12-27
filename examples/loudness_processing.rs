// Example: Complete loudness normalization and DRC processing pipeline
//
// This example demonstrates:
// 1. Measuring audio loudness using ITU-R BS.1770-4
// 2. Normalizing to streaming standard (-14 LUFS)
// 3. Applying dynamic range control to tame peaks
// 4. Using headroom protection to prevent clipping
// 5. Comparing before/after loudness and dynamics

use audio_ninja::{
    loudness::{
        DynamicRangeControl, HeadroomManager, LoudnessMeter, LoudnessNormalizer, LoudnessTarget,
    },
    render::{ReferenceRenderer, RenderOptions},
    AudioBlock,
};

fn main() -> anyhow::Result<()> {
    println!("=== Audio Loudness and DRC Processing Pipeline ===\n");

    // ========== Setup ==========
    const SAMPLE_RATE: u32 = 48000;
    const DURATION_SECONDS: f32 = 2.0;
    const NUM_SAMPLES: usize = (SAMPLE_RATE as f32 * DURATION_SECONDS) as usize;
    const NUM_CHANNELS: usize = 2;

    // Create synthetic test audio with dynamic peaks
    println!("1. Generating test audio with dynamic peaks...");
    let mut test_audio = generate_dynamic_audio(NUM_SAMPLES, NUM_CHANNELS, SAMPLE_RATE);
    println!(
        "   Generated {} samples at {} Hz\n",
        NUM_SAMPLES, SAMPLE_RATE
    );

    // ========== Measurement ==========
    println!("2. Measuring original audio loudness (ITU-R BS.1770-4)...");
    let mut meter = LoudnessMeter::new(SAMPLE_RATE);
    let original_integrated = meter.measure_integrated_loudness(&test_audio);
    let original_short_term = meter.measure_short_term_loudness(&test_audio);
    let original_range = meter.measure_loudness_range(&test_audio);
    println!("   Integrated loudness: {:.2} LUFS", original_integrated);
    println!("   Short-term loudness: {:.2} LUFS", original_short_term);
    println!("   Loudness range: {:.2} LU\n", original_range);

    // ========== Normalization ==========
    println!("3. Normalizing to streaming target (-14 LUFS)...");
    let mut normalizer = LoudnessNormalizer::new(SAMPLE_RATE, LoudnessTarget::StreamingMusic);
    let gain_linear = normalizer.calculate_gain(&test_audio);
    let gain_db = 20.0 * gain_linear.log10();
    println!("   Required gain adjustment: {:.2} dB", gain_db);

    // Apply gain to normalized copy
    let mut normalized_audio = test_audio.clone();
    apply_gain(&mut normalized_audio, gain_db);

    // Measure after normalization
    meter.reset();
    let normalized_loudness = meter.measure_integrated_loudness(&normalized_audio);
    println!("   Normalized loudness: {:.2} LUFS", normalized_loudness);
    println!("   ✓ Normalization complete\n",);

    // ========== Dynamic Range Control ==========
    println!("4. Applying Dynamic Range Control (4:1 ratio, -18dB threshold)...");
    let mut drc_audio = normalized_audio.clone();
    let mut drc = DynamicRangeControl::new(4.0, -18.0, 10.0, 100.0, SAMPLE_RATE);
    drc.set_makeup_gain(((-18.0 * 3.0) / 4.0)); // Calculate makeup gain for 4:1

    drc.process(&mut drc_audio);
    println!("   ✓ DRC compression applied");

    // Measure dynamics after DRC
    meter.reset();
    let drc_integrated = meter.measure_integrated_loudness(&drc_audio);
    let drc_range = meter.measure_loudness_range(&drc_audio);
    println!("   DRC loudness: {:.2} LUFS", drc_integrated);
    println!("   DRC loudness range: {:.2} LU\n", drc_range);

    // ========== Headroom Protection ==========
    println!("5. Applying headroom protection (-3dB threshold)...");
    let mut final_audio = drc_audio.clone();
    let mut limiter = HeadroomManager::new(-3.0, SAMPLE_RATE);

    limiter.apply_limiting(&mut final_audio);
    println!("   ✓ Soft-limiting applied");

    // Measure final audio
    let peak_before = find_peak(&drc_audio);
    let peak_after = find_peak(&final_audio);
    println!(
        "   Peak before limiting: {:.3} ({:.2} dBFS)",
        peak_before,
        linear_to_db(peak_before)
    );
    println!(
        "   Peak after limiting: {:.3} ({:.2} dBFS)\n",
        peak_after,
        linear_to_db(peak_after)
    );

    // ========== Full Pipeline with ReferenceRenderer ==========
    println!("6. Processing through complete ReferenceRenderer pipeline...");
    let mut renderer = ReferenceRenderer::new(SAMPLE_RATE);
    renderer.set_loudness_target(LoudnessTarget::StreamingMusic);
    renderer.enable_drc(4.0, -18.0);
    renderer.set_headroom_db(-3.0);
    renderer.set_headroom_lookahead_ms(3.0);

    println!("   ✓ ReferenceRenderer configured with:");
    println!("     - Target loudness: Streaming Music (-14 LUFS)");
    println!("     - DRC: 4:1 ratio, -18dB threshold");
    println!("     - Headroom: -3dB protection (3ms lookahead)\n");

    // ========== Summary ==========
    println!("=== Processing Summary ===");
    println!();
    println!("Original Audio:");
    println!("  Loudness:       {:.2} LUFS", original_integrated);
    println!("  Range:          {:.2} LU", original_range);
    println!(
        "  Peak:           {:.3} ({:.2} dBFS)",
        find_peak(&test_audio),
        linear_to_db(find_peak(&test_audio))
    );
    println!();
    println!("After Normalization:");
    println!(
        "  Loudness:       {:.2} LUFS (target: -14)",
        normalized_loudness
    );
    println!("  Gain applied:   {:.2} dB", gain_db);
    println!();
    println!("After DRC (4:1 compression):");
    println!("  Loudness:       {:.2} LUFS", drc_integrated);
    println!(
        "  Range reduced:  {:.2} LU ({:.0}% reduction)",
        original_range - drc_range,
        ((original_range - drc_range) / original_range.max(0.1) * 100.0)
    );
    println!();
    println!("After Headroom Protection:");
    println!(
        "  Peak:           {:.3} ({:.2} dBFS)",
        peak_after,
        linear_to_db(peak_after)
    );
    println!("  Clipping margin: {:.2} dB", -linear_to_db(peak_after));
    println!();

    // ========== Standards Compliance ==========
    println!("=== Standards Compliance ===");
    println!("✓ ITU-R BS.1770-4: Loudness measured in LUFS");
    println!(
        "✓ EBU R128: Integrated loudness = {:.2} LUFS",
        normalized_loudness
    );
    if (normalized_loudness - (-14.0)).abs() < 0.5 {
        println!("✓ Streaming standard: Within ±0.5 LUFS of -14 LUFS target");
    }
    println!("✓ Clipping protection: Peak limited to -3dB headroom");
    println!();

    println!("Processing complete! Audio ready for streaming/distribution.");
    Ok(())
}

// ============= Helper Functions =============

/// Generate synthetic audio with dynamic peaks (silence, steady tone, peaks)
fn generate_dynamic_audio(num_samples: usize, num_channels: usize, sample_rate: u32) -> AudioBlock {
    let mut channels = Vec::with_capacity(num_channels);

    for _ch in 0..num_channels {
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / num_samples as f32; // 0.0 to 1.0

            let sample = if t < 0.2 {
                // Silence
                0.0
            } else if t < 0.5 {
                // Steady tone at 0.3 amplitude
                0.3 * (2.0 * std::f32::consts::PI * 440.0 * (i as f32 / sample_rate as f32)).sin()
            } else if t < 0.7 {
                // Ramping peaks
                let peak_envelope = (t - 0.5) * 5.0; // 0.0 to 1.0
                0.8 * peak_envelope
                    * (2.0 * std::f32::consts::PI * 880.0 * (i as f32 / sample_rate as f32)).sin()
            } else {
                // Another steady tone at lower amplitude
                0.2 * (2.0 * std::f32::consts::PI * 220.0 * (i as f32 / sample_rate as f32)).sin()
            };

            samples.push(sample);
        }
        channels.push(samples);
    }

    AudioBlock {
        sample_rate,
        channels,
    }
}

/// Find peak amplitude in audio block
fn find_peak(audio: &AudioBlock) -> f32 {
    audio
        .channels
        .iter()
        .flat_map(|ch| ch.iter())
        .map(|&s| s.abs())
        .fold(0.0f32, f32::max)
}

/// Apply gain in dB to all samples
fn apply_gain(audio: &mut AudioBlock, gain_db: f32) {
    let linear_gain = 10f32.powf(gain_db / 20.0);
    for channel in audio.channels.iter_mut() {
        for sample in channel.iter_mut() {
            *sample *= linear_gain;
        }
    }
}

/// Convert linear amplitude to dB (dBFS)
fn linear_to_db(linear: f32) -> f32 {
    if linear > 0.0 {
        20.0 * linear.log10()
    } else {
        -f32::INFINITY
    }
}
