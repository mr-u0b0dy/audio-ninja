# VBAP vs HOA vs HRTF Comparison

Comprehensive comparison of Audio Ninja's three spatial rendering algorithms.

## Feature Comparison

| Feature | VBAP | HOA | HRTF |
|---------|------|-----|------|
| **Best For** | Localized objects | Ambient soundfields | Headphone listening |
| **Representation** | Vector panning | Spherical harmonics | Head transfer function |
| **Source Type** | Object-based | Scene-based | Binaural stereo |
| **Speaker Count** | 3+ (any config) | 5+ (recommended) | 2 (stereo) |
| **Localization** | Sharp, precise | Smooth, natural | 3D via headphones |
| **CPU Cost** | Low | Medium | Medium |
| **Latency** | Very low | Low | Low |
| **Elevation Support** | Yes | Yes | Yes |
| **Arbitrary Layouts** | Yes | Yes | N/A |

## When to Use Each

### Use VBAP When

✅ Working with object-based audio (IAMF objects)
✅ Need sharp, precise localization
✅ Have arbitrary or unusual speaker layouts
✅ Want minimal CPU overhead
✅ Need real-time object positioning
✅ Using stereo pairs for panning

**Examples**:
- Dialog panning (character moving left to right)
- Sound effects with precise positioning
- Interactive audio (game audio)
- Spatial effects rendering

### Use HOA When

✅ Working with scene-based audio (IAMF scenes)
✅ Need smooth, natural soundfields
✅ Have 5+ speakers in a regular layout
✅ Can afford medium CPU cost
✅ Want to preserve acoustic ambience
✅ Using Ambisonic format streams

**Examples**:
- Environmental ambience (wind, rain, crowd)
- Spacious reverbs and room tone
- Concert hall recordings
- VR/XR immersive environments

### Use HRTF When

✅ Rendering for headphone playback
✅ Want full 3D immersive audio
✅ Don't have speaker systems
✅ Supporting mobile/personal devices
✅ Need realistic spatial cues
✅ Converting speaker-based to binaural

**Examples**:
- Headphone-only streaming
- Mobile audio apps
- Personal audio systems
- Binaural mixing/monitoring

## Mixed Mode

Audio Ninja supports using multiple algorithms simultaneously:

```
IAMF Stream
├─ Object-based elements → VBAP → Speaker output
├─ Scene-based elements → HOA → Speaker output
└─ Binaural render → HRTF → Headphone output
```

## Algorithm Details

### VBAP Strengths
- Excellent localization
- Works with any speaker count/layout
- Very efficient (low CPU)
- Direct control over object position

### VBAP Weaknesses
- Limited to discrete sources
- Requires speaker triplets
- May have holes in coverage

### HOA Strengths
- Natural sound fields
- Captures complete acoustic space
- Good with many speakers
- Smooth, continuous rendering

### HOA Weaknesses
- Requires sufficient speaker count
- Higher CPU usage
- Best with regular layouts
- Slower than VBAP

### HRTF Strengths
- True 3D on any stereo headphone
- No hardware dependencies
- Excellent for mobile
- Realistic spatial cues

### HRTF Weaknesses
- Limited to stereo output
- Headphone-dependent coloration
- Requires proper profile
- Can cause fatigue with poor profiles

## See Also

- [VBAP Deep Dive](/spatial/vbap.md)
- [HOA Deep Dive](/spatial/hoa.md)
- [HRTF Deep Dive](/spatial/hrtf.md)
- [Configuration Guide](/guide/configuration.md)
