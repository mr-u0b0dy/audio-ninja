# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Binaural Rendering**: HRTF-based spatial audio virtualization for headphone playback
  - 4 headphone profiles: Flat, ClosedBack, OpenBack, IEM
  - Spatial positioning: azimuth, elevation, distance controls
  - Automatic multi-channel to stereo downmix
  - Integration with loudness/DRC pipeline
  - Documentation: `docs/binaural_rendering.md`; example: `examples/binaural_rendering.rs`
- Loudness management module (`src/loudness.rs`) implementing ITU-R BS.1770-4 loudness measurement (LUFS), normalization, headroom management, and DRC.
- Renderer integration: `ReferenceRenderer` supports loudness targets, DRC, headroom protection, and binaural rendering (`src/render.rs`).
- **DRC Presets**: `DRCPreset` enum with Speech, Music, and Cinema presets; `apply_drc_preset()` method for convenient preset selection.
- Documentation: `docs/loudness_drc.md` (guide) and example `examples/loudness_processing.rs`.
- Tests: Integration tests for renderer loudness/headroom (`tests/loudness_render_tests.rs`) and DRC peak reduction (`tests/drc_tests.rs`).
- CI: GitHub Actions workflow to run fmt, clippy, build, and tests (`.github/workflows/ci.yml`).
- IAMF (Immersive Audio Model and Formats) parsing and rendering
- 3D VBAP (Vector-Based Amplitude Panning) spatial renderer
  - Support for arbitrary speaker layouts with elevation
  - Standard presets: 2.0, 5.1, 7.1.4
  - Energy-preserving panning algorithm
- HOA (Higher-Order Ambisonics) decoder
  - 1st, 2nd, and 3rd order support
  - Basic, Max-rE, and In-Phase decoding modes
  - Standard layouts: stereo, 5.1, 7.1.4, cube
- Network transport layer
  - UDP/RTP streaming with timestamp-based sync
  - mDNS speaker discovery
  - Forward Error Correction (FEC) with XOR encoding
  - Packet loss concealment (silence/repeat/interpolate)
  - Jitter buffer with adaptive latency compensation
- Clock synchronization
  - PTP (Precision Time Protocol) support
  - NTP (Network Time Protocol) support
  - System clock fallback
- BLE GATT control plane
  - Speaker identity and role assignment
  - Layout configuration (azimuth, elevation, distance)
  - Volume trim and delay compensation
  - Pairing and capability negotiation
- Room calibration pipeline
  - Log sweep and MLS generation
  - Impulse response extraction and analysis
  - FIR and IIR filter design
  - Parametric EQ, shelving, and crossover filters
  - CamillaDSP and BruteFIR export
- Comprehensive test suite (151 tests)
- Documentation for major modules (VBAP, HOA)
- Apache 2.0 license with SPDX identifiers

### Changed
- README updated with Loudness/DRC section and example instructions.
- CI clippy relaxed to not fail on warnings (until codebase warnings are addressed).

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - TBD

Initial public release.

[Unreleased]: https://github.com/yourusername/audio-ninja/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/audio-ninja/releases/tag/v0.1.0
