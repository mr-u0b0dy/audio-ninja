# Copilot Instructions

## Scope
This repo is an OSS wireless immersive audio platform (IAMF-first) with flexible speaker layouts, networked transport/sync, DSP, and room calibration.

**Architecture**: Cargo workspace with client-server design
- `crates/core/` - Core library (audio-ninja crate)
- `crates/daemon/` - Background engine service with REST API
- `crates/gui/` - Desktop GUI client (Tauri)

## Coding style
- Rust 2021, prefer explicit structs and enums; avoid unnecessary macros.
- Keep functions small and focused; add concise comments only when logic is non-obvious.
- Serialization uses `serde`; error handling via `anyhow`/`thiserror` where appropriate.
- Maintain ASCII-only unless existing files require otherwise.
- Run `cargo clippy --fix` before committing; address warnings.

## Architecture guidelines
- **Workspace structure**: All code in `crates/` with shared dependencies in root Cargo.toml
- **Daemon-first**: Core audio engine runs in daemon service; GUI/CLI are thin clients
- **REST API**: Daemon exposes HTTP API on port 8080 for control and monitoring
- Maintain clear separation: iamf (parse/decode), render (object/channel mapping), transport (RTP/WebRTC-style, sync), control (BLE/WiFi), calibration (measurement/EQ), dsp (filters).
- Renderer must support arbitrary layouts (2.0 through height layouts like 9.1.6) with downmix/upmix rules.
- Transport should carry timestamps and support PTP/NTP-based skew correction.
- Calibration flow: measure → solve delays/trims/EQ → apply via DSP configs (e.g., CamillaDSP/BruteFIR).

## Licensing
- SPDX: Apache-2.0 in sources; LICENSE file present. Avoid adding other licensed code without notice.

## Testing
- Add unit tests for serialization, transport, renderer mappings, and calibration math
- Prefer property/fuzz tests for parsers
- Add integration tests for daemon API endpoints
- Target 80%+ coverage for core library

## Workspace Commands
```bash
# Build entire workspace
cargo build --workspace --release

# Build specific crate
cargo build -p audio-ninja-daemon --release

# Run tests
cargo test --workspace

# Run end-to-end daemon ↔ CLI tests
cargo test -p audio-ninja-cli --test e2e_daemon_cli

# Lint and fix
cargo clippy --workspace --fix

# Run daemon
cargo run -p audio-ninja-daemon --release

# Run GUI
cargo run -p audio-ninja-gui --release

# Run core benchmarks
cargo bench -p audio-ninja --bench main_benchmarks
```

## Build Optimization
- Profiles: `.cargo/config.toml` sets `strip = true`, `lto = "thin"`, `codegen-units = 1`, `opt-level = 3` for `release`/`bench`; `panic = "abort"` in `release` to reduce size. `dev` uses `debug = 0` and `incremental = false` to curb target bloat.
- First-run builds: E2E tests wait up to 60s for the daemon to start to accommodate compilation when using `cargo run` during tests.
- Measure size: Use `cargo clean` before measuring and `du -sh target target/release` to profile footprint.
- GUI notes: Tauri pulls GTK/WebKit on Linux; expect larger `deps`. Build GUI with `--release` for smaller binaries.
- CI caching: Cache cargo registry and `target` to speed builds.

Example GitHub Actions cache:
```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}-profiles-${{ hashFiles('.cargo/config.toml') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

## Contributions
- Keep new files small and purposeful; avoid large auto-generated blobs.
- Document new public APIs with brief Rust doc comments.

## Backlog (working TODO)

### Infrastructure (Completed)
- ✅ Cargo workspace structure with crates/core, crates/daemon, crates/gui
- ✅ Daemon service with REST API (Axum on port 8080)
- ✅ Desktop GUI client (Tauri + vanilla JS)
- ✅ Systemd service file for Linux deployment
- ✅ GitHub Actions CI: fmt, clippy, build, test, benchmarks, doc
- ✅ GitHub Actions Release: automated binary builds (x86_64/aarch64 Linux)

### Core Modules (Completed)
- ✅ iamf-core: parse/render with element types (channel/object/scene), metadata, mix presentations
- ✅ transport-sync: RTP packet format, PTP/NTP/System clock sync, jitter buffer, loopback transport
- ✅ latency: per-speaker latency compensation, multi-speaker sync buffers
- ✅ mapping: VBAP stereo panning, downmix/upmix, layout presets (2.0, 5.1)
- ✅ vbap: full 3D VBAP for arbitrary speaker arrays with elevation support
- ✅ hoa: Higher-Order Ambisonics (1st/2nd/3rd order, Basic/Max-rE/In-Phase modes)
- ✅ hrtf: Binaural rendering with 4 headphone profiles (Flat, ClosedBack, OpenBack, IEM)
- ✅ loudness: ITU-R BS.1770-4 measurement, normalization, headroom management
- ✅ drc: Dynamic Range Control with Speech/Music/Cinema presets
- ✅ ffmpeg: demuxer/decoder stubs for Opus/AAC/FLAC/PCM
- ✅ pipeline: demux→decode→render pipeline with IamfRenderBlock output
- ✅ network: UDP/RTP sender/receiver, mDNS discovery, multi-speaker broadcast
- ✅ fec: XOR-based FEC, loss statistics, packet concealment (silence/repeat/interpolate)
- ✅ ble: GATT profiles for speaker control, pairing, calibration, layout config
- ✅ calibration: Sweep generation, IR analysis, FIR/IIR filter design, DSP export

### Format Support & Codecs
- Integrate real libiamf/AOM reference decoder (replace stubs)
- Add FFmpeg bindings for AC-3, E-AC-3, TrueHD decoding (licensing permitting)
- Dolby Atmos metadata parser for object positioning and mix presentations
- Support additional codecs: Opus, AAC, FLAC beyond PCM
- Add bitstream validation and conformance tests

### Spatial Renderer & Object Positioning
- ✅ Implement full VBAP for 3D speaker arrays (beyond stereo)
- ✅ Add HOA (Higher-Order Ambisonics) decoder for scene-based elements
- ✅ HRTF processing for binaural downmix
- ✅ Headroom management and loudness normalization per ITU-R BS.1770
- ✅ DRC (Dynamic Range Control) handling
- ✅ Support all layouts: 2.0, 2.1, 3.1, 4.0, 5.1, 5.1.2, 7.1, 7.1.4, 9.1.6, custom

### Transport & Networking
- ✅ Real UDP/RTP sender and receiver (replace loopback)
- ✅ mDNS service discovery for speaker announcement/discovery
- ✅ Packet loss handling and FEC (Forward Error Correction)
- WiFi Direct peer-to-peer mode
- RTSP session management
- Sample-accurate sync across speakers (±5ms tolerance initially, ±1ms target)
- Adaptive bitrate for varying network conditions

### Control Plane & API
- ✅ BLE GATT profiles: pairing, speaker identity, layout config, trims, delays
- ✅ REST API endpoints: speaker management, layout config, transport control, calibration
- ✅ Desktop GUI client with Tauri
- ✅ CLI tool for command-line control (audio-ninja binary)
- Speaker registration and capability negotiation
- Firmware update mechanism
- Low-bandwidth BLE audio fallback (LC3/BIS for stereo)

### Room Calibration
- ✅ Sweep generation (log sweep, MLS) for impulse response capture
- ✅ IR analysis: peak detection for delay, magnitude response for EQ
- ✅ FIR filter design (linear-phase, windowed sinc)
- ✅ IIR biquad cascade design (PEQ, shelf, high/low-pass)
- ✅ Export to CamillaDSP/BruteFIR config formats
- Microphone input handling (ALSA/PortAudio)
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
- ✅ CLI tool for command-line control (10 tests passing)
- Speaker registration API
- Stream configuration API
- Object positioning API (real-time updates)
- Format negotiation and capability exchange
- C API for embedded integration
- Python bindings for scripting
- Example applications (CLI player, GUI controller)

### Testing & Quality
- ✅ Unit tests: serialization, codec round-trip, renderer mapping (55 tests passing)
- ✅ Integration tests: end-to-end decode→render→transport with loopback
- ✅ Latency measurement and profiling
- ✅ Multi-speaker sync validation (phase alignment)
- ✅ CI pipeline: fmt, clippy, test, benchmark
- ✅ Daemon API endpoint tests (21 tests passing)
- ✅ CLI tests (10 tests passing)
- ✅ End-to-end daemon ↔ CLI integration tests (5 tests passing)
- GUI integration tests
- End-to-end daemon ↔ GUI ↔ CLI integration tests
- Fuzz testing: IAMF parser, RTP deserializer
- Benchmarking suite with regression tracking (VBAP, HRTF, loudness performance)

### Tooling & Documentation
- ✅ Build instructions (Linux, macOS, embedded targets)
- ✅ API documentation with examples
- ✅ Architecture diagrams
- ✅ Contribution guidelines
- ✅ OpenAPI/Swagger spec for REST API
- REST API usage examples (curl, HTTP clients)
- Daemon usage examples and workflow guides
- Calibration workflow documentation
- Performance benchmarks and latency/throughput specs
- Sequence diagrams for data flow
- Add NOTICE file if third-party code included

## Priority Tasks (Next Steps)

### High Priority (Completed ✅)
1. ✅ **Fix clippy warnings**: Applied auto-fixes to 13 files
2. ✅ **Add daemon tests**: 21 API endpoint tests with full coverage
3. ✅ **Create CLI tool**: `audio-ninja-cli` crate with 10 tests
4. ✅ **Update repository metadata**: Updated GitHub URLs to mr-u0b0dy
5. ✅ **Tag v0.1.0 release**: Tagged baseline version

### Medium Priority (Do Next)
6. ✅ **Add API documentation**: OpenAPI/Swagger spec for REST API endpoints
7. ✅ **Create integration tests**: End-to-end daemon ↔ CLI tests
8. ✅ **Optimize build**: Added Cargo profiles, reduced binaries to ~2-6 MB
9. ✅ **Add benchmarks**: Track VBAP, loudness, Vec3 performance with `cargo bench`
10. ✅ **Developer tooling**: Makefile, setup script, VS Code configs
11. **Design proper icons**: Replace placeholder blue circles with real branding

### Low Priority (Later)
11. **Fuzz testing**: Add `cargo-fuzz` for IAMF/RTP parsers
12. **Cross-platform**: Test on macOS, add Windows support
13. ✅ **Release automation**: GitHub Actions workflow for binary builds and releases
14. **Real codec integration**: Replace FFmpeg stubs with actual Opus/AAC/FLAC decoding
15. **IAMF decoder**: Integrate libiamf/AOM reference implementation
16. **ARM/embedded**: Configure cross-compilation targets
17. **Demo applications**: Example projects using the daemon API
