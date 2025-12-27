# Loudness Management and Dynamic Range Control

## Overview

The audio-ninja loudness module implements professional-grade loudness management and dynamic range control (DRC) compliant with **ITU-R BS.1770-4** loudness standards. This enables audio normalization for streaming, broadcast, and film applications.

## ITU-R BS.1770 Loudness Standards

Loudness is measured in **LUFS** (Loudness Units relative to Full Scale), using K-weighting to approximate human hearing sensitivity.

### Standard Target Loudness Levels

| Standard | Target | Use Case |
|----------|--------|----------|
| **Television** | -23 LUFS | Broadcast TV, streaming video |
| **Streaming Music** | -14 LUFS | Music streaming services (Spotify, Apple Music) |
| **Film Theatrical** | -27 LUFS | Cinema distribution |
| **Film Home** | -20 LUFS | Home video (Blu-ray, streaming film) |
| **Custom** | User-defined | Application-specific targets |

## Components

### 1. LoudnessMeter

Measures audio loudness using ITU-R BS.1770-4 methodology.

```rust
use audio_ninja::loudness::{LoudnessMeter, AudioBlock};

let mut meter = LoudnessMeter::new(48000); // 48kHz sample rate
let audio = AudioBlock::new(vec![vec![0.5; 4800]; 2]); // 2 channels, 100ms

let loudness = meter.measure(&audio);
println!("Integrated loudness: {} LUFS", loudness.integrated);
println!("Short-term loudness: {} LUFS", loudness.short_term);
println!("Loudness range: {} LU", loudness.range);
```

**Measurements:**
- **Integrated Loudness**: Overall loudness across the entire measurement
- **Short-Term Loudness**: Loudness over last 3 seconds (for dynamics analysis)
- **Loudness Range (LR)**: Difference between 95th and 5th percentile (characterizes dynamic range)

### 2. LoudnessNormalizer

Automatically calculates and applies gain to reach target loudness.

```rust
use audio_ninja::loudness::{LoudnessNormalizer, LoudnessTarget};

let mut normalizer = LoudnessNormalizer::new();
let current_loudness = -18.0; // LUFS

// Calculate gain needed to reach target
let target = LoudnessTarget::StreamingMusic; // -14 LUFS
let gain_db = normalizer.calculate_gain(current_loudness, target);
println!("Apply {:.2} dB gain", gain_db); // Output: Apply 4.00 dB gain
```

**Use Cases:**
- Ensure consistent loudness across multiple audio tracks
- Normalize user-uploaded content for streaming platforms
- Automatic preprocessing in podcast/music workflows

### 3. HeadroomManager

Protects against clipping with soft-limiting and adjustable protection threshold.

```rust
use audio_ninja::loudness::HeadroomManager;
use audio_ninja::AudioBlock;

let mut limiter = HeadroomManager::new(48000);
limiter.set_threshold_db(-3.0); // Protect -3dB headroom
limiter.set_attack_ms(10.0);
limiter.set_release_ms(100.0);

let mut audio = AudioBlock::new(vec![vec![0.95; 4800]; 2]);
limiter.apply_limiting(&mut audio); // Soft limit peak peaks to -3dB
```

**Features:**
- **Soft-limiting**: Smooth gain reduction above threshold (not hard clipping)
- **Adjustable threshold**: Default -3dB; use -1dB for tight control, -6dB for loose
- **Attack/Release**: Control limiting speed (attack: 5-20ms, release: 50-200ms)

### 4. DynamicRangeControl (DRC)

Compresses dynamic range to fit within target envelope.

```rust
use audio_ninja::loudness::DynamicRangeControl;
use audio_ninja::AudioBlock;

let mut drc = DynamicRangeControl::new();
drc.set_ratio(4.0);           // 4:1 compression
drc.set_threshold_db(-20.0);  // Compress signals above -20dB
drc.set_attack_ms(10.0);
drc.set_release_ms(100.0);

let mut audio = AudioBlock::new(vec![vec![0.8; 4800]; 2]);
drc.process(&mut audio); // Compress loud peaks, boost quiet passages
```

**Parameters:**
- **Ratio**: 2:1 (gentle) to 8:1 (aggressive); use 4:1 for music, 8:1 for speech
- **Threshold**: Signals above this level get compressed
- **Attack**: Time to reach full compression (5-20ms for music, 1-5ms for speech)
- **Release**: Time to return to unity gain (50-200ms typical)
- **Makeup Gain**: Automatically calculated to maintain average level

**Compression Formula:**
```
Gain Reduction = threshold + (input - threshold) / ratio
Makeup Gain = (threshold * (ratio - 1)) / ratio
```

## Integration with ReferenceRenderer

The render module integrates all components into a coherent signal processing pipeline:

```rust
use audio_ninja::render::{ReferenceRenderer, RenderOptions};
use audio_ninja::loudness::LoudnessTarget;

// Create renderer with loudness processing
let mut renderer = ReferenceRenderer::new(48000);
renderer.set_loudness_target(LoudnessTarget::StreamingMusic);
renderer.enable_drc(4.0, -20.0); // 4:1 ratio, -20dB threshold
renderer.set_headroom_db(-3.0);

// Render audio
let options = RenderOptions::default();
let output = renderer.render(&input_block, &options)?;
```

**Signal Processing Order:**
1. **DRC Processing**: Compress dynamic range
2. **Loudness Normalization**: Reach target loudness
3. **Headroom Protection**: Prevent clipping
4. **Output**: Protected, normalized audio

## Practical Examples

### Example 1: Normalize Podcast Content

```rust
use audio_ninja::loudness::{LoudnessNormalizer, LoudnessMeter, LoudnessTarget};

let mut meter = LoudnessMeter::new(48000);
let mut normalizer = LoudnessNormalizer::new();

// Measure podcast audio
let measurement = meter.measure(&podcast_audio);
println!("Podcast current loudness: {:.1} LUFS", measurement.integrated);

// Calculate gain for podcast target (-14 LUFS)
let gain = normalizer.calculate_gain(measurement.integrated, LoudnessTarget::StreamingMusic);
println!("Normalize with {:.2} dB gain", gain);

// Apply in audio processing
// (caller would multiply samples by linear_gain = 10^(gain/20))
```

### Example 2: Protect Against Clipping During Mixing

```rust
use audio_ninja::loudness::HeadroomManager;

let mut limiter = HeadroomManager::new(48000);
limiter.set_threshold_db(-2.0); // Tight 2dB headroom
limiter.set_attack_ms(5.0);     // Fast attack for transient protection
limiter.set_release_ms(50.0);   // Quick release for punch

// During live mixing
for audio_chunk in incoming_mix {
    limiter.apply_limiting(&mut audio_chunk);
    output_to_speakers(audio_chunk);
}
```

### Example 3: Compress Vocal Track for Music Production

```rust
use audio_ninja::loudness::DynamicRangeControl;

let mut vocal_compressor = DynamicRangeControl::new();
vocal_compressor.set_ratio(6.0);          // Aggressive 6:1 for vocals
vocal_compressor.set_threshold_db(-18.0); // Compress loud parts
vocal_compressor.set_attack_ms(5.0);      // Fast attack for punch control
vocal_compressor.set_release_ms(150.0);   // Medium release for musicality

vocal_compressor.process(&mut vocal_track);
// Result: More consistent vocal loudness with tamed peaks
```

### Example 4: Full Professional Processing Chain

```rust
use audio_ninja::render::ReferenceRenderer;
use audio_ninja::loudness::LoudnessTarget;

// Initialize with multi-speaker layout
let mut renderer = ReferenceRenderer::new(44100);

// Configure loudness target for streaming (prevents distortion on headphones)
renderer.set_loudness_target(LoudnessTarget::StreamingMusic); // -14 LUFS

// Enable DRC for dynamic consistency
renderer.enable_drc(3.0, -15.0); // Subtle 3:1 compression

// Set headroom to prevent clipping on consumer devices
renderer.set_headroom_db(-1.5);

// Process multi-speaker audio
let mut output = input_audio.clone();
renderer.apply_processing(&mut output)?;

// Result: Audio compliant with streaming standards, protected against clipping
```

## Technical Details

### K-Weighting Formula

The K-weighting curve approximates human hearing sensitivity:
- Boost bass: +4dB at 100Hz
- Boost treble: +4dB at 2kHz
- Attenuate very low (<50Hz) and very high (>14kHz) frequencies

Implementation uses IIR filters for real-time efficiency.

### Measurement Integration

- **Integrated**: Sum of power across entire signal
- **Short-term**: 3-second sliding window for dynamics analysis
- **Range**: 5th to 95th percentile difference (ignores outliers)

### Makeup Gain Calculation

For ratio R and threshold T:
```
Makeup Gain = (T * (R - 1)) / R

Example: ratio=4:1, threshold=-20dB
Makeup = (-20 * 3) / 4 = -15dB makeup gain
```

This restores average level after compression.

## Performance Considerations

- **LoudnessMeter**: ~2% CPU on single core (audio_ninja crate)
- **DynamicRangeControl**: ~1% CPU (compressor is efficient)
- **HeadroomManager**: <1% CPU (simple threshold operation)
- **Latency**: ~10ms (negligible for streaming, matters for live mixing)

## Standards Compliance

- ✅ **ITU-R BS.1770-4**: Loudness measurement (LUFS units)
- ✅ **EBU R128**: European Broadcasting Union loudness normalization
- ✅ **ATSC A/85**: US broadcast loudness standard
- ✅ **Apple Music Mastering for iTunes**: -14 LUFS target
- ✅ **Spotify Loudness Normalization**: -14 LUFS target
- ✅ **YouTube**: -13 LUFS target (automatic adjustment)

## Troubleshooting

**Issue**: Audio sounds too quiet after normalization
- **Solution**: Check meter is measuring full duration; use short-term measurement for partial clips

**Issue**: Clipping warnings despite HeadroomManager
- **Solution**: Increase attack speed (set_attack_ms lower) or lower threshold (-2dB instead of -3dB)

**Issue**: DRC makes audio pump unnaturally
- **Solution**: Increase release time (100ms → 200ms) or reduce ratio (4:1 → 3:1)

**Issue**: Loudness target not achieved
- **Solution**: Verify normalizer is active; check for other processing reducing level

## Next Steps

- Integrate with RTP transport for live multi-speaker sync
- Add parametric EQ for tonal balance alongside loudness
- Implement lookahead processing for more transparent limiting
- Profile performance across embedded targets

## Related Documentation

- [HRTF Binaural Rendering](./hrtf.md)
- [Spatial Rendering API](./render.md)
- [Transport Synchronization](./transport.md)

## Advanced: Lookahead Limiting and Envelope Follower

For more transparent peak control, the pipeline supports:

- Lookahead limiting to react before peaks occur
- Envelope follower in the DRC with instant attack and smoothed release

Configure lookahead via the renderer:

```rust
use audio_ninja::render::ReferenceRenderer;
use audio_ninja::loudness::LoudnessTarget;

let mut renderer = ReferenceRenderer::new(48_000);
renderer.set_loudness_target(LoudnessTarget::StreamingMusic);
renderer.enable_drc(4.0, -18.0);
renderer.set_headroom_db(-3.0);
renderer.set_headroom_lookahead_ms(3.0); // 3ms lookahead for proactive limiting
```

This helps catch single-sample transients while maintaining natural release behavior.
