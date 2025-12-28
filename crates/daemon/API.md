# Audio Ninja Daemon API Documentation

## OpenAPI Specification

See [openapi.yaml](openapi.yaml) for the complete OpenAPI 3.0 specification.

## Base URL

```
http://127.0.0.1:8080/api/v1
```

## Endpoints

### Status & Info

#### `GET /status`
Get daemon runtime status.

**Response:**
```json
{
  "status": "running",
  "version": "0.1.0",
  "uptime_secs": 3600
}
```

#### `GET /info`
Get daemon capabilities and features.

**Response:**
```json
{
  "name": "Audio Ninja Engine",
  "version": "0.1.0",
  "features": ["IAMF", "VBAP", "HOA", "RTP Transport", "Room Calibration"]
}
```

### Speaker Management

#### `GET /speakers`
List all discovered speakers.

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Left Speaker",
    "address": "192.168.1.100",
    "position": {
      "azimuth": -30.0,
      "elevation": 0.0,
      "distance": 2.0
    },
    "online": true
  }
]
```

#### `POST /speakers/discover`
Start speaker discovery on the network.

**Response:** `202 Accepted`

#### `GET /speakers/{id}`
Get information about a specific speaker.

**Parameters:**
- `id` (UUID): Speaker ID

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Left Speaker",
  "address": "192.168.1.100",
  "position": {
    "azimuth": -30.0,
    "elevation": 0.0,
    "distance": 2.0
  },
  "online": true
}
```

**Error:** `404 Not Found` if speaker doesn't exist

#### `DELETE /speakers/{id}`
Remove a speaker from the system.

**Parameters:**
- `id` (UUID): Speaker ID

**Response:** `204 No Content`

**Error:** `404 Not Found` if speaker doesn't exist

#### `GET /speakers/{id}/stats`
Get statistics for a specific speaker.

**Parameters:**
- `id` (UUID): Speaker ID

**Response:**
```json
{
  "packets_sent": 10000,
  "packets_lost": 5,
  "latency_ms": 12.5,
  "jitter_ms": 2.1,
  "buffer_fill": 0.75
}
```

**Error:** `404 Not Found` if speaker doesn't exist

### Layout Configuration

#### `GET /layout`
Get current speaker layout.

**Response:**
```json
{
  "speakers": [
    {
      "azimuth": -30.0,
      "elevation": 0.0,
      "distance": 2.0
    },
    {
      "azimuth": 30.0,
      "elevation": 0.0,
      "distance": 2.0
    }
  ]
}
```

**Error:** `404 Not Found` if no layout is configured

#### `POST /layout`
Set speaker layout from preset or custom positions.

**Request Body:**
```json
{
  "preset": "stereo"
}
```

Supported presets: `stereo`, `5.1`, `7.1`, `7.1.4`, `9.1.6`

Or custom layout:
```json
{
  "speakers": [
    {
      "speaker_id": "550e8400-e29b-41d4-a716-446655440000",
      "azimuth": -30.0,
      "elevation": 0.0,
      "distance": 2.0
    }
  ]
}
```

**Response:** `200 OK`

**Error:** `400 Bad Request` for invalid preset

### Transport Control

#### `POST /transport/play`
Start audio playback.

**Response:** `200 OK`

#### `POST /transport/pause`
Pause audio playback.

**Response:** `200 OK`

#### `POST /transport/stop`
Stop audio playback.

**Response:** `200 OK`

#### `GET /transport/status`
Get transport state.

**Response:**
```json
{
  "state": "Playing"
}
```

States: `Stopped`, `Playing`, `Paused`

### Calibration

#### `POST /calibration/start`
Start room calibration process.

**Response:** `202 Accepted`

#### `GET /calibration/status`
Get calibration progress.

**Response:**
```json
{
  "running": true,
  "progress": 0.45,
  "measurements": 3
}
```

#### `POST /calibration/apply`
Apply calibration results.

**Response:** `501 Not Implemented` (planned feature)

### Statistics

#### `GET /stats`
Get system-wide statistics.

**Response:**
```json
{
  "total_speakers": 6,
  "online_speakers": 5,
  "transport_state": "Playing",
  "has_layout": true
}
```

## Error Responses

All endpoints may return standard HTTP error codes:

- `400 Bad Request`: Invalid request parameters
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error
- `501 Not Implemented`: Feature not yet implemented

Error response format:
```json
{
  "error": "Error description"
}
```

## CORS

The API includes CORS headers allowing requests from any origin during development.

## Content Type

All requests and responses use `application/json` unless otherwise specified.

## Examples

### Using curl

```bash
# Get daemon status
curl http://127.0.0.1:8080/api/v1/status

# Discover speakers
curl -X POST http://127.0.0.1:8080/api/v1/speakers/discover

# List speakers
curl http://127.0.0.1:8080/api/v1/speakers

# Set stereo layout
curl -X POST http://127.0.0.1:8080/api/v1/layout \
  -H "Content-Type: application/json" \
  -d '{"preset": "stereo"}'

# Start playback
curl -X POST http://127.0.0.1:8080/api/v1/transport/play

# Get statistics
curl http://127.0.0.1:8080/api/v1/stats
```

### Using the CLI

```bash
audio-ninja status
audio-ninja speaker discover
audio-ninja speaker list
audio-ninja layout set stereo
audio-ninja transport play
audio-ninja stats
```

### Using JavaScript/fetch

```javascript
// Get daemon status
const response = await fetch('http://127.0.0.1:8080/api/v1/status');
const status = await response.json();
console.log(status);

// Set layout
await fetch('http://127.0.0.1:8080/api/v1/layout', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ preset: 'stereo' })
});

// Start playback
await fetch('http://127.0.0.1:8080/api/v1/transport/play', {
  method: 'POST'
});
```
