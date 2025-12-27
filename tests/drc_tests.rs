// SPDX-License-Identifier: Apache-2.0

use audio_ninja::loudness::DynamicRangeControl;
use audio_ninja::AudioBlock;

fn peak_linear(samples: &[f32]) -> f32 {
    samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max)
}

#[test]
fn test_drc_reduces_peaks_without_makeup_gain() {
    let sr = 48000;
    let frames = 2000;

    // Mono block with transient peaks over threshold
    let mut samples = vec![0.0f32; frames];
    // Create some peaks
    for i in (100..frames).step_by(200) {
        if i < frames { samples[i] = 0.9; }
        if i + 1 < frames { samples[i + 1] = 0.7; }
    }

    let block = AudioBlock { sample_rate: sr, channels: vec![samples.clone()] };
    let before_peak = peak_linear(&block.channels[0]);

    let mut drc = DynamicRangeControl::new(4.0, -20.0, 5.0, 80.0, sr);
    drc.set_makeup_gain(0.0); // focus on compression effect
    let mut processed = block.clone();
    drc.process(&mut processed);

    let after_peak = peak_linear(&processed.channels[0]);
    assert!(after_peak < before_peak * 0.96, "peak not sufficiently reduced: before {} after {}", before_peak, after_peak);
}
