FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Install audio-ninja specific dependencies
RUN apt-get update && apt-get install -y \
    # GUI (Tauri)
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    # Audio (ALSA, PulseAudio)
    libasound2-dev \
    libpulse-dev \
    # Additional tools
    pkg-config \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install additional Rust tools for development
RUN rustup component add clippy rustfmt && \
    cargo install cargo-watch cargo-expand cargo-edit cargo-audit --locked

# Set default shell
ENV SHELL=/bin/bash
