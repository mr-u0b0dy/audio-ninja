#!/usr/bin/env bash
# Audio Ninja Development Setup Script

set -e

echo "🥷 Audio Ninja Development Setup"
echo "================================"
echo ""

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust $(rustc --version)"
fi

# Check Rust version
REQUIRED_VERSION="1.70"
RUST_VERSION=$(rustc --version | awk '{print $2}' | cut -d'.' -f1,2)
if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
    echo "⚠️  Rust version $RUST_VERSION found, but $REQUIRED_VERSION+ required"
    echo "   Run: rustup update"
fi

# Install required components
echo ""
echo "Installing Rust components..."
rustup component add rustfmt clippy

# Platform-specific dependencies
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo ""
    echo "Checking Linux dependencies..."
    
    if command -v apt-get &> /dev/null; then
        echo "Installing Debian/Ubuntu dependencies..."
        sudo apt-get update
        sudo apt-get install -y \
            libwebkit2gtk-4.0-dev \
            libgtk-3-dev \
            libssl-dev \
            pkg-config \
            build-essential
    elif command -v dnf &> /dev/null; then
        echo "Installing Fedora dependencies..."
        sudo dnf install -y \
            webkit2gtk4.0-devel \
            gtk3-devel \
            openssl-devel \
            gcc \
            gcc-c++
    elif command -v pacman &> /dev/null; then
        echo "Installing Arch dependencies..."
        sudo pacman -S --needed \
            webkit2gtk \
            gtk3 \
            openssl \
            base-devel
    else
        echo "⚠️  Unknown package manager. Please install manually:"
        echo "   - webkit2gtk-4.0"
        echo "   - gtk3"
        echo "   - openssl"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo ""
    echo "macOS detected. Checking dependencies..."
    if ! command -v brew &> /dev/null; then
        echo "⚠️  Homebrew not found. Install from https://brew.sh"
    else
        echo "Installing macOS dependencies..."
        brew install pkg-config openssl
    fi
fi

# Build the project
echo ""
echo "Building workspace..."
if cargo build --workspace; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Run tests
echo ""
echo "Running tests..."
if cargo test --workspace --quiet; then
    echo "✅ All tests passed"
else
    echo "❌ Some tests failed"
    exit 1
fi

# Configure git hooks
echo ""
echo "Configuring git hooks..."
if git config core.hooksPath .githooks; then
    echo "✅ Git hooks enabled (.githooks/pre-commit)"
    echo "   Pre-commit validation will run automatically"
    echo "   To skip: git commit --no-verify"
else
    echo "⚠️  Failed to configure git hooks"
fi

# Optional tools
echo ""
echo "Optional development tools:"
echo ""
echo "  cargo-watch: Auto-rebuild on file changes"
echo "    cargo install cargo-watch"
echo ""
echo "  cargo-expand: Expand macros for debugging"
echo "    cargo install cargo-expand"
echo ""
echo "  cargo-audit: Security vulnerability scanning"
echo "    cargo install cargo-audit"
echo ""

# Print next steps
echo "================================"
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Review README.md for architecture overview"
echo "  2. Run 'make help' to see available commands"
echo "  3. Start the daemon: make run-daemon"
echo "  4. In another terminal: make run-gui"
echo ""
echo "Quick start:"
echo "  make dev          # Format, lint, and test"
echo "  make run-daemon   # Start the audio engine"
echo "  make run-cli CMD='status'  # Check daemon status"
echo ""
echo "Happy hacking! 🥷"
