# Signed releases

Repository: `https://github.com/brunorwanda4/amategeko-yumuhanda`.

1. Set the workspace version in `Cargo.toml`.
2. Tag the release `v<version>`; it must equal `env!("CARGO_PKG_VERSION")`.
3. Build these assets:
   - `amategeko-yumuhanda-<version>-windows-setup.exe`
   - `amategeko-yumuhanda-<version>-android-arm64.apk`
   Run `powershell -File tools/package-windows.ps1` for the Windows installer.
   Run `powershell -File tools/package-android.ps1` for the unsigned sideload APK.
   Android-sign the APK with the owner's offline Android keystore, then give it the release filename above.
4. The owner's local key is stored at `%USERPROFILE%\Documents\AmategekoSigning\update.key`; AI agents must never open or use it.
5. The owner signs the installer and APK by hand with `minisign -Sm <file> -s "$HOME\Documents\AmategekoSigning\update.key"`.
6. Upload each file and its matching `.minisig` to the same GitHub Release.

The updater calls `GET /repos/brunorwanda4/amategeko-yumuhanda/releases/latest`,
which excludes drafts and prereleases, and reads `tag_name`, `body`, and assets.
