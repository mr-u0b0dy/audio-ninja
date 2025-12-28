#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Icon generation script for Audio Ninja
# Requires ImageMagick: brew install imagemagick (macOS) or apt-get install imagemagick (Linux)

set -e

SOURCE="${1:-audio-ninja-icon.png}"
OUTPUT_DIR="${2:-crates/gui/icons}"

if [ ! -f "$SOURCE" ]; then
    echo "Error: Source icon not found: $SOURCE"
    echo "Usage: $0 <source_image.png> [output_dir]"
    echo "  source_image.png: 1024x1024 PNG with transparent background"
    echo "  output_dir: Output directory (default: crates/gui/icons)"
    exit 1
fi

if ! command -v convert &> /dev/null; then
    echo "Error: ImageMagick not found"
    echo "Install with:"
    echo "  macOS: brew install imagemagick"
    echo "  Linux: sudo apt-get install imagemagick"
    exit 1
fi

echo "📦 Generating Audio Ninja icons from: $SOURCE"
echo "📁 Output directory: $OUTPUT_DIR"

mkdir -p "$OUTPUT_DIR"

# Generate PNG sizes
echo "🎨 Generating PNG icons..."

for size in 16 32 64 128 256 512 1024; do
    echo "  Generating ${size}x${size}.png..."
    convert "$SOURCE" \
        -resize ${size}x${size} \
        -background none \
        -gravity center \
        -extent ${size}x${size} \
        "$OUTPUT_DIR/${size}x${size}.png"
done

# Generate @2x (Retina/HiDPI) version
echo "  Generating 128x128@2x.png (256x256)..."
convert "$OUTPUT_DIR/256x256.png" \
    "$OUTPUT_DIR/128x128@2x.png"

# Generate Windows ICO
echo "🪟 Generating Windows icon (icon.ico)..."
convert "$OUTPUT_DIR/256x256.png" \
    "$OUTPUT_DIR/128x128.png" \
    "$OUTPUT_DIR/64x64.png" \
    "$OUTPUT_DIR/32x32.png" \
    "$OUTPUT_DIR/16x16.png" \
    "$OUTPUT_DIR/icon.ico"

# Generate macOS ICNS
echo "🍎 Generating macOS icon (icon.icns)..."
# Create temporary directory for icns generation
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Copy PNG files for iconutil
mkdir -p "$TEMP_DIR/audio-ninja.iconset"
cp "$OUTPUT_DIR/16x16.png" "$TEMP_DIR/audio-ninja.iconset/icon_16x16.png"
cp "$OUTPUT_DIR/32x32.png" "$TEMP_DIR/audio-ninja.iconset/icon_16x16@2x.png"
cp "$OUTPUT_DIR/32x32.png" "$TEMP_DIR/audio-ninja.iconset/icon_32x32.png"
cp "$OUTPUT_DIR/64x64.png" "$TEMP_DIR/audio-ninja.iconset/icon_32x32@2x.png"
cp "$OUTPUT_DIR/128x128.png" "$TEMP_DIR/audio-ninja.iconset/icon_128x128.png"
cp "$OUTPUT_DIR/256x256.png" "$TEMP_DIR/audio-ninja.iconset/icon_128x128@2x.png"
cp "$OUTPUT_DIR/256x256.png" "$TEMP_DIR/audio-ninja.iconset/icon_256x256.png"
cp "$OUTPUT_DIR/512x512.png" "$TEMP_DIR/audio-ninja.iconset/icon_256x256@2x.png"
cp "$OUTPUT_DIR/512x512.png" "$TEMP_DIR/audio-ninja.iconset/icon_512x512.png"
cp "$OUTPUT_DIR/1024x1024.png" "$TEMP_DIR/audio-ninja.iconset/icon_512x512@2x.png"

# Convert to ICNS if iconutil is available (macOS only)
if command -v iconutil &> /dev/null; then
    iconutil -c icns "$TEMP_DIR/audio-ninja.iconset" -o "$OUTPUT_DIR/icon.icns"
else
    # Fall back to convert for cross-platform
    convert "$OUTPUT_DIR/512x512.png" \
        -define icon:auto-resize \
        "$OUTPUT_DIR/icon.icns"
fi

# Optimize PNG files
echo "📦 Optimizing PNG files..."
if command -v optipng &> /dev/null; then
    for png in "$OUTPUT_DIR"/*.png; do
        optipng -o2 -q "$png"
    done
elif command -v pngquant &> /dev/null; then
    for png in "$OUTPUT_DIR"/*.png; do
        pngquant --skip-if-larger --force --ext .png "$png"
    done
fi

echo ""
echo "✅ Icon generation complete!"
echo ""
echo "Generated files:"
ls -lh "$OUTPUT_DIR"/*.png "$OUTPUT_DIR"/*.ico "$OUTPUT_DIR"/*.icns 2>/dev/null | awk '{print "  " $9, "(" $5 ")"}'

echo ""
echo "📋 Next steps:"
echo "  1. Review icons: open $OUTPUT_DIR"
echo "  2. Test in app: cd crates/gui && cargo tauri build --release"
echo "  3. Verify icons appear in taskbar, app window, and file manager"
echo "  4. Commit: git add crates/gui/icons/ && git commit -m 'feat: update application icons'"
