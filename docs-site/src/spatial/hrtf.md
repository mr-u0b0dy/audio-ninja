# HRTF (Head-Related Transfer Function)

## Overview

HRTF provides binaural audio rendering, converting multi-channel audio into 3D spatial audio suitable for headphone playback.

## Features

- **Binaural Rendering**: Convert any audio to stereo headphones
- **4 Headphone Profiles**: Flat, ClosedBack, OpenBack, IEM
- **3D Positioning**: Full azimuth and elevation support
- **Standard Dataset**: KEMAR measurements

## Usage

### Basic Example

```rust
use audio_ninja::hrtf::{BinauralRenderer, HeadphoneProfile, HrtfDatabase};

let mut db = HrtfDatabase::new();
db.load_default_kemar().unwrap();

let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

// Render mono to stereo binaural
let mono_input = vec![0.5; 512];
let position = HrtfPosition::new(45.0, 0.0, 1.0);
let (left, right) = renderer.render(&mono_input, &position)?;
```

## Headphone Profiles

- **Flat**: Neutral response
- **ClosedBack**: Closed-back headphone coloration
- **OpenBack**: Open-back headphone coloration
- **IEM**: In-ear monitor characteristics

## When to Use HRTF

✅ Headphone listening
✅ Full 3D immersive audio needed
✅ No speaker system available
✅ Mobile/personal devices
✅ Natural spatial cues required

## See Also

- [VBAP (3D Panning)](/spatial/vbap.md)
- [HOA (Ambisonics)](/spatial/hoa.md)
- [Algorithm Comparison](/spatial/comparison.md)

---


