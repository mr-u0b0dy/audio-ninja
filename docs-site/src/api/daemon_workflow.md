# Audio Ninja Daemon Workflow Guide

Complete guide for deploying, configuring, and operating the Audio Ninja daemon.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Deployment](#deployment)
- [Common Workflows](#common-workflows)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)
- [Integration](#integration)

## Installation

### Prerequisites

```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# System dependencies (Linux)
sudo apt-get install -y \
  build-essential \
  libasound2-dev \
  pkg-config \
  libssl-dev

# System dependencies (macOS)
brew install portaudio pkg-config
```

### Building from Source

```bash
# Clone repository
git clone https://github.com/mr-u0b0dy/audio-ninja.git
cd audio-ninja

# Build daemon
cargo build -p audio-ninja-daemon --release

# Binary location
ls -lh target/release/audio-ninja-daemon
```

### Installing System-Wide (Linux)

```bash
# Install binary
sudo install -m 755 target/release/audio-ninja-daemon /usr/local/bin/

# Verify installation
audio-ninja-daemon --version
```

## Configuration

### Command-Line Options

```bash
audio-ninja-daemon [OPTIONS]

OPTIONS:
    --bind <ADDRESS>     Bind address [default: 127.0.0.1]
    --port <PORT>        HTTP port [default: 8080]
    --verbose            Enable verbose logging
    --log-level <LEVEL>  Set log level: trace, debug, info, warn, error [default: info]
    --state-dir <PATH>   State directory [default: /var/lib/audio-ninja]
    --config <FILE>      Configuration file [default: /etc/audio-ninja/daemon.toml]
```

### Configuration File

Create `/etc/audio-ninja/daemon.toml`:

```toml
[server]
bind_address = "127.0.0.1"
port = 8080
max_connections = 100

[audio]
sample_rate = 48000
buffer_size = 256
channels = 8

[network]
multicast_address = "239.255.77.77"
rtp_base_port = 5004
mtu = 1400

[sync]
clock_source = "ptp"  # Options: ptp, ntp, system
sync_tolerance_ms = 1.0

[calibration]
default_sweep_duration_ms = 1000
default_sweep_start_freq = 20
default_sweep_end_freq = 20000

[logging]
level = "info"
file = "/var/log/audio-ninja/daemon.log"
max_size_mb = 100
max_backups = 5

[state]
directory = "/var/lib/audio-ninja"
persist_sessions = true
auto_reconnect = true
```

### Environment Variables

```bash
# Override config file location
export AUDIO_NINJA_CONFIG=/path/to/daemon.toml

# Override state directory
export AUDIO_NINJA_STATE_DIR=/tmp/audio-ninja

# Set log level
export AUDIO_NINJA_LOG_LEVEL=debug

# Bind address (useful for containers)
export AUDIO_NINJA_BIND=0.0.0.0
export AUDIO_NINJA_PORT=8080
```

## Deployment

### Development Deployment

```bash
# Run with verbose logging
cargo run -p audio-ninja-daemon --release -- --verbose

# Run with custom config
cargo run -p audio-ninja-daemon --release -- \
  --config config/daemon.toml \
  --log-level debug
```

### Production Deployment (Systemd)

#### 1. Create System User

```bash
sudo useradd -r -s /bin/false -G audio audio-ninja
sudo usermod -a -G bluetooth audio-ninja  # If using BLE
```

#### 2. Create Directories

```bash
# State directory
sudo mkdir -p /var/lib/audio-ninja
sudo chown audio-ninja:audio /var/lib/audio-ninja
sudo chmod 750 /var/lib/audio-ninja

# Log directory
sudo mkdir -p /var/log/audio-ninja
sudo chown audio-ninja:audio /var/log/audio-ninja
sudo chmod 750 /var/log/audio-ninja

# Configuration directory
sudo mkdir -p /etc/audio-ninja
sudo chmod 755 /etc/audio-ninja
```

#### 3. Install Configuration

```bash
# Copy configuration
sudo cp config/daemon.toml /etc/audio-ninja/
sudo chown root:audio-ninja /etc/audio-ninja/daemon.toml
sudo chmod 640 /etc/audio-ninja/daemon.toml
```

#### 4. Install Systemd Service

```bash
# Copy service file
sudo cp crates/daemon/audio-ninja-daemon.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable audio-ninja-daemon

# Start service
sudo systemctl start audio-ninja-daemon

# Check status
sudo systemctl status audio-ninja-daemon
```

#### 5. Verify Installation

```bash
# Check daemon is running
curl http://127.0.0.1:8080/api/v1/status

# View logs
sudo journalctl -u audio-ninja-daemon -f

# Check listening ports
sudo ss -tlnp | grep audio-ninja
```

### Container Deployment (Docker)

#### Dockerfile

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /build
COPY . .

RUN apt-get update && apt-get install -y \
    build-essential \
    libasound2-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build -p audio-ninja-daemon --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libasound2 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false -G audio audio-ninja

COPY --from=builder /build/target/release/audio-ninja-daemon /usr/local/bin/
COPY config/daemon.toml /etc/audio-ninja/daemon.toml

RUN mkdir -p /var/lib/audio-ninja && \
    chown audio-ninja:audio /var/lib/audio-ninja

USER audio-ninja
EXPOSE 8080

CMD ["audio-ninja-daemon", "--bind", "0.0.0.0", "--config", "/etc/audio-ninja/daemon.toml"]
```

#### Docker Compose

```yaml
version: '3.8'

services:
  audio-ninja-daemon:
    image: audio-ninja/daemon:latest
    build:
      context: .
      dockerfile: Dockerfile
    container_name: audio-ninja-daemon
    restart: unless-stopped
    
    ports:
      - "8080:8080"
      - "5004-5020:5004-5020/udp"  # RTP ports
    
    volumes:
      - audio-ninja-state:/var/lib/audio-ninja
      - ./config/daemon.toml:/etc/audio-ninja/daemon.toml:ro
    
    environment:
      - AUDIO_NINJA_LOG_LEVEL=info
      - AUDIO_NINJA_BIND=0.0.0.0
    
    networks:
      - audio-ninja-net
    
    # Access to host audio devices (if needed)
    devices:
      - /dev/snd:/dev/snd
    
    # Capabilities for network access
    cap_add:
      - NET_ADMIN
      - NET_RAW

volumes:
  audio-ninja-state:

networks:
  audio-ninja-net:
    driver: bridge
```

#### Running with Docker

```bash
# Build image
docker build -t audio-ninja/daemon:latest .

# Run container
docker run -d \
  --name audio-ninja-daemon \
  -p 8080:8080 \
  -p 5004-5020:5004-5020/udp \
  -v audio-ninja-state:/var/lib/audio-ninja \
  audio-ninja/daemon:latest

# Check logs
docker logs -f audio-ninja-daemon

# Check status
curl http://localhost:8080/api/v1/status

# Stop container
docker stop audio-ninja-daemon
```

## Common Workflows

### Workflow 1: Basic Stereo Playback

```bash
# 1. Start daemon
sudo systemctl start audio-ninja-daemon

# 2. Register speakers
curl -X POST http://127.0.0.1:8080/api/v1/speakers \
  -H "Content-Type: application/json" \
  -d '{"id": "left", "name": "Left", "address": "192.168.1.101", "port": 5004}'

curl -X POST http://127.0.0.1:8080/api/v1/speakers \
  -H "Content-Type: application/json" \
  -d '{"id": "right", "name": "Right", "address": "192.168.1.102", "port": 5004}'

# 3. Configure stereo layout
curl -X POST http://127.0.0.1:8080/api/v1/layout \
  -H "Content-Type: application/json" \
  -d '{"type": "2.0", "mapping": "auto"}'

# 4. Start playback
curl -X POST http://127.0.0.1:8080/api/v1/transport/start \
  -H "Content-Type: application/json" \
  -d '{"source": "/path/to/audio.mp4", "format": "iamf", "transport": "rtp"}'

# 5. Monitor status
watch -n 1 curl -s http://127.0.0.1:8080/api/v1/transport/status

# 6. Stop playback
curl -X POST http://127.0.0.1:8080/api/v1/transport/stop \
  -H "Content-Type: application/json" \
  -d '{"session_id": "session-abc123"}'
```

### Workflow 2: 5.1 Surround Setup with Calibration

```bash
# 1. Register all 6 speakers (FL, FR, C, LFE, SL, SR)
for speaker in fl fr c lfe sl sr; do
  curl -X POST http://127.0.0.1:8080/api/v1/speakers \
    -H "Content-Type: application/json" \
    -d "{\"id\": \"$speaker\", \"name\": \"$speaker\", \"address\": \"192.168.1.10X\", \"port\": 5004}"
done

# 2. Configure 5.1 layout
curl -X POST http://127.0.0.1:8080/api/v1/layout \
  -H "Content-Type: application/json" \
  -d '{"type": "5.1", "mapping": "auto"}'

# 3. Run calibration
cal_id=$(curl -s -X POST http://127.0.0.1:8080/api/v1/calibration/start \
  -H "Content-Type: application/json" \
  -d '{"method": "sweep", "microphone": "default"}' \
  | jq -r '.calibration_id')

# 4. Wait for calibration to complete
while true; do
  status=$(curl -s http://127.0.0.1:8080/api/v1/calibration/$cal_id | jq -r '.status')
  progress=$(curl -s http://127.0.0.1:8080/api/v1/calibration/$cal_id | jq -r '.progress')
  echo "Calibration: $status ($progress%)"
  [ "$status" = "complete" ] && break
  sleep 2
done

# 5. Apply calibration
curl -X POST http://127.0.0.1:8080/api/v1/calibration/$cal_id/apply \
  -H "Content-Type: application/json" \
  -d '{"verify": true}'

# 6. Export calibration for backup
curl http://127.0.0.1:8080/api/v1/calibration/$cal_id/export?format=camilladsp \
  > calibration-$(date +%Y%m%d).yaml

# 7. Start playback with calibrated system
curl -X POST http://127.0.0.1:8080/api/v1/transport/start \
  -H "Content-Type: application/json" \
  -d '{"source": "/path/to/movie.mp4", "format": "iamf", "transport": "rtp"}'
```

### Workflow 3: Immersive 7.1.4 Atmos Setup

```bash
# 1. Use CLI for easier setup
audio-ninja speaker add fl --address 192.168.1.101 --position 330,0,2.5
audio-ninja speaker add fr --address 192.168.1.102 --position 30,0,2.5
audio-ninja speaker add c --address 192.168.1.103 --position 0,0,2.5
audio-ninja speaker add lfe --address 192.168.1.104 --position 0,-30,3.0
audio-ninja speaker add sl --address 192.168.1.105 --position 250,0,2.5
audio-ninja speaker add sr --address 192.168.1.106 --position 110,0,2.5
audio-ninja speaker add bl --address 192.168.1.107 --position 210,0,2.5
audio-ninja speaker add br --address 192.168.1.108 --position 150,0,2.5
audio-ninja speaker add tfl --address 192.168.1.109 --position 330,45,2.5
audio-ninja speaker add tfr --address 192.168.1.110 --position 30,45,2.5
audio-ninja speaker add tbl --address 192.168.1.111 --position 210,45,2.5
audio-ninja speaker add tbr --address 192.168.1.112 --position 150,45,2.5

# 2. Configure 7.1.4 layout
audio-ninja layout set 7.1.4

# 3. Calibrate system
audio-ninja calibrate start --method sweep --auto-apply

# 4. Start Atmos content playback
audio-ninja transport start atmos-movie.mp4 --format iamf --transport rtp

# 5. Monitor via GUI
audio-ninja-gui
```

### Workflow 4: Multi-Zone Audio

```bash
# Zone 1: Living Room (5.1)
curl -X POST http://127.0.0.1:8080/api/v1/zones \
  -H "Content-Type: application/json" \
  -d '{"id": "living-room", "name": "Living Room", "layout": "5.1", "speakers": ["lr-fl", "lr-fr", "lr-c", "lr-lfe", "lr-sl", "lr-sr"]}'

# Zone 2: Bedroom (2.0)
curl -X POST http://127.0.0.1:8080/api/v1/zones \
  -H "Content-Type: application/json" \
  -d '{"id": "bedroom", "name": "Bedroom", "layout": "2.0", "speakers": ["br-left", "br-right"]}'

# Start playback in living room
curl -X POST http://127.0.0.1:8080/api/v1/zones/living-room/transport/start \
  -H "Content-Type: application/json" \
  -d '{"source": "/media/movie.mp4", "format": "iamf"}'

# Start different content in bedroom
curl -X POST http://127.0.0.1:8080/api/v1/zones/bedroom/transport/start \
  -H "Content-Type: application/json" \
  -d '{"source": "/media/music.flac", "format": "iamf"}'
```

## Monitoring

### Health Checks

```bash
# Basic health check
curl -f http://127.0.0.1:8080/api/v1/status || echo "Daemon down!"

# Detailed status
curl http://127.0.0.1:8080/api/v1/status | jq '.'

# Check all speakers
curl http://127.0.0.1:8080/api/v1/speakers | jq '.speakers[] | {id, status, latency_ms}'

# Transport statistics
curl http://127.0.0.1:8080/api/v1/transport/status | jq '.'
```

### Prometheus Metrics

Add to `daemon.toml`:
```toml
[metrics]
enabled = true
port = 9090
path = "/metrics"
```

Metrics endpoint: `http://127.0.0.1:9090/metrics`

### Log Monitoring

```bash
# Follow daemon logs (systemd)
sudo journalctl -u audio-ninja-daemon -f

# Follow daemon logs (file)
tail -f /var/log/audio-ninja/daemon.log

# Filter for errors
sudo journalctl -u audio-ninja-daemon --priority=err -f

# Show last 100 lines
sudo journalctl -u audio-ninja-daemon -n 100
```

### Alerting Script

```bash
#!/bin/bash
# check-daemon.sh

BASE_URL="http://127.0.0.1:8080/api/v1"
ALERT_EMAIL="admin@example.com"

# Check daemon status
if ! curl -sf "$BASE_URL/status" >/dev/null; then
  echo "ALERT: Audio Ninja daemon is down!" | mail -s "Daemon Alert" "$ALERT_EMAIL"
  exit 1
fi

# Check speaker connectivity
offline_speakers=$(curl -s "$BASE_URL/speakers" | jq -r '.speakers[] | select(.status != "online") | .id')

if [ -n "$offline_speakers" ]; then
  echo "ALERT: Offline speakers: $offline_speakers" | mail -s "Speaker Alert" "$ALERT_EMAIL"
  exit 1
fi

# Check transport health
packet_loss=$(curl -s "$BASE_URL/transport/status" | jq -r '.sessions[0].packets_lost // 0')

if [ "$packet_loss" -gt 100 ]; then
  echo "ALERT: High packet loss: $packet_loss packets" | mail -s "Transport Alert" "$ALERT_EMAIL"
  exit 1
fi

echo "All checks passed"
```

Run with cron:
```cron
*/5 * * * * /usr/local/bin/check-daemon.sh
```

## Troubleshooting

### Daemon Won't Start

```bash
# Check service status
sudo systemctl status audio-ninja-daemon

# Check logs
sudo journalctl -u audio-ninja-daemon --no-pager

# Common issues:
# 1. Port already in use
sudo ss -tlnp | grep :8080

# 2. Permissions
sudo chown audio-ninja:audio /var/lib/audio-ninja
sudo chmod 750 /var/lib/audio-ninja

# 3. Missing dependencies
ldd /usr/local/bin/audio-ninja-daemon

# 4. Configuration errors
audio-ninja-daemon --config /etc/audio-ninja/daemon.toml --verbose
```

### Speakers Not Connecting

```bash
# Verify network connectivity
ping 192.168.1.101

# Check RTP ports
sudo tcpdump -i any port 5004

# Check speaker registration
curl http://127.0.0.1:8080/api/v1/speakers | jq '.speakers[] | {id, address, status}'

# Test manual RTP send
ffmpeg -i test.wav -f rtp rtp://192.168.1.101:5004

# Check firewall
sudo iptables -L -n | grep 5004
sudo ufw status
```

### High Latency or Jitter

```bash
# Check transport statistics
curl http://127.0.0.1:8080/api/v1/transport/status | jq '.sessions[] | {jitter_ms, packets_lost}'

# Network diagnostics
mtr -n 192.168.1.101

# Check buffer sizes
curl http://127.0.0.1:8080/api/v1/info | jq '.buffer_config'

# Adjust configuration
cat >> /etc/audio-ninja/daemon.toml <<EOF
[audio]
buffer_size = 512  # Increase from 256
buffer_count = 4   # Increase from 2
EOF

sudo systemctl restart audio-ninja-daemon
```

### Synchronization Issues

```bash
# Check sync source
curl http://127.0.0.1:8080/api/v1/sync/status | jq '.'

# Test PTP daemon (if using PTP)
sudo systemctl status ptp4l
sudo pmc -u -b 0 'GET CURRENT_DATA_SET'

# Check per-speaker sync
curl http://127.0.0.1:8080/api/v1/speakers | jq '.speakers[] | {id, sync_offset_ms}'

# Recalibrate
curl -X POST http://127.0.0.1:8080/api/v1/calibration/start \
  -H "Content-Type: application/json" \
  -d '{"method": "sync", "auto_apply": true}'
```

## Integration

### Systemd Service with Restart Policy

```ini
[Unit]
Description=Audio Ninja Daemon
After=network.target sound.target bluetooth.target
Wants=ptp4l.service

[Service]
Type=simple
User=audio-ninja
Group=audio
ExecStart=/usr/local/bin/audio-ninja-daemon --config /etc/audio-ninja/daemon.toml
Restart=always
RestartSec=10
StartLimitInterval=200
StartLimitBurst=5

# Hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/audio-ninja /var/log/audio-ninja
CapabilityBoundingSet=CAP_NET_RAW CAP_NET_ADMIN

[Install]
WantedBy=multi-user.target
```

### Integration with Home Assistant

```yaml
# configuration.yaml
rest:
  - resource: http://127.0.0.1:8080/api/v1/status
    sensor:
      - name: "Audio Ninja Status"
        value_template: "{{ value_json.status }}"
  
  - resource: http://127.0.0.1:8080/api/v1/speakers
    sensor:
      - name: "Audio Ninja Speakers Online"
        value_template: "{{ value_json.speakers | selectattr('status', 'equalto', 'online') | list | count }}"

# automations.yaml
- alias: "Audio Ninja - Start on Movie Night"
  trigger:
    - platform: state
      entity_id: input_boolean.movie_mode
      to: 'on'
  action:
    - service: rest_command.audio_ninja_start
      data:
        source: "/media/movies/current.mp4"

# rest_command
rest_command:
  audio_ninja_start:
    url: "http://127.0.0.1:8080/api/v1/transport/start"
    method: POST
    headers:
      content-type: "application/json"
    payload: '{"source": "{{ source }}", "format": "iamf", "transport": "rtp"}'
```

## See Also

- [REST API Usage](API_USAGE.md) - API endpoint examples
- [Daemon README](../crates/daemon/README.md) - Daemon overview
- [CLI README](../crates/cli/README.md) - Command-line interface
- [Calibration Workflow](CALIBRATION.md) - Detailed calibration guide
