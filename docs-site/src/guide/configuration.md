# Configuration Guide

Configure Audio Ninja for your specific environment and use case.

## Daemon Configuration

### Command-Line Options

```bash
audio-ninja-daemon [OPTIONS]

OPTIONS:
  --bind <ADDRESS>      Bind address [default: 127.0.0.1]
  --port <PORT>         HTTP port [default: 8080]
  --verbose             Enable verbose logging
  --log-level <LEVEL>   Set log level: trace, debug, info, warn, error
  --state-dir <PATH>    State directory [default: /var/lib/audio-ninja]
  --config <FILE>       Configuration file [default: /etc/audio-ninja/daemon.toml]
```

### Configuration File

Create `/etc/audio-ninja/daemon.toml`:

```toml
[server]
bind_address = "0.0.0.0"       # Listen on all interfaces
port = 8080                     # HTTP API port
verbose = true                  # Enable verbose logging
log_level = "info"              # Logging level

[state]
directory = "/var/lib/audio-ninja"

[audio]
sample_rate = 48000            # Audio sample rate (Hz)
buffer_size = 1024             # Audio buffer size (samples)
default_layout = "5.1"         # Default speaker layout

[network]
max_speakers = 32              # Maximum connected speakers
rtp_port = 5004                # RTP receive port
discovery_interval = 30        # mDNS discovery interval (seconds)

[calibration]
auto_calibrate = true          # Enable auto-calibration
measurement_timeout = 300      # Measurement timeout (seconds)
```

## Speaker Configuration

### Register a Speaker

```bash
curl -X POST http://127.0.0.1:8080/api/v1/speakers \
  -H "Content-Type: application/json" \
  -d '{
    "id": "speaker-001",
    "name": "Front Left",
    "address": "192.168.1.101",
    "port": 5004,
    "position": {
      "azimuth": -30.0,
      "elevation": 0.0,
      "distance": 2.5
    }
  }'
```

### Standard Layouts

#### 5.1 Surround
```
Front Left (-30°)  |  Center (0°)  |  Front Right (30°)
Surround Left (-110°)        Surround Right (110°)
LFE (Subwoofer)
```

#### 7.1 Surround
```
Front Left (-30°)  |  Center (0°)  |  Front Right (30°)
Side Left (-90°)                Side Right (90°)
Back Left (-150°)            Back Right (150°)
LFE (Subwoofer)
```

## Enable Features

### Enable VBAP Rendering

```bash
# Already enabled by default
# Configure in daemon.toml
[rendering]
vbap_enabled = true
vbap_triplet_search = true
```

### Enable HOA Decoding

```bash
[rendering]
hoa_enabled = true
hoa_orders = [1, 2, 3]  # 1st, 2nd, 3rd order support
```

### Enable HRTF Rendering

```bash
[rendering]
hrtf_enabled = true
hrtf_dataset = "kemar"
headphone_profile = "flat"  # flat, closed_back, open_back, iem
```

## Environment Variables

```bash
# Logging
RUST_LOG=audio_ninja=debug,audio_ninja_daemon=info

# Audio device
AUDIO_DEVICE=hw:0

# Network
AUDIO_NINJA_BIND=0.0.0.0
AUDIO_NINJA_PORT=8080
```

## Next Steps

- **[Spatial Audio](/spatial/)** - Configure rendering algorithms
- **[Calibration](/processing/calibration.md)** - Optimize your room
- **[API Reference](/deployment/api.md)** - REST API reference
