# 3D VBAP (Vector-Based Amplitude Panning)

## Overview

The `vbap` module provides full 3D spatial audio rendering using Vector-Based Amplitude Panning for arbitrary speaker layouts with elevation support.

## Features

- **3D Vector Math**: Cartesian and spherical coordinate conversion
- **Arbitrary Speaker Layouts**: Support for any speaker configuration from 2.0 stereo up to 9.1.6+ layouts
- **Elevation Support**: Full 3D positioning with azimuth and elevation
- **Energy Preservation**: Normalized gains maintain audio energy
- **Automatic Triplet Finding**: Discovers valid speaker triangles for panning
- **Standard Layout Presets**: Stereo, 5.1, 7.1.4 configurations included

## Usage

### Basic Example

```rust
use audio_ninja::vbap::{Vbap3D, create_5_1_layout, Vec3};

// Create a 5.1 surround layout
let speakers = create_5_1_layout();
let vbap = Vbap3D::new(speakers);

// Position a sound source 30° left, 15° elevated
let source = Vec3::from_spherical(30.0, 15.0, 1.0);
let gains = vbap.render(&source);

// Apply gains to your audio channels
for (channel, gain) in audio_channels.iter_mut().zip(gains.iter()) {
    for sample in channel.iter_mut() {
        *sample *= gain;
    }
}
```

### Custom Speaker Layout

```rust
use audio_ninja::vbap::{Speaker3D, Vbap3D};

let speakers = vec![
    Speaker3D::new(0, -30.0, 0.0),  // Front Left
    Speaker3D::new(1, 30.0, 0.0),   // Front Right
    Speaker3D::new(2, 0.0, 0.0),    // Center
    Speaker3D::new(3, -110.0, 0.0), // Surround Left
    Speaker3D::new(4, 110.0, 0.0),  // Surround Right
    Speaker3D::new(5, 0.0, -30.0),  // LFE (below listener)
];

let vbap = Vbap3D::new(speakers);
```

### Height Channels (7.1.4)

```rust
use audio_ninja::vbap::create_7_1_4_layout;

// 12-channel layout with 4 height speakers
let speakers = create_7_1_4_layout();
let vbap = Vbap3D::new(speakers);

// Render a sound above and in front
let overhead = Vec3::from_spherical(0.0, 45.0, 1.0);
let gains = vbap.render(&overhead);
```

## Coordinate System

- **Azimuth**: 0° = front, 90° = left, -90° = right, ±180° = back
- **Elevation**: 0° = horizontal plane, 90° = directly above, -90° = directly below
- **Distance**: Normalized to unit sphere (radius = 1.0)

## Standard Layouts

### Stereo (2.0)
- Front Left: -30°
- Front Right: +30°

### 5.1 Surround
- Front Left: -30°
- Front Right: +30°
- Center: 0°
- Surround Left: -110°
- Surround Right: +110°
- LFE: 0° (below)

### 7.1.4 (Dolby Atmos Home)
- 8 ear-level speakers (FL, FR, C, SL, SR, BL, BR, plus LFE)
- 4 height speakers at +30° elevation:
  - Top Front Left: -30°, +30°
  - Top Front Right: +30°, +30°
  - Top Back Left: -135°, +30°
  - Top Back Right: +135°, +30°

## Algorithm

VBAP renders audio by:

1. **Speaker Triplet Finding**: Identifies valid triangles formed by 3 speakers
2. **Containment Test**: Finds which triplet contains the source position
3. **Gain Calculation**: Solves linear equation using pre-computed inverse matrices
4. **Energy Normalization**: Normalizes gains to preserve audio power

Valid triplets must:
- Form non-degenerate triangles (determinant ≠ 0)
- Have sufficient area (cross product threshold)
- Span 3D space (not coplanar in degenerate cases)

## Performance

- O(n³) triplet finding at initialization (cached)
- O(k) rendering per source, where k = number of triplets
- Typical layouts:
  - Stereo: 1 triplet
  - 5.1: ~20 triplets
  - 7.1.4: ~220 triplets

## Testing

The module includes 26 tests covering:
- Vector math operations (normalize, dot, cross products)
- Spherical/Cartesian conversion
- Speaker triplet creation and validation
- Gain calculation accuracy
- Layout presets
- Edge cases (degenerate triangles, sources outside all triplets)

Run tests:
```bash
cargo test vbap
```

## References

- Pulkki, V. (1997). "Virtual Sound Source Positioning Using Vector Base Amplitude Panning"
- ITU-R BS.2051: Advanced sound system for programme production
- Dolby Atmos Home Theater Installation Guidelines

## Integration with IAMF

The VBAP renderer integrates with the IAMF pipeline for object-based audio:

1. IAMF objects provide azimuth/elevation metadata
2. VBAP converts to 3D vectors and calculates gains
3. Gains applied to decoded audio channels
4. Mixed into final speaker feeds

See `src/mapping.rs` for integration examples.
