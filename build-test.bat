@echo off
echo ========================================
echo Tauri Build and Test Script
echo ========================================

echo.
echo [1/6] Cleaning previous builds...
if exist dist rmdir /s /q dist
if exist src-tauri\target rmdir /s /q src-tauri\target

echo.
echo [2/6] Installing dependencies...
call npm install
if %errorlevel% neq 0 (
    echo ERROR: Failed to install npm dependencies
    pause
    exit /b 1
)

echo.
echo [3/6] Building frontend...
call npm run build
if %errorlevel% neq 0 (
    echo ERROR: Frontend build failed
    pause
    exit /b 1
)

echo.
echo [4/6] Building Tauri release...
call npm run tauri build
if %errorlevel% neq 0 (
    echo ERROR: Tauri build failed
    pause
    exit /b 1
)

echo.
echo [5/6] Testing release executable...
if exist src-tauri\target\release\weather-dashboard.exe (
    echo SUCCESS: Release executable created!
    echo Location: src-tauri\target\release\weather-dashboard.exe
) else (
    echo ERROR: Release executable not found
    pause
    exit /b 1
)

echo.
echo [6/6] Checking installer packages...
if exist src-tauri\target\release\bundle\msi\*.msi (
    echo SUCCESS: MSI installer created!
)
if exist src-tauri\target\release\bundle\nsis\*.exe (
    echo SUCCESS: NSIS installer created!
)

echo.
echo ========================================
echo BUILD COMPLETED SUCCESSFULLY!
echo ========================================
echo.
echo Release files location:
echo - Executable: src-tauri\target\release\weather-dashboard.exe
echo - MSI Installer: src-tauri\target\release\bundle\msi\
echo - NSIS Installer: src-tauri\target\release\bundle\nsis\
echo.
echo To test the app, run:
echo src-tauri\target\release\weather-dashboard.exe
echo.
pause
