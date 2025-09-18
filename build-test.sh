#!/bin/bash

echo "========================================"
echo "Tauri Build and Test Script"
echo "========================================"

echo ""
echo "[1/6] Cleaning previous builds..."
rm -rf dist/
rm -rf src-tauri/target/

echo ""
echo "[2/6] Installing dependencies..."
npm install
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to install npm dependencies"
    exit 1
fi

echo ""
echo "[3/6] Building frontend..."
npm run build
if [ $? -ne 0 ]; then
    echo "ERROR: Frontend build failed"
    exit 1
fi

echo ""
echo "[4/6] Building Tauri release..."
npm run tauri build
if [ $? -ne 0 ]; then
    echo "ERROR: Tauri build failed"
    exit 1
fi

echo ""
echo "[5/6] Testing release executable..."
if [ -f "src-tauri/target/release/weather-dashboard" ]; then
    echo "SUCCESS: Release executable created!"
    echo "Location: src-tauri/target/release/weather-dashboard"
elif [ -f "src-tauri/target/release/weather-dashboard.exe" ]; then
    echo "SUCCESS: Release executable created!"
    echo "Location: src-tauri/target/release/weather-dashboard.exe"
else
    echo "ERROR: Release executable not found"
    exit 1
fi

echo ""
echo "[6/6] Checking installer packages..."
if ls src-tauri/target/release/bundle/*/*.dmg 1> /dev/null 2>&1; then
    echo "SUCCESS: DMG installer created!"
fi
if ls src-tauri/target/release/bundle/*/*.appimage 1> /dev/null 2>&1; then
    echo "SUCCESS: AppImage installer created!"
fi
if ls src-tauri/target/release/bundle/*/*.msi 1> /dev/null 2>&1; then
    echo "SUCCESS: MSI installer created!"
fi
if ls src-tauri/target/release/bundle/*/*.exe 1> /dev/null 2>&1; then
    echo "SUCCESS: NSIS installer created!"
fi

echo ""
echo "========================================"
echo "BUILD COMPLETED SUCCESSFULLY!"
echo "========================================"
echo ""
echo "Release files location:"
echo "- Executable: src-tauri/target/release/weather-dashboard"
echo "- Installers: src-tauri/target/release/bundle/"
echo ""
echo "To test the app, run:"
echo "src-tauri/target/release/weather-dashboard"
echo ""
