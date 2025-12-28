# Audio Ninja Icon Design Guide

This guide covers designing and generating application icons for Audio Ninja across all platforms (Linux, macOS, Windows).

## Icon Design Overview

Audio Ninja requires icons in multiple formats and sizes for different platforms:

| Size | Purpose | Platforms | Format |
|------|---------|-----------|--------|
| 16x16 | Taskbar/menu icon | Windows, Linux | PNG |
| 32x32 | Application icon | All | PNG |
| 64x64 | High-DPI taskbar | Linux | PNG |
| 128x128 | App drawer, Linux appimage | Linux, Windows | PNG |
| 128x128@2x | Retina display | macOS | PNG |
| 256x256 | File explorer (Windows 10+) | Windows | PNG |
| 512x512 | Installer, store listings | All | PNG |
| 1024x1024 | App store submissions | macOS, Windows | PNG |
| .icns | macOS bundle | macOS | Binary |
| .ico | Windows executable | Windows | Binary |

## Current Icon Status

Audio Ninja uses placeholder blue circle icons in the following files:

```
crates/gui/icons/
├── 32x32.png         # 32x32 application icon
├── 128x128.png       # 128x128 application icon
├── 128x128@2x.png    # 256x256 (2x retina) application icon
├── icon.ico          # Windows icon bundle
└── icon.icns         # macOS icon bundle
```

## Design Guidelines

### Visual Style
- **Color Scheme**: Use colors that complement audio visualization
  - Suggested primary: Dark navy or black background
  - Suggested accent: Vibrant blue (#00A4EF) or purple (#9D4EDD)
  - Suggested secondary: Orange (#FF6B35) for activity indicators

- **Shape**: Audio Ninja suggests:
  - Waveform-inspired shapes
  - Ninja silhouette with audio/sound elements
  - Sound wave patterns
  - Abstract speaker/audio symbols

- **Simplicity**: Icons must be recognizable at 16x16 and scale to 1024x1024
  - Avoid fine details that disappear at small sizes
  - Use solid colors and clear edges
  - Ensure good contrast on light and dark backgrounds

### Design Inspiration
- Professional audio software: Audacity, Adobe Audition, Pro Tools
- Minimalist approach: Preserve clarity at small sizes
- Modern flat design: Clean, simple shapes without gradients (optional)
- High-DPI awareness: Design for 2x/3x scaling

## Creating Icons

### Option 1: Using Design Software

#### Figma (Free, Cloud-Based)
1. Open https://figma.com
2. Create a new file
3. Set up artboards for each size (16x16, 32x32, 128x128, 512x512, 1024x1024)
4. Design your icon
5. Export as PNG for each size

#### Adobe Illustrator / Photoshop
1. Create a new document (1024x1024 recommended)
2. Design your icon using vector shapes (Illustrator) or smart objects (Photoshop)
3. Export as PNG for each size with proper DPI settings

#### Inkscape (Free, Open Source)
```bash
# Install on Linux
sudo apt-get install inkscape

# Design in Inkscape, then export:
# File → Export As → PNG Image
# Set resolution for each size
```

### Option 2: Using Icon Generators

#### Icon Forge Online
- Visit: https://www.favicon-generator.org/
- Upload your design
- Generate all sizes automatically

#### ImageMagick (Command Line)
```bash
# Generate multiple sizes from a single source image
convert source.png -define icon:auto-resize="16,32,64,128,256,512" favicon.ico

# Or generate individual PNGs
for size in 32 64 128 256 512; do
  convert source.png -resize ${size}x${size} ${size}x${size}.png
done
```

### Option 3: Professional Icon Packs

- **Flaticon** (https://www.flaticon.com): Search "audio" or "ninja"
- **Icons8** (https://icons8.com): Download audio/music related icons
- **Iconfinder** (https://www.iconfinder.com): Premium and free icon collections

## Icon Conversion Tools

### PNG to ICO (Windows)
```bash
# Using ImageMagick
convert 128x128.png icon.ico

# Or using online converter: https://convertio.co/png-ico/
```

### PNG to ICNS (macOS)
```bash
# Using ImageMagick with iconutil
convert 256x256.png -bordercolor none -border 0 -define png:color-type=6 icon.png

# Or using online converter: https://cloudconvert.com/png-to-icns
```

### Batch Generation with ImageMagick
```bash
#!/bin/bash
# Generate all required icon sizes from a 1024x1024 source

SOURCE="audio-ninja-1024.png"

# PNG sizes
for size in 16 32 64 128 256 512 1024; do
  convert "$SOURCE" -resize ${size}x${size} -background none -gravity center \
    -extent ${size}x${size} "${size}x${size}.png"
done

# Create @2x (retina)
convert 128x128.png -resize 256x256 "128x128@2x.png"

# Convert to ICO (Windows)
convert 128x128.png 64x64.png 32x32.png 16x16.png icon.ico

# Convert to ICNS (macOS) - requires multiple sizes
convert 512x512.png -define icon:auto-resize icon.icns
```

## Replacing the Current Icons

### Step 1: Design Your Icon
- Create a design in your preferred tool
- Export as PNG at 1024x1024 resolution with transparent background

### Step 2: Generate Required Sizes
```bash
cd crates/gui/icons

# Using the batch script above, generate:
# - 32x32.png
# - 128x128.png
# - 128x128@2x.png (256x256)
# - 512x512.png
```

### Step 3: Generate Platform-Specific Formats
```bash
# Windows ICO
convert 128x128.png icon.ico

# macOS ICNS
convert 512x512.png -define icon:auto-resize icon.icns
```

### Step 4: Update Tauri Configuration
Edit `crates/gui/tauri.conf.json`:
```json
{
  "tauri": {
    "bundle": {
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    }
  }
}
```

### Step 5: Test the Icons
```bash
cd crates/gui

# Build the GUI with new icons
cargo tauri build --release

# Check that icons appear correctly in:
# - Application window (title bar)
# - System taskbar
# - File explorer
# - macOS Dock
# - Windows start menu
```

## Icon Testing

### Linux
- Check `.AppImage` file manager icon
- Check `.deb` package installation
- Verify in application menu and taskbar

### macOS
- Verify in Dock when running
- Check Finder icon preview
- Test on both Intel and Apple Silicon Macs

### Windows
- Check File Explorer icon
- Verify Windows 11 taskbar icon
- Test on high-DPI displays (check @2x scaling)

## Icon Format Specifications

### PNG Requirements
- **Color Space**: RGBA (with alpha transparency)
- **DPI**: 72 DPI (standard screen resolution)
- **Background**: Transparent (alpha channel)
- **Compression**: Optimized with `pngquant` or `optipng`

### ICO Requirements (Windows)
- **Formats**: BMP or PNG-based
- **Sizes**: 16x16, 32x32, 48x48, 64x64 (minimum)
- **Color Depth**: 32-bit (8 bits per channel + alpha)

### ICNS Requirements (macOS)
- **Sizes**: 16x16, 32x32, 64x64, 128x128, 256x256, 512x512
- **Color Space**: RGBA
- **Format**: macOS-specific binary format

## Tools and Resources

### Icon Design Tools
- **Figma** (Free, browser-based): https://figma.com
- **Inkscape** (Free, open-source): https://inkscape.org
- **Adobe Illustrator** (Paid): https://adobe.com/products/illustrator
- **Photoshop** (Paid): https://adobe.com/products/photoshop

### Icon Generation
- **ImageMagick**: `brew install imagemagick` or `apt-get install imagemagick`
- **FFmpeg**: Can also convert images
- **Online Converter**: https://icoconvert.com or https://convertio.co

### Icon Inspiration
- **Flaticon**: https://www.flaticon.com (search "audio")
- **Icons8**: https://icons8.com
- **Noun Project**: https://thenounproject.com
- **Material Design Icons**: https://fonts.google.com/icons

## Contributing Icons

To submit custom icons for Audio Ninja:

1. Design your icon following the guidelines above
2. Generate all required sizes (32x32, 128x128, 128x128@2x, icon.ico, icon.icns)
3. Verify icons look good at all sizes
4. Submit a pull request with:
   - All PNG files in `crates/gui/icons/`
   - Updated `.ico` and `.icns` files
   - Brief description of the design inspiration
   - Attribution if using existing icon sources

## Next Steps

1. Choose your design tool (recommended: Figma for simplicity)
2. Create a 1024x1024 master icon design
3. Generate all required sizes using the batch script
4. Test icons in the built application
5. Update this document with final icon details

## References

- [Tauri Icon Guide](https://tauri.app/v1/guides/building/icons/)
- [macOS Icon Design Guidelines](https://developer.apple.com/design/human-interface-guidelines/macos/icons-and-images/app-icons/)
- [Windows Icon Design Guidelines](https://docs.microsoft.com/en-us/windows/win32/uxguide/vis-icons)
- [Linux Icon Standards](https://specifications.freedesktop.org/icon-spec/icon-spec-latest.html)
