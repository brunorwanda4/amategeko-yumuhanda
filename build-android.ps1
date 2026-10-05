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

Write-Host "==> [1/3] Building Rust shared library (arm64-v8a)..." -ForegroundColor Cyan
cargo ndk -t arm64-v8a -o crates/mobile/android/app/src/main/jniLibs --platform 31 build --release -p mobile

# Clean any stray debug/intermediate libraries
Get-ChildItem "crates/mobile/android/app/src/main/jniLibs/arm64-v8a" -File -Filter "libgpui_mobile-*.so" -ErrorAction SilentlyContinue | Remove-Item -Force

Push-Location "crates/mobile/android"
try {
    if ($Aab) {
        Write-Host "==> [2/3] Building Android App Bundle (AAB)..." -ForegroundColor Cyan
        .\gradlew.bat bundleRelease
        Write-Host "==> [3/3] Done! Bundle generated at:" -ForegroundColor Green
        Write-Host "crates/mobile/android/app/build/outputs/bundle/release/app-release.aab" -ForegroundColor Yellow
    } else {
        Write-Host "==> [2/3] Building signed Release APK..." -ForegroundColor Cyan
        .\gradlew.bat assembleRelease
        $apkPath = "app/build/outputs/apk/release/app-release.apk"
        Write-Host "==> [3/3] Done! APK generated at:" -ForegroundColor Green
        Write-Host "crates/mobile/android/$apkPath" -ForegroundColor Yellow

        if ($Install) {
            Write-Host "==> Installing on connected device..." -ForegroundColor Cyan
            adb install -r $apkPath
            Write-Host "==> Launching app..." -ForegroundColor Cyan
            adb shell am start -n "dev.gpui.mobile.example/dev.gpui.mobile.GpuiActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER
        }
    }
} finally {
    Pop-Location
}
