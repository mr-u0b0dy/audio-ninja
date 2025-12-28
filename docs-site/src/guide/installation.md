# Installation Guide

Complete guide for installing Audio Ninja on your system.

## System Requirements

### Linux (Ubuntu 20.04+, Debian 11+)

```bash
# Build dependencies
sudo apt-get install -y \
  build-essential \
  rustup \
  pkg-config \
  libssl-dev \
  libasound2-dev

# Or install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### macOS (Intel & Apple Silicon)

```bash
# Using Homebrew
brew install rust pkg-config openssl

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

Audio Ninja is not currently supported on Windows. Use WSL2 with Ubuntu or Docker.

## Clone Repository

```bash
git clone https://github.com/mr-u0b0dy/audio-ninja.git
cd audio-ninja
```

## Build from Source

### Release Build (Optimized)

```bash
cargo build --workspace --release

# Binaries in target/release/
ls -lh target/release/audio-ninja*
```

### Development Build

```bash
cargo build --workspace

# Binaries in target/debug/ (slower, better debugging)
```

## Install System-Wide (Linux)

```bash
# Install to /usr/local/bin
sudo install -m 755 target/release/audio-ninja-daemon /usr/local/bin/
sudo install -m 755 target/release/audio-ninja /usr/local/bin/

# Verify installation
which audio-ninja-daemon
audio-ninja-daemon --version
```

## Install as Systemd Service (Linux)

```bash
# Copy service file
sudo cp audio-ninja-daemon.service /etc/systemd/system/

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable audio-ninja-daemon
sudo systemctl start audio-ninja-daemon

# Check status
sudo systemctl status audio-ninja-daemon
```

## Install via Docker

```bash
# Build Docker image
docker build -t audio-ninja:latest .

# Run daemon container
docker run -d \
  --name audio-ninja \
  -p 8080:8080 \
  audio-ninja:latest
```

## Verify Installation

```bash
# Check daemon is running
curl http://127.0.0.1:8080/api/v1/status

# Should return:
# {"status":"running","version":"0.1.0","uptime_seconds":123}
```

## Next Steps

- **[Quick Start](/guide/quick-start.md)** - Get daemon running in 5 minutes
- **[Configuration](/guide/configuration.md)** - Configure for your environment
- **[Deployment](/deployment/daemon.md)** - Production deployment guide
