param(
    [switch]$Install,
    [switch]$Aab
)

$ErrorActionPreference = "Stop"

# Auto-detect and set environment variables
$SdkPath = "$env:LOCALAPPDATA\Android\Sdk"
$NdkPath = "$SdkPath\ndk\27.1.12297006"
$JdkPath = "C:\Program Files\Microsoft\jdk-17.0.20.101-hotspot"

if (Test-Path $NdkPath) {
    $env:ANDROID_NDK_HOME = $NdkPath
    $env:NDK_HOME = $NdkPath
}
if (Test-Path $JdkPath) {
    $env:JAVA_HOME = $JdkPath
}
if (Test-Path "$SdkPath\platform-tools") {
    $env:PATH = "$SdkPath\platform-tools;" + $env:PATH
}

$AdbPath = "$SdkPath\platform-tools\adb.exe"
if ($Install) {
    if (-not (Test-Path -LiteralPath $AdbPath)) {
        throw "ADB was not found at $AdbPath. Install Android SDK Platform-Tools."
    }

    & $AdbPath start-server | Out-Null
    $adbDevices = & $AdbPath devices
    if ($LASTEXITCODE -ne 0) {
        throw "ADB failed with exit code $LASTEXITCODE."
    }
    if ($adbDevices -match "`tunauthorized$") {
        throw "The Android device is unauthorized. Unlock it and accept the USB debugging prompt."
    }
    if (-not ($adbDevices -match "`tdevice$")) {
        throw "No Android device or emulator is connected. Enable USB debugging, connect a data-capable USB cable, and run 'adb devices' before using -Install."
    }
}

Write-Host "==> [1/3] Building Rust shared library (arm64-v8a)..." -ForegroundColor Cyan
cargo ndk -t arm64-v8a -o crates/mobile/android/app/src/main/jniLibs --platform 31 build --release -p mobile
if ($LASTEXITCODE -ne 0) {
    throw "The Rust Android build failed with exit code $LASTEXITCODE."
}

# Clean any stray debug/intermediate libraries
Get-ChildItem "crates/mobile/android/app/src/main/jniLibs/arm64-v8a" -File -Filter "libgpui_mobile-*.so" -ErrorAction SilentlyContinue | Remove-Item -Force

Push-Location "crates/mobile/android"
try {
    if ($Aab) {
        Write-Host "==> [2/3] Building Android App Bundle (AAB)..." -ForegroundColor Cyan
        .\gradlew.bat bundleRelease
        if ($LASTEXITCODE -ne 0) {
            throw "The Android App Bundle build failed with exit code $LASTEXITCODE."
        }
        Write-Host "==> [3/3] Done! Bundle generated at:" -ForegroundColor Green
        Write-Host "crates/mobile/android/app/build/outputs/bundle/release/app-release.aab" -ForegroundColor Yellow
    } else {
        Write-Host "==> [2/3] Building signed Release APK..." -ForegroundColor Cyan
        .\gradlew.bat assembleRelease
        if ($LASTEXITCODE -ne 0) {
            throw "The Android APK build failed with exit code $LASTEXITCODE."
        }
        $apkPath = "app/build/outputs/apk/release/app-release.apk"
        Write-Host "==> [3/3] Done! APK generated at:" -ForegroundColor Green
        Write-Host "crates/mobile/android/$apkPath" -ForegroundColor Yellow

        if ($Install) {
            Write-Host "==> Installing on connected device..." -ForegroundColor Cyan
            & $AdbPath install -r $apkPath
            if ($LASTEXITCODE -ne 0) {
                throw "APK installation failed with exit code $LASTEXITCODE."
            }
            Write-Host "==> Launching app..." -ForegroundColor Cyan
            & $AdbPath shell am start -n "dev.gpui.mobile.example/dev.gpui.mobile.GpuiActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER
            if ($LASTEXITCODE -ne 0) {
                throw "App launch failed with exit code $LASTEXITCODE."
            }
        }
    }
} finally {
    Pop-Location
}
