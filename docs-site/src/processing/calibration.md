# Room Calibration

Automatic acoustic measurement and optimization for your room.

## Overview

Audio Ninja's calibration system measures and corrects:

- **Time Alignment**: Synchronize speaker arrival times
- **Level Matching**: Consistent playback level
- **Frequency Response**: EQ to flatten room response
- **Phase Alignment**: Optimize crossover performance

## Quick Calibration

```bash
# Start calibration wizard
audio-ninja calibration --interactive

# Steps:
# 1. Position measurement microphone
# 2. Play test signals
# 3. Analyze room response
# 4. Generate filters
# 5. Apply and verify
```

## Manual Calibration

```bash
# Generate sweep signal
audio-ninja calibration sweep --duration 10s --output sweep.wav

# Record response
arecord -f S16_LE -r 48000 response.wav < /dev/hw:1

# Analyze
audio-ninja calibration analyze response.wav

# Generate filters
audio-ninja calibration generate-filters response.wav --output config.json
```

## See Also

- [Loudness Normalization](/processing/loudness.md)
- [DRC](/processing/drc.md)
- [Configuration Guide](/guide/configuration.md)

---

📖 **[Full Calibration Guide](../../docs/calibration.md)**
