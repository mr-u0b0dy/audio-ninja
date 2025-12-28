# Reference Documentation

Complete technical reference for Audio Ninja.

## Core Components

### Spatial Audio Rendering
- [VBAP (Vector Base Amplitude Panning)](./vbap.md) - 3D spatial audio rendering with arbitrary speaker layouts
- [HOA (Higher-Order Ambisonics)](./hoa.md) - Scene-based spatial audio with multiple orders
- [HRTF (Head-Related Transfer Function)](./hrtf.md) - Binaural rendering for headphone playback

### Audio Processing
- [Loudness and DRC](./loudness_drc.md) - ITU-R BS.1770 loudness measurement, normalization, and dynamic range control
- [Calibration](./calibration.md) - Room calibration, impulse response measurement, filter design, and DSP export
- [Codec Integration](./codec_integration.md) - Codec support, FFmpeg integration, and error handling

### System Architecture
- [Daemon Workflow](./daemon_workflow.md) - REST API endpoints, service lifecycle, configuration, and client integration
- [API Usage](./api_usage.md) - Complete REST API reference with examples (cURL, Python, JavaScript)
- [Firmware Update](./firmware_update.md) - OTA update mechanism, rollback strategy, and deployment
- [Release](./release.md) - Version management, CI/CD pipeline, and publishing workflow

## Quick Navigation

**Getting Started?** → Start with [Installation](/guide/installation.md)

**Building with Audio Ninja?** → See [API Usage](./api_usage.md) and [Daemon Workflow](./daemon_workflow.md)

**Tuning your setup?** → Check [Calibration](./calibration.md) and [Loudness and DRC](./loudness_drc.md)

**Deploying to production?** → Review [Release](./release.md) and [Firmware Update](./firmware_update.md)
