# Audio Codec Integration

Supported audio formats and codecs in Audio Ninja.

## Supported Codecs

| Codec | Container | Status | Quality |
|-------|-----------|--------|---------|
| PCM | WAV, RAW | ✅ Stable | Lossless |
| Opus | Ogg, WebM | ✅ Stable | Variable |
| AAC | MP4, M4A | 🔄 Planned | Lossy |
| FLAC | FLAC | 🔄 Planned | Lossless |
| AC-3 | MPEG-TS | 🔄 Deferred | Lossy |

## Usage

### Play WAV File (PCM)

```bash
audio-ninja play /path/to/audio.wav
```

### Play Opus Stream

```bash
audio-ninja play https://example.com/stream.opus
```

### Convert Format

```bash
# FFmpeg integration (planned)
ffmpeg -i input.mp4 -c:a opus output.opus
```

## See Also

- [Loudness Normalization](/processing/loudness.md)
- [Calibration Guide](/processing/calibration.md)
- [Configuration Guide](/guide/configuration.md)

---


