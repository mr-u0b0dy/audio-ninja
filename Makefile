.PHONY: help build test clean clippy fmt bench doc run-daemon run-gui run-cli install dev release check

# Default target
help:
	@echo "Audio Ninja Development Makefile"
	@echo ""
	@echo "Common tasks:"
	@echo "  make dev          - Quick development check (fmt + clippy + test)"
	@echo "  make build        - Build all workspace packages"
	@echo "  make test         - Run all tests"
	@echo "  make bench        - Build benchmarks (use 'make bench-run' to execute)"
	@echo "  make check        - Full CI-like check (fmt + clippy + test + doc)"
	@echo ""
	@echo "Code quality:"
	@echo "  make fmt          - Format code with rustfmt"
	@echo "  make clippy       - Run clippy lints"
	@echo "  make clippy-fix   - Auto-fix clippy warnings"
	@echo "  make doc          - Build documentation"
	@echo ""
	@echo "Running:"
	@echo "  make run-daemon   - Run the daemon service"
	@echo "  make run-gui      - Run the GUI client"
	@echo "  make run-cli CMD='status' - Run CLI command"
	@echo ""
	@echo "Release:"
	@echo "  make release      - Build optimized release binaries"
	@echo "  make install      - Install binaries to ~/.cargo/bin"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make clean-all    - Deep clean including cargo cache"

# Development workflow
dev: fmt clippy test
	@echo "✅ Development checks passed!"

# Build targets
build:
	cargo build --workspace

release:
	cargo build --workspace --release
	@echo "📦 Release binaries in target/release/"
	@ls -lh target/release/audio-ninja-daemon target/release/audio-ninja 2>/dev/null || true

# Test targets
test:
	cargo test --workspace

test-verbose:
	cargo test --workspace -- --nocapture

test-e2e:
	cargo test -p audio-ninja-cli --test e2e_daemon_cli

# Code quality
fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets

clippy-fix:
	cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

# Benchmarks
bench:
	cargo bench -p audio-ninja --bench main_benchmarks --no-run

bench-run:
	cargo bench -p audio-ninja --bench main_benchmarks

# Documentation
doc:
	cargo doc --workspace --no-deps --open

doc-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

# Running services
run-daemon:
	cargo run -p audio-ninja-daemon --release

run-daemon-dev:
	cargo run -p audio-ninja-daemon

run-gui:
	cargo run -p audio-ninja-gui --release

run-gui-dev:
	cargo run -p audio-ninja-gui

run-cli:
	cargo run -p audio-ninja-cli --release -- $(CMD)

# Installation
install:
	cargo install --path crates/daemon --force
	cargo install --path crates/cli --force
	@echo "✅ Installed to ~/.cargo/bin/"

# Full check (CI-like)
check: fmt-check clippy test doc-check
	@echo "✅ All checks passed!"

# Maintenance
clean:
	cargo clean
	rm -f Cargo.lock

clean-all: clean
	rm -rf ~/.cargo/registry/cache
	rm -rf ~/.cargo/git/db

# Size analysis
size:
	@echo "Binary sizes:"
	@du -h target/release/audio-ninja-daemon target/release/audio-ninja 2>/dev/null || echo "Run 'make release' first"
	@echo ""
	@echo "Target directory:"
	@du -sh target 2>/dev/null || echo "No build artifacts"

# Git helpers
tag:
	@read -p "Enter version (e.g., 0.1.0): " VERSION; \
	echo "Creating tag v$$VERSION..."; \
	git tag -a "v$$VERSION" -m "Release v$$VERSION"; \
	echo "Push with: git push origin v$$VERSION"

# Examples
examples:
	@echo "Running binaural rendering example..."
	cargo run --example binaural_rendering
	@echo ""
	@echo "Running loudness processing example..."
	cargo run --example loudness_processing

# Quick commands
.PHONY: d daemon g gui c cli
d daemon: run-daemon
g gui: run-gui
c cli: run-cli
