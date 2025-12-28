# Getting Started Guide

Welcome to Audio Ninja! This guide will help you get up and running with the wireless immersive audio platform.

## What is Audio Ninja?

Audio Ninja is an open-source wireless immersive audio platform with:

- **IAMF Support**: Complete parsing and rendering of Immersive Audio Format
- **3D Spatial Audio**: Multiple rendering algorithms (VBAP, HOA, HRTF)
- **Network Transport**: Wireless speaker delivery with sync and error correction
- **Room Calibration**: Automatic acoustic measurement and optimization
- **REST API**: Complete control interface for clients
- **Production Ready**: 250+ tests, benchmarks, and cross-platform CI

## Key Concepts

### Architecture

Audio Ninja uses a modular **daemon-first** architecture:

```
GUI/CLI Clients → REST API (port 8080) → Audio Ninja Daemon → Core Engine → Speakers
```

The core engine handles:
- IAMF parsing and rendering
- Spatial audio calculations (VBAP, HOA, HRTF)
- Network transport (RTP, FEC)
- Room calibration and DSP
- Speaker synchronization

### Spatial Rendering

Choose the best algorithm for your use case:

| Algorithm | Use Case | Strength |
|-----------|----------|----------|
| **VBAP** | Discrete point sources | Sharp localization |
| **HOA** | Ambient soundfields | Smooth, diffuse sounds |
| **HRTF** | Binaural headphones | Realistic 3D via headphones |

### Speaker Layouts

Audio Ninja supports flexible speaker layouts:

- **Stereo**: 2.0 (Left, Right)
- **Surround**: 5.1, 7.1, 7.1.4
- **Immersive**: 9.1.6+
- **Custom**: Any arbitrary layout

## Next Steps

1. **[Installation](/guide/installation.md)** - Set up Audio Ninja on your system
2. **[Quick Start](/guide/quick-start.md)** - Get the daemon running in 5 minutes
3. **[Configuration](/guide/configuration.md)** - Customize for your environment
4. **[Spatial Audio](/spatial/)** - Deep dive into rendering algorithms
