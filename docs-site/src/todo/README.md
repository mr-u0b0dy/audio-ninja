# Project Roadmap & Todos

This section tracks the Audio Ninja project roadmap with tasks organized by phase and priority.

## Legend

- ✅ **Completed**: Task is done and deployed
- 🚧 **In Progress**: Currently being worked on
- ⏳ **Pending**: Ready to start, waiting for dependencies
- 🔄 **Blocked**: Waiting for external dependencies or decisions
- 📋 **Backlog**: Future work, lower priority

---

## Phase 1: Audio I/O Architecture ✅ COMPLETE

**Timeline**: 8 weeks | **Effort**: 80 hours | **Status**: Production-Ready

### Core Implementation
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
- ✅ Integration tests (end-to-end daemon ↔ CLI)

### Testing & Quality
- ✅ 276 tests passing
- ✅ 80%+ code coverage
- ✅ Clippy: 0 warnings
- ✅ All major modules tested

**Status**: ✅ **COMPLETE - Ready for Phase 2**

---

## Phase 2: GUI Refactoring & Branding 🚧 IN PROGRESS

**Timeline**: 5 weeks | **Effort**: 40-50 hours | **Status**: Planning & Design Complete

### Phase 2a: Logo & Color Scheme (1 week)
- ✅ Logo design complete (assets/logo.png)
- ✅ Magma Orange theme designed (WCAG AA verified)
- ✅ CSS component templates created
- 🚧 Copy logo to /crates/gui/icons/
- 🚧 Implement CSS variables for colors
- 🚧 Update button styling
- 🚧 Refactor panel backgrounds
- 🚧 Test contrast ratios

**Priority**: 🔴 **HIGH** | **Est. Hours**: 5-7 | **Blocker**: None

### Phase 2b: I/O & Transport Panel (2 weeks)
- ✅ API endpoints documented
- ✅ REST API ready for integration
- 🚧 Add I/O device panels to HTML
- 🚧 Fetch devices from REST API
- 🚧 Implement device selection UI
- 🚧 Add file picker integration
- 🚧 Implement transport controls (play/pause/stop)
- 🚧 Add mode selector (file/stream/mixed)
- 🚧 Error handling & user feedback

**Priority**: 🔴 **HIGH** | **Est. Hours**: 8-10 | **Blocker**: Phase 2a

### Phase 2c: Visualization & Calibration (2 weeks)
- ✅ Calibration architecture documented
- 🚧 Set up Canvas for layout visualization
- 🚧 Implement speaker positioning
- 🚧 Add layout preset configurations
- 🚧 Create calibration UI
- 🚧 IR curve visualization
- 🚧 Filter design preview (FIR/IIR)
- 🚧 VBAP test routing display

**Priority**: 🟡 **MEDIUM** | **Est. Hours**: 8-10 | **Blocker**: Phase 2b

### Phase 2d: Stats Dashboard & Polish (2 weeks)
- 🚧 Build real-time stats dashboard
- 🚧 Create speaker status table
- 🚧 Network bandwidth monitoring
- 🚧 CPU/memory usage graphs
- 🚧 Sync error visualization
- 🚧 Performance profiling
- 🚧 Cross-platform testing (Linux, macOS, Windows)
- 🚧 Responsive design optimization

**Priority**: 🟡 **MEDIUM** | **Est. Hours**: 10-12 | **Blocker**: Phase 2c

**Overall Phase 2 Status**: 🚧 **Ready to Start - All Dependencies Satisfied**

---

## Phase 3: Backend Audio I/O Implementation ⏳ PLANNED

**Timeline**: 3-4 months | **Effort**: 95-135 hours | **Status**: Detailed Planning Complete

### ALSA Bindings (Linux)
- ⏳ Implement ALSA PCM device I/O
- ⏳ Device enumeration via mixer/control
- ⏳ Format negotiation (sample rates, channels, bit depths)
- ⏳ Unit tests for ALSA backend
- ⏳ Integration tests with real devices

**Priority**: 🔴 **HIGH** | **Est. Hours**: 20-30 | **Blocker**: None

### PulseAudio Bindings (System Audio Routing)
- ⏳ Implement PulseAudio sink/source abstractions
- ⏳ App-specific routing with module-loopback
- ⏳ Default device fallback mechanism
- ⏳ Unit tests for PulseAudio backend
- ⏳ Integration tests

**Priority**: 🟡 **MEDIUM** | **Est. Hours**: 15-20 | **Blocker**: ALSA (can work in parallel)

### CoreAudio Bindings (macOS)
- ⏳ Implement HAL device abstractions
- ⏳ Input/output device enumeration
- ⏳ Support headphone and speaker detection
- ⏳ Unit tests for CoreAudio backend
- ⏳ macOS-specific testing

**Priority**: 🟡 **MEDIUM** | **Est. Hours**: 20-30 | **Blocker**: None (parallel)

### FFmpeg Codec Support
- ⏳ Add FFmpeg bindings (opus, aac, flac, ac-3, e-ac-3, truehd)
- ⏳ Streaming demux/decode with frame accuracy
- ⏳ Format auto-detection and negotiation
- ⏳ Codec integration tests
- ⏳ Performance profiling

**Priority**: 🟡 **MEDIUM** | **Est. Hours**: 25-35 | **Blocker**: None (parallel)

### Phase 3 Testing & Optimization
- ⏳ Unit tests per backend (ALSA, PulseAudio, CoreAudio)
- ⏳ Integration tests with real devices
- ⏳ Latency profiling and optimization
- ⏳ Cross-platform validation (Linux, macOS, Windows)
- ⏳ Performance targets: <50ms latency, 99% reliability

**Priority**: 🟡 **MEDIUM** | **Est. Hours**: 15-20 | **Blocker**: All backends

**Overall Phase 3 Status**: ⏳ **Planned - Ready to Start After Phase 2**

---

## Backlog & Future Work 📋

### Format Support & Codecs
- 📋 Integrate libiamf/AOM reference decoder (replace stubs)
- 📋 Dolby Atmos metadata parser for object positioning
- 📋 Support for additional codecs beyond current list
- 📋 Bitstream validation and conformance tests

### Extended Features
- 📋 WiFi Direct peer-to-peer mode
- 📋 RTSP session management
- 📋 Sample-accurate sync across speakers (±1ms target)
- 📋 Adaptive bitrate for varying network conditions
- 📋 Firmware update mechanism with rollback
- 📋 Advanced calibration verification loop

### SDK & Integration
- 📋 C API for embedded integration
- 📋 Python bindings for scripting
- 📋 Example applications (CLI player, GUI controller)
- 📋 Language-specific SDKs (Go, Ruby, PHP)

### Documentation
- 📋 Architecture decision records (ADRs)
- 📋 Performance benchmarking suite
- 📋 Video tutorials
- 📋 Community contribution guide
- 📋 Deployment best practices

---

## Priority Matrix

| Phase | Component | Priority | Effort | Status |
|-------|-----------|----------|--------|--------|
| 2 | Logo & Theme | 🔴 HIGH | 5-7h | 🚧 Ready |
| 2 | I/O Controls | 🔴 HIGH | 8-10h | 🚧 Ready |
| 2 | Visualization | 🟡 MEDIUM | 8-10h | 🚧 Ready |
| 2 | Stats Dashboard | 🟡 MEDIUM | 10-12h | 🚧 Ready |
| 3 | ALSA | 🔴 HIGH | 20-30h | ⏳ Planned |
| 3 | PulseAudio | 🟡 MEDIUM | 15-20h | ⏳ Planned |
| 3 | CoreAudio | 🟡 MEDIUM | 20-30h | ⏳ Planned |
| 3 | FFmpeg | 🟡 MEDIUM | 25-35h | ⏳ Planned |

---

## Timeline

```
Jan 2026:
  Week 1-2: Phase 2a (Logo & Theme)
  Week 2-3: Phase 2b (I/O & Transport)
  Week 3-4: Phase 2c (Visualization)
  Week 4-5: Phase 2d (Stats & Polish)

Feb-Mar 2026:
  Phase 3a: ALSA + PulseAudio (parallel)
  Phase 3b: CoreAudio + FFmpeg (parallel)
  Phase 3c: Testing & Optimization

Apr 2026+:
  Extended features & backlog items
```

---

## Dependencies & Blockers

### Phase 2 Dependencies
- ✅ REST API (Phase 1) - **READY**
- ✅ Design system & colors - **READY**
- ✅ Logo design - **READY**
- ✅ Daemon integration - **READY**

**Blockers**: None - Phase 2 can begin immediately

### Phase 3 Dependencies
- ✅ Phase 2 completion (can start in parallel)
- ✅ Core architecture (Phase 1) - **READY**
- ✅ Trait abstractions - **READY**
- 🚧 Platform-specific build tools (ALSA-dev, PulseAudio-dev)

**Blockers**: Build environment setup per platform

---

## Success Criteria

### Phase 2
- [ ] All 4 sub-phases (2a-2d) completed on schedule
- [ ] 100% feature parity with CLI
- [ ] WCAG AA accessibility achieved
- [ ] <2 second startup time
- [ ] <100ms UI response time
- [ ] Works on Linux, macOS, Windows
- [ ] 50+ end-to-end tests passing

### Phase 3
- [ ] All 4 backends (ALSA, PulseAudio, CoreAudio, FFmpeg) working
- [ ] <50ms end-to-end latency
- [ ] 99% packet delivery reliability
- [ ] Support all major audio formats
- [ ] Cross-platform tests on 3+ platforms
- [ ] Performance benchmarks documented

---

## Resources & References

- **Phase 2 Details**: [GUI Phase 2 Tasks](../design/phase2-tasks.md)
- **Design System**: [Design System Guide](../design/design-system.md)
- **API Reference**: [REST API Reference](../api/reference.md)
- **Backend Planning**: [Copilot Instructions](https://github.com/mr-u0b0dy/audio-ninja/blob/main/.github/copilot-instructions.md#phase-3-backend-audio-io-implementation-production-ready)

---

## Contributing

To help with any of these tasks:

1. **Choose a task** from the roadmap above
2. **Check dependencies** to ensure it's unblocked
3. **Create an issue** on GitHub for tracking
4. **Start implementation** with focus on testing
5. **Submit PR** with comprehensive tests and documentation

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for contribution guidelines.

---

**Last Updated**: January 1, 2026  
**Next Review**: After Phase 2a completion
