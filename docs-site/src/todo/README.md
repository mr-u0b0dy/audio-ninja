# Project Roadmap

**Last Updated**: January 2, 2026

Audio Ninja is a wireless immersive audio platform with IAMF support. This roadmap tracks development progress across three major phases.

## Quick Navigation

- [Current Status](#current-status)
- [Phase 1: Foundation](#phase-1-audio-io-architecture--complete) (✅ Complete)
- [Phase 2: GUI Enhancement](#phase-2-gui-refactoring--branding--in-progress) (🚧 In Progress)
- [Phase 3: Backend Implementation](#phase-3-backend-audio-io-implementation--planned) (⏳ Planned)
- [Future Work](#future-work)
- [Timeline](#timeline)

---

## Current Status

| Component | Status | Progress | Next Milestone |
|-----------|--------|----------|----------------|
| **Core Architecture** | ✅ Complete | 100% | Phase 2 GUI |
| **REST API** | ✅ Complete | 100% | Integration |
| **CLI/TUI** | ✅ Complete | 100% | Maintenance |
| **GUI** | 🚧 In Progress | 60% | Phase 2c |
| **Audio Backends** | ⏳ Planned | 0% | Phase 3 |

**Recent Achievements**:
- ✅ Fixed all CI/CD pipeline issues
- ✅ Documentation site deployed
- ✅ GUI Phase 2 planning complete
- ✅ 293 tests passing, 80%+ coverage
- ✅ 10 layout presets (2.0 through 9.1.6)
- ✅ 5 stats sub-endpoints (network, latency, daemon, sync, audio-levels)
- ✅ GUI toast notifications, keyboard shortcuts, drag-drop, localStorage
- ✅ WCAG accessibility pass (ARIA roles, focus-visible)
- ✅ CI code coverage job enabled
- ✅ OpenAPI spec updated with all I/O and stats endpoints

---

## Phase 1: Audio I/O Architecture ✅ COMPLETE

**Duration**: 8 weeks | **Effort**: 80 hours | **Completed**: December 2025

### Deliverables

<details>
<summary><strong>Core Implementation</strong> (10 items)</summary>

- ✅ Input source enumeration (System/App/External)
- ✅ Output device abstraction (Speaker/Headphones/USB/HDMI)
- ✅ InputManager with device selection
- ✅ OutputManager with device lifecycle
- ✅ PlaybackStream & CaptureStream traits
- ✅ REST API endpoints (7 total)
- ✅ Daemon integration
- ✅ CLI commands (input/output/transport)
- ✅ TUI screens with device panels
- ✅ Unit tests (18 tests)

</details>

<details>
<summary><strong>Quality Metrics</strong></summary>

- ✅ 293 tests passing
- ✅ 80%+ code coverage
- ✅ Clippy: 0 warnings
- ✅ Rustdoc: 0 errors
- ✅ CI/CD: All checks passing

</details>

**Outcome**: Production-ready architecture with complete test coverage and documentation.

---

## Phase 2: GUI Refactoring & Branding 🚧 IN PROGRESS

**Duration**: 5 weeks | **Effort**: 40-50 hours | **Status**: In Progress (~60%)

### Overview

Transform the GUI from functional prototype to professional audio workstation with:
- ✨ Magma Orange branding and professional logo
- 🎛️ Complete audio I/O controls
- 📊 Real-time visualization and stats
- 🎨 Responsive design and accessibility

### Sub-Phases

#### Phase 2a: Logo & Color Scheme ✅ MOSTLY COMPLETE
**Priority**: 🔴 HIGH | **Effort**: 5-7 hours | **Week**: 1

- ✅ Logo design complete (assets/logo.png)
- ✅ Magma Orange theme designed (WCAG AA verified)
- ✅ CSS component templates created
- ✅ CSS theme variables implemented
- ✅ Toast notifications, transitions, hover effects
- ✅ WCAG accessibility (ARIA roles, focus-visible)
- 🚧 Logo integration in header

**Details**: [GUI Phase 2 Tasks - Section 2a](gui-phase2-tasks.md#phase-2a-logo--color-scheme-priority-1)

#### Phase 2b: I/O & Transport Panel
**Priority**: 🔴 HIGH | **Effort**: 8-10 hours | **Week**: 2

- ✅ REST API endpoints ready
- ✅ Device selection dropdowns (wired to API)
- ✅ Audio source routing interface
- ✅ File loader with drag-drop support
- ✅ Transport mode selector with localStorage
- ✅ Error handling & toast feedback
- ✅ Keyboard shortcuts (Space, Escape, 1-7, Ctrl+O)

**Details**: [GUI Phase 2 Tasks - Section 2b](gui-phase2-tasks.md#phase-2b-io--transport-panel-priority-1)

#### Phase 2c: Visualization & Calibration
**Priority**: 🟡 MEDIUM | **Effort**: 8-10 hours | **Week**: 3

- 🚧 3D speaker layout canvas
- 🚧 Layout editor (drag-drop)
- 🚧 Calibration sweep controls
- 🚧 IR curve visualization
- 🚧 Filter design preview

**Details**: [GUI Phase 2 Tasks - Section 2c](gui-phase2-tasks.md#phase-2c-visualization--calibration-priority-2)

#### Phase 2d: Stats Dashboard & Polish
**Priority**: 🟡 MEDIUM | **Effort**: 10-12 hours | **Week**: 4-5

- ✅ Real-time metrics dashboard (wired to 5 stats endpoints)
- ✅ Audio level meters with peak hold
- ✅ Network bandwidth display
- ✅ Latency statistics display
- ✅ Daemon CPU/memory monitoring
- 🚧 Speaker status monitoring
- 🚧 Network bandwidth graphs (Chart.js)
- 🚧 Cross-platform testing
- 🚧 Responsive design optimization

**Details**: [GUI Phase 2 Tasks - Section 2d](gui-phase2-tasks.md#phase-2d-stats-dashboard--polish-priority-2)

### Success Criteria

- [ ] WCAG AA accessibility compliance
- [ ] <2 second startup time
- [ ] <100ms UI response time
- [ ] Works on Linux, macOS, Windows
- [ ] 50+ end-to-end tests
- [ ] Complete feature parity with CLI

---

## Phase 3: Backend Audio I/O Implementation ⏳ PLANNED

**Duration**: 3-4 months | **Effort**: 95-135 hours | **Start**: February 2026

### Overview

Implement real audio I/O with platform-specific backends and codec support.

### Components

#### 3.1 ALSA Bindings (Linux)
**Priority**: 🔴 HIGH | **Effort**: 20-30 hours

- ⏳ ALSA PCM device I/O
- ⏳ Device enumeration
- ⏳ Format negotiation
- ⏳ Integration tests

#### 3.2 PulseAudio Bindings
**Priority**: 🟡 MEDIUM | **Effort**: 15-20 hours

- ⏳ Sink/source abstractions
- ⏳ App-specific routing
- ⏳ Default device fallback
- ⏳ Integration tests

#### 3.3 CoreAudio Bindings (macOS)
**Priority**: 🟡 MEDIUM | **Effort**: 20-30 hours

- ⏳ HAL device abstractions
- ⏳ Device enumeration
- ⏳ Headphone/speaker detection
- ⏳ macOS testing

#### 3.4 FFmpeg Codec Support
**Priority**: 🟡 MEDIUM | **Effort**: 25-35 hours

- ⏳ Codec bindings (Opus, AAC, FLAC, AC-3, E-AC-3, TrueHD)
- ⏳ Streaming demux/decode
- ⏳ Format auto-detection
- ⏳ Performance profiling

#### 3.5 Testing & Optimization
**Priority**: 🟡 MEDIUM | **Effort**: 15-20 hours

- ⏳ Backend-specific unit tests
- ⏳ Real device integration tests
- ⏳ Latency profiling (<50ms target)
- ⏳ Cross-platform validation
- ⏳ 99% reliability verification

### Success Criteria

- [ ] All 4 backends functional (ALSA, PulseAudio, CoreAudio, FFmpeg)
- [ ] <50ms end-to-end latency
- [ ] 99% packet delivery reliability
- [ ] Support major audio formats
- [ ] Cross-platform tests on 3+ platforms
- [ ] Performance benchmarks documented

---

## Future Work

### High Priority (Post-Phase 3)
- 📋 libiamf/AOM reference decoder integration
- 📋 Dolby Atmos metadata parser
- 📋 WiFi Direct peer-to-peer mode
- 📋 Sample-accurate sync (±1ms)

### Medium Priority
- 📋 RTSP session management
- 📋 Adaptive bitrate streaming
- 📋 Firmware update mechanism
- 📋 Advanced calibration verification

### Community & Ecosystem
- 📋 C API for embedded integration
- 📋 Python bindings
- 📋 Example applications
- 📋 Video tutorials
- 📋 Community contribution guide

---

## Timeline

```
January 2026:
  ├─ Week 1-2: Phase 2a (Logo & Theme) [5-7h]
  ├─ Week 2-3: Phase 2b (I/O Controls) [8-10h]
  ├─ Week 3-4: Phase 2c (Visualization) [8-10h]
  └─ Week 4-5: Phase 2d (Stats & Polish) [10-12h]

February-March 2026:
  ├─ Phase 3.1: ALSA Bindings [20-30h]
  ├─ Phase 3.2: PulseAudio [15-20h] (parallel)
  ├─ Phase 3.3: CoreAudio [20-30h] (parallel)
  └─ Phase 3.4: FFmpeg [25-35h] (parallel)

April 2026:
  └─ Phase 3.5: Testing & Optimization [15-20h]

May 2026+:
  └─ Future work & backlog items
```

---

## Dependencies & Blockers

### Phase 2 (Current)
**Dependencies**: ✅ All satisfied
- ✅ REST API complete
- ✅ Design system ready
- ✅ Logo designed
- ✅ Daemon integration working

**Blockers**: None

### Phase 3 (Upcoming)
**Dependencies**: 
- 🚧 Phase 2 completion (optional, can start in parallel)
- ✅ Core architecture ready
- ✅ Trait abstractions defined

**Blockers**: 
- ⏳ Platform-specific build tools (alsa-dev, pulseaudio-dev, xcode)

---

## Resources

- **Detailed Tasks**: [GUI Phase 2 Tasks](gui-phase2-tasks.md)
- **Design System**: [Design Guide](../design/design-system.md)
- **API Reference**: [REST API Docs](../api/reference.md)
- **Backend Guide**: [Copilot Instructions](../../.github/copilot-instructions.md#phase-3-backend-audio-io-implementation-production-ready)

---

## Contributing

**Want to help?** Follow these steps:

1. Choose a task from the roadmap
2. Check dependencies are satisfied
3. Create a GitHub issue for tracking
4. Implement with comprehensive tests
5. Submit PR with documentation

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for detailed guidelines.

---

**Legend**: ✅ Complete | 🚧 In Progress | ⏳ Planned | 📋 Backlog | 🔴 High Priority | 🟡 Medium Priority

---

**Last Updated**: January 2, 2026  
**Next Review**: After Phase 2c completion
