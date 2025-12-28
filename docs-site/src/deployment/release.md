# Release Process

How Audio Ninja versions are released and managed.

## Version Numbering

Audio Ninja follows [Semantic Versioning](https://semver.org/):

- **Major (X.0.0)**: Breaking API changes
- **Minor (0.X.0)**: New features (backward compatible)
- **Patch (0.0.X)**: Bug fixes

## Release Workflow

```
1. Update Cargo.toml version
2. Update CHANGELOG.md
3. Commit: "chore: bump version to X.Y.Z"
4. Create tag: git tag vX.Y.Z
5. Push tag to GitHub
6. GitHub Actions builds and publishes
```

## Check Latest Release

```bash
# List all releases
audio-ninja releases list

# Get release notes
audio-ninja releases info v0.1.0
```

## Download Release Binaries

Releases are available on [GitHub Releases](https://github.com/mr-u0b0dy/audio-ninja/releases):

- `audio-ninja-daemon-linux-{arch}` - Daemon binary
- `audio-ninja-cli-linux-{arch}` - CLI tool
- `audio-ninja-gui-linux-{arch}` - GUI application

## Release History

See [CHANGELOG](../../CHANGELOG.md) for complete release history.

## Support Policy

- **Latest major version**: Full support
- **Previous major version**: Security fixes only (6 months)
- **Older versions**: No support

## See Also

- [Firmware Update Guide](/deployment/firmware.md)
- [Daemon Deployment](/deployment/daemon.md)
- [GitHub Repository](https://github.com/mr-u0b0dy/audio-ninja)

---

📖 **[Full Release Documentation](../../docs/release.md)**
