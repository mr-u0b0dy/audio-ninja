# Copilot Instructions

## Scope
This repo is an OSS wireless immersive audio platform (IAMF-first) with flexible speaker layouts, networked transport/sync, DSP, and room calibration.

## Coding style
- Rust 2021, prefer explicit structs and enums; avoid unnecessary macros.
- Keep functions small and focused; add concise comments only when logic is non-obvious.
- Serialization uses `serde`; error handling via `anyhow`/`thiserror` where appropriate.
- Maintain ASCII-only unless existing files require otherwise.

## Architecture guidelines
- Maintain clear separation: iamf (parse/decode), render (object/channel mapping), transport (RTP/WebRTC-style, sync), control (BLE/WiFi), calibration (measurement/EQ), dsp (filters).
- Renderer must support arbitrary layouts (2.0 through height layouts like 9.1.6) with downmix/upmix rules.
- Transport should carry timestamps and support PTP/NTP-based skew correction.
- Calibration flow: measure → solve delays/trims/EQ → apply via DSP configs (e.g., CamillaDSP/BruteFIR).

## Licensing
- SPDX: Apache-2.0 in sources; LICENSE file present. Avoid adding other licensed code without notice.

## Testing
- Add unit tests for serialization, transport, renderer mappings, and calibration math; prefer property/fuzz tests for parsers.

## Contributions
- Keep new files small and purposeful; avoid large auto-generated blobs.
- Document new public APIs with brief Rust doc comments.

## Backlog (working TODO)

### Core Modules (Completed)
- ✅ iamf-core: parse/render with element types (channel/object/scene), metadata, mix presentations
- ✅ transport-sync: RTP packet format, PTP/NTP/System clock sync, jitter buffer, loopback transport
- ✅ latency: per-speaker latency compensation, multi-speaker sync buffers
- ✅ mapping: VBAP stereo panning, downmix/upmix, layout presets (2.0, 5.1)
- ✅ vbap: full 3D VBAP for arbitrary speaker arrays with elevation support
- ✅ ffmpeg: demuxer/decoder stubs for Opus/AAC/FLAC/PCM
- ✅ pipeline: demux→decode→render pipeline with IamfRenderBlock output
- ✅ network: UDP/RTP sender/receiver, mDNS discovery, multi-speaker broadcast
- ✅ fec: XOR-based FEC, loss statistics, packet concealment (silence/repeat/interpolate)
- ✅ ble: GATT profiles for speaker control, pairing, calibration, layout config

### Format Support & Codecs
- Integrate real libiamf/AOM reference decoder (replace stubs)
- Add FFmpeg bindings for AC-3, E-AC-3, TrueHD decoding (licensing permitting)
- Dolby Atmos metadata parser for object positioning and mix presentations
- Support additional codecs: Opus, AAC, FLAC beyond PCM
- Add bitstream validation and conformance tests

### Spatial Renderer & Object Positioning
- ✅ Implement full VBAP for 3D speaker arrays (beyond stereo)
- ✅ Add HOA (Higher-Order Ambisonics) decoder for scene-based elements
- HRTF processing for binaural downmix
- Headroom management and loudness normalization per ITU-R BS.1770
- DRC (Dynamic Range Control) handling
- ✅ Support all layouts: 2.0, 2.1, 3.1, 4.0, 5.1, 5.1.2, 7.1, 7.1.4, 9.1.6, custom

### Transport & Networking
- ✅ Real UDP/RTP sender and receiver (replace loopback)
- ✅ mDNS service discovery for speaker announcement/discovery
- WiFi Direct peer-to-peer mode
- RTSP session management
- Sample-accurate sync across speakers (±5ms tolerance initially, ±1ms target)
- ✅ Packet loss handling and FEC (Forward Error Correction)
- Adaptive bitrate for varying network conditions

### Control Plane
- ✅ BLE GATT profiles: pairing, speaker identity, layout config, trims, delays
- WiFi REST API or gRPC endpoints for control
- Speaker registration and capability negotiation
- Firmware update mechanism
- Low-bandwidth BLE audio fallback (LC3/BIS for stereo)

### Room Calibration
- ✅ Sweep generation (log sweep, MLS) for impulse response capture
- Microphone input handling (ALSA/PortAudio)
- ✅ IR analysis: peak detection for delay, magnitude response for EQ
- ✅ FIR filter design (linear-phase, windowed sinc)
- ✅ IIR biquad cascade design (PEQ, shelf, high/low-pass)
- ✅ Export to CamillaDSP/BruteFIR config formats
- Multi-point averaging and target curve selection
- Calibration verification loop (re-measure after applying filters)

### DSP Pipeline Integration
- ✅ CamillaDSP integration: config generation, process communication
- BruteFIR integration: convolution engine control
- JACK/PipeWire/ALSA backend selection
- Per-speaker DSP profiles with hot-reload
- Safety limits (clipping prevention, thermal protection)
- Crossover filters for active speaker designs

### SDK & Integration APIs
- Speaker registration API
- Stream configuration API
- Object positioning API (real-time updates)
- Format negotiation and capability exchange
- C API for embedded integration
- Python bindings for scripting
- Example applications (CLI player, GUI controller)

### Testing & Quality
- ✅ Unit tests: serialization, codec round-trip, renderer mapping
- ✅ Integration tests: end-to-end decode→render→transport with loopback
- Fuzz testing: IAMF parser, RTP deserializer
- ✅ Latency measurement and profiling
- ✅ Multi-speaker sync validation (phase alignment)
- CI pipeline: fmt, clippy, test, benchmark

### Tooling & Documentation
- Build instructions (Linux, macOS, embedded targets)
- API documentation with examples
- Architecture diagrams
- Performance benchmarks and optimization
- Add NOTICE file if third-party code included
- Contribution guidelines
