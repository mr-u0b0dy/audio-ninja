# Loudness and ITU-R BS.1770

Professional loudness management for audio normalization and mastering.

## What is Loudness?

Loudness is measured in **LUFS** (Loudness Units relative to Full Scale), using K-weighting to match human hearing.

## Target Loudness Levels

| Standard | Target | Use Case |
|----------|--------|----------|
| Television | -23 LUFS | Broadcast TV, streaming video |
| Streaming Music | -14 LUFS | Spotify, Apple Music |
| Film Theatrical | -27 LUFS | Cinema distribution |
| Film Home | -20 LUFS | Blu-ray, streaming film |

## Loudness Measurement

```bash
# Measure loudness of audio file
audio-ninja loudness /path/to/audio.wav

# Output:
# Integrated Loudness: -18.5 LUFS
# Short-Term Loudness: -16.2 LUFS
# Loudness Range: 8.5 LU
```

## Loudness Normalization

Automatically adjust audio to target loudness:

```bash
# Normalize to streaming music loudness
audio-ninja normalize /path/to/audio.wav --target-loudness -14.0

# Apply and save
audio-ninja normalize /path/to/audio.wav --target-loudness -14.0 --output normalized.wav
```

## See Also

- [DRC (Dynamic Range Control)](/processing/drc.md)
- [Calibration Guide](/processing/calibration.md)
- [Configuration Guide](/guide/configuration.md)

---

📖 **[Full Loudness Documentation](../../docs/loudness_drc.md)**
