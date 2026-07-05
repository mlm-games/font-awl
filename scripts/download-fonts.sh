#!/usr/bin/env bash
set -euo pipefail

FONTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/fonts"
mkdir -p "$FONTS_DIR"

# Noto Sans Latin subset — basic Latin + common punctuation
# Uses Google Fonts API static download
echo "Downloading NotoSans-Latin.ttf..."
curl -sL "https://github.com/notofonts/latin/raw/main/NotoSans/static/NotoSans-Regular.ttf" \
  -o "$FONTS_DIR/NotoSans-Latin.ttf"

# Noto Color Emoji
echo "Downloading NotoColorEmoji-Regular.ttf..."
curl -sL "https://github.com/notofonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf" \
  -o "$FONTS_DIR/NotoColorEmoji.ttf"

# Noto Sans CJK Regular
echo "Downloading NotoSansCJK-Regular.ttc..."
curl -sL "https://github.com/notofonts/noto-cjk/raw/main/Sans/OTF/SimplifiedChinese/NotoSansCJKsc-Regular.otf" \
  -o "$FONTS_DIR/NotoSansCJK-Regular.ttc"

echo "Done. Fonts downloaded to $FONTS_DIR"
