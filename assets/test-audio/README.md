# Spatial Audio Test Files

This directory contains reference spatial audio files for testing Audio Ninja's rendering and playback capabilities.

## File Format Guide

### Recommended Test Files

#### 1. **IAMF Files** (Ideal for spatial audio testing)
- **Source**: Alliance for Open Media (AOM) Reference Implementation
- **Format**: `.mp4` or `.iamf` (Immersive Audio Mux Format)
- **Recommended**: AOM test vectors from official repository
- **Spatial Layout**: Full surround (5.1, 7.1, 7.1.4, 9.1.6) with HOA channels
- **Codecs**: Opus, AAC-LC
- **Features**: Object-based audio, mix presentations, loudness metadata

**Download Source**:
```bash
# Clone AOM reference samples (requires git-lfs)
git clone https://github.com/AOMediaCodec/av1-testdata.git
# Look for IAMF samples in the repository
```

#### 2. **Multi-Channel WAV Files** (Format validation)
- **Format**: `.wav` (PCM)
- **Channels**: 5.1 (6 channels) or 7.1 (8 channels)
- **Sample Rate**: 48 kHz (recommended) or 44.1 kHz
- **Bit Depth**: 24-bit or 32-bit floating point
- **Use Case**: Testing VBAP downmix/upmix, HOA decoding, speaker layout mapping

**How to Create**:
```bash
# Using FFmpeg to generate multi-channel test tone
ffmpeg -f lavfi -i sine=frequency=1000:duration=10 \
  -c:a pcm_s24le -f wav -ac 6 -ar 48000 test_5.1.wav
```

#### 3. **Ambisonics Files** (HOA testing)
- **Format**: `.wav` or `.amb` (ACN format)
- **Ambisonics Order**: 1st order (4 channels: WXYC) or higher
- **Channel Layout**: ACN/Furse-Malham ordering
- **Use Case**: Testing HOA decoder with various speaker layouts (2D, 3D, custom)

**What We Test**:
- HOA decoding to arbitrary speaker arrays
- Mix presentation selection and loudness metadata
- Binaural HRTF rendering for headphone playback
- Multi-speaker synchronization and latency compensation

## Phase 1: MVP Test Files (January 2026)

For initial release, we'll focus on:

1. **Synthetic test content** (generated locally):
   - Stereo sine sweep (440 Hz, 1 kHz, 10 kHz)
   - 5.1 channel test signal (each speaker plays distinct frequency)
   - 1st order HOA (WXYZ) test content

2. **Reference downloads** (optional):
   - Single IAMF sample from AOM (requires user download)
   - Multi-channel WAV example generated with FFmpeg

## Phase 2: Comprehensive Test Suite (Future)

- Full IAMF test vector suite from AOM
- Dolby Atmos sample with object metadata
- Real-world spatial audio recordings (music, podcasts, immersive content)
- Performance benchmarks with large files (30+ minutes)

## Using Test Files with Audio Ninja

### Load a file for playback:
```bash
# Via REST API
curl -X POST http://localhost:8080/api/v1/transport/load-file \
  -H "Content-Type: application/json" \
  -d '{"file_path": "/path/to/test-audio.wav"}'

# Via CLI
audio-ninja transport load-file /path/to/test-audio.wav
```

### Validate spatial audio:
```bash
# Select output device (speaker or headphones)
audio-ninja output list
audio-ninja output select headphones

# Select input source (if streaming)
audio-ninja input list
audio-ninja input select system

# Start playback
audio-ninja transport play
```

### Monitor rendering pipeline:
```bash
# Check playback status
audio-ninja transport status

# Monitor output device status
audio-ninja output status

# Check input source status
audio-ninja input status
```

## Known Limitations & TODOs

- ❌ **IAMF Parser**: Currently stubbed; requires libiamf integration from AOM
- ❌ **FFmpeg Integration**: Codec binding stubs; need full FFmpeg bindings for MP4/IAMF demux
- ⚠️ **File Format Support**: WAV PCM only (initial); AAC/Opus require FFmpeg
- ⚠️ **Real Audio Output**: Placeholder; requires ALSA/PulseAudio backend implementation
- ⚠️ **App-Level Routing**: Phase 2; currently loopback device only

## References

- **AOM IAMF Specification**: https://github.com/AOMediaCodec/iamf
- **ITU-R BS.2051-3** (Speaker layouts): https://www.itu.int/rec/R-REC-BS.2051-3-202112-I/en
- **ITU-R BS.1770-4** (Loudness measurement): https://www.itu.int/rec/R-REC-BS.1770-4-201510-I/en
- **AOM Test Media**: https://github.com/AOMediaCodec/av1-testdata (requires git-lfs)
- **FFmpeg Documentation**: https://ffmpeg.org/documentation.html

## Contribute Test Files

If you have spatial audio test content (properly licensed), please:
1. Create a GitHub issue with file details (format, layout, codec, license)
2. Provide download link or attach compressed sample
3. Ensure proper licensing (CC-BY, public domain, or permission)

---

**Last Updated**: January 2026  
**Status**: MVP Phase (IAMF parser pending, placeholder implementations)
