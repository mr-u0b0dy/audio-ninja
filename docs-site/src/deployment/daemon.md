# Daemon Deployment Guide

Complete guide for deploying and operating the Audio Ninja daemon in production.

## Quick Start

```bash
# Build release binary
cargo build -p audio-ninja-daemon --release

# Run daemon
./target/release/audio-ninja-daemon --bind 0.0.0.0 --port 8080 --verbose
```

## Installation

### System-Wide Installation (Linux)

```bash
sudo install -m 755 target/release/audio-ninja-daemon /usr/local/bin/

# Verify
audio-ninja-daemon --version
```

### Systemd Service

Copy `audio-ninja-daemon.service`:

```bash
sudo cp audio-ninja-daemon.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable audio-ninja-daemon
sudo systemctl start audio-ninja-daemon
```

## Configuration

### Command-Line Options

```bash
audio-ninja-daemon [OPTIONS]

OPTIONS:
  --bind <ADDRESS>      Bind address [default: 127.0.0.1]
  --port <PORT>         HTTP port [default: 8080]
  --verbose             Enable verbose logging
  --log-level <LEVEL>   Log level: trace, debug, info, warn, error
  --state-dir <PATH>    State directory [default: /var/lib/audio-ninja]
  --config <FILE>       Configuration file [default: /etc/audio-ninja/daemon.toml]
```

### Configuration File

Create `/etc/audio-ninja/daemon.toml`:

```toml
[server]
bind_address = "0.0.0.0"
port = 8080
verbose = true

[audio]
sample_rate = 48000
buffer_size = 1024
```

## Monitoring

### Check Status

```bash
# Via REST API
curl http://127.0.0.1:8080/api/v1/status

# Via systemd
sudo systemctl status audio-ninja-daemon

# View logs
sudo journalctl -u audio-ninja-daemon -f
```

## Security

- Bind to localhost by default (127.0.0.1)
- Use firewall rules to restrict access
- Run as non-root user
- Enable authentication for REST API (future)

## See Also

- [REST API Reference](/deployment/api.md)
- [CLI Tool Guide](/deployment/cli.md)
- [Configuration Guide](/guide/configuration.md)

---

📖 **[Full Daemon Documentation](../../docs/daemon_workflow.md)**
