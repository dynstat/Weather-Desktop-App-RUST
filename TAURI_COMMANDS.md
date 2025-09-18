# Tauri Development & Release Commands Guide

This guide provides all the essential commands for developing, building, testing, and releasing Tauri applications.

## Prerequisites

Make sure you have the following installed:
- Node.js (v16 or later)
- Rust (latest stable)
- Tauri CLI: `npm install -g @tauri-apps/cli@latest`

## Development Workflow

### 1. Initial Setup
```bash
# Clone and setup project
git clone <your-repo>
cd weather-desktop-app

# Install dependencies
npm install

# Install Rust dependencies (first time only)
cd src-tauri
cargo build
cd ..
```

### 2. Development Commands

#### Start Development Server
```bash
# Start the development server (hot reload enabled)
npm run tauri dev
# OR
npm run dev
```

#### Development with Custom Port
```bash
# If you need to use a different port
npm run tauri dev -- --port 3000
```

#### Check for Issues
```bash
# TypeScript type checking
npm run build

# Lint frontend code
npm run lint

# Check Rust code
cd src-tauri
cargo check
cargo clippy
cd ..
```

## Building & Testing

### 3. Frontend Build Only
```bash
# Build frontend for production (creates dist/ folder)
npm run build

# Preview built frontend
npm run preview
```

### 4. Tauri Build Commands

#### Debug Build
```bash
# Build Tauri app in debug mode
npm run tauri build -- --debug
```

#### Release Build
```bash
# Build Tauri app for production (optimized)
npm run tauri build
```

#### Build with Specific Target
```bash
# Build for specific architecture
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target aarch64-apple-darwin  # macOS ARM
npm run tauri build -- --target x86_64-unknown-linux-gnu  # Linux
```

## Testing & Validation

### 5. Testing Commands

#### Test Frontend
```bash
# Run frontend tests (if configured)
npm test

# Run tests in watch mode
npm run test:watch
```

#### Test Rust Backend
```bash
cd src-tauri

# Run Rust tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

cd ..
```

#### Integration Testing
```bash
# Test the built application
# Windows
src-tauri\target\release\weather-dashboard.exe

# macOS
src-tauri/target/release/weather-dashboard

# Linux
src-tauri/target/release/weather-dashboard
```

## Release & Distribution

### 6. Release Build Process

#### Complete Release Build
```bash
# 1. Clean previous builds
npm run clean  # if available
# OR manually
rm -rf dist/
rm -rf src-tauri/target/

# 2. Build frontend
npm run build

# 3. Build Tauri app for release
npm run tauri build
```

#### Build Specific Bundle Types
```bash
# Build only MSI installer (Windows)
npm run tauri build -- --bundles msi

# Build only NSIS installer (Windows)
npm run tauri build -- --bundles nsis

# Build only DMG (macOS)
npm run tauri build -- --bundles dmg

# Build only AppImage (Linux)
npm run tauri build -- --bundles appimage

# Build all available bundles
npm run tauri build -- --bundles all
```

### 7. Release Artifacts Location

After building, find your release files in:
```
src-tauri/target/release/
├── weather-dashboard.exe          # Windows executable
├── weather-dashboard              # macOS/Linux executable
└── bundle/
    ├── msi/
    │   └── weather-dashboard_0.1.0_x64_en-US.msi
    ├── nsis/
    │   └── weather-dashboard_0.1.0_x64-setup.exe
    ├── dmg/                       # macOS
    └── appimage/                  # Linux
```

## Troubleshooting Commands

### 8. Debug & Fix Issues

#### Clear Caches
```bash
# Clear npm cache
npm cache clean --force

# Clear Rust cache
cd src-tauri
cargo clean
cd ..

# Clear Tauri cache
rm -rf node_modules/.cache
```

#### Check Dependencies
```bash
# Update npm dependencies
npm update

# Update Rust dependencies
cd src-tauri
cargo update
cd ..

# Check for outdated packages
npm outdated
```

#### Verify Installation
```bash
# Check Tauri CLI version
tauri --version

# Check Rust version
rustc --version

# Check Node version
node --version
npm --version
```

## Advanced Commands

### 9. Custom Build Scripts

Add these to your `package.json` for convenience:

```json
{
  "scripts": {
    "dev": "tauri dev",
    "build": "tsc && vite build",
    "build:tauri": "tauri build",
    "build:debug": "tauri build --debug",
    "build:release": "tauri build",
    "clean": "rm -rf dist/ && cargo clean",
    "test:frontend": "vitest",
    "test:rust": "cd src-tauri && cargo test",
    "lint": "eslint src/",
    "lint:fix": "eslint src/ --fix",
    "type-check": "tsc --noEmit"
  }
}
```

### 10. Environment-Specific Builds

#### Development Environment
```bash
# Set environment variables for development
NODE_ENV=development npm run tauri dev
```

#### Production Environment
```bash
# Set environment variables for production
NODE_ENV=production npm run tauri build
```

#### Custom Configuration
```bash
# Build with custom config
npm run tauri build -- --config tauri.conf.production.json
```

## Complete Workflow Examples

### 11. Daily Development Workflow
```bash
# 1. Start development
npm run tauri dev

# 2. Make changes to code
# 3. Test changes (hot reload should handle this)

# 4. Before committing, run checks
npm run build
npm run test:rust
```

### 12. Release Workflow
```bash
# 1. Update version numbers
# Edit package.json and Cargo.toml

# 2. Clean build
npm run clean

# 3. Build for release
npm run tauri build

# 4. Test the release build
src-tauri/target/release/weather-dashboard.exe

# 5. Create git tag
git tag v1.0.0
git push origin v1.0.0
```

### 13. CI/CD Pipeline Commands
```bash
# For GitHub Actions or similar CI/CD
npm ci
npm run build
npm run tauri build
```

## Platform-Specific Notes

### Windows
- Use PowerShell or Command Prompt
- Executable: `weather-dashboard.exe`
- Installers: `.msi` and `.exe` (NSIS)

### macOS
- Requires Xcode Command Line Tools
- Executable: `weather-dashboard`
- Installer: `.dmg`

### Linux
- May require additional system dependencies
- Executable: `weather-dashboard`
- Installer: `.AppImage`, `.deb`, `.rpm`

## Quick Reference

| Task | Command |
|------|---------|
| Start development | `npm run tauri dev` |
| Build frontend | `npm run build` |
| Build release | `npm run tauri build` |
| Test Rust | `cd src-tauri && cargo test` |
| Clean build | `npm run clean` |
| Check types | `npm run type-check` |
| Run release app | `src-tauri/target/release/weather-dashboard.exe` |

Remember: Always use `npm run tauri build` for production builds, never `cargo build --release` directly!
