# CLI Terminal User Interface (TUI)

The Audio Ninja CLI includes an interactive Terminal User Interface (TUI) built with **Ratatui 0.28** for easy system control from the terminal.

## Quick Start

```bash
# Start the daemon (Terminal 1)
cargo run -p audio-ninja-daemon --release

# Launch the TUI (Terminal 2)
cargo run -p audio-ninja-cli --release -- tui

# Or use the binary
./target/release/audio-ninja tui
```

## Features

- **5 Interactive Screens**: Dashboard, Speakers, Layout, Transport, Calibration
- **Real-time Data**: Live daemon status and statistics
- **Keyboard Navigation**: Arrow keys, vim keybindings (hjkl), intuitive shortcuts
- **Professional UI**: Color-coded with cyan, green, yellow accents
- **Responsive**: 250ms event loop with immediate visual feedback

## Keyboard Controls

### Navigation

| Key | Action |
|-----|--------|
| `←` or `p` | Previous screen |
| `→` or `n` | Next screen |
| `↑` or `k` | Previous item |
| `↓` or `j` | Next item |

### Operations

| Key | Action |
|-----|--------|
| `r` | Refresh data |
| `d` | Discover speakers |
| `c` | Start calibration |
| `a` | Apply calibration |
| `p` | Play |
| `s` | Stop |

### Exit

| Key | Action |
|-----|--------|
| `q` or `Esc` | Quit |

## Screen Guide

### Dashboard

The main overview screen showing:
- **Daemon Status**: Current health, version, and uptime
- **Statistics**: System-wide metrics and audio information

### Speakers

Speaker management interface:
- List of connected speakers with details
- Speaker capabilities and status
- Press `[d]` to discover new speakers on the network

### Layout

Speaker layout configuration:
- Current layout display (stereo, 5.1, 7.1, etc.)
- Available presets for quick setup
- Visual speaker configuration

### Transport

Playback control and monitoring:
- Current playback status
- Play, pause, and stop controls
- Stream timing information

### Calibration

Room calibration status and controls:
- Current calibration state
- Measurement and solving progress
- Apply calibration filters to speakers

## Advanced Usage

### Custom Daemon URL

```bash
# Connect to remote daemon
audio-ninja --daemon http://192.168.1.100:8080 tui

# Or use hostname
audio-ninja --daemon http://audio-server:8080 tui
```

### Color Scheme

- **Cyan**: Borders and screen titles
- **Green**: Section headers and emphasis
- **Yellow**: Highlights and keyboard hints
- **White**: Default text
- **Red**: Error messages

## Troubleshooting

### TUI Won't Start

**Problem**: Connection refused or daemon not found

**Solution**: Ensure daemon is running on the specified address:
```bash
# Terminal 1
cargo run -p audio-ninja-daemon --release

# Terminal 2 (after daemon starts)
audio-ninja tui
```

### Garbled Display

**Problem**: Terminal colors or characters look wrong

**Solution**:
- Ensure terminal supports 256 colors
- Try setting terminal type: `TERM=xterm-256color audio-ninja tui`
- Check terminal window is at least 80x24 characters

### Keyboard Not Responding

**Problem**: Keys don't seem to work

**Solution**:
- Press `[q]` to quit and restart
- Make sure no other programs are capturing input
- Try a different terminal emulator

## Performance

- **Memory**: ~5-10 MB
- **CPU**: <1% idle usage
- **Refresh Rate**: 60 FPS capable (250ms input poll)
- **Network**: Non-blocking async API calls

## Architecture

The TUI is built with three main components:

### App State (`app.rs`)
- Manages current screen and navigation
- Holds API response data
- Tracks user selection and error messages

### UI Rendering (`ui.rs`)
- Draws the interface using Ratatui widgets
- Manages layout and constraints
- Renders per-screen content

### Input Handler (`handler.rs`)
- Polls keyboard events asynchronously
- Processes keybindings
- Updates application state

## Future Enhancements

Potential additions for future versions:
- Auto-refresh with configurable intervals
- Latency graphs and visualizations
- Level metering displays
- Interactive speaker configuration
- Mouse support
- Settings/preferences panel
- Real-time daemon logging
- Custom theme support

## Development

### Building

```bash
# Build the CLI with TUI support
cargo build -p audio-ninja-cli --release

# Run tests
cargo test -p audio-ninja-cli
```

### Testing

The TUI state machine has comprehensive tests:
```bash
# Run CLI tests
cargo test -p audio-ninja-cli

# Run with daemon for integration testing
cargo run -p audio-ninja-daemon --release &
cargo test -p audio-ninja-cli --test e2e_daemon_cli
```

### Contributing

When extending the TUI:
1. Keep screen components in separate functions
2. Add keyboard hints to the footer help text
3. Follow the color scheme for consistency
4. Test on minimal terminal sizes (80x24)
5. Use async/await patterns for I/O
6. Document new shortcuts in this guide

## See Also

- [Installation](./installation.md) - Setup guide
- [Quick Start](./quick-start.md) - Getting started
- [API Documentation](../api/endpoints.md) - REST API reference
