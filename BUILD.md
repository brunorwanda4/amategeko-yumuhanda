# Build commands

Run commands from the project folder in PowerShell.

## Build everything for testing

```powershell
./tools/build-all.ps1
```

This creates two files in `dist`:

- A Windows installer ending in `-windows-setup.exe`.
- An Android APK ending in `-android-arm64-debugsigned.apk`.

The debug-signed APK is for installing and testing on your own Android device. Do not publish it as a release.

## Build everything for a release

```powershell
./tools/build-all.ps1 -Release
```

This requires your permanent Android release keystore. The release APK ends in `-android-arm64.apk`. Always use the same keystore so Android accepts updates.

## Build only one application

Windows installer:

```powershell
./tools/package-windows.ps1
```

Android APK for local testing:

```powershell
./tools/package-android.ps1 --allow-debug-signing
```

Android APK for release:

```powershell
./tools/package-android.ps1
```

The Android release command requires `crates/mobile/android/keystore.properties` or the `ANDROID_KEYSTORE_*` environment variables.

## Required tools

- Rust and Cargo.
- Inno Setup 6 for the Windows installer.
- Android SDK, Android NDK, and `cargo-ndk` for the Android APK.

If PowerShell blocks a script, run this once in the current terminal and try again:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
```
