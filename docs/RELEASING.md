# Signed releases

Repository: `https://github.com/brunorwanda4/amategeko-yumuhanda`.

1. Bump workspace version in `Cargo.toml` and Android `versionCode`/`versionName`.
2. Update `CHANGELOG.md` with release additions and fixes.
3. Build the Windows installer (`tools/package-windows.ps1`) and Android APK (`tools/package-android.ps1`).
4. Sign the installer and APK with minisign (`minisign -Sm <file> -s "$HOME\Documents\AmategekoSigning\update.key"`).
5. Tag `v<version>` equal to the Cargo version.
6. Upload release files and `.minisig` signatures to GitHub Releases.
7. Paste the release notes as the release body.
8. Publish the release (not draft, not prerelease).
9. Test the update from an older build.

The updater calls `GET /repos/brunorwanda4/amategeko-yumuhanda/releases/latest`,
which excludes drafts and prereleases, and reads `tag_name`, `body`, and assets.
