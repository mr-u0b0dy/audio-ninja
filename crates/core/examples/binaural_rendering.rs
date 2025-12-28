// Example: Binaural rendering for headphone playback
//
// This example demonstrates:
// 1. Converting multi-channel surround audio to binaural stereo
// 2. Configuring headphone profiles (Flat, ClosedBack, OpenBack, IEM)
// 3. Spatial positioning of virtual speakers
// 4. Combining with loudness normalization and DRC
// 5. Processing different channel configurations

use audio_ninja::{
    hrtf::HeadphoneProfile,
    loudness::LoudnessTarget,
    render::{DRCPreset, ReferenceRenderer, RenderOptions, Renderer},
    AudioBlock,
};

fn main() -> anyhow::Result<()> {
    println!("=== Binaural Rendering for Headphone Playback ===\n");

    const SAMPLE_RATE: u32 = 48000;
    const DURATION_SAMPLES: usize = 48000; // 1 second

    // ========== Headphone Profile Comparison ==========
    println!("1. Demonstrating different headphone profiles...\n");

    let profiles = vec![
        ("Flat", HeadphoneProfile::Flat),
        ("ClosedBack", HeadphoneProfile::ClosedBack),
        ("OpenBack", HeadphoneProfile::OpenBack),
        ("IEM", HeadphoneProfile::IEM),
    ];

    // Create test audio: 5.1 surround format
    let surround_5_1 = AudioBlock {
        sample_rate: SAMPLE_RATE,
        channels: vec![
            vec![0.1; DURATION_SAMPLES],  // FL (Front Left)
            vec![0.1; DURATION_SAMPLES],  // FR (Front Right)
            vec![0.05; DURATION_SAMPLES], // C (Center)
            vec![0.08; DURATION_SAMPLES], // LFE (Subwoofer)
            vec![0.08; DURATION_SAMPLES], // SL (Side Left)
            vec![0.08; DURATION_SAMPLES], // SR (Side Right)
        ],
    };

    for (name, profile) in profiles {
        let mut renderer = ReferenceRenderer::new(SAMPLE_RATE);
        renderer.set_loudness_target(LoudnessTarget::StreamingMusic);
        renderer.enable_binaural(profile)?;
        renderer.set_binaural_position(0.0, 0.0, 1.0); // Front-center

        let opts = RenderOptions::default();
        let output = renderer.render(surround_5_1.clone(), &opts);

        println!("  {} profile:", name);
        println!(
            "    Input: 5.1 surround ({} channels)",
            surround_5_1.channels.len()
        );
        println!(
            "    Output: Binaural stereo ({} channels)",
            output.channels.len()
        );
        println!("    Sample rate: {} Hz", output.sample_rate);
    }

    // ========== Spatial Positioning ==========
    println!("\n2. Spatial positioning examples...\n");

    let positions = vec![
        ("Front", 0.0, 0.0),
        ("Left", -90.0, 0.0),
        ("Right", 90.0, 0.0),
        ("Above", 0.0, 45.0),
        ("Front-Left", -45.0, 0.0),
        ("Back-Right", 135.0, 0.0),
    ];

    let stereo_input = AudioBlock {
        sample_rate: SAMPLE_RATE,
        channels: vec![
            vec![0.2; DURATION_SAMPLES], // Left
            vec![0.2; DURATION_SAMPLES], // Right
        ],
    };

    for (label, azimuth, elevation) in positions {
        let mut renderer = ReferenceRenderer::new(SAMPLE_RATE);
        renderer.enable_binaural(HeadphoneProfile::ClosedBack)?;
        renderer.set_binaural_position(azimuth, elevation, 1.0);

        let opts = RenderOptions::default();
        let _output = renderer.render(stereo_input.clone(), &opts);

        println!(
            "  {} position (az={}°, el={}°): Rendered to binaural stereo",
            label, azimuth, elevation
        );
    }

    // ========== Combined Processing ==========
    println!("\n3. Combined loudness + DRC + binaural processing...\n");

    let mut renderer = ReferenceRenderer::new(SAMPLE_RATE);

    // Configure full processing chain
    renderer.set_loudness_target(LoudnessTarget::StreamingMusic);
    renderer.apply_drc_preset(DRCPreset::Music);
    renderer.set_headroom_db(3.0);
    renderer.set_headroom_lookahead_ms(3.0);
    renderer.enable_binaural(HeadphoneProfile::ClosedBack)?;

    println!("  Processing chain configured:");
    println!("    ✓ Loudness target: Streaming Music (-14 LUFS)");
    println!("    ✓ DRC preset: Music (4:1, -18dB, 10ms/100ms)");
    println!("    ✓ Headroom: 3dB with 3ms lookahead");
    println!("    ✓ Binaural profile: ClosedBack");

    let mut surround_dynamic = AudioBlock {
        sample_rate: SAMPLE_RATE,
        channels: vec![
            vec![0.1; DURATION_SAMPLES],
            vec![0.1; DURATION_SAMPLES],
            vec![0.05; DURATION_SAMPLES],
            vec![0.15; DURATION_SAMPLES], // Stronger subwoofer
            vec![0.08; DURATION_SAMPLES],
            vec![0.08; DURATION_SAMPLES],
        ],
    };

    // Simulate dynamic peaks
    for i in 0..5 {
        let start = i * 10000;
        let end = (i + 1) * 10000;
        if end <= DURATION_SAMPLES {
            for ch in &mut surround_dynamic.channels {
                for sample in &mut ch[start..end] {
                    *sample *= 1.5; // Add peaks
                }
            }
        }
    }

    let opts = RenderOptions::default();
    let output = renderer.render(surround_dynamic, &opts);

    println!("\n  Result:");
    println!("    Input: 5.1 surround with dynamic peaks");
    println!(
        "    Output: {} channels (binaural stereo)",
        output.channels.len()
    );
    println!("    ✓ All processing applied in correct order");

    // ========== Channel Conversion Examples ==========
    println!("\n4. Handling different input formats...\n");

    let test_cases = vec![
        ("Mono", vec![vec![0.3; DURATION_SAMPLES]]),
        (
            "Stereo",
            vec![vec![0.2; DURATION_SAMPLES], vec![0.2; DURATION_SAMPLES]],
        ),
        (
            "5.1",
            vec![
                vec![0.1; DURATION_SAMPLES],
                vec![0.1; DURATION_SAMPLES],
                vec![0.05; DURATION_SAMPLES],
                vec![0.1; DURATION_SAMPLES],
                vec![0.08; DURATION_SAMPLES],
                vec![0.08; DURATION_SAMPLES],
            ],
        ),
        (
            "7.1",
            vec![
                vec![0.1; DURATION_SAMPLES],
                vec![0.1; DURATION_SAMPLES],
                vec![0.05; DURATION_SAMPLES],
                vec![0.1; DURATION_SAMPLES],
                vec![0.08; DURATION_SAMPLES],
                vec![0.08; DURATION_SAMPLES],
                vec![0.08; DURATION_SAMPLES],
                vec![0.08; DURATION_SAMPLES],
            ],
        ),
    ];

    for (format, channels) in test_cases {
        let mut renderer = ReferenceRenderer::new(SAMPLE_RATE);
        renderer.enable_binaural(HeadphoneProfile::Flat)?;

        let input = AudioBlock {
            sample_rate: SAMPLE_RATE,
            channels,
        };

        println!("  {} -> {}", format, input.channels.len());

        let opts = RenderOptions::default();
        let output = renderer.render(input.clone(), &opts);

        println!(
            "    Binaural conversion: {} channels → {} channels",
            input.channels.len(),
            output.channels.len()
        );
    }

    // ========== Summary ==========
    println!("\n=== Binaural Processing Complete ===");
    println!("\nKey Features Demonstrated:");
    println!("  • Multi-channel to stereo binaural downmix");
    println!("  • Headphone profile selection (4 profiles)");
    println!("  • Spatial positioning (azimuth, elevation, distance)");
    println!("  • Integration with loudness normalization");
    println!("  • Integration with DRC and headroom management");
    println!("  • Support for various input formats (mono to 7.1)");
    println!("\nNext Steps:");
    println!("  • Use renderer.has_binaural() to check status");
    println!("  • Adjust renderer.set_binaural_position() for different speakers");
    println!("  • Try different profiles for optimal headphone experience");

    Ok(())
}
