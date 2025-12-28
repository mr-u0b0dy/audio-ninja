# REST API Reference

Complete REST API documentation for Audio Ninja daemon.

## Base URL

```
http://127.0.0.1:8080/api/v1
```

## Status Endpoints

### Get Daemon Status

```bash
curl http://127.0.0.1:8080/api/v1/status
```

Response:
```json
{
  "status": "running",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
```

## Speaker Management

### List Speakers

```bash
curl http://127.0.0.1:8080/api/v1/speakers
```

### Register Speaker

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

## Audio Control

### Start Playback

```bash
curl -X POST http://127.0.0.1:8080/api/v1/playback/start \
  -H "Content-Type: application/json" \
  -d '{
    "source": "file",
    "path": "/path/to/audio.wav"
  }'
```

### Stop Playback

```bash
curl -X POST http://127.0.0.1:8080/api/v1/playback/stop
```

## Error Handling

All endpoints return standard HTTP status codes:

- `200 OK` - Success
- `400 Bad Request` - Invalid parameters
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

Error response:
```json
{
  "error": "speaker_not_found",
  "message": "Speaker with ID 'speaker-999' does not exist",
  "code": 404
}
```

## See Also

- [Daemon Deployment](/deployment/daemon.md)
- [CLI Tool](/deployment/cli.md)
- [Configuration Guide](/guide/configuration.md)

---

📖 **[Full API Documentation](../../docs/api_usage.md)**
