# Dynamic Range Control (DRC)

Manage audio dynamics and prevent clipping with DRC.

## What is DRC?

Dynamic Range Control compresses loud peaks and expands quiet passages to control audio dynamics.

## Presets

- **Speech**: Optimized for dialog and speech
- **Music**: Balanced for musical content
- **Cinema**: For film and theatrical content

## Apply DRC

```bash
# Apply music preset
audio-ninja drc /path/to/audio.wav --preset music

# Custom parameters
audio-ninja drc /path/to/audio.wav \
  --threshold -20 \
  --ratio 4 \
  --attack 10ms \
  --release 100ms
```

## See Also

- [Loudness Normalization](/processing/loudness.md)
- [Calibration Guide](/processing/calibration.md)
- [Configuration Guide](/guide/configuration.md)

---

📖 **[Full DRC Documentation](../../docs/loudness_drc.md)**
