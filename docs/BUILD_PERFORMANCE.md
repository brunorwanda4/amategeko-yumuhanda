# Rust development performance

Configuration review: 2026-10-05. This is a GPUI Cargo workspace on Windows MSVC,
not Tauri. Profiles belong in the root manifest. The app keeps optimization level
1; dependencies use level 2. Line-table debug information and incremental
compilation are enabled. Optimized dependencies trade initial compilation time
for runtime speed; this is not a guarantee of the fastest cold build.

## Daily commands

```powershell
cargo check -p desktop
cargo clippy -p desktop --all-targets -- -D warnings
bacon
cargo run -p desktop
cargo build -p desktop --timings
```

Use `cargo test -p amategeko-core` for the fast logic-test loop. Before committing,
run the workspace checks required by AGENTS.md. Bacon defaults to desktop checks;
`bacon clippy` runs desktop linting and `bacon test` runs core tests. Stop Bacon
before benchmarking to avoid contention. Prefer consistent package selections:
switching between desktop and workspace builds can change unified dependency
features and require additional compilations.

Local `.vscode/settings.json` enables `rust-analyzer.cargo.targetDir = true` and
`rust-analyzer.check.command = "clippy"`. The separate analyzer target directory
avoids the terminal target lock, at the cost of extra compilation and disk space.
The IDE file is left uncommitted under AGENTS.md's IDE-file rule.

## Optional compiler cache

`sccache --version` was unavailable. No active wrapper is configured, so Cargo
continues to work. Install it manually, then uncomment the `[build]` wrapper
example in `.cargo/config.toml`:

```powershell
cargo install sccache --locked
sccache --version
```

sccache cannot cache incrementally compiled Rust crates. Keep incremental builds
for local edits; caching is mainly useful for reusable non-incremental dependency
compilations. See [sccache Rust support](https://github.com/mozilla/sccache/blob/main/docs/Rust.md).
Bacon 3.26.0 is already installed; on another machine use `cargo install bacon --locked`.

## Dependency feature audit

Inspected `cargo tree -e features`, reverse feature trees, and the pinned source
manifests. No dependency versions or features were changed:

- `gpui` and `gpui-kit` already disable default features in the root manifest.
- `serde` defaults only enable `std`, which this application uses; `derive` is
  also required. Replacing defaults with those same features saves no work.
- `reqwest` is absent. Tokio is present in the cross-platform dependency metadata
  but not the current Windows feature tree; its default feature set is empty.
- GPUI explicitly enables many `image` formats despite disabling image defaults.
  Cargo features are additive: adding a narrower root dependency cannot disable
  those features. An upstream change or maintained fork would be needed.
- gpui-mobile enables a broad plugin set. A future Android audit could retain
  only the plugins used by startup, storage, and URL launching. This needs device
  and packaging verification; Windows compilation alone cannot establish safety.

## Existing timing report

Source: `target/cargo-timings/cargo-timing-20261005T151027888Z-fff7f2b85010d0e4.html`.
Elapsed: **1496.2 seconds (24m 56.2s)**, with **168 fresh units and 402 dirty units**.
This is a partially cached baseline, not a verified clean build. Neither
`cargo clean` nor `cargo build --timings` was rerun, at the owner's request.
Crate durations overlap and must not be summed to estimate wall time.

| Slowest compilation unit | Seconds | Possible improvement |
| --- | ---: | --- |
| windows 0.62.2 | 817.62 | Upstream audit of Win32 feature selections; preserve its build cache. |
| image 0.25.10 | 289.43 | Upstream GPUI image-format feature controls; root overrides cannot subtract features. |
| moxcms 0.8.1 | 206.56 | Unconditional image dependency; cannot remove with an image-format toggle. |
| regex-automata 0.4.18 | 191.01 | Audit upstream regex Unicode/performance needs; preserve behavior and cache. |
| derive_more-impl 2.1.1 | 165.52 | Upstream derive-feature audit; avoid global rustflags changes that rebuild macros. |

## Manual Windows options

No system settings were changed.

1. To add Defender folder exclusions, open Windows Security > Virus & threat
   protection > Manage settings > Exclusions > Add or remove exclusions > Add an
   exclusion > Folder. Select the project directory, its `target` directory, or
   `%USERPROFILE%\.cargo` (expand the environment variable in the folder picker).
   Excluding the project already covers `target`, so adding both is redundant.
   Exclusions reduce malware protection; prefer the narrower `target` exclusion
   or Dev Drive performance mode when possible.
2. On supported Windows 11 systems, open Settings > System > Storage > Advanced
   storage settings > Disks & volumes > Create Dev Drive. Create a ReFS Dev Drive
   (at least 50 GB), copy the repository there, and open that copy in your editor.
   Keep Defender enabled and use its performance mode for the trusted Dev Drive.
   A session-local `$env:CARGO_TARGET_DIR = 'D:\cargo-target\amategeko'` can place
   build artifacts there instead; replace `D:` with your Dev Drive letter.

References: [Cargo build performance](https://doc.rust-lang.org/cargo/guide/build-performance.html),
[rust-analyzer configuration](https://rust-analyzer.github.io/book/configuration),
[Defender exclusions](https://support.microsoft.com/en-us/windows/security/threat-malware-protection/virus-and-threat-protection-in-the-windows-security-app),
[Dev Drive setup](https://learn.microsoft.com/en-us/windows/dev-drive/).
