# Rwanda Road Rules Trainer (Amategeko y'Umuhanda)

An offline study app for the Rwandan driving-test theory exam. Practice 20 random questions with a timer, review your mistakes, browse all 433 questions, and track your progress. Runs on **Windows** and **Android** from one Rust codebase.

Built with [GPUI](https://gpui.rs), [GPUI Kit](https://gpui-kit.com/) and [gpui-mobile](https://github.com/longbridge/gpui-mobile). The interface is in **Kinyarwanda**.

> **Status:** in development. This README describes the planned app. Check `docs/PROGRESS.md` to see which milestones are finished.

> **Disclaimer:** this is a personal study tool, not an official exam. The question bank comes from a study document and may contain mistakes. Always check the official rules and the official pass mark.

## Features

- **Three quiz modes**, each with 20 random questions:
  - **Byoroshye (Easy):** instant feedback after every answer, no time limit, skip and go back freely.
  - **Hagati (Medium):** exam simulation with a countdown (default 20 min), a question-number grid, flags, and results at the end.
  - **Bikomeye (Hard):** shorter time (default 12 min), more sign and tricky questions, no going back, no skipping.
- **Results review:** score, pass or fail, time used, and every question with your answer next to the correct one.
- **Retry what you got wrong**, plus a practice mode built from the questions you miss most.
- **Ibibazo (question bank):** search by text or number, filter, star questions, hide answers to test yourself.
- **Imibare (stats):** attempts, average, best score, pass rate, last 10 scores, most-missed questions.
- **Igenamiterere (settings):** quiz times, pass mark, light/dark theme, font size.
- **Road-sign questions** with images.
- **Works offline.** No account, no ads, no tracking.
- **Phone-friendly:** the same app with a mobile layout (bottom navigation, big touch targets), a deadline-based timer that stays correct if the phone locks, and resume of an unfinished quiz.

## Screens

| Screen | What it does |
| --- | --- |
| Ahabanza (Home) | Pick a mode, see quick stats, resume an unfinished quiz |
| Ikizamini (Quiz) | The question page for Easy, Medium and Hard |
| Ibisubizo (Results) | Score and full review of the attempt |
| Ibibazo (Questions) | Browse and study all 433 questions |
| Imibare (Stats) | Your progress over time |
| Igenamiterere (Settings) | Times, pass mark, theme, font size |

## Quick start

### Requirements

- Rust (stable) from [rustup.rs](https://rustup.rs)
- **Windows desktop:** Visual Studio Build Tools with "Desktop development with C++" and the Windows SDK
- **Android:** Android SDK, NDK r25 or newer, `cargo-ndk`
- **iOS (optional):** a Mac with Xcode 15+ and XcodeGen

### Run on desktop

```bash
git clone https://github.com/<your-username>/rwanda-road-rules-trainer.git
cd rwanda-road-rules-trainer
cargo run -p desktop --release
```

### Build for Android

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
cd crates/mobile
./build.sh android --device --release
```

The script comes from the gpui-mobile example. Check `MOBILE_NOTES.md` for what works on mobile today. Supported target: Android arm64 (API 26+).

### Install on your phone

1. Build the release APK (see above).
2. Copy the APK to your phone, or run `adb install path/to/app.apk` with USB debugging on.
3. On the phone, allow "install unknown apps" for the app you used to open the APK, then install it.

Publishing on Google Play is not required.

## Project structure

```
.
├── AGENTS.md              rules for AI coding agents
├── docs/
│   ├── SPEC.md            full product specification
│   └── PROGRESS.md        milestone checklist
├── assets/
│   ├── questions.json     the question bank (generated)
│   ├── images/            sign and marking images
│   └── overrides.json     manual fixes to questions
├── tools/
│   └── extract_questions.py   one-time PDF to JSON extractor
└── crates/
    ├── core/              quiz rules, stats, data, i18n (no GPUI)
    ├── app/               shared GPUI + GPUI Kit screens
    ├── desktop/           desktop entry point
    └── mobile/            Android and iOS entry points
```

## Question data

The questions come from `ibibazo_byamategeko_y_umuhanda.pdf` (433 questions, Kinyarwanda). The app never reads the PDF. A one-time tool converts it to `assets/questions.json`:

```bash
pip install -r tools/requirements.txt
python tools/extract_questions.py
```

- In the PDF, the correct option is the letter written in brackets, like `(d)`.
- The tool checks every question and lists problems in `tools/needs_review.json`.
- To fix a wrong question, edit `assets/overrides.json`. Do not edit `questions.json` by hand.

**Note on the source material:** the PDF is marked "RESTRICTED". Make sure you are allowed to share the questions before publishing `questions.json` or the images in a public repository.

## Development

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --target aarch64-linux-android -p mobile
```

- The quiz rules, timers and stats live in `crates/core` and are covered by unit tests.
- All screens are written once in `crates/app` and adapt to desktop or mobile by window width.
- All Kinyarwanda text is in `crates/core/src/i18n.rs`.
- Read `AGENTS.md` before using an AI coding agent on this repository.

## Roadmap

- [ ] Extract the question bank and images
- [ ] Desktop skeleton and Android test build
- [ ] Quiz engine, Easy mode, Medium mode, Hard mode
- [ ] Results, question bank, stats, settings
- [ ] Polish, tests and phone install guide
- [ ] iOS build (optional)

See `docs/PROGRESS.md` for the detailed checklist.

## Known limitations

- gpui-mobile is experimental. Text input and some lifecycle features on phones may be limited.
- Android arm64 is the only tested phone target.
- The default pass mark is 12 out of 20. Check the official requirement and change it in Settings if needed.

## License

Apache-2.0. See `LICENSE`. The question content belongs to its original source and is not covered by this license.

## Acknowledgements

- [Zed Industries](https://zed.dev) for GPUI
- [Longbridge](https://longbridge.com) for GPUI Kit and gpui-mobile
