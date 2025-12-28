# API & Integration Reference

Complete API documentation and integration guides for Audio Ninja.

## REST API

- [API Usage](./api_usage.md) - Complete REST API reference with examples (cURL, Python, JavaScript)
- [Daemon Workflow](./daemon_workflow.md) - Service lifecycle, configuration, and client integration
- [Release Process](./release.md) - Version management and CI/CD pipeline

## Quick Start

The Audio Ninja daemon exposes a REST API on `http://127.0.0.1:8080/api/v1`.

```bash
# Start the daemon
cargo run -p audio-ninja-daemon --release

# Check status
curl http://127.0.0.1:8080/api/v1/status

# List speakers
curl http://127.0.0.1:8080/api/v1/speakers
```

## Integration Examples

See [API Usage](./api_usage.md) for complete examples in:
- cURL (command line)
- Python (with `requests`)
- JavaScript (Node.js and browser)
- WebSocket streaming

## Deployment

See [Daemon Workflow](./daemon_workflow.md) for:
- Systemd service setup
- Configuration management
- Logging and monitoring
- Production best practices
