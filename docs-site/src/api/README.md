# REST API Documentation

Complete REST API reference, usage examples, and integration guides for Audio Ninja daemon.

## Contents

- **[API Reference](reference.md)** - Complete endpoint documentation
- **[API Usage Examples](api_usage.md)** - curl, Python, JavaScript, and Rust examples
- **[Daemon Workflow](daemon_workflow.md)** - Deployment, configuration, and integration
- **[Release Process](release.md)** - Version management and release procedures

## Quick Start

```bash
# Start daemon
cargo run -p audio-ninja-daemon --release

# Test API
curl http://127.0.0.1:8080/api/v1/status

# Discover speakers
curl -X POST http://127.0.0.1:8080/api/v1/speakers/discover
```

## API Endpoints

### Status & Info
- `GET /api/v1/status` - Daemon status
- `GET /api/v1/info` - Daemon information
- `GET /api/v1/stats` - System statistics

### Speaker Management
- `GET /api/v1/speakers` - List speakers
- `POST /api/v1/speakers/discover` - Discover speakers
- `GET /api/v1/speakers/{uuid}` - Get speaker info
- `DELETE /api/v1/speakers/{uuid}` - Remove speaker

### I/O Management
- `GET /api/v1/input/devices` - List input devices
- `POST /api/v1/input/select` - Select input source
- `GET /api/v1/output/devices` - List output devices
- `POST /api/v1/output/select` - Select output device

### Transport Control
- `POST /api/v1/transport/load-file` - Load file
- `POST /api/v1/transport/play` - Start playback
- `POST /api/v1/transport/pause` - Pause playback
- `POST /api/v1/transport/stop` - Stop playback
- `GET /api/v1/transport/status` - Get playback status

For complete details, see [API Reference](reference.md).
