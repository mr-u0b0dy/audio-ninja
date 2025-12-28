# Firmware Update Guide

Over-the-air (OTA) firmware update system for Audio Ninja speakers.

## Overview

Audio Ninja supports automatic firmware updates for connected speakers:

- **Automatic Discovery**: Check for updates via mDNS or central server
- **Staged Rollout**: Version tracking and rollback capability
- **Network Resilience**: Retry logic and checksum verification
- **Zero-Downtime**: Dual-boot or staging partitions

## Check for Updates

```bash
# Manually check for updates
audio-ninja firmware check

# View available versions
audio-ninja firmware list-versions
```

## Update a Speaker

```bash
# Update single speaker
audio-ninja firmware update speaker-001

# Update all speakers
audio-ninja firmware update-all

# Force specific version
audio-ninja firmware update speaker-001 --version 0.2.0
```

## Monitor Update Progress

```bash
# Watch update progress
audio-ninja firmware watch speaker-001
```

Output:
```
Updating speaker-001 (Front Left)
  Downloading: ████████████░░░░░░░░░░░░░ 60%
  Verifying:   ████████████████████░░░░░░░ 100%
  Installing:  ██░░░░░░░░░░░░░░░░░░░░░░░░░ 5%
```

## Rollback

```bash
# Rollback to previous version
audio-ninja firmware rollback speaker-001

# Rollback all speakers
audio-ninja firmware rollback-all
```

## See Also

- [Release Process](/deployment/release.md)
- [Daemon Deployment](/deployment/daemon.md)
- [Configuration Guide](/guide/configuration.md)

---

📖 **[Full Firmware Update Guide](../../docs/firmware_update.md)**
