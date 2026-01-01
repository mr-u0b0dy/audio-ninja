# Audio Ninja Assets

This directory contains visual assets for the Audio Ninja project.

## Images

### `logo.png`
- **Dimensions**: 888 × 888 px
- **Format**: PNG with transparency (RGBA)
- **File Size**: 1.2 MB
- **Purpose**: Application logo/icon (square format)
- **Usage**: 
  - Documentation site hero image (`docs-site/public/logo.png`)
  - README.md header
  - Website branding
  - App icon reference

### `github/banner.png`
- **Dimensions**: 1170 × 584 px
- **Format**: PNG (RGB)
- **File Size**: 981 KB
- **Purpose**: GitHub profile/repository banner (2:1 aspect ratio)
- **Usage**: 
  - GitHub profile background
  - Repository promotional material
  - Website header banner

## Deployment

Images are deployed to:
- **Doc Site**: `docs-site/public/` - Used for VuePress documentation
- **Repository Assets**: `assets/` - Source directory
- **README**: Referenced directly from `assets/`

## Guidelines

- **Logo**: Use for square icon placements, app headers, documentation
- **Banner**: Use for wide banner placements (GitHub, website headers)
- **Formats**: PNG with transparency for versatile background integration
- **File Sizes**: Consider optimization if file sizes increase

## Usage Examples

### In Markdown (README)
```markdown
![Audio Ninja Logo](assets/logo.png)
```

### In VuePress Config
```typescript
logo: "/logo.png"
heroImage: "/logo.png"
```

### In HTML
```html
<img src="/logo.png" alt="Audio Ninja Logo" />
<img src="/banner.png" alt="GitHub Banner" />
```
