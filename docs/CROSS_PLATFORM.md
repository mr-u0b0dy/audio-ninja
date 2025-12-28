# Cross-Platform Support Guide

Audio Ninja supports building on Linux, macOS, and Windows. This guide covers platform-specific setup, building, and deployment.

## Platform Support Matrix

| Platform | Architecture | Status | GUI | Daemon | CLI | Tests |
|----------|-------------|--------|-----|--------|-----|-------|
| Linux    | x86_64      | ✅ Fully Supported | ✅ | ✅ | ✅ | ✅ |
| Linux    | ARM64       | ✅ Fully Supported | ✅ | ✅ | ✅ | ✅ |
| macOS    | Apple Silicon (ARM64) | ✅ Fully Supported | ✅ | ✅ | ✅ | ✅ |
| macOS    | Intel (x86_64) | ✅ Fully Supported | ✅ | ✅ | ✅ | ✅ |
| Windows  | x86_64      | ✅ Fully Supported | ✅ | ✅ | ✅ | ✅ |
| Windows  | ARM64       | ⚠️ Experimental | ✅ | ✅ | ✅ | ✅ |

## Linux

### Prerequisites
```bash
# Ubuntu/Debian
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libasound2-dev \
    libpulse-dev

# Fedora/RHEL
sudo dnf install -y \
    gcc \
    pkg-config \
    gtk3-devel \
    webkit2gtk4-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    alsa-lib-devel \
    pulseaudio-libs-devel

# Arch
sudo pacman -S \
    base-devel \
    gtk3 \
    webkit2gtk \
    libappindicator-gtk3 \
    librsvg \
    alsa-lib \
    pulseaudio
```

### Build
```bash
cargo build --workspace --release
```

### Install
```bash
# Using make
make install

# Or manual installation
sudo cp target/release/audio-ninja-daemon /usr/local/bin/
sudo cp target/release/audio-ninja /usr/local/bin/
# GUI requires additional setup (see Tauri docs)
```

## macOS

### Prerequisites
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install --formulae \
    rust \
    gtk4 \
    libsoup \
    webkitgtk
```

### Build
```bash
cargo build --workspace --release
```

### Apple Silicon (ARM64) Notes
- Rust 1.70+ has full ARM64 support
- M1/M2/M3 Macs have native arm64-apple-darwin target
- Intel Macs use x86_64-apple-darwin target

### Universal Binaries (Intel + ARM64)
To build a universal binary supporting both architectures:

```bash
# Install both targets
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# Build both
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create universal binary
lipo -create \
    target/aarch64-apple-darwin/release/audio-ninja \
    target/x86_64-apple-darwin/release/audio-ninja \
    -output target/release/audio-ninja-universal
```

### App Distribution (macOS)
To create a `.app` bundle:

```bash
# The Tauri build process handles this
cargo tauri build --release

# Output: src-tauri/target/release/bundle/macos/
```

### Code Signing (Required for Distribution)
```bash
# Set your Apple developer identity
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (XXXXXXXXXX)"

# Tauri handles signing during build
cargo tauri build --release --sign-release
```

## Windows

### Prerequisites
- Windows 10 or later (x86_64 or ARM64)
- Visual Studio Build Tools 2019+ or Visual Studio Community
- Rust 1.70+ (from rustup.rs)

```powershell
# Using chocolatey
choco install visualstudio2022-workload-nativedesktop rustup

# Or manually from:
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
# https://rustup.rs/
```

### Build
```powershell
# In PowerShell or Command Prompt
cargo build --workspace --release
```

### Audio/GUI Dependencies on Windows
- WebView2: Windows 11 includes it; Windows 10 requires separate install
- Audio: Windows provides WASAPI natively (no external dependencies needed)
- GUI: Tauri handles WebView2 detection and prompts for installation

### MSVC Toolchain
Ensure MSVC is selected (usually the default):

```powershell
rustup default stable-msvc
rustup update
```

### Visual Studio Setup
If prompted, install "Desktop development with C++" workload:
1. Open Visual Studio Installer
2. Modify Visual Studio installation
3. Check "Desktop development with C++"
4. Install

## ARM64 (aarch64) Linux

### Prerequisites
For cross-compilation on x86_64 to aarch64:

```bash
# Install cross-compilation tools
sudo apt-get install -y \
    crossbuild-essential-arm64 \
    pkg-config

# Or use `cross` crate
cargo install cross
```

### Build
```bash
# Using cargo-cross (simpler)
cross build --target aarch64-unknown-linux-gnu --release

# Or native build on ARM64 hardware
cargo build --target aarch64-unknown-linux-gnu --release
```

### On Raspberry Pi / Other ARM64 Devices
```bash
# Install Rust on the device
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/mr-u0b0dy/audio-ninja.git
cd audio-ninja
cargo build --workspace --release

# Daemon starts ~10s (includes compilation on first run)
./target/release/audio-ninja-daemon
```

## Cross-Platform Testing

### Local Platform Testing
```bash
cargo test --workspace --release
```

### Docker Testing (Simulate Different Platforms)
```bash
# Test in Linux container
docker run --rm -v $(pwd):/workspace rust:latest \
    bash -c "cd /workspace && cargo test --workspace"

# Test macOS dependencies (requires macOS host)
# Windows testing requires Windows host
```

### CI/CD Matrix Testing
See `.github/workflows/ci.yml` for the full test matrix:
- Linux x86_64 and ARM64
- macOS x86_64 and ARM64
- Windows x86_64 (with WebView2 check)

## Troubleshooting

### macOS: Xcode Command Line Tools
```bash
xcode-select --install
```

### macOS: M1/M2 Homebrew
Ensure Homebrew is in PATH (Apple Silicon Macs):
```bash
export PATH="/opt/homebrew/bin:$PATH"
```

### Windows: MSVC Not Found
```powershell
# Reinstall MSVC
rustup toolchain remove stable
rustup toolchain install stable-msvc
rustup default stable-msvc
```

### Windows: WebView2 Installation
If GUI fails to start:
1. Download WebView2 runtime: https://go.microsoft.com/fwlink/p/?LinkId=2124703
2. Install the runtime
3. Restart your application

### Linux: GTK Version Mismatch
```bash
# Check GTK version
pkg-config --modversion gtk+-3.0

# If too old, use a newer distribution or build GTK from source
```

### ARM64: Compilation Failures
Some dependencies may not have ARM64 binaries. Solutions:
1. Use `cargo +nightly` with `-Z build-std=core,alloc`
2. Build on native ARM64 hardware instead of cross-compiling
3. Check dependency repository for ARM64 support

## Cross-Compilation Matrix

| From → To | Command | Notes |
|-----------|---------|-------|
| Linux x86_64 → Linux ARM64 | `cargo build --target aarch64-unknown-linux-gnu` | Use `cross` crate |
| macOS ARM64 → macOS x86_64 | `cargo build --target x86_64-apple-darwin` | Requires Xcode CLT |
| macOS x86_64 → macOS ARM64 | `cargo build --target aarch64-apple-darwin` | Requires Xcode CLT |
| Windows x86_64 → Windows ARM64 | `cargo build --target aarch64-pc-windows-msvc` | Experimental |
| Linux x86_64 → Windows x86_64 | `cargo build --target x86_64-pc-windows-gnu` | Not recommended (incomplete toolchain) |

## Distribution Formats

| Platform | Format | Command |
|----------|--------|---------|
| Linux | tar.gz with binaries | `make release` |
| macOS | .dmg (drag-drop installer) | `cargo tauri build --release` |
| Windows | .msi (MSI installer) | `cargo tauri build --release` |
| Universal | Docker image | See Dockerfile |

## Performance Tuning by Platform

### macOS: Rosetta 2 Overhead
Native ARM64 binaries are ~30-40% faster than x86_64 under Rosetta 2 emulation.

### Windows: Antivirus Scanning
Exclude the `target/` directory from real-time scanning to speed up builds.

### Linux: Build Cache
Use `sccache` to cache builds across projects:
```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

## Further Reading

- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [Tauri Platform Documentation](https://tauri.app/v1/guides/building/)
- [Cargo Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
