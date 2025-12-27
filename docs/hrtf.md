# HRTF (Head-Related Transfer Function) Guide

## Overview

The HRTF module provides binaural audio rendering, converting mono or multi-channel audio into 3D spatial audio suitable for headphone playback. It uses Head-Related Transfer Functions to simulate how sound reaches human ears from various spatial positions.

## Core Components

### 1. HrtfPosition

Represents a 3D spatial position with automatic normalization and clamping:

```rust
use audio_ninja::hrtf::HrtfPosition;

// Create a position: azimuth, elevation, distance (meters)
let position = HrtfPosition::new(45.0, 15.0, 1.5);

// Azimuth: -180° to 180° (automatically normalized)
// Elevation: -90° to 90° (clamped)
// Distance: 0.1 to 10.0 meters (clamped)

println!("Azimuth: {}", position.azimuth);      // 45.0
println!("Elevation: {}", position.elevation);  // 15.0
println!("Distance: {}", position.distance);    // 1.5
```

### 2. HrtfDatabase

Stores and retrieves HRTF impulse responses for different spatial positions:

```rust
use audio_ninja::hrtf::{HrtfDatabase, HrtfDataset};

// Create database with KEMAR dataset (default)
let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);

// Load default KEMAR measurements
db.load_default_kemar().expect("Failed to load KEMAR data");

// Retrieve HRTF for a specific position
let position = HrtfPosition::new(0.0, 0.0, 1.0);
let ir = db.get_response(&position).expect("Failed to get response");

println!("Left IR length: {}", ir.left.len());
println!("Right IR length: {}", ir.right.len());

// Add custom HRTF measurements
let custom_ir = HrtfImpulseResponse::new(
    vec![0.5, 0.3, 0.1],  // Left ear impulse response
    vec![0.5, 0.3, 0.1],  // Right ear impulse response
);
db.add_response(&position, custom_ir);
```

### 3. BinauralRenderer

Converts mono audio to stereo binaural using HRTF convolution:

```rust
use audio_ninja::hrtf::{BinauralRenderer, HeadphoneProfile, HrtfDatabase, HrtfDataset};

// Create renderer with a database and headphone profile
let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
db.load_default_kemar().unwrap();

let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

// Render mono audio to stereo
let mono_input = vec![0.5; 512];  // 512 samples of 0.5 amplitude
let position = HrtfPosition::new(45.0, 0.0, 1.0);

let (left, right) = renderer.render(&mono_input, &position)?;

println!("Left channel: {} samples", left.len());
println!("Right channel: {} samples", right.len());
```

## Headphone Profiles

Different headphone profiles simulate acoustic characteristics:

```rust
use audio_ninja::hrtf::HeadphoneProfile;

// Available profiles:
// - Flat: Minimal coloration, neutral response
// - ClosedBack: Deeper bass, more isolation
// - OpenBack: Thinner response, more spacious
// - IEM: In-ear monitor, close-field response

let flat = HeadphoneProfile::Flat;
let closed = HeadphoneProfile::ClosedBack;
let open = HeadphoneProfile::OpenBack;
let iem = HeadphoneProfile::IEM;
```

## Complete Usage Example

### Basic Binaural Rendering

```rust
use audio_ninja::hrtf::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize database
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar()?;
    
    // 2. Create renderer
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);
    
    // 3. Generate or load mono audio
    let audio = vec![0.3; 2400];  // 50ms at 48kHz
    
    // 4. Define source position (45° azimuth, 0° elevation, 1.5m distance)
    let position = HrtfPosition::new(45.0, 0.0, 1.5);
    
    // 5. Render to stereo binaural
    let (left, right) = renderer.render(&audio, &position)?;
    
    println!("Rendered {} samples per channel", left.len());
    println!("Left max: {}", left.iter().cloned().fold(0.0, f32::max));
    println!("Right max: {}", right.iter().cloned().fold(0.0, f32::max));
    
    Ok(())
}
```

### Multi-Channel Buffer Processing

For processing multiple channels simultaneously:

```rust
use audio_ninja::hrtf::*;

fn process_multi_channel() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar()?;
    
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::ClosedBack);
    
    // Multi-channel input (3 mono sources)
    let input = vec![
        vec![0.5; 512],  // Source 1
        vec![0.3; 512],  // Source 2
        vec![0.2; 512],  // Source 3
    ];
    
    // Positions for each source
    let positions = vec![
        HrtfPosition::new(0.0, 0.0, 1.0),    // Center
        HrtfPosition::new(90.0, 0.0, 1.0),   // Right side
        HrtfPosition::new(-90.0, 15.0, 1.5), // Left elevated
    ];
    
    // Render all channels
    let output = renderer.render_buffer(input.as_slice(), positions.as_slice())?;
    
    println!("Output channels: {}", output.len());  // 2 (stereo)
    println!("Samples per channel: {}", output[0].len());
    
    Ok(())
}
```

### Dynamic Position Updates

```rust
use audio_ninja::hrtf::*;

fn animate_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar()?;
    
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);
    let audio = vec![0.5; 480];  // 10ms at 48kHz
    
    // Rotate source around listener (0° to 360°)
    for angle in (0..360).step_by(15) {
        let position = HrtfPosition::new(angle as f32, 0.0, 1.0);
        let (left, right) = renderer.render(&audio, &position)?;
        
        println!("Angle: {}°, Left: {}, Right: {}", 
                 angle, left[0], right[0]);
    }
    
    Ok(())
}
```

## Position Conventions

### Azimuth (Horizontal Plane)
- **0°**: Front (listener facing direction)
- **90°**: Right side
- **±180°**: Behind
- **-90°**: Left side

### Elevation (Vertical Plane)
- **0°**: Horizontal (ear level)
- **90°**: Above
- **-90°**: Below

### Distance
- **0.1m**: Very close (intimate)
- **1.0m**: Typical listening distance
- **3.0m**: Far field
- **10.0m**: Maximum (architectural space)

## Performance Considerations

### Sample Processing
- Each mono sample requires convolution with ~256-512 tap HRTF filters
- Longer impulse responses = higher quality but more CPU
- Real-time processing requires efficient convolution algorithms

### Buffer Sizes
- Recommend 480-960 samples per buffer (10-20ms at 48kHz)
- Smaller buffers = lower latency but higher overhead
- Larger buffers = higher throughput but increased latency

### Memory Usage
- KEMAR database: ~512KB for full resolution dataset
- Per-channel buffer: sample_count × 4 bytes (f32)
- Renderer state: ~1KB per instance

## Integration with Transport Layer

Combine HRTF rendering with networked audio delivery:

```rust
use audio_ninja::hrtf::*;
use audio_ninja::transport::*;
use audio_ninja::AudioBlock;

fn network_binaural_stream() -> Result<(), Box<dyn std::error::Error>> {
    // Setup HRTF renderer
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar()?;
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);
    
    // Setup transport
    let mut transport = LoopbackTransport::default();
    
    // Create binaural audio block
    let mono_source = vec![0.4; 960];
    let position = HrtfPosition::new(30.0, 10.0, 1.2);
    let (left, right) = renderer.render(&mono_source, &position)?;
    
    let block = AudioBlock {
        sample_rate: 48000,
        channels: vec![left, right],  // Stereo binaural
    };
    
    // Send as RTP packet
    let packet = RtpPacket::new(0, 0, 12345, 
        crate::transport::serialize_audio_block(&block));
    
    transport.send(packet)?;
    println!("Sent binaural audio via RTP");
    
    Ok(())
}
```

## Error Handling

```rust
use audio_ninja::hrtf::*;

fn handle_errors() {
    match HrtfDatabase::new(HrtfDataset::Kemar, 48000).load_default_kemar() {
        Ok(_) => println!("Database loaded successfully"),
        Err(e) => eprintln!("Failed to load HRTF database: {}", e),
    }
    
    // Position normalization handles out-of-range values gracefully
    let extreme_pos = HrtfPosition::new(540.0, 120.0, 50.0);
    assert!(extreme_pos.azimuth >= -180.0 && extreme_pos.azimuth <= 180.0);
    assert!(extreme_pos.elevation >= -90.0 && extreme_pos.elevation <= 90.0);
    assert!(extreme_pos.distance <= 10.0);
}
```

## Testing

Run the comprehensive HRTF test suite:

```bash
# Run all HRTF tests
cargo test hrtf --

# Run specific test
cargo test test_binaural_render_different_positions -- --nocapture

# Run with logging
RUST_LOG=debug cargo test hrtf --
```

## References

- **KEMAR Dataset**: MIT Media Lab Head-Related Transfer Functions
- **HRTF Standard**: ITU-R BS.2051 (Audio for Services: General requirements and characterization)
- **Binaural Audio**: International Organization for Standardization (ISO) 3382-1

## Related Modules

- [VBAP (Vector Base Amplitude Panning)](./vbap.md) - 3D spatial rendering for speaker arrays
- [HOA (Higher-Order Ambisonics)](./hoa.md) - Scene-based spatial audio decoding
- [DSP](../src/dsp.rs) - Digital Signal Processing filters and effects

