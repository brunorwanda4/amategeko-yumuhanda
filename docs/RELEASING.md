# Signed releases

Repository: `https://github.com/brunorwanda4/amategeko-yumuhanda`.

### Android APK

- Create the release key once: `keytool -genkeypair -v -storetype PKCS12 -keystore amategeko-release.jks -alias amategeko -keyalg RSA -keysize 4096 -validity 10000`.
- Put `keystore.properties` in `crates/mobile/android/`; it is git-ignored and contains `storeFile`, `storePassword`, `keyAlias`, and `keyPassword`.
- Sign every release forever with the same key; Android refuses updates signed with a different key.
- Build with `crates/mobile/build.sh android --release --no-run`; the script verifies the signed and aligned APK automatically.
- Verify manually with `apksigner verify --verbose --print-certs <apk>` and `zipalign -c -P 4 -v 4 <apk>`.
- For local testing only, add `--allow-debug-signing`; the output ends in `-debugsigned.apk` and must never be published.

### Windows

- Install Inno Setup: `winget install -e --id JRSoftware.InnoSetup`.
- Build with `installer/build.ps1`; add `-Portable` when a portable ZIP is needed`.
- Sign the setup output manually with `minisign -Sm <file>`.
- Upload the setup `.exe` and its `.minisig` file to GitHub Releases.
- Never change the AppId in `installer/amategeko.iss`.
- Test a fresh install with the desktop shortcut ticked and unticked.
- Test an upgrade over an old install and a silent update from inside the app.
- Test uninstall while keeping data, then reinstall and test deleting data`.

### Web Build

- Target: `rustup target add wasm32-unknown-unknown`.
- wasm-bindgen: `cargo install wasm-bindgen-cli --version 0.2.129`.
- wasm-opt: Binaryen `version_122` via `winget install WebAssembly.binaryen`.
- Build: run `scripts/build-web.ps1` (or `scripts/build-web.sh`). Use `-Full` only with owner authorization.
- Outputs are hashed and written to `website/public/app/`, tracked in git for static hosting.

1. Bump workspace version in `Cargo.toml` and Android `versionCode`/`versionName`.
2. Update `CHANGELOG.md` with release additions and fixes.
3. Build the Windows installer (`installer/build.ps1`) and Android APK (`tools/package-android.ps1`).
4. Sign the installer and APK with minisign (`minisign -Sm <file> -s "$HOME\Documents\AmategekoSigning\update.key"`).
5. Tag `v<version>` equal to the Cargo version.
6. Upload release files and `.minisig` signatures to GitHub Releases.
7. Paste the release notes as the release body.
8. Publish the release (not draft, not prerelease).
9. Test the update from an older build.

The updater calls `GET /repos/brunorwanda4/amategeko-yumuhanda/releases/latest`,
which excludes drafts and prereleases, and reads `tag_name`, `body`, and assets.
