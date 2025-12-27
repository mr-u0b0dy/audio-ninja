# Binaural Rendering for Headphone Audio

## Overview

The **ReferenceRenderer** now supports HRTF-based binaural rendering to virtualize multi-channel speaker layouts as immersive stereo audio for headphone playback. This enables users with headphones to experience spatial audio that was originally mixed for speaker arrays.

## What is Binaural Audio?

Binaural rendering uses **Head-Related Transfer Functions (HRTFs)** to simulate how sound from different spatial positions reaches the two ears. By convolving audio with appropriate HRTF filters, sound sources can be placed anywhere in 3D space around the listener's head, creating an immersive illusion of spatial presence.

### Key Benefits
- **Headphone Immersion**: Experience spatial audio on any headphones without special hardware
- **Spatial Localization**: Accurate front/back/up/down positioning cues
- **Format Flexibility**: Works with any channel configuration (mono, stereo, 5.1, 7.1, etc.)

## HRTF Datasets and Headphone Profiles

### Available HRTF Datasets
- **KEMAR**: Knowles Electronics Manikin for Acoustic Research (default, well-balanced)
- **CIPIC**: Center for Image Processing and Integrated Computing (individual variation)
- **MIT KEMAR**: MIT variant of KEMAR (optimized for research)

### Headphone Compensation Profiles
- **Flat**: Raw HRTF without equalization (neutral response)
- **ClosedBack**: Compensation for closed-back headphones (darker, more intimate)
- **OpenBack**: Compensation for open-back headphones (more diffuse, spacious)
- **IEM**: In-Ear Monitor compensation (extended presence peak)

## API Usage

### Basic Setup

```rust
use audio_ninja::render::{ReferenceRenderer, RenderOptions};
use audio_ninja::hrtf::HeadphoneProfile;
use audio_ninja::AudioBlock;

// Create renderer
let mut renderer = ReferenceRenderer::new(48000);

// Enable binaural rendering with headphone profile
renderer.enable_binaural(HeadphoneProfile::ClosedBack)?;

// Binaural is automatically positioned at front-center (0°, 0°, 1m)
// You can change the position:
renderer.set_binaural_position(45.0, 0.0, 1.0); // 45° azimuth, 0° elevation, 1m distance
```

### Rendering Multi-Channel Audio

```rust
// Multi-channel speaker layout audio (5.1 surround)
let surround_audio = AudioBlock {
    sample_rate: 48000,
    channels: vec![
        // FL, FR, C, LFE, SL, SR
        vec![0.1; 480],
        vec![0.1; 480],
        vec![0.05; 480],
        vec![0.2; 480],
        vec![0.08; 480],
        vec![0.08; 480],
    ],
};

// Render through binaural pipeline
let opts = RenderOptions::default();
let output = renderer.render(surround_audio, &opts);

// Output is now stereo binaural audio
assert_eq!(output.channels.len(), 2); // Stereo L/R
```

### Spatial Positioning

Position parameters define where the virtual sound source is located relative to the listener's head:

```rust
// Front-center (default)
renderer.set_binaural_position(0.0, 0.0, 1.0);

// Left side, ear level
renderer.set_binaural_position(-90.0, 0.0, 1.0);

// Above and behind
renderer.set_binaural_position(135.0, 45.0, 1.5);

// Distance can affect elevation cues
renderer.set_binaural_position(0.0, 30.0, 0.5); // Closer = stronger elevation cue
```

**Position Parameters:**
- **Azimuth**: Horizontal angle in degrees (-180° to 180°, 0° = front)
- **Elevation**: Vertical angle in degrees (-90° to 90°, 0° = ear level, 90° = above)
- **Distance**: Distance in meters (typically 0.5 to 2.0m; affects near-field effects)

## Signal Processing Pipeline

When binaural rendering is enabled, the complete audio pipeline is:

```
Input Audio
    ↓
DRC Compression (optional)
    ↓
Loudness Normalization (optional)
    ↓
Headroom Protection (always enabled)
    ↓
Binaural HRTF Convolution
    ↓
Stereo Output (L/R channels)
```

This ensures that dynamic range control and loudness management happen before spatial rendering, maintaining audio integrity while creating immersive headphone playback.

## Complete Example

```rust
use audio_ninja::render::{ReferenceRenderer, RenderOptions, DRCPreset};
use audio_ninja::loudness::LoudnessTarget;
use audio_ninja::hrtf::HeadphoneProfile;
use audio_ninja::AudioBlock;

fn main() -> anyhow::Result<()> {
    // Create renderer for headphone listening
    let mut renderer = ReferenceRenderer::new(48000);
    
    // Set loudness target for streaming
    renderer.set_loudness_target(LoudnessTarget::StreamingMusic);
    
    // Apply moderate DRC for dynamic control
    renderer.apply_drc_preset(DRCPreset::Music);
    
    // Enable binaural for headphone virtualization
    renderer.enable_binaural(HeadphoneProfile::ClosedBack)?;
    
    // Place virtual audio at front-left for testing
    renderer.set_binaural_position(-30.0, 0.0, 1.0);
    
    // Process 5.1 surround mix
    let surround_audio = AudioBlock {
        sample_rate: 48000,
        channels: vec![
            vec![0.1; 48000],  // FL
            vec![0.1; 48000],  // FR
            vec![0.05; 48000], // C
            vec![0.1; 48000],  // LFE (subwoofer)
            vec![0.08; 48000], // SL
            vec![0.08; 48000], // SR
        ],
    };
    
    let opts = RenderOptions::default();
    let stereo_binaural = renderer.render(surround_audio, &opts);
    
    println!("Converted 5.1 surround to {} stereo binaural", 
             stereo_binaural.channels.len());
    
    Ok(())
}
```

## Channel Downmixing Strategy

When rendering multi-channel audio to binaural, the module automatically mixes channels:

- **Mono Input**: Used directly for binaural convolution
- **Stereo Input**: Mixed to mono (L + R) * 0.5, then rendered
- **Surround (5.1+)**: All channels mixed to mono, then rendered

The mono-to-stereo binaural output preserves spatial positioning through HRTF filtering while integrating all input channels.

## Advanced Configuration

### Disable and Re-enable

```rust
renderer.disable_binaural();  // Turn off binaural processing
renderer.enable_binaural(HeadphoneProfile::OpenBack)?; // Re-enable with different profile
```

### Check Status

```rust
if renderer.has_binaural() {
    println!("Binaural rendering is active");
}
```

### Combine with Other Features

```rust
// Binaural works alongside all other renderer features
renderer.set_loudness_target(LoudnessTarget::Television);
renderer.enable_drc_with_params(4.0, -20.0, 10.0, 100.0);
renderer.set_headroom_db(3.0);
renderer.enable_binaural(HeadphoneProfile::IEM)?;
renderer.set_binaural_position(90.0, 45.0, 1.2);
```

## Performance Considerations

- **CPU Load**: HRTF convolution adds ~3-5% CPU overhead vs. stereo rendering
- **Latency**: Minimal (~1-2ms from HRTF filtering)
- **Memory**: HRTF database is ~2-3MB in-memory
- **Real-time Safe**: Can be used in real-time audio processing

## Troubleshooting

### No Spatial Effect
- Verify position is set via `set_binaural_position()`
- Check that `has_binaural()` returns true
- Ensure output is being listened to on headphones (not speakers)

### Audio Sounds Unnatural
- Try different headphone profiles (ClosedBack, OpenBack, IEM)
- Adjust position to 0° azimuth, 0° elevation for baseline
- Reduce elevation angles initially; humans are worse at vertical localization

### Very Quiet Output
- Ensure loudness normalization is configured or disabled
- Check headphone volume levels
- Verify DRC is not over-compressing

## References

- Blauert, J. (1997). *Spatial Hearing*. MIT Press
- Møller, H., Sorensen, M. F., & Jensen, C. B. (1996). Design and verification of a headphone-based system for binaural virtual environments
- ITU-R BS.2051: Advanced Sound System Broadcasting
