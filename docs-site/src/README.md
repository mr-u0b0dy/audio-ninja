---
home: true
icon: home
title: Audio Ninja
heroImage: /logo.png
heroAlt: Audio Ninja Logo
heroText: Audio Ninja
tagline: Wireless Immersive Audio Platform with IAMF Support
actions:
  - text: Get Started
    icon: lightbulb
    link: /guide/
    type: primary
  - text: Spatial Audio Guide
    icon: headphones
    link: /spatial/
  - text: GitHub
    icon: github
    link: https://github.com/mr-u0b0dy/audio-ninja

features:
  - title: IAMF Rendering
    icon: 🎵
    details: Complete IAMF format support with object/channel/scene-based audio parsing and rendering
    
  - title: 3D Spatial Audio
    icon: 🎧
    details: VBAP, HOA (Ambisonics), and HRTF binaural rendering for immersive audio experiences
    
  - title: Network Transport
    icon: 🌐
    details: UDP/RTP with PTP/NTP sync, mDNS discovery, and Forward Error Correction (FEC)
    
  - title: Room Calibration
    icon: 📊
    details: Automatic measurement, impulse response analysis, and EQ filter generation
    
  - title: Flexible Layouts
    icon: 🔊
    details: Support for arbitrary speaker layouts from 2.0 stereo through 9.1.6+ configurations
    
  - title: REST API
    icon: 🔌
    details: Complete REST API on port 8080 for daemon control and monitoring

highlights:
  - title: Key Capabilities
    details: |
      - **Daemon-first architecture** with thin client design
      - **250+ tests** with comprehensive test coverage
      - **Performance benchmarks** for VBAP, HOA, and HRTF
      - **Cross-platform CI** with automated releases
      - **BLE GATT profiles** for speaker control and pairing
      - **DSP Pipeline** with loudness (ITU-R BS.1770) and dynamic range control

  - title: Production Ready
    details: |
      - Optimized binaries (2-6 MB)
      - Automated deployment with systemd
      - Real-time audio sync across speakers
      - Zero-downtime firmware updates
      - Comprehensive error handling and logging
      - Full API documentation with examples

  - title: Active Development
    details: |
      - Regular security and performance updates
      - Community-driven feature requests
      - Open source (Apache 2.0 licensed)
      - Extensive documentation and examples
      - Responsive issue tracking and support
      - Roadmap transparency and planning

---

## Quick Links

<div align="center">

**[Documentation](/guide/)** • **[Installation](/guide/installation.md)** • **[API Reference](/deployment/api.md)** • **[GitHub](https://github.com/mr-u0b0dy/audio-ninja)**

</div>

## Latest Features

### Version 0.1.0 Release
- ✅ Core IAMF parser and renderer
- ✅ VBAP 3D spatial rendering
- ✅ HOA (Higher-Order Ambisonics) decoder
- ✅ HRTF binaural processing
- ✅ REST API daemon service
- ✅ CLI tool for command-line control
- ✅ Desktop GUI (Tauri)
- ✅ Room calibration system
- ✅ Network transport with sync
- ✅ BLE control interface

## Documentation Structure

- **[Getting Started](/guide/)** - Installation, quick start, and configuration
- **[Spatial Audio](/spatial/)** - VBAP, HOA, and HRTF rendering guides
- **[Audio Processing](/processing/)** - Loudness, DRC, calibration, codecs
- **[Deployment](/deployment/)** - Daemon, CLI, API, firmware updates, releases

## Try It Out

```bash
# Clone the repository
git clone https://github.com/mr-u0b0dy/audio-ninja.git
cd audio-ninja

# Build in release mode
cargo build --workspace --release

# Start the daemon
cargo run -p audio-ninja-daemon --release

# Query status via CLI
cargo run -p audio-ninja-cli --release -- status
```

See [Installation Guide](/guide/installation.md) for detailed setup instructions.
