# Mobile Notes: GPUI Mobile on Android & iOS

Findings, constraints, and architecture notes for running "Amategeko y'Umuhanda" on mobile devices.

## 1. Target Platforms & Baselines
- **Android**:
  - Target: `aarch64-linux-android` (ARM64-v8a)
  - Minimum SDK: API 26 (Android 8.0 Oreo)
  - Target SDK: API 34+
  - Graphics: Vulkan backend preferred, with OpenGL ES 3.0 fallback via wgpu
  - Entry point: NativeActivity via `android-activity` crate (`android_main`), hosted by `dev.gpui.mobile.GpuiActivity`
  - Manifest: 100% offline (NO `android.permission.INTERNET`)
  - Validated native binary: `libgpui_mobile_app.so` in `jniLibs/arm64-v8a`
  - Validated APK artifact: `crates/mobile/android/app/build/outputs/apk/debug/app-debug.apk`
- **iOS** (optional, Mac required):
  - Target: `aarch64-apple-ios` (device) / `aarch64-apple-ios-sim` (simulator)
  - Minimum iOS: 13.0+
  - Graphics: Metal via wgpu

## 2. Experimental Mobile Platform Constraints & Fallbacks
- **Text Input / Soft Keyboard (IME)**:
  - Mobile IME text composition in `gpui-mobile` is experimental.
  - **Fallback**: The "Ibibazo" screen supports numeric question lookup with large on-screen button filters so users are never blocked by virtual keyboard or IME quirks.
- **Touch Scrolling vs Taps**:
  - `gpui-mobile` translates touch events into mouse/pointer events with momentum scrolling.
  - Option cards have explicit padding and minimum touch targets >= 44x44 px (buttons >= 48px high) to avoid accidental triggers during scrolls.
- **Safe Area Insets**:
  - Top app bar accounts for camera cutouts / status bar.
  - Sticky bottom action bars float safely above the Android gesture navigation pill / system navigation bar.
- **Application Lifecycle**:
  - Android activity pause/stop events save the active `in_progress.json` quiz attempt immediately.
  - Timers use wall-clock deadlines (`now >= deadline`) instead of frame counts, guaranteeing accurate remaining time after screen locks or task switching.

## 3. Storage & Assets
- Desktop uses OS data directory via `directories` crate.
- Mobile uses the application's private files directory through the `Storage` trait.
- Assets (`questions.json` and sign images in `assets/images/`) are embedded via `include_str!` / `include_bytes!` or loaded from bundled app assets, ensuring zero network requirement.

## 4. Build Pipeline Verified on Windows
To build the Android native library and package the debug APK:
```powershell
# 1. Environment variables
$env:NDK_HOME = "C:\Users\rbhap\AppData\Local\Android\Sdk\ndk\27.1.12297006"
$env:ANDROID_NDK_HOME = "C:\Users\rbhap\AppData\Local\Android\Sdk\ndk\27.1.12297006"
$env:JAVA_HOME = "C:\Users\rbhap\AppData\Local\android-dev\jdk\jdk-17.0.20+8"

# 2. Build ARM64 native shared library into Android jniLibs
cargo ndk -t arm64-v8a --platform 31 -o crates/mobile/android/app/src/main/jniLibs build -p mobile

# 3. Assemble Debug APK with Gradle
cd crates/mobile/android
cmd /c "gradlew.bat assembleDebug"
```
Output APK is located at `crates/mobile/android/app/build/outputs/apk/debug/app-debug.apk`.

### Release APK startup fix (2026-10-08)

- R8 removed Java methods invoked by name from Rust/JNI, causing minified release APKs to close during startup.
- `proguard-rules.pro` now preserves `GpuiActivity` and `MainActivity`, including their JNI bridge methods.
- Verified by building the ARM64 sideload release and checking the generated R8 mapping. Physical-device launch still requires owner testing.
