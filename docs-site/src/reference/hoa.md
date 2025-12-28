# HOA (Higher-Order Ambisonics) Decoder

## Overview

The `hoa` module provides Higher-Order Ambisonics decoding for scene-based spatial audio. HOA represents sound fields using spherical harmonics, enabling flexible reproduction on arbitrary speaker layouts.

## Features

- **Multiple Orders**: 1st (B-format), 2nd, and 3rd order ambisonics
- **Decoding Modes**: Basic, Max-rE (energy optimized), and In-Phase
- **Flexible Layouts**: Support for any speaker configuration
- **Standard Presets**: Stereo, 5.1, 7.1.4, and Cube layouts
- **Buffer Processing**: Efficient multi-sample decoding

## Usage

### Basic Example

```rust
use audio_ninja::hoa::{HoaDecoder, AmbisonicOrder, DecodingMode, create_5_1_hoa_layout};

// Create a 1st order decoder for 5.1 layout
let speakers = create_5_1_hoa_layout();
let decoder = HoaDecoder::new(
    AmbisonicOrder::FIRST,
    DecodingMode::MaxRE,
    speakers
);

// Decode ambisonic channels to speaker feeds
// For 1st order: [W, Y, Z, X] (ACN ordering)
let ambisonic_input = vec![1.0, 0.5, 0.0, 0.3];
let speaker_gains = decoder.decode(&ambisonic_input);

// Apply to audio
for (channel, gain) in audio_channels.iter_mut().zip(speaker_gains.iter()) {
    for sample in channel.iter_mut() {
        *sample *= gain;
    }
}
```

### Buffer Processing

```rust
// Process entire buffers efficiently
let input_channels = vec![
    vec![1.0; 1024], // W channel
    vec![0.5; 1024], // Y channel
    vec![0.0; 1024], // Z channel
    vec![0.3; 1024], // X channel
];

let output_speakers = decoder.decode_buffer(&input_channels);
// output_speakers[speaker_id][sample_idx]
```

### Higher Orders

```rust
use audio_ninja::hoa::{AmbisonicOrder, create_7_1_4_hoa_layout};

// 2nd order (9 channels) for 7.1.4 layout
let speakers = create_7_1_4_hoa_layout();
let decoder = HoaDecoder::new(
    AmbisonicOrder::SECOND,
    DecodingMode::MaxRE,
    speakers
);

// 3rd order (16 channels) for even higher resolution
let decoder_3rd = HoaDecoder::new(
    AmbisonicOrder::THIRD,
    DecodingMode::MaxRE,
    speakers
);
```

### Custom Speaker Layout

```rust
use audio_ninja::hoa::HoaSpeaker;

let custom_speakers = vec![
    HoaSpeaker::new(0, 0.0, 0.0),      // Front
    HoaSpeaker::new(1, 90.0, 0.0),     // Left
    HoaSpeaker::new(2, 180.0, 0.0),    // Back
    HoaSpeaker::new(3, -90.0, 0.0),    // Right
    HoaSpeaker::new(4, 0.0, 45.0),     // Top Front
    HoaSpeaker::new(5, 180.0, 45.0),   // Top Back
];

let decoder = HoaDecoder::new(
    AmbisonicOrder::FIRST,
    DecodingMode::Basic,
    custom_speakers
);
```

## Ambisonic Orders

### 1st Order (B-format)
- **Channels**: 4 (W, Y, Z, X)
- **Resolution**: ~90° localization
- **Best for**: Small rooms, VR headsets
- **Speaker requirement**: Minimum 4 speakers

### 2nd Order
- **Channels**: 9
- **Resolution**: ~45° localization
- **Best for**: Home theaters, small venues
- **Speaker requirement**: 8-12 speakers ideal

### 3rd Order
- **Channels**: 16
- **Resolution**: ~30° localization
- **Best for**: Professional installations, large venues
- **Speaker requirement**: 12+ speakers

## Decoding Modes

### Basic
- Simple pseudoinverse decoding
- Preserves amplitude relationships
- Best for uniform speaker layouts

### Max-rE (Maximum Energy Vector)
- Optimizes energy localization
- Better directional accuracy
- Recommended for most applications
- Trade-off: slight high-frequency rolloff

### In-Phase
- Preserves phase relationships
- Smoother frequency response
- Best for music reproduction
- Trade-off: reduced spatial resolution

## Channel Ordering (ACN)

Ambisonic Channel Numbering (ACN) is used:

**1st Order** (4 channels):
- 0: W (omnidirectional)
- 1: Y (left-right, corresponds to sides)
- 2: Z (up-down, corresponds to elevation)
- 3: X (front-back, corresponds to look direction)

**2nd Order** adds 5 more channels (indices 4-8)  
**3rd Order** adds 7 more channels (indices 9-15)

## Standard Layouts

### Stereo
```rust
let speakers = create_stereo_hoa_layout();
// FL: -30°, FR: +30°
```

### 5.1
```rust
let speakers = create_5_1_hoa_layout();
// FL, FR, C, SL, SR, Back (6 speakers for better coverage)
```

### 7.1.4 (Dolby Atmos)
```rust
let speakers = create_7_1_4_hoa_layout();
// 8 ear-level + 4 height speakers
```

### Cube (Ideal for 1st Order)
```rust
let speakers = create_cube_hoa_layout();
// 8 speakers forming a cube around listener
```

## Algorithm

The decoder works by:

1. **Encoding Matrix**: Compute spherical harmonics at each speaker position
2. **Mode Weights**: Apply frequency-dependent weights (Basic/Max-rE/In-Phase)
3. **Pseudoinverse**: Compute decode matrix (transpose for uniform layouts)
4. **Normalization**: Scale for energy preservation
5. **Matrix Multiply**: Transform ambisonic channels to speaker feeds

## Integration with IAMF

HOA complements VBAP for IAMF rendering:

- **VBAP**: Object-based elements (discrete sources)
- **HOA**: Scene-based elements (ambient soundfields)
- **Channel**: Pre-mixed speaker feeds

The renderer selects the appropriate method based on element type.

## Performance

- **Initialization**: O(n²) where n = speaker count (matrix computation)
- **Decoding**: O(n × m) where n = speakers, m = ambisonic channels
- Typical decode times (1st order, 8 speakers): ~50ns per sample

## Testing

The module includes 25 comprehensive tests:

- Order and channel count validation
- Spherical harmonics accuracy
- Decoder creation and properties
- Signal decoding correctness
- Mode comparison (Basic, Max-rE, In-Phase)
- Buffer processing
- Energy preservation
- Layout presets

Run tests:
```bash
cargo test hoa
```

## Coordinate System

- **Azimuth**: 0° = front, 90° = left, -90° = right, ±180° = back
- **Elevation**: 0° = horizontal, 90° = up, -90° = down
- Follows right-handed coordinate system with Y forward

## References

- Zotter, F., Frank, M. (2019). "Ambisonics: A Practical 3D Audio Theory for Recording, Studio Production, Sound Reinforcement, and Virtual Reality"
- Daniel, J. (2001). "Représentation de champs acoustiques, application à la transmission et à la reproduction de scènes sonores complexes dans un contexte multimédia"
- ITU-R BS.2051: Advanced sound system for programme production

## Comparison: HOA vs VBAP

| Feature | HOA | VBAP |
|---------|-----|------|
| **Use Case** | Ambient soundfields | Discrete point sources |
| **Representation** | Spherical harmonics | Vector panning |
| **Strength** | Smooth, diffuse sounds | Sharp localization |
| **Speaker Requirement** | More speakers = better | Works with any layout |
| **Computational Cost** | Matrix multiply | Triplet search + gains |
| **IAMF Element Type** | Scene-based | Object-based |

Both are implemented in audio-ninja for complete IAMF support.
