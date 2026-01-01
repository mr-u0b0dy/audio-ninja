# Audio Ninja CLI

Command-line interface for controlling the Audio Ninja daemon.

## Installation

```bash
cargo install --path crates/cli
```

Or build from source:

```bash
cargo build -p audio-ninja-cli --release
# Binary will be at: target/release/audio-ninja
```

## Usage

The CLI communicates with the Audio Ninja daemon via its REST API.

### Basic Commands

```bash
# Show daemon status
audio-ninja status

# Show daemon information
audio-ninja info

# Show system statistics
audio-ninja stats
```

### Speaker Management

```bash
# List all speakers
audio-ninja speaker list

# Discover speakers on the network
audio-ninja speaker discover

# Get information about a specific speaker
audio-ninja speaker get <UUID>

# Remove a speaker
audio-ninja speaker remove <UUID>

# Show speaker statistics
audio-ninja speaker stats <UUID>
```

### Layout Configuration

```bash
# Show current layout
audio-ninja layout get

# Set layout from preset
audio-ninja layout set stereo
audio-ninja layout set 5.1
audio-ninja layout set 7.1
```

### Transport Control

```bash
# Start playback
audio-ninja transport play

# Pause playback
audio-ninja transport pause

# Stop playback
audio-ninja transport stop

# Show transport status
audio-ninja transport status
```

### Calibration

```bash
# Start calibration
audio-ninja calibration start

# Show calibration status
audio-ninja calibration status

# Apply calibration results
audio-ninja calibration apply
```

## Configuration

### Daemon URL

By default, the CLI connects to `http://127.0.0.1:8080`. You can override this:

```bash
audio-ninja --daemon http://192.168.1.100:8080 status
```

Or set an environment variable:

```bash
export AUDIO_NINJA_DAEMON=http://192.168.1.100:8080
audio-ninja status
```

## Examples

### Complete Workflow

```bash
# Check daemon status
audio-ninja status

# Discover speakers
audio-ninja speaker discover

# Wait a few seconds, then list discovered speakers
audio-ninja speaker list

# Set stereo layout
audio-ninja layout set stereo

# Start playback
audio-ninja transport play

# Check stats
audio-ninja stats
```

### Scripting

The CLI outputs JSON for easy parsing:

```bash
# Get number of online speakers
audio-ninja stats | jq '.online_speakers'

# List speaker names
audio-ninja speaker list | jq '.[].name'

# Check if transport is playing
audio-ninja transport status | jq '.state'
```

## Help

Use `--help` with any command for more information:

```bash
audio-ninja --help
audio-ninja speaker --help
audio-ninja transport --help
```

## Future Plans

### Phase 2: Enhanced TUI/Commands (5 weeks)
Expanding CLI capabilities to match GUI feature set:

- **I/O Management**: `audio-ninja input list/select`, `audio-ninja output list/select`
- **Advanced Transport**: File loading, streaming mode selection, progress tracking
- **Real-Time Stats**: Enhanced stats screen with latency, packet loss, sync metrics
- **Layout Visualization**: ASCII art layout preview in TUI
- **Calibration Control**: Sweep generation, IR analysis commands

See [tui-guide.md](../../docs-site/tui-guide.md) for interactive terminal UI design.

### Phase 3: Audio Backend Commands (3-4 months)
Phase 3 backend work will add new commands for:

- Device capability reporting (formats, sample rates, latencies)
- Backend selection (ALSA, PulseAudio, CoreAudio, etc.)
- Real-time audio metrics (CPU usage, buffer underruns)
- Codec/format testing and validation

All new commands will follow the existing CLI pattern with JSON output for scripting.
