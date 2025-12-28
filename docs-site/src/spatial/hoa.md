# HOA (Higher-Order Ambisonics)

HOA represents sound fields using spherical harmonics, enabling flexible reproduction on arbitrary speaker layouts.

## Features

- **Multiple Orders**: 1st (B-format), 2nd, and 3rd order
- **Flexible Layouts**: Support for any speaker configuration
- **Standard Presets**: Stereo, 5.1, 7.1.4, and Cube layouts
- **Buffer Processing**: Efficient multi-sample decoding

## Usage

```rust
use audio_ninja::hoa::{HoaDecoder, AmbisonicOrder, DecodingMode};

// Create a 1st order decoder for 5.1 layout
let decoder = HoaDecoder::new(
    AmbisonicOrder::FIRST,
    DecodingMode::MaxRE,
    create_5_1_hoa_layout()
);

// Decode ambisonic channels to speaker feeds
let ambisonic_input = vec![1.0, 0.5, 0.0, 0.3];
let speaker_gains = decoder.decode(&ambisonic_input);
```

## Comparison with VBAP

| Aspect | HOA | VBAP |
|--------|-----|------|
| **Representation** | Spherical harmonics | Vector panning |
| **Best For** | Ambient soundfields | Localized objects |
| **Speaker Count** | More is better | Works with any layout |
| **CPU Cost** | Medium | Low |
| **IAMF Type** | Scene-based | Object-based |

## When to Use

✅ Scene-based audio streams
✅ Ambient soundfield rendering
✅ 5+ speakers available
✅ Natural spatial sound needed
✅ Medium CPU available

## See Also

- [VBAP (3D Panning)](/spatial/vbap.md)
- [HRTF (Binaural)](/spatial/hrtf.md)
- [Algorithm Comparison](/spatial/comparison.md)

---


