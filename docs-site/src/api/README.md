# REST API Documentation

Complete REST API reference, usage examples, and integration guides for Audio Ninja daemon.

## Contents

- **[API Reference](reference.md)** - Complete endpoint documentation with request/response formats
- **[API Usage Examples](api_usage.md)** - curl, Python, JavaScript, and Rust client examples
- **[Daemon Workflow](daemon_workflow.md)** - Deployment, configuration, monitoring, and integration
- **[Release Process](release.md)** - Version management and release procedures

## Quick Start

### Start the Daemon

```bash
# Run in development mode with verbose logging
cargo run -p audio-ninja-daemon --release -- --verbose

# Or install as system service (Linux)
sudo systemctl start audio-ninja-daemon
```

### Test the API

```bash
# Check daemon status
curl http://127.0.0.1:8080/api/v1/status

# Get daemon info
curl http://127.0.0.1:8080/api/v1/info

# Discover speakers on network
curl -X POST http://127.0.0.1:8080/api/v1/speakers/discover

# List discovered speakers
curl http://127.0.0.1:8080/api/v1/speakers
```

## API Overview

The Audio Ninja REST API provides complete control over the audio engine:

### Core Resources

- **Status & Info**: Daemon health, version, and system stats
- **Speakers**: Device discovery, listing, configuration
- **Layout**: Speaker arrangement and configuration
- **Transport**: Playback control and file management
- **I/O**: Input/output device management
- **Calibration**: Room measurement and optimization

## Endpoint Categories

### Status & Info (`GET`)
| Endpoint | Purpose |
|----------|---------|
| `/api/v1/status` | Current daemon status and health |
| `/api/v1/info` | Daemon version and capabilities |
| `/api/v1/stats` | System metrics (CPU, memory, latency) |

### Speaker Management
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/v1/speakers` | GET | List discovered speakers |
| `/api/v1/speakers/discover` | POST | Scan network for speakers |
| `/api/v1/speakers/{uuid}` | GET | Get speaker configuration |
| `/api/v1/speakers/{uuid}` | DELETE | Remove speaker |

### Layout & Configuration
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/v1/layout/get` | GET | Current speaker layout |
| `/api/v1/layout/set` | POST | Configure layout from preset or custom |

### Transport & Playback
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/v1/transport/load-file` | POST | Load audio file for playback |
| `/api/v1/transport/play` | POST | Start playback |
| `/api/v1/transport/pause` | POST | Pause playback |
| `/api/v1/transport/stop` | POST | Stop and reset playback |
| `/api/v1/transport/status` | GET | Current playback state |

### I/O Management
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/v1/input/devices` | GET | List input devices and sources |
| `/api/v1/input/select` | POST | Select active input source |
| `/api/v1/output/devices` | GET | List output devices |
| `/api/v1/output/select` | POST | Select active output device |

## Response Format

All responses are JSON with the following structure:

```json
{
  "status": "ok",
  "data": { },
  "timestamp": "2025-01-01T12:00:00Z"
}
```

### Success Response
```json
{
  "status": "ok",
  "data": {
    "version": "0.2.0",
    "uptime_secs": 3600,
    "speakers": 2
  },
  "timestamp": "2025-01-01T12:00:00Z"
}
```

### Error Response
```json
{
  "status": "error",
  "error": "Speaker not found",
  "code": "SPEAKER_NOT_FOUND",
  "timestamp": "2025-01-01T12:00:00Z"
}
```

## Authentication

The daemon API on localhost (`127.0.0.1:8080`) is **not authenticated** by default.

For production deployments over a network, use:
- Reverse proxy with authentication (nginx, caddy)
- VPN or private network isolation
- Firewall rules to restrict access

See [Daemon Workflow](daemon_workflow.md#security) for production security setup.

## Rate Limiting

No rate limiting is currently enforced. For high-frequency requests:
- Buffer requests on the client side
- Respect a 100ms minimum interval between state changes
- Use polling with reasonable intervals (1-5 seconds recommended)

## Versioning

The API follows semantic versioning:
- Current version: **v1**
- Backward compatibility: Maintained within v1
- Breaking changes: Increment to v2+ (will exist alongside v1)

All endpoints are prefixed with `/api/v1/`.

## See Also

- [Complete API Reference](reference.md) - Full endpoint documentation
- [API Usage Examples](api_usage.md) - Real-world client examples
- [Daemon Workflow Guide](daemon_workflow.md) - Deployment and operation
- [CLI Tool](../guide/cli-tui.md) - Command-line interface
