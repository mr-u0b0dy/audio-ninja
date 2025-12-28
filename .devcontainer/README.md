# DevContainer Setup for Audio Ninja

This project includes a DevContainer configuration for VS Code that provides a fully containerized development environment with all dependencies pre-configured.

## Quick Start

### Prerequisites
- VS Code with [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- Docker (typically bundled with Docker Desktop)

### Opening the Project in a Container

1. **Clone the repository:**
   ```bash
   git clone https://github.com/mr-u0b0dy/audio-ninja.git
   cd audio-ninja
   ```

2. **Open in VS Code:**
   ```bash
   code .
   ```

3. **Reopen in Container:**
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
   - Select "Dev Containers: Reopen in Container"
   - Wait for the container to build and initialize (~2-3 minutes on first run)

### What's Included

- **Rust toolchain** (latest stable with clippy, rustfmt)
- **GUI dependencies** (GTK 3, WebKit 2, AppIndicator)
- **Audio libraries** (ALSA, PulseAudio)
- **Development tools** (cargo-watch, cargo-expand, cargo-edit, cargo-audit)
- **VS Code extensions**:
  - rust-analyzer (with advanced inlay hints)
  - LLDB debugger
  - Even Better TOML
  - Crates.io helper
  - GitLens

## Common Workflows

### Build in Container
```bash
make build
```

### Run Tests
```bash
make test
```

### Run Daemon Service
```bash
make run-daemon
```
The daemon will be accessible on `localhost:8080` from your host machine.

### Run GUI Client
```bash
make run-gui
```

### Run CLI Tool
```bash
make run-cli
```

### Format & Lint
```bash
make format
cargo clippy --workspace --fix
```

### Run Benchmarks
```bash
make bench
```

## File Structure

```
.devcontainer/
├── devcontainer.json      # VS Code configuration
├── Dockerfile             # Custom container image with all dependencies
└── README.md             # This file
```

## Customization

### Modifying the Container

Edit `.devcontainer/devcontainer.json` or `.devcontainer/Dockerfile` and rebuild:

1. Press `Ctrl+Shift+P` and select "Dev Containers: Rebuild Container"
2. Wait for the rebuild to complete

### Adding Extensions

To add VS Code extensions, edit the `extensions` array in `devcontainer.json`:

```json
"extensions": [
  "rust-lang.rust-analyzer",
  "your.new-extension-id"
]
```

## Port Forwarding

The daemon API port (8080) is automatically forwarded to your host. You can access it at `http://localhost:8080` when the container is running.

## Performance Tips

- **Mount Cargo cache**: The `.devcontainer/devcontainer.json` mounts your host's `~/.cargo` directory to speed up builds and avoid re-downloading dependencies.
- **Use cargo-watch**: Install with `cargo install cargo-watch` and run `cargo watch -x 'test --lib'` for continuous testing during development.

## Troubleshooting

### Container Build Fails
- Ensure Docker is running and has sufficient resources (4GB+ RAM recommended)
- Delete the container image and rebuild: `Dev Containers: Rebuild Container`

### Slow First Build
- First build includes compiling all dependencies (~5-10 minutes depending on machine)
- Subsequent builds are much faster due to caching

### Audio/GUI Issues in Container
- GUI rendering requires X11 forwarding or Wayland support (not needed for daemon/CLI/tests)
- For audio development: Focus on CLI/daemon/tests in container; run GUI locally if needed

### Extensions Not Loading
- Rebuild the container: `Dev Containers: Rebuild Container`
- Ensure extensions are compatible with Rust development

## Building Container Image Locally

To manually build the container image:

```bash
docker build -f .devcontainer/Dockerfile -t audio-ninja-dev .
docker run -it --rm -v $(pwd):/workspace audio-ninja-dev bash
```

## Further Reading

- [VS Code Dev Containers Documentation](https://code.visualstudio.com/docs/devcontainers/containers)
- [Dev Containers Specification](https://containers.dev/)
- [Audio Ninja README](../README.md)
- [Development Guide](../CONTRIBUTING.md)
