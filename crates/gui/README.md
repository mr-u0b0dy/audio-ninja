# Audio Ninja Desktop GUI

A modern, intuitive desktop application for professional audio processing with DRC, loudness normalization, and binaural rendering.

## Features

✨ **Professional Audio Processing**
- Dynamic Range Control (DRC) with presets (Speech, Music, Cinema)
- ITU-R BS.1770 loudness normalization
- Headroom protection with lookahead limiting
- HRTF-based binaural rendering for headphones

🎨 **Beautiful, Responsive UI**
- Modern gradient design with smooth animations
- Real-time spatial visualization
- Interactive configuration controls
- Live status display

🚀 **Built with Modern Tech**
- Tauri desktop framework (lightweight, secure)
- Rust backend for performance
- Vanilla JavaScript frontend (no dependencies)
- Cross-platform (Windows, macOS, Linux)

## Building

### Prerequisites
- Rust 1.70+
- Node.js 14+ (for frontend tooling, if used)
- On Linux: additional development dependencies

### macOS & Linux
```bash
cd gui
cargo build --release
```

### Windows
```bash
cd gui
cargo build --release
```

The compiled application will be in:
- `target/release/bundle/` (installers)
- `target/release/audio-ninja-gui` (executable)

## Running

### Development Mode
```bash
cd gui
cargo tauri dev
```

### Production Build
```bash
cd gui
cargo tauri build
```

## Usage

1. **Configure Audio Processing**
   - Select DRC preset (Speech/Music/Cinema)
   - Choose loudness target (TV/Streaming/Film)
   - Adjust headroom and lookahead

2. **Enable Binaural (Optional)**
   - Toggle "Enable Binaural for Headphones"
   - Select headphone profile
   - Adjust azimuth, elevation, distance
   - Watch the spatial visualization update

3. **Apply & Process**
   - Click "Apply Configuration"
   - Click "Process Audio"
   - View results and status

## Configuration

### DRC Presets

**Speech**: 3:1 ratio, -16dB threshold (podcasts, audiobooks)
- Fast attack (5ms), moderate release (80ms)
- Preserves intelligibility

**Music**: 4:1 ratio, -18dB threshold (streaming, mixed)
- Balanced attack (10ms), smooth release (100ms)
- Suitable for most music content

**Cinema**: 2:1 ratio, -14dB threshold (film distribution)
- Slow attack (20ms), slow release (150ms)
- Maintains dynamic range and spaciousness

### Loudness Targets

- **Television**: -23 LUFS (broadcast, streaming video)
- **Streaming Music**: -14 LUFS (Spotify, Apple Music, YouTube)
- **Film Theatrical**: -27 LUFS (cinema distribution)
- **Film Home**: -20 LUFS (Blu-ray, streaming films)

### Binaural Profiles

- **Flat**: Reference/neutral (raw HRTF)
- **ClosedBack**: Most closed-back headphones
- **OpenBack**: Open-back headphones
- **IEM**: In-ear monitors

### Spatial Positioning

**Azimuth** (horizontal angle):
- 0° = Front
- -90° = Left
- 90° = Right
- 180° = Behind

**Elevation** (vertical angle):
- 0° = Ear level
- 45° = Above
- -45° = Below

**Distance**:
- 0.5m = Close (strong near-field effects)
- 1.0m = Normal (typical listening)
- 2.0m = Far (spacious, weak near-field)

## Architecture

```
┌─────────────────────────────────────┐
│        Tauri Desktop App            │
├─────────────────────────────────────┤
│  Frontend (HTML/CSS/JavaScript)     │
├─────────────────────────────────────┤
│    IPC Bridge (Tauri Commands)      │
├─────────────────────────────────────┤
│     Rust Backend (audio-ninja)      │
│  • ReferenceRenderer                │
│  • DRC + Loudness + Binaural        │
└─────────────────────────────────────┘
```

## Commands

**invoke('initialize_renderer', { sampleRate })**
- Initializes the audio renderer at specified sample rate

**invoke('apply_config', { config })**
- Applies DRC, loudness, headroom, and binaural configuration

**invoke('process_audio', { channels, numSamples })**
- Processes test audio through configured pipeline

**invoke('get_status')**
- Returns current renderer status

**invoke('get_available_options')**
- Returns available presets and profiles

## Troubleshooting

**Window won't open**
- Ensure Tauri dependencies are installed
- On Linux, check development library requirements

**Audio not processing**
- Verify "Apply Configuration" was clicked
- Check browser console for errors (F12 in dev mode)
- Try resetting to defaults

**GUI is slow**
- Close unnecessary applications
- Check system resources
- Rebuild in release mode

## Performance

- **CPU**: ~2-5% for real-time processing
- **Memory**: ~50-100MB for GUI + audio library
- **Latency**: <5ms from input to output

## License

Apache License 2.0 - See LICENSE file

## Contributing

Contributions welcome! Please see main repository CONTRIBUTING guidelines.

## Support

- 📖 [Main Documentation](../docs/)
- 💬 [GitHub Issues](https://github.com/yourusername/audio-ninja/issues)
- 🌐 [Project Repository](https://github.com/yourusername/audio-ninja)
