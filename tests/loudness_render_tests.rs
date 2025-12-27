// SPDX-License-Identifier: Apache-2.0

use audio_ninja::loudness::{LoudnessMeter, LoudnessTarget};
use audio_ninja::render::{ReferenceRenderer, RenderOptions, Renderer};
use audio_ninja::AudioBlock;

fn peak_linear(samples: &[f32]) -> f32 {
    samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max)
}

fn db_to_linear(db: f32) -> f32 { 10f32.powf(db / 20.0) }

#[test]
fn test_renderer_normalizes_to_streaming_target() {
    let sr = 48000;
    let mut renderer = ReferenceRenderer::new(sr);
    renderer.set_loudness_target(LoudnessTarget::StreamingMusic);

    // Create stereo block with moderate level
    let frames = 4800;
    let block = AudioBlock {
        sample_rate: sr,
        channels: vec![vec![0.2; frames], vec![0.2; frames]],
    };

    let opts = RenderOptions::default();
    let output = renderer.render(block, &opts);

    // Measure integrated loudness after normalization
    let mut meter = LoudnessMeter::new(sr);
    let lufs = meter.measure_integrated_loudness(&output);

    // Expect near -14 LUFS within reasonable tolerance
    assert!((lufs - (-14.0)).abs() < 1.0, "normalized loudness {} LUFS not near -14", lufs);
}

#[test]
fn test_renderer_headroom_limits_peaks() {
    let sr = 48000;
    let mut renderer = ReferenceRenderer::new(sr);
    renderer.set_headroom_db(3.0);

    // Block with peaks close to full scale
    let frames = 1000;
    let block = AudioBlock {
        sample_rate: sr,
        channels: vec![vec![0.99; frames], vec![0.99; frames]],
    };

    let before_peak = peak_linear(&block.channels[0]);

    let opts = RenderOptions::default();
    let output = renderer.render(block, &opts);

    let after_peak = peak_linear(&output.channels[0]);
    let threshold = db_to_linear(-3.0);

    assert!(after_peak <= threshold * 1.01, "peak {} exceeds -3dB threshold {}", after_peak, threshold);
    assert!(after_peak <= before_peak, "peak should not increase after limiting");
}
