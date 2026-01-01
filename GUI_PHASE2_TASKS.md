# GUI Phase 2: Refactoring & Branding - Task Breakdown

**Objective**: Transform GUI from functional prototype to professional audio workstation with integrated I/O controls, real-time visualization, and unified dark orange/magma branding.

**Target Timeline**: 8 implementation phases with validation checkpoints

---

## Phase 2a: Logo & Color Scheme (Priority 1)

### Task 1: Design Professional Audio Ninja Logo
- [ ] Create SVG logo with geometric audio waveform + ninja silhouette
- [ ] Primary color: Magma Orange (#E65100)
- [ ] Secondary accent: Neon Amber (#FF8C00)
- [ ] Highlight: Mist White (#F5F5F5)
- [ ] Save to: `/crates/gui/icons/audio-ninja-logo.svg`

### Task 2: Generate Logo Variants
- [ ] Full logo (horizontal layout, 512x256px)
- [ ] Icon-only variant (square, 256x256px, 512x512px)
- [ ] Monochrome variant (for dark/light backgrounds)
- [ ] Save PNG fallbacks to `/crates/gui/icons/`

### Task 3: Update CSS with Magma Theme Variables
- [ ] Define `:root` CSS variables:
  - `--magma-orange: #E65100`
  - `--neon-amber: #FF8C00`
  - `--void-black: #050203`
  - `--deep-bronze: #26140D`
  - `--mist-white: #F5F5F5`
  - `--blade-glow: #FFD580`
- [ ] File: `/crates/gui/public/style.css`
- [ ] Verify contrast ratios (WCAG AA: 4.5:1 minimum)

### Task 4: Refactor Existing Panel Styling
- [ ] Update button backgrounds to Magma Orange
- [ ] Update button hover to Neon Amber
- [ ] Update panel backgrounds to Deep Bronze
- [ ] Update text colors to Mist White
- [ ] Add Blade Glow highlights for active states
- [ ] Test on all existing panels (DRC, Loudness, Headroom, Binaural)

### Task 5: Add Smooth Transitions & Effects
- [ ] Apply 200ms ease-in-out transitions to all state changes
- [ ] Add glow effects to active buttons/tabs
- [ ] Update gradients: Orange-to-bronze progression
- [ ] Test performance impact (target <5% CPU)

### Task 6: Update Logo Integration
- [ ] Add logo to header in `/crates/gui/public/index.html`
- [ ] Update `tauri.conf.json` icon paths
- [ ] Integrate logo into splash screen (if applicable)
- [ ] Test rendering at multiple DPI levels

### Task 7: Color Scheme Validation
- [ ] Test on Linux (GNOME, KDE, Wayland)
- [ ] Test on macOS (light/dark themes)
- [ ] Verify WCAG AA accessibility
- [ ] Check color blindness compatibility (protanopia, deuteranopia)
- [ ] Document accessibility results

---

## Phase 2b: I/O & Transport Panel (Priority 2)

### Task 8: Add Input/Output Device Panel HTML
- [ ] Create new panels in `/crates/gui/public/index.html`:
  - Input panel with device selection dropdown
  - Output panel with device selection dropdown
  - Source type toggles (System/Application/External)
- [ ] Add device info display (channels, sample rates, capabilities)
- [ ] Add device availability indicators

### Task 9: Fetch & Display Input Devices
- [ ] GET `/api/v1/input/devices` on app startup
- [ ] Parse JSON response (id, name, channels, sample_rate, available)
- [ ] Populate input device dropdown in app.js
- [ ] Add error handling (log to console, fallback UI)
- [ ] Test with 0, 1, and 3+ devices

### Task 10: Fetch & Display Output Devices
- [ ] GET `/api/v1/output/devices` on app startup
- [ ] Parse JSON response (id, name, device_type, channels, sample_rates, default)
- [ ] Populate output device dropdown in app.js
- [ ] Group by device type (Speaker, Headphones, USB, etc.)
- [ ] Add error handling and fallback

### Task 11: Implement Device Selection Handlers
- [ ] POST `/api/v1/input/select` with device ID
- [ ] POST `/api/v1/output/select` with device ID
- [ ] Update UI after selection (highlight active device)
- [ ] Add success/error feedback (toast notification)
- [ ] Test switching devices during playback

### Task 12: Add Transport Panel HTML
- [ ] Create transport panel in index.html:
  - File loader (input type="file", accept audio formats)
  - Play/Pause/Stop buttons
  - Progress slider
  - Mode selector (File-only, Stream-only, Mixed)
  - Current file display
  - Playback duration

### Task 13: Implement File Loading
- [ ] File picker UI with drag-drop support
- [ ] POST `/api/v1/transport/load-file` with file path
- [ ] Display selected file name in UI
- [ ] Add error handling for invalid files
- [ ] Test with .wav, .mp3, .flac, .opus

### Task 14: Implement Playback Controls
- [ ] POST `/api/v1/transport/play` on Play button
- [ ] POST `/api/v1/transport/pause` on Pause button
- [ ] POST `/api/v1/transport/stop` on Stop button
- [ ] Update button states (Play → Pause when playing)
- [ ] Add keyboard shortcuts (Space for play/pause)

### Task 15: Add Progress Tracking
- [ ] GET `/api/v1/transport/playback-status` every 100ms
- [ ] Parse response (current_position, duration, is_playing, sample_rate)
- [ ] Update progress slider position
- [ ] Display time elapsed / total duration
- [ ] Handle slider drag events for seeking

### Task 16: Implement Transport Mode Selector
- [ ] POST `/api/v1/transport/mode` with selected mode
- [ ] UI dropdown: File-only | Stream-only | Mixed
- [ ] Add mode descriptions/tooltips
- [ ] Test mode switching during playback
- [ ] Save user preference (localStorage)

### Task 17: I/O & Transport Testing
- [ ] Test device enumeration with multiple configurations
- [ ] Test file loading and playback
- [ ] Test mode switching (pause/resume)
- [ ] Test error scenarios (invalid file, device unavailable)
- [ ] Test with network latency (simulate with tc delay)

---

## Phase 2c: Visualization & Calibration (Priority 3)

### Task 18: Create Layout Visualization Canvas
- [ ] Add canvas element to index.html
- [ ] Implement 2D speaker layout renderer (top-down view)
- [ ] Support presets: 2.0, 2.1, 3.1, 4.0, 5.1, 5.1.2, 7.1, 7.1.4, 9.1.6
- [ ] Draw speaker positions with labels (FL, FR, C, etc.)
- [ ] Use Magma theme colors for visualization

### Task 19: Add Layout Editor Interactivity
- [ ] Enable drag-to-reposition speakers on canvas
- [ ] Add angle/distance input fields for precise positioning
- [ ] Add preset selector dropdown
- [ ] Store custom layouts in localStorage
- [ ] Validate speaker positions (prevent overlap, min distance)

### Task 20: Implement VBAP Test Signal Display
- [ ] Show audio source routing to speakers in real-time
- [ ] Visualize panning direction (3D arrow on canvas)
- [ ] Display per-speaker amplitude (as bar chart)
- [ ] Highlight active speakers during playback

### Task 21: Add Calibration Panel HTML
- [ ] Create calibration section in index.html:
  - Sweep controls (start/stop, frequency range 20-20kHz, duration)
  - Microphone input device selector
  - IR visualization canvas
  - Filter design preview (FIR/IIR)
  - Export button for CamillaDSP format

### Task 22: Implement Sweep Generation UI
- [ ] GET `/api/v1/calibration/sweep` with parameters
- [ ] POST `/api/v1/calibration/start-measurement` to begin
- [ ] Real-time status updates (measuring..., done)
- [ ] Audio level meter during sweep
- [ ] Add frequency range inputs (min 20Hz, max 20kHz)

### Task 23: IR Curve Visualization
- [ ] Display impulse response curve (magnitude + phase)
- [ ] Plot frequency response (20Hz-20kHz log scale)
- [ ] Show delay detection markers
- [ ] Overlay target curve for comparison
- [ ] Use Chart.js or Canvas API for graphing

### Task 24: Filter Design Preview
- [ ] Display FIR/IIR filter magnitude response
- [ ] Parameter adjustment UI (frequency, Q factor, gain)
- [ ] Real-time filter preview on IR curve
- [ ] Export to CamillaDSP format
- [ ] Add undo/redo for filter adjustments

### Task 25: Visualization & Calibration Testing
- [ ] Test layout with all 9 presets
- [ ] Test drag-drop speaker repositioning
- [ ] Test sweep generation and IR capture
- [ ] Test filter design and export
- [ ] Test on multiple screen resolutions

---

## Phase 2d: Stats Dashboard & Polish (Priority 4)

### Task 26: Build Real-Time Stats Panel HTML
- [ ] Create stats section in index.html:
  - Speaker status table (name, address, latency, packet loss %)
  - Network bandwidth graph (sent/received kbps)
  - Latency histogram (min/max/mean/stddev)
  - CPU/memory usage chart
  - Sync error visualization
  - Audio level meters

### Task 27: Implement Speaker Status Table
- [ ] GET `/api/v1/speakers` to list connected speakers
- [ ] Parse response (id, name, address, latency_ms, packet_loss_pct, sync_error)
- [ ] Display in HTML table with live updates (every 200ms)
- [ ] Color-code status (green: ok, yellow: warning, red: error)
- [ ] Add "View Details" modal for each speaker

### Task 28: Add Network Bandwidth Monitoring
- [ ] GET `/api/v1/stats/network` for bandwidth data
- [ ] Display sent/received kbps as live graph
- [ ] Use sparkline or Chart.js for visualization
- [ ] Show peak/average rates
- [ ] Add historical 5/60-minute views

### Task 29: Create Latency Histogram
- [ ] GET `/api/v1/stats/latency` for latency samples
- [ ] Compute statistics (min, max, mean, stddev)
- [ ] Plot histogram with bin size 5ms
- [ ] Highlight target range (±5ms initially, ±1ms future)
- [ ] Show per-speaker latency comparison

### Task 30: Implement CPU/Memory Monitoring
- [ ] GET `/api/v1/stats/daemon` for daemon process stats
- [ ] Display CPU usage (% of 1 core, cores utilized)
- [ ] Display memory usage (MB, % of system)
- [ ] Show resource trends (5-minute graph)
- [ ] Alert if CPU >80% or memory >500MB

### Task 31: Add Sync Error Visualization
- [ ] GET `/api/v1/stats/sync` for phase alignment data
- [ ] Display per-speaker sync error (sample offset, phase degrees)
- [ ] Plot on circular plot (0°-360°) or time-series
- [ ] Color-code by severity (green <5ms, yellow 5-20ms, red >20ms)
- [ ] Show sync status indicator (locked/drift/unlocked)

### Task 32: Implement Audio Level Meters
- [ ] GET `/api/v1/stats/audio-levels` every 50ms
- [ ] Display input level meter (dBFS from -60 to 0)
- [ ] Display output level meter (dBFS per speaker)
- [ ] Use gradient colors: green (-60 to -12), yellow (-12 to -3), red (-3 to 0)
- [ ] Show peak hold with 3-second decay
- [ ] Add clipping indicator (red flash on overload)

### Task 33: Polish Transitions & Animations
- [ ] Add smooth fade-in animations for panels
- [ ] Animate graph/meter updates (no sudden jumps)
- [ ] Add button ripple effects on click
- [ ] Smooth tab transitions (slide or fade)
- [ ] Test animation performance (target 60fps, <5% CPU)

### Task 34: Responsive Layout Design
- [ ] Test on 1366x768 (minimal laptop), 1920x1080 (FHD), 3840x2160 (4K)
- [ ] Implement responsive grid layout (CSS Grid)
- [ ] Stack panels vertically on small screens
- [ ] Collapse non-essential sections on mobile
- [ ] Test on landscape/portrait tablet orientation

### Task 35: Accessibility Improvements
- [ ] Add ARIA labels to all interactive elements
- [ ] Implement keyboard navigation (Tab through controls)
- [ ] Test with screen reader (NVDA on Linux)
- [ ] Ensure color contrast WCAG AA (4.5:1 minimum)
- [ ] Add focus indicators for keyboard navigation

### Task 36: Dashboard Testing
- [ ] Test stats updates with 0, 2, 4, 8 speakers
- [ ] Test with network latency variations (50ms, 100ms, 500ms)
- [ ] Test with CPU load (run stress-ng in background)
- [ ] Test on Linux (GNOME), macOS (Big Sur+), Windows (if applicable)
- [ ] Performance benchmark: <100ms UI response time, <5% idle CPU

---

## Cross-Cutting Tasks

### Task 37: Error Handling & User Feedback
- [ ] Implement toast notification system
- [ ] Show connection status indicator (green/yellow/red)
- [ ] Add retry logic for failed API calls
- [ ] Display meaningful error messages to user
- [ ] Log errors to console with timestamps

### Task 38: State Persistence
- [ ] Save selected input/output devices to localStorage
- [ ] Save transport mode preference
- [ ] Save custom speaker layouts
- [ ] Save theme preference (if implementing light/dark toggle)
- [ ] Clear stale state on app restart

### Task 39: REST API Integration Testing
- [ ] Create integration test suite for all endpoints
- [ ] Test with real daemon running
- [ ] Test with daemon unavailable (handle gracefully)
- [ ] Test with slow network (simulate with tc delay)
- [ ] Verify response schemas match OpenAPI spec

### Task 40: Performance Optimization
- [ ] Profile GUI startup time (target <2 seconds)
- [ ] Optimize CSS/JS bundle size (minify, tree-shake)
- [ ] Implement virtual scrolling for large lists
- [ ] Debounce rapid API calls (e.g., slider dragging)
- [ ] Use requestAnimationFrame for smooth animations

### Task 41: Documentation & Code Comments
- [ ] Add JSDoc comments to all functions
- [ ] Document CSS class naming conventions
- [ ] Create HTML structure guide (component organization)
- [ ] Add implementation notes for complex features
- [ ] Update GUI README.md with new features

### Task 42: Cross-Platform Testing
- [ ] Test on Debian Linux (KDE, GNOME)
- [ ] Test on Ubuntu 20.04/22.04
- [ ] Test on macOS (Intel + Apple Silicon)
- [ ] Test on Windows 10/11 (if applicable)
- [ ] Document platform-specific issues

### Task 43: Release Preparation
- [ ] Update version in Cargo.toml, tauri.conf.json, package.json
- [ ] Build release binaries (Linux x86_64, aarch64)
- [ ] Create release notes documenting GUI changes
- [ ] Tag git commit with version
- [ ] Test release binary on clean system

---

## Acceptance Criteria

### Phase 2a Checklist (Logo & Color Scheme)
- [ ] Logo visible in window title and header
- [ ] All buttons display in Magma Orange
- [ ] All panels have Deep Bronze backgrounds
- [ ] Text is readable in Mist White on Dark backgrounds
- [ ] Hover effects show Neon Amber
- [ ] WCAG AA contrast verified
- [ ] No color-blind accessibility issues
- [ ] Performance: <5% CPU idle, <100ms response time

### Phase 2b Checklist (I/O & Transport)
- [ ] Input devices enumerate and display
- [ ] Output devices enumerate and display
- [ ] Device selection works and updates UI
- [ ] File picker allows audio file selection
- [ ] Play/Pause/Stop controls work
- [ ] Progress slider tracks playback position
- [ ] Mode selector switches transport modes
- [ ] Error messages display for invalid operations

### Phase 2c Checklist (Visualization & Calibration)
- [ ] Speaker layout visualization renders
- [ ] All 9 layout presets display correctly
- [ ] Drag-drop repositioning works smoothly
- [ ] VBAP routing visualization updates in real-time
- [ ] Calibration panel captures sweep
- [ ] IR curve plots display correctly
- [ ] Filter design preview reflects adjustments
- [ ] Export to CamillaDSP format works

### Phase 2d Checklist (Stats Dashboard & Polish)
- [ ] Speaker status table updates every 200ms
- [ ] Bandwidth graph displays sent/received rates
- [ ] Latency histogram shows distribution
- [ ] CPU/memory graph tracks daemon resources
- [ ] Sync error visualization updates smoothly
- [ ] Audio level meters show input/output levels
- [ ] All animations run at 60fps
- [ ] Responsive layout works on 1366x768 and 3840x2160
- [ ] Keyboard navigation complete
- [ ] Screen reader compatible

### Final Release Checklist
- [ ] All 43 tasks completed ✅
- [ ] 100% feature coverage (0 TODOs in code)
- [ ] All tests passing (276+ tests)
- [ ] Zero console warnings/errors (Tauri DevTools clean)
- [ ] Cross-platform testing complete (Linux, macOS, Windows)
- [ ] Release binaries under 10MB (with all features)
- [ ] Documentation complete (README, guides, API docs)
- [ ] Git history clean (meaningful commits)

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| GUI Startup Time | <2 seconds | TBD |
| UI Response Latency | <100ms | TBD |
| CPU Usage (Idle) | <5% | TBD |
| Memory Usage | <100MB | TBD |
| Frame Rate | 60fps | TBD |
| WCAG AA Contrast | 100% | TBD |
| Cross-Platform Tested | Linux, macOS, Windows | TBD |
| Feature Completeness | 100% (43/43 tasks) | TBD |

---

## Implementation Timeline

- **Week 1**: Phase 2a (Logo & Color Scheme) - Tasks 1-7
- **Week 2**: Phase 2b (I/O & Transport) - Tasks 8-17
- **Week 3**: Phase 2c (Visualization & Calibration) - Tasks 18-25
- **Week 4**: Phase 2d (Stats & Polish) - Tasks 26-42
- **Week 5**: Testing, Optimization, Release - Tasks 43, Cross-platform validation

**Total Estimated Effort**: 40-50 hours of development

---

## Next Steps

1. Start with Task 1: Design professional Audio Ninja logo (geometric audio + ninja style)
2. Proceed with Task 3: Update CSS with Magma theme variables
3. Complete Phase 2a validation before moving to Phase 2b
4. Maintain test coverage >80% throughout implementation
5. Commit after each 5-task group for easy rollback if needed
