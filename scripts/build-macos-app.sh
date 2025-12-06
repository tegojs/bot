#!/bin/bash
set -e

# Build macOS app and create DMG
# Usage: ./scripts/build-macos-app.sh [version]

APP_NAME="Aumate"
APP_ID="com.tegojs.aumate"
VERSION="${1:-0.0.0-dev}"
ASSETS_DIR="packages/aumate/assets"
OUTPUT_DIR="dist/desktop"

echo "Building $APP_NAME v$VERSION for macOS..."

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Build release binary
echo "Compiling Rust binary..."
cargo build --release --package aumate

# Create .app bundle
echo "Creating app bundle..."
APP_DIR="$OUTPUT_DIR/$APP_NAME.app"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy binary
cp "target/release/aumate" "$APP_DIR/Contents/MacOS/$APP_NAME"

# Copy icon
cp "$ASSETS_DIR/icon.icns" "$APP_DIR/Contents/Resources/AppIcon.icns"

# Create Info.plist
cat > "$APP_DIR/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>$APP_ID</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
</dict>
</plist>
EOF

echo "App bundle created at: $APP_DIR"

# Create DMG if create-dmg is available
if command -v create-dmg &> /dev/null; then
    echo "Creating DMG installer..."
    DMG_PATH="$OUTPUT_DIR/$APP_NAME-$VERSION-macos.dmg"
    rm -f "$DMG_PATH"

    create-dmg \
        --volname "$APP_NAME" \
        --volicon "$ASSETS_DIR/icon.icns" \
        --background "$ASSETS_DIR/dmg-background.png" \
        --window-pos 200 120 \
        --window-size 660 400 \
        --icon-size 100 \
        --icon "$APP_NAME.app" 165 200 \
        --hide-extension "$APP_NAME.app" \
        --app-drop-link 495 200 \
        "$DMG_PATH" \
        "$APP_DIR"

    echo "DMG created at: $DMG_PATH"
else
    echo "Note: Install create-dmg for fancy DMG: brew install create-dmg"
    echo "Creating simple DMG..."
    DMG_PATH="$OUTPUT_DIR/$APP_NAME-$VERSION-macos.dmg"
    rm -f "$DMG_PATH"
    hdiutil create -volname "$APP_NAME" -srcfolder "$APP_DIR" -ov -format UDZO "$DMG_PATH"
    echo "DMG created at: $DMG_PATH"
fi

echo ""
echo "Build complete!"
echo "  App: $APP_DIR"
echo "  DMG: $DMG_PATH"
