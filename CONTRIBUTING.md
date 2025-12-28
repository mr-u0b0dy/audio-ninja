# Contributing to Audio Ninja

Thank you for your interest in contributing to Audio Ninja! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check the [issue tracker](https://github.com/mr-u0b0dy/audio-ninja/issues) for existing reports
2. Verify the bug exists in the latest version

When filing a bug report, include:
- Clear, descriptive title
- Steps to reproduce the behavior
- Expected vs actual behavior
- Environment details (OS, Rust version, hardware)
- Relevant logs or error messages

### Suggesting Features

Feature requests are welcome! Please:
1. Check existing issues/discussions first
2. Provide clear use case and rationale
3. Consider implementation approach
4. Be open to discussion and feedback

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/mr-u0b0dy/audio-ninja.git
   cd audio-ninja
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-number-description
   ```

3. **Make Changes**
   - Follow the [coding style](#coding-style)
   - Add tests for new functionality
   - Update documentation as needed
   - Test in workspace context:
     ```bash
     # Build all crates
     cargo build --workspace
     
     # Run all tests
     cargo test --workspace
     
     # Check specific crate
     cargo check -p audio-ninja-daemon
     ```
   - Ensure all tests pass

4. **Commit**
   ```bash
   # Add SPDX header to new files
   
   # Run tests
   cargo test --workspace
   
   # Run linters and auto-fix
   cargo clippy --workspace --fix --allow-dirty
   cargo fmt --all
   
   # Commit with descriptive message
   git commit -m "feat: brief description"
   # or: fix:, docs:, refactor:, test:, chore:
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a pull request on GitHub.

### Pull Request Guidelines

- **Title**: Clear, concise description of changes
- **Description**: 
  - What problem does this solve?
  - How does it solve it?
  - Link related issues (`Fixes #123`)
  - List any breaking changes
- **Tests**: Include tests for new features/fixes
- **Documentation**: Update docs for user-facing changes
- **Commits**: Keep commits atomic and well-described
- **Size**: Prefer smaller, focused PRs over large changes

## Development Guidelines

### Coding Style

**Rust Style**
- Use Rust 2021 edition
- Follow `rustfmt` defaults (run `cargo fmt`)
- Address all `clippy` warnings (run `cargo clippy`)
- Prefer explicit types over inference in public APIs
- Use `anyhow` for application errors, `thiserror` for library errors

**Module Organization**
- Keep modules small and focused
- Use clear separation of concerns
- Maintain public API stability
- Document public items with `///` doc comments

**Workspace Structure**
- Core library code goes in `crates/core/src/`
- Daemon service code in `crates/daemon/src/`
- GUI client code in `crates/gui/src/`
- Shared dependencies defined in workspace root `Cargo.toml`
- Examples in `crates/core/examples/`
- Tests in `crates/core/tests/` (integration) or alongside code (unit)

**Naming Conventions**
- `snake_case` for functions, variables, modules
- `CamelCase` for types, traits, enums
- `SCREAMING_SNAKE_CASE` for constants
- Descriptive names over abbreviations

**Comments**
- Add comments for non-obvious logic
- Use doc comments (`///`) for public APIs
- Keep comments concise and up-to-date
- Prefer code clarity over excessive comments

**Testing**
- Unit tests in same file under `#[cfg(test)] mod tests`
- Integration tests in `tests/` directory
- Test edge cases and error conditions
- Use descriptive test names: `test_feature_behavior`

**Example**
```rust
// SPDX-License-Identifier: Apache-2.0

//! Module description
//!
//! Longer explanation of purpose and usage.

use std::collections::HashMap;
use anyhow::Result;

/// A spatial audio renderer using VBAP.
///
/// # Examples
///
/// ```
/// let renderer = VbapRenderer::new(speakers);
/// let gains = renderer.render(&source_position);
/// ```
pub struct VbapRenderer {
    speakers: Vec<Speaker>,
    triplets: Vec<Triplet>,
}

impl VbapRenderer {
    /// Creates a new VBAP renderer.
    pub fn new(speakers: Vec<Speaker>) -> Self {
        let triplets = Self::find_triplets(&speakers);
        Self { speakers, triplets }
    }
    
    /// Renders a source position to speaker gains.
    pub fn render(&self, source: &Vec3) -> Vec<f32> {
        // Implementation
    }
    
    fn find_triplets(speakers: &[Speaker]) -> Vec<Triplet> {
        // Helper method
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_renderer_creation() {
        let speakers = create_test_speakers();
        let renderer = VbapRenderer::new(speakers);
        assert_eq!(renderer.speaker_count(), 2);
    }
}
```

### File Headers

All source files must include the SPDX license identifier:

```rust
// SPDX-License-Identifier: Apache-2.0
```

### Architecture Guidelines

**Separation of Concerns**
- `iamf`: Parsing and decoding
- `render`: Spatial audio rendering (VBAP, HOA)
- `transport`: Network protocols (RTP, UDP)
- `control`: Speaker management (BLE, WiFi)
- `calibration`: Room measurement and correction
- `dsp`: Audio processing filters

**Transport Layer**
- Carry timestamps for synchronization
- Support PTP/NTP clock sync
- Handle packet loss gracefully

**Renderer**
- Support arbitrary layouts (2.0 to 9.1.6+)
- Implement downmix/upmix rules
- Maintain energy preservation

**Calibration**
- Measure → Solve → Apply workflow
- Export to standard DSP formats
- Support multiple measurement points

### Testing

**Test Coverage**
- Aim for high coverage of core functionality
- Test both success and failure paths
- Include edge cases and boundary conditions
- Test integration between modules

**Test Organization**
```
tests/
  ├── ble_tests.rs          # BLE control plane
  ├── calibration_tests.rs  # Room calibration
  ├── fec_tests.rs          # Forward error correction
  ├── hoa_tests.rs          # HOA decoder
  ├── iamf_tests.rs         # IAMF parsing/rendering
  ├── network_tests.rs      # Network transport
  ├── transport_tests.rs    # RTP, sync, jitter
  └── vbap_tests.rs         # VBAP renderer
```

**Running Tests**
```bash
# All tests
cargo test

# Specific module
cargo test vbap

# With output
cargo test -- --nocapture

# Single test
cargo test test_name
```

### Documentation

**Code Documentation**
- Public APIs must have doc comments
- Include examples in doc comments where helpful
- Document panics, errors, and safety concerns
- Keep docs synchronized with code

**Project Documentation**
- Update README.md for user-facing changes
- Add module-specific docs in `docs/`
- Include usage examples
- Document breaking changes

**Commit Messages**
```
Type: Brief description (50 chars or less)

More detailed explanation if needed. Wrap at 72 characters.
Explain what changed and why, not how (code shows how).

- Bullet points are okay
- Use present tense: "Add feature" not "Added feature"

Fixes #123
```

Common types: `feat`, `fix`, `docs`, `test`, `refactor`, `perf`, `chore`

## Project Structure

```
audio-ninja/
├── .github/
│   ├── workflows/          # CI/CD (planned)
│   └── copilot-instructions.md
├── docs/                   # Documentation
│   ├── vbap.md
│   ├── hoa.md
│   └── ...
├── src/
│   ├── lib.rs             # Library root
│   ├── iamf.rs            # IAMF parsing
│   ├── render.rs          # Rendering
│   ├── vbap.rs            # VBAP renderer
│   ├── hoa.rs             # HOA decoder
│   ├── transport.rs       # RTP transport
│   ├── network.rs         # UDP/networking
│   ├── sync.rs            # Clock sync
│   ├── jitter.rs          # Jitter buffer
│   ├── latency.rs         # Latency compensation
│   ├── fec.rs             # Forward error correction
│   ├── ble.rs             # BLE control
│   ├── calibration.rs     # Room calibration
│   ├── dsp.rs             # DSP filters
│   ├── dspconfig.rs       # DSP config export
│   ├── mapping.rs         # Channel mapping
│   ├── pipeline.rs        # Audio pipeline
│   ├── control.rs         # Control protocols
│   └── ffmpeg.rs          # Codec interfaces
├── tests/                 # Integration tests
├── Cargo.toml             # Dependencies
├── LICENSE                # Apache 2.0
├── README.md              # Project overview
├── CONTRIBUTING.md        # This file
└── CODE_OF_CONDUCT.md     # Code of conduct
```

## Review Process

1. **Automated Checks**
   - All tests must pass
   - Code must be formatted (`cargo fmt`)
   - No clippy warnings
   - Documentation builds without warnings

2. **Manual Review**
   - Code quality and style
   - Test coverage
   - Documentation completeness
   - Architecture fit

3. **Merge**
   - Squash small fixup commits
   - Use descriptive merge commit message
   - Update CHANGELOG (when established)

## Getting Help

- **Questions**: Use [GitHub Discussions](https://github.com/mr-u0b0dy/audio-ninja/discussions)
- **Issues**: Use [GitHub Issues](https://github.com/mr-u0b0dy/audio-ninja/issues)
- **Chat**: (Discord/Matrix/IRC - if established)

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

---

Thank you for contributing to Audio Ninja! 🥷
