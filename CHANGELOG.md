# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **GUI Phase 2**: 7-tab desktop interface (Status, Config, I/O, Transport, Layout, Calibration, Stats)
  - Magma/dark orange theme with CSS custom properties and smooth transitions
  - Audio level meters (input L/R, output L/R) with dBFS display
  - Canvas-based speaker layout visualization with drag-to-reposition
  - Latency history, bandwidth, and sync error canvas graphs
  - Speaker stats table with per-speaker metrics
  - Calibration panel with sweep settings, IR visualization, and filter design
  - Transport controls: file loading, play/pause/stop, seek slider, mode selector (File/Stream/Mixed)
  - I/O device management: input/output device dropdowns with real-time status
  - Toast notification system for user feedback
  - Keyboard shortcuts (1-7 for tabs, Space play/pause, Ctrl+O file picker)
  - Drag-and-drop file loading on transport panel
  - Responsive CSS with breakpoints at 640px, 768px, 1024px
  - Daemon connection status indicator with exponential backoff retry
  - localStorage persistence for user preferences (tab, devices, mode, layout)
  - ARIA attributes and focus-visible styles for accessibility
- **Daemon**: `POST /api/v1/transport/seek` endpoint for sample-accurate seeking
- **Daemon**: Real CPU/memory monitoring from `/proc/self` on Linux
- **Daemon**: Per-speaker sync status classification (locked/drift/unlocked)
- **Logo**: Professional Audio Ninja logo integrated in GUI header

### Changed
- Stats tab element IDs normalized to camelCase (`statCpu`, `statMemory`, etc.)
- Logo path uses `public/` directory for correct Tauri production builds

### Fixed
- Stats dashboard not updating: HTML element IDs mismatched JS queries (e.g. `statsCPU` vs `statCpu`)
- Logo not loading in production Tauri builds due to `../icons/` path escaping distDir
- Release workflow: removed orphaned `upload_url` output referencing nonexistent step
- Release workflow: checksum job now uses `v`-prefixed tag for `gh release download`

## [0.1.0] - 2025-12-28

### Added

#### Core Audio Features
- IAMF (Immersive Audio Model and Formats) parsing and rendering
- 3D VBAP (Vector-Based Amplitude Panning) renderer with elevation support
  - Support for arbitrary speaker layouts
  - Standard presets: 2.0, 5.1, 7.1.4
  - Energy-preserving panning algorithm
- HOA (Higher-Order Ambisonics) decoder (1st/2nd/3rd order)
  - Basic, Max-rE, and In-Phase decoding modes
  - Standard layouts: stereo, 5.1, 7.1.4, cube
- HRTF binaural rendering for headphone playback
  - 4 headphone profiles: Flat, ClosedBack, OpenBack, IEM
  - KEMAR HRTF database integration
  - Spatial positioning: azimuth, elevation, distance controls
  - Automatic multi-channel to stereo downmix
- Loudness measurement and normalization (ITU-R BS.1770-4)
  - Integrated, short-term, and momentary loudness (LUFS)
  - Loudness range (LRA) measurement
  - Normalization to streaming targets
- Dynamic Range Control (DRC)
  - Speech, Music, and Cinema presets
  - Configurable threshold, ratio, attack, and release
- Headroom management with soft limiting (configurable dB, 3ms lookahead)

#### Network & Transport
- UDP/RTP audio streaming with timestamp synchronization
- mDNS service discovery for automatic speaker detection
- Forward Error Correction (FEC) with XOR encoding
- Packet loss concealment (silence, repeat, interpolate)
- Jitter buffer with adaptive latency compensation
- Clock synchronization (PTP, NTP, System)
- Multi-speaker broadcast support

#### Control & Configuration
- BLE GATT profiles for wireless speaker control
  - Speaker identity and role assignment
  - Layout configuration (azimuth, elevation, distance)
  - Calibration settings (volume trim, delay, EQ enable)
  - Pairing and connection management
- REST API daemon service (Axum, port 8080)
  - Speaker management endpoints
  - Layout configuration
  - Transport control (play/pause/stop)
  - Calibration runner
  - Real-time statistics
- Desktop GUI client (Tauri + vanilla JS)
- Command-line interface (audio-ninja-cli) with full daemon control

#### Room Calibration
- Measurement sweep generation (log sweep, MLS)
- Impulse response analysis
  - Automatic delay detection
  - Magnitude response extraction
- Filter design
  - FIR filters (linear-phase, windowed sinc)
  - IIR biquad cascades (PEQ, shelf, high/low-pass)
- DSP configuration export
  - CamillaDSP format
  - BruteFIR format

#### Infrastructure
- Cargo workspace with 4 crates (core, daemon, cli, gui)
- Comprehensive test suite (250+ tests)
  - Unit tests for all modules
  - Integration tests for transport and network
  - End-to-end CLI ↔ daemon tests
- GitHub Actions CI/CD
  - Format checking (rustfmt)
  - Linting (clippy with -D warnings)
  - Build and test automation
  - Benchmark compilation
  - Documentation building
- GitHub Actions release workflow
  - Automated binary builds for x86_64 and aarch64 Linux
  - Tarball generation with SHA256 checksums
- Performance benchmarks (Criterion)
  - VBAP rendering
  - Loudness measurement
  - Vec3 operations
- OpenAPI/Swagger REST API documentation
- Systemd service file for Linux deployment
- Apache 2.0 license with SPDX identifiers

#### Documentation
- Architecture diagrams and workflow descriptions
- API documentation with code examples
- Module-specific guides:
  - Binaural rendering (`docs/binaural_rendering.md`)
  - Loudness and DRC (`docs/loudness_drc.md`)
  - HRTF usage (`docs/hrtf.md`)
  - VBAP spatial panning (`docs/vbap.md`)
  - HOA decoding (`docs/hoa.md`)
- Contributing guidelines
- Release process documentation (`docs/RELEASE.md`)
- Build optimization notes in copilot instructions

### Known Issues
- FFmpeg codec integration uses stubs (Opus, AAC, FLAC decoding planned)
- IAMF decoder awaits libiamf/AOM reference library integration
- GUI feature set incomplete
- No Windows or macOS support yet
- Some unused variables and dead code warnings in core library

[Unreleased]: https://github.com/mr-u0b0dy/audio-ninja/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mr-u0b0dy/audio-ninja/releases/tag/v0.1.0
