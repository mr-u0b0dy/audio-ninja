# Spatial Audio Overview

Audio Ninja provides three complementary spatial rendering algorithms for immersive audio experiences.

## Rendering Algorithms

### VBAP (Vector-Based Amplitude Panning)

**Best for**: Discrete point sources, localized sound objects

- 3D panning for arbitrary speaker layouts
- Sharp, precise localization
- Efficient computation
- Ideal for object-based IAMF streams

**Example**: A helicopter sound positioned at azimuth 45°, elevation 30°

[Learn more about VBAP →](/spatial/vbap.md)

### HOA (Higher-Order Ambisonics)

**Best for**: Ambient soundfields, diffuse sources

- Scene-based spatial audio
- Smooth, continuous sound fields
- Supports 1st, 2nd, and 3rd order
- Uses spherical harmonics representation

**Example**: Environmental ambience or crowd noise

[Learn more about HOA →](/spatial/hoa.md)

### HRTF (Head-Related Transfer Function)

**Best for**: Headphone listening, binaural audio

- 3D audio via stereo headphones
- Uses head-related transfer functions
- Multiple headphone profiles (Flat, Closed-back, Open-back, IEM)
- Simulates speaker radiation patterns

**Example**: Full 3D immersive audio on any headphones

[Learn more about HRTF →](/spatial/hrtf.md)

## Comparison Table

| Feature | VBAP | HOA | HRTF |
|---------|------|-----|------|
| **Use Case** | Point sources | Soundfields | Headphones |
| **Speaker Requirement** | Any layout | More = better | 2 (headphones) |
| **Localization** | Sharp | Smooth | Realistic |
| **Computational Cost** | Low | Medium | Medium |
| **IAMF Element** | Object-based | Scene-based | Rendering only |
| **Headphone Support** | No | No | Yes |

## Quick Decision Guide

**Choose VBAP if:**
- Working with object-based audio
- Need sharp, precise localization
- Have arbitrary speaker layouts
- Want minimal CPU usage

**Choose HOA if:**
- Working with scene-based audio
- Need smooth, natural soundfields
- Have multiple speakers (5+ recommended)
- Can afford medium CPU cost

**Choose HRTF if:**
- Rendering for headphones
- Want full 3D immersive audio on any stereo headset
- Can choose headphone profile
- Need to support mobile/personal listening

## Next Steps

- **[VBAP Deep Dive](/spatial/vbap.md)** - Learn 3D vector panning
- **[HOA Deep Dive](/spatial/hoa.md)** - Understand ambisonics
- **[HRTF Deep Dive](/spatial/hrtf.md)** - Binaural rendering
- **[Comparison Guide](/spatial/comparison.md)** - Feature-by-feature breakdown
