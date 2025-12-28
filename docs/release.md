# Release Process

This document describes the release process for Audio Ninja.

## Version Numbering

Audio Ninja follows [Semantic Versioning](https://semver.org/):
- **Major (X.0.0)**: Breaking API changes
- **Minor (0.X.0)**: New features, backward compatible
- **Patch (0.0.X)**: Bug fixes, backward compatible

## Automated Release Process

Releases are automated via GitHub Actions. To create a new release:

### 1. Update Version Numbers

Update version in workspace `Cargo.toml`:

```bash
# Edit Cargo.toml
[workspace.package]
version = "0.2.0"  # Update this line
```

### 2. Update CHANGELOG

Document changes in `CHANGELOG.md`:

```markdown
## [0.2.0] - 2025-01-15

### Added
- Feature X
- Feature Y

### Fixed
- Bug Z
```

### 3. Commit and Tag

```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

### 4. Automated Build

The release workflow will automatically:
1. Create a GitHub release
2. Build binaries for:
   - `x86_64-unknown-linux-gnu` (Intel/AMD Linux)
   - `aarch64-unknown-linux-gnu` (ARM64 Linux)
3. Upload assets:
   - `audio-ninja-daemon-linux-{arch}`
   - `audio-ninja-cli-linux-{arch}`
   - Tarballs with both binaries
   - SHA256 checksums

### 5. Release Notes

Edit the GitHub release to add:
- High-level summary
- Breaking changes (if any)
- Migration guide (if needed)
- Known issues

## Manual Release (Local Build)

If you need to build binaries locally:

### Prerequisites

```bash
# Install cross-compilation tools
sudo apt-get install gcc-aarch64-linux-gnu

# Add Rust targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
```

### Build Commands

```bash
# x86_64 Linux
cargo build --release --target x86_64-unknown-linux-gnu -p audio-ninja-daemon
cargo build --release --target x86_64-unknown-linux-gnu -p audio-ninja-cli

# ARM64 Linux
cargo build --release --target aarch64-unknown-linux-gnu -p audio-ninja-daemon
cargo build --release --target aarch64-unknown-linux-gnu -p audio-ninja-cli
```

### Create Release Archive

```bash
VERSION="0.2.0"
for TARGET in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu; do
  cd target/$TARGET/release
  tar -czf audio-ninja-$TARGET-$VERSION.tar.gz \
    audio-ninja-daemon \
    audio-ninja
  sha256sum audio-ninja-$TARGET-$VERSION.tar.gz >> checksums.txt
  cd -
done
```

## Pre-release Checklist

Before creating a release, ensure:

- [ ] All tests pass: `cargo test --workspace`
- [ ] No clippy warnings: `cargo clippy --workspace -- -D warnings`
- [ ] Code is formatted: `cargo fmt --all -- --check`
- [ ] Benchmarks build: `cargo bench --no-run`
- [ ] Documentation builds: `cargo doc --workspace --no-deps`
- [ ] CHANGELOG is updated
- [ ] Version numbers are consistent across workspace
- [ ] README reflects current features
- [ ] Examples work: `cargo run --example binaural_rendering`

## Post-release Tasks

After release:

1. Announce on:
   - GitHub Discussions
   - Project website/blog
   - Social media channels

2. Update documentation sites

3. Monitor for issues in the new release

## Hotfix Process

For urgent bug fixes:

1. Create a hotfix branch from the release tag:
   ```bash
   git checkout -b hotfix/0.2.1 v0.2.0
   ```

2. Make the fix and test thoroughly

3. Update version to next patch (0.2.1)

4. Commit, tag, and push:
   ```bash
   git commit -m "fix: critical bug in transport"
   git tag v0.2.1
   git push origin hotfix/0.2.1
   git push origin v0.2.1
   ```

5. Merge back to main:
   ```bash
   git checkout main
   git merge hotfix/0.2.1
   git push origin main
   ```

## Release Schedule

- **Major releases**: As needed for breaking changes
- **Minor releases**: Every 1-2 months with new features
- **Patch releases**: As needed for critical bugs

## Support Policy

- **Latest major version**: Full support
- **Previous major version**: Security fixes only for 6 months
- **Older versions**: No support (upgrade recommended)

## See Also

- [CHANGELOG](../CHANGELOG.md) - Project version history and changes
- [Daemon Workflow](daemon_workflow.md) - Deployment and operation
- [Calibration](calibration.md) - Room calibration versioning
