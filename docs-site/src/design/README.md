# GUI Design & Implementation

This section covers the Audio Ninja GUI design system and Phase 2 implementation roadmap.

## Contents

- **[Design System](design-system.md)** - Magma Orange color scheme, CSS components, responsive design, and accessibility standards
- **[Phase 2 Tasks](phase2-tasks.md)** - 43 implementation tasks for GUI refactoring (5 weeks, 40-50 hours)

## Quick Facts

- **Framework**: Tauri 1.5 (Rust daemon + vanilla JavaScript frontend)
- **Logo**: Professional design available in `assets/logo.png`
- **Color Theme**: Magma Orange (#E65100) primary, Neon Amber (#FF8C00) accent
- **Target Platforms**: Linux, macOS, Windows
- **Design Status**: ✅ Complete with WCAG AA accessibility
- **Implementation Status**: 🚧 Ready to start (depends on HTML/CSS/JS only)

## Design Philosophy

The Audio Ninja GUI follows these principles:

1. **Clarity**: Dark theme with high contrast for precise audio work
2. **Efficiency**: Organized panels with quick access to common tasks
3. **Professionalism**: Magma Orange theme for audio industry aesthetics
4. **Accessibility**: WCAG AA compliant with keyboard navigation
5. **Responsiveness**: Works on laptops to ultra-wide displays

## Phase 2 Overview

### Phase 2a: Logo & Color Scheme (1 week)
- Integrate professional logo into GUI header
- Implement complete CSS theme with Magma Orange colors
- Refactor existing panels to match new design

### Phase 2b: I/O & Transport Panel (2 weeks)
- Device selection dropdowns (input/output)
- Audio source routing interface
- File loader and playback controls
- Transport mode selector (file/stream/mixed)

### Phase 2c: Visualization & Calibration (2 weeks)
- 3D speaker layout visualization with Canvas/WebGL
- Layout editor with drag-and-drop support
- Calibration UI with sweep controls
- IR curve visualization
- Filter design preview (FIR/IIR responses)

### Phase 2d: Stats & Polish (2 weeks)
- Real-time metrics dashboard
- Speaker status table with live updates
- Network bandwidth monitoring
- CPU/memory usage graphs
- Sync error visualization
- Performance optimization and polishing

## Key Components

### Existing Components ✅
- **DRC Panel**: Dynamic Range Control with presets
- **Loudness Panel**: ITU-R BS.1770 normalization
- **Headroom Panel**: Lookahead limiting
- **HRTF Panel**: Binaural rendering configuration
- **Status Panel**: Live metrics display

### New Components 🚧
- **I/O Controls**: Input/output device selection
- **Transport Controls**: File loading, playback, progress
- **Layout Visualization**: 3D speaker array display
- **Calibration Tools**: Sweep generation and analysis
- **Stats Dashboard**: Real-time monitoring

## Design System Details

The design system includes:

- **10 Core Colors**: Magma theme with background, text, accent, and status colors
- **Typography**: Clear hierarchy with 14px-24px font sizes
- **Spacing**: 8px grid system for consistent layout
- **Components**: Buttons, inputs, selects, panels, cards
- **Icons**: Material-style icons for all actions
- **Animations**: Smooth 200ms transitions on state changes

For complete specifications, see [Design System](design-system.md).

## Implementation Checklist

### Phase 2a Tasks ✅ Ready
- [ ] Copy logo to `/crates/gui/icons/`
- [ ] Create CSS custom properties for colors
- [ ] Update button styling (Magma Orange)
- [ ] Refactor panel backgrounds (Deep Bronze)
- [ ] Update text colors (Mist White)
- [ ] Add hover effects (Neon Amber)
- [ ] Test WCAG AA contrast ratios

### Phase 2b Tasks 🚧 Ready
- [ ] Add I/O device panels to HTML
- [ ] Fetch devices from REST API
- [ ] Implement device selection
- [ ] Add file picker integration
- [ ] Implement transport controls
- [ ] Add mode selector UI
- [ ] Error handling and feedback

### Phase 2c Tasks 🚧 Ready
- [ ] Set up Canvas for layout visualization
- [ ] Implement speaker positioning
- [ ] Add layout presets dropdown
- [ ] Create calibration UI
- [ ] IR curve visualization
- [ ] Filter design preview
- [ ] VBAP test routing display

### Phase 2d Tasks 🚧 Ready
- [ ] Build stats dashboard
- [ ] Create speaker status table
- [ ] Network monitoring graph
- [ ] CPU/memory metrics
- [ ] Sync error visualization
- [ ] Performance profiling
- [ ] Cross-platform testing

## Development Resources

- **Design Tokens**: See `design-system.md` for colors, spacing, typography
- **CSS Templates**: Pre-built components ready to customize
- **API Integration**: All endpoints documented in `../api/reference.md`
- **CLI Reference**: See `../guide/tui.md` for feature parity goals

## Getting Started

1. **Understand the Design**: Read [Design System](design-system.md) for colors and components
2. **Review Phase 2 Tasks**: Check [Phase 2 Tasks](phase2-tasks.md) for detailed breakdown
3. **Start with Phase 2a**: Begin with logo and CSS theme (easiest, high impact)
4. **Follow Phase 2b-d**: Implement panels in order of business value
5. **Test Continuously**: Verify responsive design and accessibility

## Related Documentation

- [GUI Quick Reference](../guide/gui-quick-reference.md) - UI patterns and best practices
- [REST API Reference](../api/reference.md) - Endpoints for I/O, transport, speakers
- [Daemon Workflow](../api/daemon_workflow.md) - Deployment and configuration
- [CLI Guide](../guide/cli-tui.md) - Terminal UI for feature parity goals
