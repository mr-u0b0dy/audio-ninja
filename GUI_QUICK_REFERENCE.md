# GUI Phase 2 - Quick Reference Card

## Color Palette (Copy-Paste)
```css
:root {
  --magma-orange: #E65100;      /* Primary buttons, CTAs, active tabs */
  --neon-amber: #FF8C00;         /* Hover states, toggles, highlights */
  --blade-glow: #FFD580;         /* Progress, sliders, focus indicators */
  --void-black: #050203;         /* Main background */
  --deep-bronze: #26140D;        /* Panel backgrounds, cards */
  --mist-white: #F5F5F5;         /* Text, labels, headings */
  --success-green: #4CAF50;      /* Connected, active, OK status */
  --warning-yellow: #FFC107;     /* Warnings, sync drift */
  --error-red: #F44336;          /* Errors, disconnected */
  --info-blue: #2196F3;          /* Info, loading, processing */
}
```

## Component Checklist
- [ ] **Button**: Magma Orange bg, Mist White text, Neon Amber hover
- [ ] **Panel**: Deep Bronze bg, 1px border #3d1f15, rounded 8px
- [ ] **Text**: Mist White color, readable on Deep Bronze
- [ ] **Input**: Dark bg #1a0f08, focus border Magma Orange
- [ ] **Slider**: Accent color Magma Orange, thumb Neon Amber
- [ ] **Status**: Green OK, Yellow warning, Red error, Blue info
- [ ] **Tab**: Active tab Blade Glow color, border Magma Orange
- [ ] **Progress**: Magma Orange fill, animation smooth 300ms
- [ ] **Modal**: Deep Bronze bg, Blade Glow header, Mist White text
- [ ] **Toast**: Border-left Magma Orange, auto-dismiss 3s

## Files to Modify (Priority Order)
1. **`crates/gui/public/style.css`** - Replace blue/cyan with Magma theme
2. **`crates/gui/public/index.html`** - Add logo, I/O panels, transport controls
3. **`crates/gui/public/app.js`** - Add device handlers, transport listeners
4. **`crates/gui/icons/`** - Create new Audio Ninja logo SVG
5. **`crates/gui/tauri.conf.json`** - Update icon paths

## API Endpoints (Already Implemented)
```
GET    /api/v1/input/devices          List input devices (system, external)
POST   /api/v1/input/select           Select input device
GET    /api/v1/input/status           Get current input status

GET    /api/v1/output/devices         List output devices (speakers, headphones)
POST   /api/v1/output/select          Select output device
GET    /api/v1/output/status          Get current output status

POST   /api/v1/transport/load-file    Load audio file
POST   /api/v1/transport/play         Start playback
POST   /api/v1/transport/pause        Pause playback
POST   /api/v1/transport/stop         Stop playback
POST   /api/v1/transport/mode         Set transport mode (file/stream/mixed)
GET    /api/v1/transport/status       Get playback status
```

## Logo Specifications
- **Format**: SVG (primary), PNG fallback (256x256, 512x512)
- **Style**: Geometric audio waveform + ninja silhouette
- **Main Color**: Magma Orange (#E65100)
- **Accent**: Neon Amber (#FF8C00)
- **Highlight**: Mist White (#F5F5F5)
- **Placement**: `/crates/gui/icons/audio-ninja-logo.svg`

## GUI Panels to Add
1. **Input Panel**: Device dropdown, source type (System/App/External)
2. **Output Panel**: Device dropdown, device type (Speaker/Headphone/USB)
3. **Transport Panel**: File loader, play/pause/stop, progress slider, mode selector
4. **Layout Panel**: Speaker layout visualization, presets (2.0-9.1.6), drag editor
5. **Calibration Panel**: Sweep controls, IR visualization, filter preview
6. **Stats Panel**: Speaker status table, bandwidth graph, latency histogram, CPU/memory, sync error, level meters

## Testing Checklist
- [ ] Logo renders at 32x32, 64x64, 128x128, 256x256, 512x512
- [ ] All buttons are Magma Orange with Neon Amber hover
- [ ] Panel backgrounds are Deep Bronze, text is Mist White
- [ ] Contrast ratios meet WCAG AA (4.5:1 minimum)
- [ ] Animations smooth at 60fps, no jank
- [ ] Device lists load without errors
- [ ] Transport controls respond to API calls
- [ ] Responsive layout works at 1366x768 and 3840x2160
- [ ] Keyboard navigation works (Tab through controls)
- [ ] Works on Linux (GNOME, KDE), macOS, Windows

## Performance Targets
- **GUI Startup**: <2 seconds
- **UI Response**: <100ms
- **CPU Idle**: <5%
- **Memory**: <100MB
- **Frame Rate**: 60fps minimum
- **Binary Size**: <10MB

## Timeline (5 Weeks)
- **Week 1**: Logo design + CSS theme (Tasks 1-7)
- **Week 2**: I/O + Transport panels (Tasks 8-17)
- **Week 3**: Visualization + Calibration (Tasks 18-25)
- **Week 4**: Stats + Polish (Tasks 26-42)
- **Week 5**: Testing + Release (Task 43)

## Git Workflow
1. Create feature branch: `git checkout -b gui/phase2-logo`
2. Implement 5 tasks per commit
3. Push and create PR when phase complete
4. Merge after review and testing
5. Tag release: `git tag -a v0.2.0 -m "GUI Phase 2: Magma theme, I/O controls, stats dashboard"`

## Links
- **Full Task List**: [GUI_PHASE2_TASKS.md](GUI_PHASE2_TASKS.md)
- **Design System**: [GUI_DESIGN_SYSTEM.md](GUI_DESIGN_SYSTEM.md)
- **Implementation Guide**: [GUI_PHASE2_SUMMARY.md](GUI_PHASE2_SUMMARY.md)
- **Copilot Instructions**: [.github/copilot-instructions.md](.github/copilot-instructions.md)

## Quick Commands
```bash
# Build GUI
cargo build -p audio-ninja-gui --release

# Run GUI
cargo run -p audio-ninja-gui --release

# Run daemon (GUI depends on this)
cargo run -p audio-ninja-daemon --release

# Run tests
cargo test --workspace

# Test API endpoints
curl http://localhost:8080/api/v1/input/devices
curl http://localhost:8080/api/v1/output/devices
```

---

**Start Here**: Task 1 in [GUI_PHASE2_TASKS.md](GUI_PHASE2_TASKS.md) - Design professional Audio Ninja logo

**Status**: ✅ Planning Complete | 🚧 Ready for Implementation
