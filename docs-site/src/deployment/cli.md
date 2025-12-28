# CLI Tool Guide

Command-line interface for controlling Audio Ninja daemon.

## Installation

```bash
cargo build -p audio-ninja-cli --release

# Binary location
./target/release/audio-ninja
```

## Basic Commands

### Check Status

```bash
audio-ninja status
```

Output:
```
Daemon Status:
  Status: running
  Version: 0.1.0
  Uptime: 3600s (1h)
  Speakers: 5 registered
```

### List Speakers

```bash
audio-ninja speakers list
```

### Register Speaker

```bash
audio-ninja speakers add \
  --id speaker-001 \
  --name "Front Left" \
  --address 192.168.1.101 \
  --azimuth -30 \
  --elevation 0 \
  --distance 2.5
```

### Play Audio

```bash
audio-ninja play /path/to/audio.wav
audio-ninja play https://example.com/stream.m4a
```

### Stop Playback

```bash
audio-ninja stop
```

## Configuration

### Set Default Daemon Address

```bash
export AUDIO_NINJA_DAEMON=http://192.168.1.100:8080
```

### Enable Verbose Output

```bash
audio-ninja --verbose status
```

## See Also

- [REST API Reference](/deployment/api.md)
- [Daemon Deployment](/deployment/daemon.md)
- [Quick Start](/guide/quick-start.md)

---

📖 **[Full CLI Documentation](../../crates/cli/README.md)**
