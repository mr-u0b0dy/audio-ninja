# Audio Ninja Documentation Site (VuePress + Theme Hope)

<div align="center">

![Audio Ninja Logo](../assets/logo.png)

</div>

VuePress documentation site for Audio Ninja wireless immersive audio platform.

## Setup

### Prerequisites

- Node.js 18+ and npm
- VuePress 2.x
- VuePress Theme Hope 2.x

### Installation

```bash
cd docs-site
npm install
```

## Development

### Start Development Server

```bash
npm run docs:dev
```

The site will be available at `http://localhost:8080`

Hot-reload is enabled for development changes.

## Build

### Production Build

```bash
npm run docs:build
```

Output is in `src/.vuepress/dist/`

### Clean Cache

```bash
npm run docs:clean
```

## Documentation Structure

```
docs-site/src/
├── api/                      # API reference and workflow guides
│   ├── README.md             # API overview
│   ├── reference.md          # REST API endpoint reference
│   ├── api_usage.md          # API usage examples (curl, Python, JS, Rust)
│   ├── daemon_workflow.md    # Daemon deployment and integration guide
│   └── release.md            # Release process documentation
├── guide/                    # Getting started and user guides
│   ├── README.md             # Guide overview
│   ├── quick-start.md        # Quick start guide
│   ├── installation.md       # Installation instructions
│   ├── configuration.md      # Daemon configuration guide
│   ├── cli-tui.md            # CLI and TUI usage guide
│   ├── tui.md                # Terminal UI detailed guide
│   └── gui-quick-reference.md # GUI quick reference
├── design/                   # GUI design and implementation
│   ├── README.md             # Design overview
│   ├── design-system.md      # Magma Orange design system
│   └── phase2-tasks.md       # 43 Phase 2 implementation tasks
├── spatial/                  # Spatial audio processing
│   ├── overview.md           # 3D spatial audio overview
│   ├── vbap.md               # VBAP (Vector Base Amplitude Panning)
│   ├── hoa.md                # HOA (Higher-Order Ambisonics)
│   ├── hrtf.md               # HRTF (Head-Related Transfer Function)
│   └── comparison.md         # Spatial rendering comparison
└── processing/               # DSP and audio processing
    ├── calibration.md        # Room calibration workflow
    ├── loudness.md           # Loudness normalization (ITU-R BS.1770)
    ├── drc.md                # Dynamic Range Control
    ├── codecs.md             # Codec support and formats
    ├── codec_integration.md  # FFmpeg and codec integration guide
    └── firmware_update.md    # Firmware update mechanism
```

## Configuration

### VuePress Config (`src/.vuepress/config.ts`)

Key settings:

```typescript
export default defineUserConfig({
  base: "/audio-ninja/",        // Deployment base path
  lang: "en-US",
  title: "Audio Ninja",
  theme: hopeTheme({
    navbar: [...],               // Top navigation
    sidebar: {...},              // Side navigation
    plugins: {...},              // Theme plugins
  }),
});
```

### Theme Configuration

- **Logo**: Add image to `src/.vuepress/public/logo.svg`
- **Navigation**: Edit navbar config in `config.ts`
- **Sidebar**: Edit sidebar config in `config.ts`
- **Footer**: Customize in theme config

## GitHub Pages Deployment

### Automatic Deployment

Push to `docs/vuepress-theme-hope` branch:

```bash
git checkout docs/vuepress-theme-hope
git push origin docs/vuepress-theme-hope
```

GitHub Actions workflow (`.github/workflows/docs-deploy.yml`) will:
1. Install dependencies
2. Build the VuePress site
3. Deploy to GitHub Pages

**Site URL**: `https://mr-u0b0dy.github.io/audio-ninja/`

### Manual Deployment

```bash
# Build site
npm run docs:build

# Deploy to GitHub Pages (requires gh-pages package)
npm install gh-pages --save-dev
npx gh-pages -d src/.vuepress/dist -b gh-pages
```

## Contributing to Documentation

### Adding New Pages

1. Create markdown file in appropriate section (guide, spatial, processing, api, design)
2. Update navbar/sidebar config in `src/.vuepress/config.ts` if it's a major section
3. Build and test locally: `npm run docs:dev`
4. Commit and push

### Markdown Features

VuePress Theme Hope supports:

- **Callouts**: :::info, :::warning, :::danger, :::success
- **Code Blocks**: Syntax highlighting with language selection
- **Diagrams**: Mermaid flowcharts and diagrams
- **Tabs**: Tabbed content blocks
- **Components**: Custom Vue components

### Documentation Standards

- Use clear, concise headings (## for sections, ### for subsections)
- Include code examples where relevant
- Link to related documentation
- Update the main navbar/sidebar when adding major sections
- Keep file sizes reasonable (split large docs into subsections)

## Theme & Color Palette

Current design uses the **Magma Orange/Dark Bronze** theme:

| Component | Color | Hex |
|-----------|-------|-----|
| Background | Void Black | #050203 |
| Cards/Sidebars | Deep Bronze | #26140D |
| Primary CTA | Magma Orange | #E65100 |
| Hover/Accents | Neon Amber | #FF8C00 |
| Highlights | Blade Glow | #FFD580 |
| Text | Mist White | #F5F5F5 |

Theme colors are configured in `src/.vuepress/config.ts`:

```typescript
themeColor: {
  "#E65100": "Magma Orange",
  "#FF8C00": "Neon Amber",
  "#FFD580": "Blade Glow",
}
```

## Troubleshooting

### Common Issues

**Build fails with missing dependencies**:
```bash
rm -rf node_modules package-lock.json
npm install
```

**Port 8080 already in use**:
```bash
npm run docs:dev -- --port 9000
```

**Changes not reflecting**:
```bash
npm run docs:clean
npm run docs:dev
```

## Performance & Optimization

- ⚡ Hot Module Replacement (HMR) for instant updates
- 📦 Optimized production builds (tree-shaking, compression)
- 🔍 Built-in search with Algolia integration
- 📱 Fully responsive design (mobile-first)
- ♿ WCAG AA accessibility standards

## Related Resources

- [VuePress 2.x Documentation](https://v2.vuepress.vuejs.org/)
- [VuePress Theme Hope Guide](https://theme-hope.vuejs.press/)
- [GitHub Pages Deployment](https://docs.github.com/en/pages)
- [Audio Ninja Repository](https://github.com/mr-u0b0dy/audio-ninja)

## License

Apache 2.0 - Same as Audio Ninja project
