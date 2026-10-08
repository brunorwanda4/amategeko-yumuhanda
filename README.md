# Rwanda Road Rules Trainer (Amategeko y'Umuhanda)
https://amategeko-yumuhanda-alpha.vercel.app/

An offline study app for the Rwandan driving-test theory exam. Choose 5–100 questions in Easy and Medium, practice the fixed 20-question Hard exam, review your mistakes, browse 404 questions with sign images, and track your progress. Runs on **Windows (Desktop)** and **Android (Mobile)** from one single Rust codebase.

Built with [GPUI](https://gpui.rs), [GPUI Kit](https://gpui-kit.com/) and [gpui-mobile](https://github.com/longbridge/gpui-mobile). The entire interface is in **Kinyarwanda**.

> **Status:** Fully implemented and verified. Both desktop Windows executable and Android ARM64 APK build cleanly with zero warnings (`-D warnings` enforced). Check `docs/PROGRESS.md` for milestone details.

> **Disclaimer:** This is a study tool designed to aid preparation for the provisional driving license exam. The question bank is extracted from study materials. Always confirm official rules and regulations with the Rwanda National Police (RNP).

## Download version 1.8.0

Download the latest release from [GitHub Releases](https://github.com/brunorwanda4/amategeko-yumuhanda/releases/latest):

- **Windows:** `amategeko-yumuhanda-1.8.0-windows-setup.exe`
- **Android:** `amategeko-yumuhanda-1.8.0-android-arm64.apk` (Android 8.0 or newer, ARM64)

---

## Features

- **Three Quiz Modes** (a chosen question count for Easy and Medium; 20 randomized questions for Hard):
  - **Byoroshye (Easy):** Learning mode with instant visual feedback (green/red highlights and explanation), star bookmarking, free back/forward/skip navigation, locked answer upon first choice, and optional elapsed time indicator.
  - **Hagati (Medium):** Exam simulation with a countdown timer (default 20 min, turns red in last 2 mins), 20-cell question jump grid, answer modification, question flagging, and auto-submit upon timer expiration or manual finish dialog.
  - **Bikomeye (Hard):** Strict exam conditions with a 12-minute timer, weighted towards complex questions and sign/marking images, segmented progress, single confirmation button per question, and strict forward-only progression (no skipping, no going back, no grid).
- **Practice Modes**:
  - **Ibibazo Nakosheje (Weak Questions Practice):** Automatically generates a 20-question practice quiz drawing primarily from questions you have answered incorrectly most often.
  - **Subiramo Ibyo Wakosheje (Retry Wrong):** Directly retry all missed or unanswered questions from your last attempt under Easy rules.
- **Ibibazo (Question Bank & Study Browser)**:
  - Search by question text or question number (e.g. `12` or `amatara`).
  - Filter chips: All, Has Image, Mistakes Only, Starred Only.
  - **"Hisha ibisubizo" (Hide Answers) Interactive Flashcard Mode:** Answers are hidden until tapped, letting you test yourself on each question individually.
  - Persisted star/bookmark status for every question.
- **Imibare (Comprehensive Statistics Dashboard)**:
  - 4 key summary metrics: Total attempts, average score, high score, and overall pass rate percentage.
  - 10-attempt vertical score bar chart with green/red bars and a clear pass-threshold reference line.
  - Mode breakdown statistics table (Byoroshye, Hagati, Bikomeye, Weak practice).
  - Overall question bank coverage progress bar (e.g. 150/404 seen).
  - Most-missed questions ranking with a direct "Gukora Imyitozo" practice launcher.
- **Igenamiterere (Customizable Settings)**:
  - Pass mark stepper (10 to 20; default 12/20).
  - Medium mode duration stepper (10 to 40 min; default 20 min).
  - Hard mode duration stepper (5 to 20 min; default 12 min).
  - Easy mode elapsed timer visibility toggle.
  - Hard mode image weighting toggle.
  - Desktop keyboard shortcuts toggle (Aâ€“D, 1â€“4, Enter, F).
  - **Theme Switcher:** System, Light, and Dark modes with live GPUI Kit theme synchronization.
  - **Font Size Scaler:** 85% to 125% with live sample preview card.
  - **Akarere ko Kwitonda (Danger Zone):** Clear all attempt history, statistics, and saved state with double-confirmation protection.
- **Deadline-Based Timers & State Persistence**:
  - Timers calculate remaining time against absolute start time and duration (`start_time + duration - now`). The timer remains accurate if the phone screen locks, the app is backgrounded, or the process is killed.
  - In-progress attempts are continuously persisted to local storage after every answer. If the app is closed mid-quiz, users are greeted on launch with a "Komeza ikizamini" resume option. Expired timed attempts are automatically submitted to Results upon app launch.
- **Responsive Multi-Platform Shell**:
  - Desktop layout (window width >= 700px): Collapsible left sidebar navigation (`160px`) and wide content area.
  - Mobile layout (window width < 700px, Android/iOS): Safe-area padded top app bar and bottom navigation bar (hidden during active quizzes for distraction-free focus mode). Touch targets meet or exceed 44Ã—44 px minimums (options >= 48px).
- **100% Offline & Private**:
  - No internet connection required (`android.permission.INTERNET` removed).
  - Zero analytics, zero telemetry, zero ads, zero user tracking.

---

## Screens

| Screen | Kinyarwanda Title | Functionality |
| :--- | :--- | :--- |
| **Home** | Ahabanza | Mode selection cards, quick stats overview, active quiz resume banner, weak questions practice launcher |
| **Quiz** | Ikizamini | Active question view for Byoroshye, Hagati, and Bikomeye with timers, answer options, grid, and navigation |
| **Results** | Ibisubizo | Circular score badge, pass/fail banner, filtered question review cards, and "Subiramo ibyo wakosheje" retry |
| **Questions** | Ibibazo | Searchable, filterable question bank with interactive flashcard mode and bookmark toggles |
| **Statistics** | Imibare | Metric tiles, 10-attempt bar chart, mode breakdown, and top-missed question list |
| **Settings** | Igenamiterere | Pass mark, timers, theme switcher, font size slider, feature switches, and data reset |

---

## Local Rust development settings

The dev profile uses incremental compilation and line-table debug information.
Workspace code keeps optimization level 1; dependencies use level 2.
On Windows MSVC, `.cargo/config.toml` selects `rust-lld.exe` bundled with the active
Rust toolchain, without a machine-specific absolute path. Other targets keep
their linkers.

Local Zed settings in `.zed/settings.json` (git-ignored) select no extra Cargo
features and preserve manifest defaults. Open the workspace root in Zed. At most
two analyzer Cargo jobs run, and all-target loading is disabled to reduce memory
pressure on this 8 GB machine. There is no Tauri subproject.
Global Zed settings use `cargo check` and a separate
`target/rust-analyzer` cache. That cache avoids build-lock contention but uses
additional disk space. Switch `lsp.rust-analyzer.initialization_options.check.command`
to `clippy` for lint warnings, or run `cargo clippy --workspace --all-targets`.

`sccache` was not installed when this setup was configured, so no wrapper is set.
Install it with `cargo install sccache --locked`, then enable the commented wrapper
in `.cargo/config.toml`. Run `bacon` for continuous desktop checks (already installed
on this machine). Local VS Code settings isolate rust-analyzer's target directory
and enable Clippy. See [Rust build performance](docs/BUILD_PERFORMANCE.md) for the
dependency audit, timing results, Windows options, and daily commands.

## Project Structure

```
amategeko-yumuhanda/
â”œâ”€â”€ assets/
â”‚   â”œâ”€â”€ questions.json           # 404 structured questions with options, answer & image metadata
â”‚   â”œâ”€â”€ images/                  # 130 referenced PNG road sign and marking images
â”‚   â””â”€â”€ overrides.json           # Optional manual overrides for question data
â”œâ”€â”€ crates/
â”‚   â”œâ”€â”€ core/                    # Pure Rust domain logic (zero GPUI dependency, 100% testable)
â”‚   â”‚   â”œâ”€â”€ src/
â”‚   â”‚   â”‚   â”œâ”€â”€ models.rs        # Question, QuizMode, Attempt, Result, Settings, Progress
â”‚   â”‚   â”‚   â”œâ”€â”€ quiz.rs          # QuizEngine logic for Easy, Medium, Hard, WeakPractice, RetryWrong
â”‚   â”‚   â”‚   â”œâ”€â”€ timer.rs         # Deadline-based monotonic/wall-clock timer calculation
â”‚   â”‚   â”‚   â”œâ”€â”€ stats.rs         # StatsCalculator for summary, bar chart, and missed questions
â”‚   â”‚   â”‚   â”œâ”€â”€ data.rs          # QuestionBank loading, filtering, and text/numeric search
â”‚   â”‚   â”‚   â”œâ”€â”€ platform.rs      # Storage and Clock abstraction traits
â”‚   â”‚   â”‚   â””â”€â”€ i18n.rs          # 100% Kinyarwanda UI string catalog (Strings struct)
â”‚   â”‚   â””â”€â”€ tests/core_tests.rs  # Full unit test suite (10 tests covering all domain rules)
â”‚   â”œâ”€â”€ app/                     # Shared presentation layer (GPUI + GPUI Kit components)
â”‚   â”‚   â””â”€â”€ src/
â”‚   â”‚       â”œâ”€â”€ state.rs         # AppState managing persistent progress, settings, and navigation
â”‚   â”‚       â””â”€â”€ ui/              # ShellView, HomeView, QuizView, ResultsView, QuestionsView, StatsView, SettingsView
â”‚   â”œâ”€â”€ desktop/                 # Windows Desktop binary entry point
â”‚   â”‚   â””â”€â”€ src/main.rs          # Native desktop window setup, GPUI Kit theme init, disk storage
â”‚   â””â”€â”€ mobile/                  # Android and iOS mobile native library entry point
â”‚       â”œâ”€â”€ src/lib.rs           # android_main (android-activity + jni) and gpui_ios_register_app
â”‚       â””â”€â”€ android/             # Gradle wrapper, AndroidManifest, NativeActivity, and APK build setup
â”œâ”€â”€ tools/
â”‚   â”œâ”€â”€ extract_questions.py     # PDF extraction script using pdfplumber and Pillow
â”‚   â””â”€â”€ requirements.txt         # Python dependencies for extraction
â””â”€â”€ docs/
    â”œâ”€â”€ PROGRESS.md              # Milestone tracking and completion status
    â””â”€â”€ SPEC.md                  # Comprehensive product specification
```

---

## Technology Stack & Versions

- **Rust:** `1.85+` stable
- **GPUI Ecosystem:**
  - `gpui-pre = "=0.3.4"` (wgpu/Vulkan backend)
  - `gpui-pre-mobile = "0.1.0"` (Android JNI & iOS Metal platform integration)
  - `gpui-kit` (git rev `7d9efcd2069f9eaa6eb3ba6345aac4aa7d87c9f7`)
- **Android Target:**
  - Architecture: `arm64-v8a` (`aarch64-linux-android`)
  - Minimum SDK: `26` (Android 8.0 Oreo, required for Vulkan 1.0)
  - Target SDK: `34` (Android 14)
  - NDK: `r25+` (tested with NDK `27.1.12297006`)

---

## Building and Running

### Fast Development Loop (Auto Rebuild & State Restore)

For rapid development on desktop, file watchers and automatic state restoration are supported:

#### Prerequisites
Install `watchexec`:
```powershell
cargo install watchexec-cli
```

#### Running Auto-Rebuild Tasks
Using `just`:
```bash
just dev    # Watch crates/ & assets/, rebuild and restart desktop app on change
just check  # Watch crates/, run cargo check --workspace on change
```

Or using PowerShell script on Windows:
```powershell
.\scripts\dev.ps1 dev    # Auto rebuild and restart desktop app
.\scripts\dev.ps1 check  # Continuous workspace typecheck
```

#### State Restoration (Debug Builds Only)
- In debug builds, the active screen, open Browse/Stats filters, and scroll position are automatically saved to `dev_state.json` on change and restored on next startup.
- Together with persisted window bounds and in-progress quiz attempts, the app restarts directly back to your exact working state.
- In release builds, `dev_state.json` tracking and restore logic are completely omitted (`#[cfg(debug_assertions)]`).

#### Testing Timer Warning & Expiration States
To test Medium/Hard countdown alert banners and auto-submission without waiting for 12 or 20 minutes, pass the debug-only `--timer` flag:
```powershell
cargo run -p desktop -- --timer 30s
```

### 1. Windows Desktop

#### Prerequisites
- Rust stable: `rustup default stable`
- Visual Studio Build Tools with **Desktop development with C++** and the Windows SDK.

#### Run in Debug Mode
```powershell
cargo run -p desktop
```

#### Build Release Binary
```powershell
cargo build -p desktop --release
```
The optimized executable will be located at `target/release/amategeko.exe`.

## Build and install (Windows)

Install Inno Setup 6, then run the installer build from the repository root:
`.\scripts\build-installer.ps1`
The installer is written to `dist\amategeko-yumuhanda-1.8.0-windows-setup.exe`.

---

### 2. Android Mobile (ARM64 APK)

#### Prerequisites
- Android SDK (API 34) and NDK (r25+).
- Rust Android target:
  ```powershell
  rustup target add aarch64-linux-android
  cargo install cargo-ndk
  ```
- Java JDK 17 (e.g. `jdk-17.0.20+8`).

#### Step A: Compile Native ARM64 Library
From the repository root, compile the Rust cdylib and export it to the Android app's `jniLibs` folder:
```powershell
$env:ANDROID_NDK_ROOT = 'C:\Users\<username>\AppData\Local\Android\Sdk\ndk\27.1.12297006'
$env:NDK_HOME = $env:ANDROID_NDK_ROOT
cargo ndk -t arm64-v8a --platform 31 -o crates/mobile/android/app/src/main/jniLibs build -p mobile
```

#### Step B: Assemble the APK with Gradle
```powershell
$env:JAVA_HOME = 'C:\Users\<username>\AppData\Local\android-dev\jdk\jdk-17.0.20+8'
cd crates/mobile/android
.\gradlew.bat assembleDebug
```
The built APK will be located at:
```
crates/mobile/android/app/build/outputs/apk/debug/app-debug.apk
```

#### Step C: Install on Android Device
Enable **Developer Options** and **USB Debugging** on your Android device, connect via USB, and run:
```powershell
adb install -r crates/mobile/android/app/build/outputs/apk/debug/app-debug.apk
```

---

## Running Tests and Linting

Enforce zero compiler warnings and verify 100% of domain test assertions:

```powershell
# Run all unit tests (models, quiz engine, timers, persistence, stats)
cargo test --workspace

# Run Clippy across all workspace crates and targets with warnings treated as errors
cargo clippy --workspace --all-targets -- -D warnings

# Check Android compilation from Windows
cargo check --target aarch64-linux-android -p mobile
```

---

## Question Data Extraction

The question bank was extracted directly from the supplied PDF (`ibibazo_byamategeko_y_umuhanda.pdf`, 75 landscape pages). Its numbering ends at 433, but this copy contains 404 recoverable question records; see `docs/PROGRESS.md` for the source gaps. To re-run or inspect the extraction pipeline:

```powershell
cd tools
pip install -r requirements.txt
python extract_questions.py
```

- Correct answers are automatically detected via bracketed letters (e.g. `(a)`, `(b)`, `(c)`, `(d)`).
- Two-column layout splitting handles multi-line options and questions spanning page/column breaks.
- Road sign and marking images are cropped and saved to `assets/images/q{id}.png`.
- Validation ensures 3â€“4 options per question and exactly 1 correct answer. Manual overrides can be placed in `assets/overrides.json` without modifying extraction scripts.

---

## License

This project is open-source under the [Apache-2.0 License](LICENSE).
The traffic exam questions and sign images belong to their respective official Rwandan authorities and are included for educational study purposes.
