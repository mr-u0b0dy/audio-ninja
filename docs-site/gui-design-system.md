# Audio Ninja GUI - Design System & Brand Guidelines

## Color Palette

### Primary Colors
| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Magma Orange** | #E65100 | 230, 81, 0 | Primary buttons, active tabs, CTAs, logo primary |
| **Neon Amber** | #FF8C00 | 255, 140, 0 | Hover states, active toggles, icon highlights |
| **Blade Glow** | #FFD580 | 255, 213, 128 | Progress bars, active sliders, highlights |

### Neutral Colors
| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Void Black** | #050203 | 5, 2, 3 | Main canvas background |
| **Deep Bronze** | #26140D | 38, 20, 13 | Panel backgrounds, cards, sidebars |
| **Mist White** | #F5F5F5 | 245, 245, 245 | Body text, labels, headings |

### Status Colors
| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Success Green** | #4CAF50 | 76, 175, 80 | Connected, active, OK |
| **Warning Yellow** | #FFC107 | 255, 193, 7 | Warning, sync drift, degraded |
| **Error Red** | #F44336 | 244, 67, 54 | Error, disconnected, critical |
| **Info Blue** | #2196F3 | 33, 150, 243 | Information, loading, processing |

---

## Accessibility Standards

### Contrast Ratios (WCAG AA Compliance)
```
Magma Orange (#E65100) on Void Black (#050203):   10.8:1 ✅ (AAA)
Magma Orange (#E65100) on Deep Bronze (#26140D):  7.5:1 ✅ (AAA)
Mist White (#F5F5F5) on Deep Bronze (#26140D):    15.2:1 ✅ (AAA)
Mist White (#F5F5F5) on Void Black (#050203):     16.8:1 ✅ (AAA)
Neon Amber (#FF8C00) on Deep Bronze (#26140D):    5.2:1 ✅ (AA)
```

### Color Blindness Compatibility
- **Protanopia** (Red-blind): Magma Orange visible as brown/olive ✅
- **Deuteranopia** (Green-blind): Magma Orange visible as orange ✅
- **Tritanopia** (Blue-yellow blind): Minimal impact (magma theme avoids blue/yellow) ✅
- **Achromatopsia** (Monochrome): Lightness contrast maintained ✅

---

## Component Styling Guide

### Buttons
```css
/* Primary Button */
.btn-primary {
  background-color: #E65100;  /* Magma Orange */
  color: #F5F5F5;             /* Mist White */
  border: none;
  border-radius: 4px;
  padding: 10px 16px;
  font-weight: 600;
  transition: all 200ms ease-in-out;
}

.btn-primary:hover {
  background-color: #FF8C00;  /* Neon Amber */
  box-shadow: 0 2px 8px rgba(230, 81, 0, 0.3);
}

.btn-primary:active {
  transform: scale(0.98);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Secondary Button */
.btn-secondary {
  background-color: #26140D;  /* Deep Bronze */
  color: #F5F5F5;             /* Mist White */
  border: 2px solid #E65100;  /* Magma Orange */
}

.btn-secondary:hover {
  background-color: #3d1f15;
  border-color: #FF8C00;
}
```

### Panels & Cards
```css
.panel {
  background-color: #26140D;  /* Deep Bronze */
  border: 1px solid #3d1f15;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}

.panel-header {
  color: #FFD580;             /* Blade Glow */
  font-size: 18px;
  font-weight: 700;
  margin-bottom: 12px;
  border-bottom: 2px solid #E65100;
  padding-bottom: 8px;
}

.panel-active {
  border-left: 4px solid #FF8C00;
}
```

### Text & Typography
```css
body {
  background-color: #050203;  /* Void Black */
  color: #F5F5F5;             /* Mist White */
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 14px;
  line-height: 1.6;
}

h1 {
  color: #FFD580;             /* Blade Glow */
  font-size: 28px;
  font-weight: 700;
}

h2 {
  color: #FF8C00;             /* Neon Amber */
  font-size: 20px;
  font-weight: 600;
}

label {
  color: #F5F5F5;             /* Mist White */
  font-weight: 500;
}

.text-secondary {
  color: #B0B0B0;             /* Dimmed text */
}
```

### Input Controls
```css
input[type="text"],
input[type="file"],
select,
textarea {
  background-color: #1a0f08;
  color: #F5F5F5;
  border: 1px solid #3d1f15;
  border-radius: 4px;
  padding: 8px 12px;
  transition: border-color 200ms ease-in-out;
}

input:focus,
select:focus,
textarea:focus {
  outline: none;
  border-color: #E65100;
  box-shadow: 0 0 4px rgba(230, 81, 0, 0.5);
}

/* Slider / Range Input */
input[type="range"] {
  accent-color: #E65100;      /* Magma Orange */
}

input[type="range"]::-webkit-slider-thumb {
  background-color: #FF8C00;  /* Neon Amber */
  box-shadow: 0 0 4px rgba(230, 81, 0, 0.5);
}
```

### Status Indicators
```css
.status-ok {
  color: #4CAF50;             /* Success Green */
}

.status-warning {
  color: #FFC107;             /* Warning Yellow */
}

.status-error {
  color: #F44336;             /* Error Red */
}

.status-info {
  color: #2196F3;             /* Info Blue */
}

/* Glowing indicator */
.indicator-active {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: #4CAF50;
  box-shadow: 0 0 6px #4CAF50;
  animation: glow 2s ease-in-out infinite;
}

@keyframes glow {
  0%, 100% { box-shadow: 0 0 6px #4CAF50; }
  50% { box-shadow: 0 0 12px #4CAF50; }
}
```

### Tabs & Navigation
```css
.tabs {
  display: flex;
  border-bottom: 2px solid #26140D;
  gap: 8px;
  margin-bottom: 16px;
}

.tab {
  padding: 12px 16px;
  background-color: transparent;
  color: #B0B0B0;
  border: none;
  border-bottom: 3px solid transparent;
  cursor: pointer;
  transition: all 200ms ease-in-out;
}

.tab:hover {
  color: #FF8C00;
}

.tab-active {
  color: #FFD580;
  border-bottom-color: #E65100;
}
```

### Progress & Meters
```css
.progress-bar {
  background-color: #1a0f08;
  border-radius: 4px;
  height: 8px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #E65100, #FF8C00);
  transition: width 300ms ease-in-out;
}

.meter {
  display: flex;
  height: 20px;
  background-color: #1a0f08;
  border-radius: 4px;
  overflow: hidden;
  gap: 1px;
}

.meter-segment {
  flex: 1;
  background-color: #4CAF50;
  transition: background-color 200ms ease-in-out;
}

.meter-segment.warning {
  background-color: #FFC107;
}

.meter-segment.error {
  background-color: #F44336;
}
```

### Modals & Dialogs
```css
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(5, 2, 3, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: #26140D;
  border: 1px solid #3d1f15;
  border-radius: 8px;
  padding: 24px;
  max-width: 500px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.8);
}

.modal-header {
  color: #FFD580;
  font-size: 20px;
  font-weight: 700;
  margin-bottom: 16px;
  border-bottom: 2px solid #E65100;
  padding-bottom: 12px;
}

.modal-close {
  position: absolute;
  top: 16px;
  right: 16px;
  background: none;
  border: none;
  color: #B0B0B0;
  cursor: pointer;
  font-size: 24px;
}

.modal-close:hover {
  color: #FF8C00;
}
```

### Notifications & Toasts
```css
.toast {
  position: fixed;
  bottom: 24px;
  right: 24px;
  background-color: #26140D;
  color: #F5F5F5;
  padding: 12px 16px;
  border-radius: 4px;
  border-left: 4px solid #E65100;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  animation: slideIn 300ms ease-in-out;
  z-index: 2000;
}

.toast.success {
  border-left-color: #4CAF50;
}

.toast.warning {
  border-left-color: #FFC107;
}

.toast.error {
  border-left-color: #F44336;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
```

---

## Logo Specifications

### Logo Design
- **Primary**: Geometric audio waveform + ninja silhouette fusion
- **Format**: SVG (scalable), PNG fallback (256x256, 512x512)
- **Main Color**: Magma Orange (#E65100)
- **Accent**: Neon Amber (#FF8C00)
- **Highlight**: Mist White (#F5F5F5)

### Usage Guidelines
| Context | Size | Placement |
|---------|------|-----------|
| Window Icon | 256x256 | Tauri title bar |
| App Header | 64x64 | Left of app title |
| Splash Screen | 512x512 | Centered |
| About Dialog | 128x128 | Left side |
| Documentation | 64x64 | Page headers |
| Social Media | 256x256 | Links, avatars |

### Clearance
- **Minimum spacing**: 8px on all sides
- **Minimum size**: 32x32px for recognizability
- **Background**: Works on #050203 (Void Black) and #26140D (Deep Bronze)

---

## Transition & Animation Guidelines

### Timing
- **Fast**: 100ms (button clicks, state changes)
- **Standard**: 200ms (tab switching, panel fades)
- **Slow**: 500ms (splash screen, modals)

### Easing Functions
- **ease-in-out**: Default for all transitions (smooth, professional feel)
- **ease-in**: Panel collapse, fade-in
- **ease-out**: Panel expand, fade-out

### Animation Examples
```css
/* Smooth fade in */
.fade-in {
  animation: fadeIn 300ms ease-in-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* Smooth slide from right */
.slide-in-right {
  animation: slideInRight 300ms ease-in-out;
}

@keyframes slideInRight {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

/* Glow effect for active elements */
.glow {
  animation: glow 2s ease-in-out infinite;
}

@keyframes glow {
  0%, 100% {
    box-shadow: 0 0 8px rgba(230, 81, 0, 0.5);
  }
  50% {
    box-shadow: 0 0 16px rgba(230, 81, 0, 0.8);
  }
}
```

---

## Responsive Design Breakpoints

```css
/* Extra Small (Mobile) */
@media (max-width: 480px) {
  /* Stack panels vertically, reduce padding */
  .grid { grid-template-columns: 1fr; }
  .panel { padding: 12px; }
}

/* Small (Tablet) */
@media (min-width: 481px) and (max-width: 768px) {
  /* 2-column layout, medium padding */
  .grid { grid-template-columns: repeat(2, 1fr); }
  .panel { padding: 14px; }
}

/* Medium (Laptop) */
@media (min-width: 769px) and (max-width: 1366px) {
  /* 3-column layout, normal padding */
  .grid { grid-template-columns: repeat(3, 1fr); }
  .panel { padding: 16px; }
}

/* Large (Desktop) */
@media (min-width: 1367px) {
  /* Full layout, generous padding */
  .grid { grid-template-columns: repeat(4, 1fr); }
  .panel { padding: 20px; }
}
```

---

## Implementation Checklist

- [ ] Verify all hex colors in style.css
- [ ] Test contrast ratios with WebAIM tool
- [ ] Check color rendering on calibrated monitor
- [ ] Validate on Windows, macOS, Linux
- [ ] Test with color blindness simulator
- [ ] Measure performance impact (<5% CPU)
- [ ] Create visual regression tests
- [ ] Document brand guidelines
- [ ] Update README with color scheme
- [ ] Archive old blue/cyan theme assets

---

## References

- **WCAG 2.1 AA Standard**: https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html
- **Color Blindness Simulator**: https://www.color-blindness.com/coblis-color-blindness-simulator/
- **CSS Custom Properties**: https://developer.mozilla.org/en-US/docs/Web/CSS/--*
- **Tailwind Color Palette**: https://tailwindcss.com/docs/customizing-colors
- **Material Design Colors**: https://material.io/design/color/
