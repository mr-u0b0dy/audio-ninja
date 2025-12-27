// SPDX-License-Identifier: Apache-2.0
//! Quick-start example: Binaural HRTF rendering

use audio_ninja::hrtf::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Audio-Ninja HRTF Binaural Rendering Example ===\n");

    // Step 1: Create and load HRTF database
    println!("📊 Loading HRTF database...");
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar()?;
    println!("✓ KEMAR database loaded\n");

    // Step 2: Create binaural renderer with headphone profile
    println!("🎧 Creating binaural renderer...");
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);
    println!("✓ Renderer ready with Flat headphone profile\n");

    // Step 3: Generate or load mono audio (simulating a point source)
    println!("🔊 Preparing audio signal...");
    let sample_rate = 48000;
    let duration_ms = 100;
    let num_samples = (sample_rate as usize * duration_ms) / 1000;
    
    // Create a simple test signal: 1kHz sine wave
    let mono_input: Vec<f32> = (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let freq = 1000.0; // 1kHz
            (2.0 * std::f32::consts::PI * freq * t).sin() * 0.3 // 0.3 amplitude
        })
        .collect();
    
    println!("✓ Generated {}ms test signal ({} samples) at {}Hz\n", 
             duration_ms, num_samples, sample_rate);

    // Step 4: Render from different positions
    println!("🎯 Rendering from various spatial positions:\n");
    
    let test_positions = vec![
        ("Front Center", HrtfPosition::new(0.0, 0.0, 1.0)),
        ("Front Right", HrtfPosition::new(45.0, 0.0, 1.0)),
        ("Right Side", HrtfPosition::new(90.0, 0.0, 1.0)),
        ("Right Elevated", HrtfPosition::new(90.0, 30.0, 1.2)),
        ("Back Right", HrtfPosition::new(135.0, 0.0, 1.0)),
    ];

    for (name, position) in test_positions {
        let (left, right) = renderer.render(&mono_input, &position)?;
        
        let left_max = left.iter().cloned().fold(0.0, f32::max);
        let right_max = right.iter().cloned().fold(0.0, f32::max);
        let left_rms = (left.iter().map(|x| x * x).sum::<f32>() / left.len() as f32).sqrt();
        let right_rms = (right.iter().map(|x| x * x).sum::<f32>() / right.len() as f32).sqrt();
        
        println!("  {} (Az: {:6.1}°, El: {:5.1}°, Dist: {:.1}m)", 
                 name, position.azimuth, position.elevation, position.distance);
        println!("    Left:  Peak={:.3}, RMS={:.3}", left_max, left_rms);
        println!("    Right: Peak={:.3}, RMS={:.3}", right_max, right_rms);
        println!();
    }

    // Step 5: Demonstrate multi-channel processing
    println!("📻 Multi-channel buffer processing example:\n");
    
    let multi_input = vec![
        mono_input.clone(),                    // Source 1
        mono_input.iter().map(|x| x * 0.7).collect::<Vec<_>>(), // Source 2 (quieter)
    ];
    
    let positions = vec![
        HrtfPosition::new(30.0, 0.0, 1.0),
        HrtfPosition::new(-30.0, 0.0, 1.0),
    ];
    
    let output = renderer.render_buffer(multi_input.as_slice(), positions.as_slice())?;
    
    println!("  Input: {} channels × {} samples", multi_input.len(), num_samples);
    println!("  Output: {} channels × {} samples (stereo binaural)", output.len(), output[0].len());
    
    let left_energy: f32 = output[0].iter().map(|x| x * x).sum();
    let right_energy: f32 = output[1].iter().map(|x| x * x).sum();
    
    println!("  Left channel energy: {:.6}", left_energy);
    println!("  Right channel energy: {:.6}", right_energy);
    println!("  Energy ratio: {:.2}", left_energy / right_energy.max(0.0001));
    println!();

    // Step 6: Demonstrate different headphone profiles
    println!("🎵 Comparing headphone profiles:\n");
    
    let position = HrtfPosition::new(45.0, 15.0, 1.0);
    let test_signal = vec![0.5; 960];
    
    let profiles = vec![
        ("Flat", HeadphoneProfile::Flat),
        ("Closed-Back", HeadphoneProfile::ClosedBack),
        ("Open-Back", HeadphoneProfile::OpenBack),
        ("IEM", HeadphoneProfile::IEM),
    ];
    
    let mut db2 = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db2.load_default_kemar()?;
    
    for (name, profile) in profiles {
        let renderer = BinauralRenderer::new(db2.clone(), profile);
        let (left, right) = renderer.render(&test_signal, &position)?;
        
        let diff: f32 = left.iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum();
        
        println!("  {:<15} L/R difference: {:.3}", name, diff);
    }
    println!();

    println!("✅ HRTF binaural rendering demonstration complete!");
    println!("\n📖 For more details, see docs/hrtf.md");

    Ok(())
}
