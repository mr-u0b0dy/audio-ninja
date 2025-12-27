# Audio Ninja 🥷

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-151%20passing-brightgreen.svg)]()

**Audio Ninja** is an open-source wireless immersive audio platform with IAMF-first architecture, flexible speaker layouts, networked transport with sync, DSP processing, and room calibration.

### New: Loudness & DRC
- ITU-R BS.1770 loudness measurement (LUFS) and normalization
- Dynamic Range Control (DRC) with ratio/threshold and makeup gain
- Headroom protection via soft limiting (configurable dB)
- See the guide: [docs/loudness_drc.md](docs/loudness_drc.md)

## ✨ Features

### 🎵 Spatial Audio Rendering
- **IAMF (Immersive Audio Model and Formats)**: Parse, decode, and render object-based, channel-based, and scene-based audio
- **3D VBAP**: Vector-Based Amplitude Panning for arbitrary speaker layouts with elevation support
- **HOA Decoder**: Higher-Order Ambisonics (1st/2nd/3rd order) with Basic, Max-rE, and In-Phase modes
- **Flexible Layouts**: Support from 2.0 stereo through 9.1.6+ immersive configurations

### 🌐 Networked Audio
- **UDP/RTP Transport**: Real-time audio streaming with timestamp-based synchronization
- **Clock Sync**: PTP, NTP, and system clock support for multi-speaker alignment
- **mDNS Discovery**: Automatic speaker detection and registration
- **Forward Error Correction**: XOR-based FEC with packet loss concealment (silence/repeat/interpolate)
- **Jitter Buffer**: Adaptive buffering with latency compensation

### 🎛️ Control & Configuration
- **BLE GATT Profiles**: Wireless speaker control, pairing, and configuration
- **Speaker Identity**: Role assignment (FL, FR, C, LFE, SL, SR, height channels, etc.)
- **Layout Management**: Dynamic speaker positioning (azimuth, elevation, distance)
- **Calibration Settings**: Per-speaker volume trim, delay compensation, EQ enable/disable

### 🔧 Room Calibration
- **Sweep Generation**: Log sweep and MLS (Maximum Length Sequence) for impulse response capture
- **IR Analysis**: Automatic delay detection, magnitude response extraction
- **Filter Design**: FIR (linear-phase) and IIR (biquad cascade) filters
- **PEQ, Shelving, and Crossover Filters**: Parametric EQ, low/high shelf, high/low pass
- **DSP Export**: Generate CamillaDSP and BruteFIR configuration files

### 🎚️ Audio Processing
- **Multi-Format Support**: Opus, AAC, FLAC, PCM (with FFmpeg bindings planned)
- **Downmix/Upmix**: Automatic channel count adaptation
- **Pipeline Architecture**: Demux → Decode → Render workflow
- **Per-Speaker DSP**: Individual processing chains with filter management

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- Linux (primary target), macOS (planned), embedded targets (planned)

### Installation

```bash
git clone https://github.com/yourusername/audio-ninja.git
cd audio-ninja
cargo build --release
```

### Running Tests

```bash
cargo test
```

All 196 tests should pass:
- 30 HRTF tests
- 20 IAMF tests
- 16 transport/RTP tests
- 14 network/UDP tests
- 25 transport/sync/latency/jitter tests
- 8 BLE tests
- 12 calibration tests
- 13 FEC tests
- 32 VBAP tests
- 32 HOA tests

## 📖 Documentation

### Modules

- **[HRTF](docs/hrtf.md)**: Head-Related Transfer Function binaural rendering
- **[VBAP](docs/vbap.md)**: 3D Vector-Based Amplitude Panning
- **[HOA](docs/hoa.md)**: Higher-Order Ambisonics Decoder
- **Calibration**: Room measurement and correction
- **Network**: UDP/RTP streaming and speaker discovery
- **BLE**: Bluetooth Low Energy control plane
- **FEC**: Forward Error Correction
- **Transport**: RTP, jitter buffer, clock sync

### Examples

#### HRTF Binaural Rendering

```rust
use audio_ninja::hrtf::{HrtfDatabase, HrtfDataset, BinauralRenderer, HeadphoneProfile, HrtfPosition};

let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
db.load_default_kemar()?;

let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

// Render mono audio from a 45° angle
let mono_input = vec![0.5; 512];
let position = HrtfPosition::new(45.0, 15.0, 1.5);
let (left, right) = renderer.render(&mono_input, &position)?;

// Now you have binaural stereo audio for headphone playback
```

#### Loudness Normalization + DRC + Headroom

Run the end-to-end example:

```bash
cargo run --example loudness_processing
```

This demonstrates:
- Measuring loudness (integrated/short-term/LRA)
- Normalizing to streaming target (-14 LUFS)
- Applying DRC (e.g., 4:1 ratio, -18 dB threshold)
- Soft-limiting peaks to maintain headroom

For details, read: [docs/loudness_drc.md](docs/loudness_drc.md)

#### VBAP Rendering

```rust
use audio_ninja::vbap::{Vbap3D, create_7_1_4_layout, Vec3};

let speakers = create_7_1_4_layout();
let vbap = Vbap3D::new(speakers);

// Position a sound 30° left, 15° elevated
let source = Vec3::from_spherical(30.0, 15.0, 1.0);
let gains = vbap.render(&source);

// Apply gains to audio channels
for (channel, gain) in audio_channels.iter_mut().zip(gains.iter()) {
    for sample in channel.iter_mut() {
        *sample *= gain;
    }
}
```

#### HOA Decoding

```rust
use audio_ninja::hoa::{HoaDecoder, AmbisonicOrder, DecodingMode, create_5_1_hoa_layout};

let speakers = create_5_1_hoa_layout();
let decoder = HoaDecoder::new(
    AmbisonicOrder::SECOND,
    DecodingMode::MaxRE,
    speakers
);

// Decode 2nd order ambisonics (9 channels) to speakers
let ambisonic_input = vec![1.0, 0.2, 0.3, 0.1, 0.15, 0.05, 0.1, 0.08, 0.12];
let speaker_gains = decoder.decode(&ambisonic_input);
```

#### Room Calibration

```rust
use audio_ninja::calibration::*;

// Generate measurement sweep
let sweep = generate_log_sweep(48000, 2.0, 20.0, 20000.0);

// After recording the response, extract impulse response
let ir = extract_ir_from_sweep(&recorded, &sweep, 48000);

// Analyze and design correction filters
let delay_ms = compute_delay(&ir, 48000);
let peq = design_peq(48000, 1000.0, -3.0, 2.0);
```

#### Network Streaming

```rust
use audio_ninja::network::*;
use audio_ninja::fec::XorFec;

// Create sender with FEC
let mut sender = UdpRtpSender::new("0.0.0.0:0", "192.168.1.100:5004", 12345)?;
sender.enable_fec(8); // Group size of 8

// Send audio block
sender.send_block(&audio_block)?;

// Receiver with loss statistics
let receiver = UdpRtpReceiver::new("0.0.0.0:5004")?;
let audio_block = receiver.recv_block()?;
let stats = receiver.statistics();
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Audio Ninja Platform                     │
├─────────────────────────────────────────────────────────────┤
│  Input Layer                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │
│  │  IAMF    │  │  Opus    │  │  AAC     │  ...              │
│  │  Parser  │  │  Decoder │  │  Decoder │                   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘                  │
│       └─────────────┴─────────────┘                          │
│                      │                                        │
│  ┌──────────────────▼───────────────────────┐               │
│  │         Render Pipeline                   │               │
│  │  ┌──────────┐  ┌──────────┐  ┌─────────┐│               │
│  │  │  VBAP    │  │   HOA    │  │ Channel ││               │
│  │  │ Object   │  │  Scene   │  │  Bed    ││               │
│  │  │ Renderer │  │ Decoder  │  │  Mix    ││               │
│  │  └────┬─────┘  └────┬─────┘  └────┬────┘│               │
│  │       └─────────────┴─────────────┘     │               │
│  │                Mixer                      │               │
│  └──────────────────┬────────────────────────┘               │
│                     │                                        │
│  ┌──────────────────▼────────────────────────┐              │
│  │         DSP Processing                     │              │
│  │  ┌─────────┐  ┌──────────┐  ┌──────────┐ │              │
│  │  │ Delay   │  │  Volume  │  │   EQ     │ │              │
│  │  │  Comp   │  │   Trim   │  │ Filters  │ │              │
│  │  └─────────┘  └──────────┘  └──────────┘ │              │
│  └──────────────────┬────────────────────────┘              │
│                     │                                        │
│  ┌──────────────────▼────────────────────────┐              │
│  │         Network Transport                  │              │
│  │  ┌─────────┐  ┌──────────┐  ┌──────────┐ │              │
│  │  │   FEC   │  │   RTP    │  │  Jitter  │ │              │
│  │  │ Encoder │  │ Packetize│  │  Buffer  │ │              │
│  │  └─────────┘  └──────────┘  └──────────┘ │              │
│  └──────────────────┬────────────────────────┘              │
│                     │                                        │
│                UDP/IP Network                                │
│                     │                                        │
│  ┌──────────────────▼────────────────────────┐              │
│  │         Speaker Nodes (1..N)               │              │
│  │  ┌─────────┐  ┌──────────┐  ┌──────────┐ │              │
│  │  │   FEC   │  │  Clock   │  │   DAC    │ │              │
│  │  │ Decoder │  │   Sync   │  │  Output  │ │              │
│  │  └─────────┘  └──────────┘  └──────────┘ │              │
│  │          (BLE Control Plane)              │              │
│  └───────────────────────────────────────────┘              │
└─────────────────────────────────────────────────────────────┘
```

## 🛠️ Current Status

### ✅ Completed
- Core IAMF parsing and rendering
- 3D VBAP spatial renderer
- HOA decoder (1st/2nd/3rd order)
- UDP/RTP network transport
- Clock synchronization (PTP/NTP/System)
- Forward Error Correction
- BLE GATT control plane
- Room calibration pipeline
- DSP filter design and export
- Comprehensive test suite (151 tests)

### 🚧 In Progress
- Real libiamf integration
- FFmpeg codec bindings (AC-3, E-AC-3, TrueHD)
- HRTF binaural rendering
- REST/gRPC WiFi control API

### 📋 Planned
- RTSP session management
- Adaptive bitrate streaming
- Firmware update mechanism
- CI/CD pipeline
- Python bindings
- Example applications (CLI player, GUI controller)

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/audio-ninja.git
cd audio-ninja

# Install development dependencies
rustup component add clippy rustfmt

# Run tests
cargo test

# Run linters
cargo clippy -- -D warnings
cargo fmt --check
```

### Code Style

- Rust 2021 edition
- Prefer explicit structs and enums over macros
- Small, focused functions with concise comments
- `serde` for serialization, `anyhow`/`thiserror` for error handling
- Maintain ASCII-only unless existing files require otherwise

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

```
Copyright 2025 Audio Ninja Contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

## 🙏 Acknowledgments

- IAMF specification by Alliance for Open Media (AOM)
- Dolby Atmos for immersive audio innovation
- ITU-R BS.2051 for spatial audio standards
- Open-source audio community

## 📞 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/audio-ninja/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/audio-ninja/discussions)
- **Documentation**: [docs/](docs/)

## 🌟 Star History

If you find this project useful, please consider giving it a star! ⭐

---

**Built with ❤️ and Rust**
