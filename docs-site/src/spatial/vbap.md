# VBAP (Vector-Based Amplitude Panning)

VBAP is a spatial audio panning technique that positions sound sources in 3D space using vector-based amplitude panning.

## Features

- **3D Vector Math**: Cartesian and spherical coordinate conversion
- **Arbitrary Speaker Layouts**: Support for any speaker configuration
- **Elevation Support**: Full 3D positioning with azimuth and elevation
- **Energy Preservation**: Normalized gains maintain audio energy
- **Automatic Triplet Finding**: Discovers valid speaker triangles for panning

## Usage

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

## Standard Layouts

- **2.0**: Stereo
- **5.1**: 5.1 surround
- **7.1**: 7.1 surround with side channels
- **7.1.4**: 7.1 surround with height speakers
- **9.1.6**: Full immersive with multiple heights

## When to Use

✅ Object-based audio streams
✅ Precise sound localization needed
✅ Arbitrary speaker layouts
✅ Low CPU cost important
✅ Real-time object positioning

## See Also

- [HOA (Ambisonics)](/spatial/hoa.md)
- [HRTF (Binaural)](/spatial/hrtf.md)
- [Algorithm Comparison](/spatial/comparison.md)

---

📖 **[Full VBAP Documentation](../../docs/vbap.md)**
