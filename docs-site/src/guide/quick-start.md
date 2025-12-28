# Quick Start (5 Minutes)

Get Audio Ninja running in 5 minutes.

## Prerequisites

- Rust 1.70 or later
- Linux (Ubuntu/Debian) or macOS
- 200MB disk space

## Installation

### Option 1: Using Dev Container (Recommended)

```bash
git clone https://github.com/mr-u0b0dy/audio-ninja.git
cd audio-ninja

# Open in VS Code with Dev Container
code .
# Ctrl+Shift+P → "Dev Containers: Reopen in Container"
```

### Option 2: Manual Installation

```bash
# Clone repository
git clone https://github.com/mr-u0b0dy/audio-ninja.git
cd audio-ninja

# Build release binaries
cargo build --workspace --release

# Binaries are now in target/release/
```

## Start the Daemon

```bash
# Development mode with logging
cargo run -p audio-ninja-daemon --release -- --verbose

# Or run the built binary directly
./target/release/audio-ninja-daemon --bind 127.0.0.1 --port 8080
```

You should see:
```
2025-01-15T10:30:45 INFO audio_ninja_daemon: Starting daemon on 127.0.0.1:8080
2025-01-15T10:30:45 INFO audio_ninja_daemon: API available at http://127.0.0.1:8080/api/v1
```

## Check Status

In a new terminal:

```bash
# Using CLI tool
cargo run -p audio-ninja-cli --release -- status

# Or using curl
curl http://127.0.0.1:8080/api/v1/status
```

Expected response:
```json
{
  "status": "running",
  "version": "0.1.0",
  "uptime_seconds": 42
}
```

## Next Steps

✅ **Daemon running!** Now:

1. **[Configure your speakers](/guide/configuration.md)** - Add speaker positions
2. **[Explore the REST API](/deployment/api.md)** - Full API examples
3. **[Set up calibration](/processing/calibration.md)** - Optimize your room
4. **[Read spatial audio guide](/spatial/)** - Understanding VBAP, HOA, HRTF
