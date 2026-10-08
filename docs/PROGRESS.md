# Project Progress & Milestones

Tracking milestones for "Amategeko y'Umuhanda" (Rwanda driving-test theory study app in Rust with GPUI + GPUI Kit).

## Rust development setup (2026-10-05)

- [x] Merge global Zed Rust performance settings without replacing existing values;
  save a settings backup and validate JSONC preservation.
- [x] Configure local Zed manifest features, two analyzer Cargo jobs, and reduced
  target loading; retain the git-ignored `.zed` directory.
- [x] Make dev incremental compilation explicit, preserve existing optimization
  levels, and select the installed Windows MSVC `rust-lld.exe`. A standalone
  compile-and-run smoke test passed after replacing the incompatible
  `gcc-ld/lld-link.exe` entry point.
- [x] Review dependency features without changing them: gpui-mobile's broad
  default plugin set is a candidate for a separately approved mobile audit.
- [ ] Full verification: cargo check is waiting for an existing cargo build's
  target-directory lock. Formatting check found pre-existing differences in
  `crates/app/src/ui/quiz.rs` and `crates/app/tests/layout.rs`; source untouched.

## Application milestones

- [x] **Milestone 1: Question Bank & Asset Extraction** (Completed 2026-10-03)
  - [x] Implement `tools/extract_questions.py` and `tools/requirements.txt`
  - [x] Extract all recoverable questions into `assets/questions.json` (404 questions; corrected 2026-10-06)
  - [x] Extract, crop, and optimize sign/marking images into `assets/images/q{id}.png` (130 referenced images)
  - [x] Identify correct options via parenthesized letters `(a)`, `(b)`, `(c)`, `(d)` and PDF red text spans
  - [x] Validate 3-4 options per question, exactly 1 answer (0 errors in validation)
  - [x] Generate `tools/needs_review.json` and support `assets/overrides.json`
  - [x] Preserve existing app IDs while recovering 13 questions hidden by PDF numbering errors; fix answer detection for `(c.)` markers and red spans (2026-10-06)
  - [x] Recover the image question between source questions 271 and 273 from its visible choices/answer and the prompt preserved by matching copies (2026-10-06)
  - [ ] Obtain a corrected source for the remaining 29 records needed to reach 433. This PDF jumps from 139 to 171 and has other missing, duplicated, or incorrect labels.
- [x] **Milestone 2: Workspace Setup & Mobile Spike** (Completed 2026-10-03)
  - [x] Set up Cargo workspace (`crates/core`, `crates/app`, `crates/desktop`, `crates/mobile`)
  - [x] Pin exact GPUI/GPUI Kit/gpui-mobile dependencies
  - [x] Desktop skeleton with GPUI Kit init and Theme
  - [x] Mobile spike on Android (compiled native ARM64 shared library & verified Gradle APK packaging)
  - [x] Document findings in `MOBILE_NOTES.md`
- [x] **Milestone 3: Core Logic & Quiz Engine** (Completed 2026-10-03)
  - [x] Data models (`models.rs`), validation, question loader (`data.rs`)
  - [x] Platform abstraction traits (`Storage`, `Clock`) (`platform.rs`)
  - [x] Quiz engine (`quiz.rs`): Easy, Medium, Hard, weak questions practice
  - [x] Deadline-based monotonic/wall-clock timer (`timer.rs`)
  - [x] In-progress attempt persistence & recovery
  - [x] Statistics calculator (`stats.rs`)
  - [x] Complete Kinyarwanda translations (`i18n.rs`)
  - [x] Comprehensive unit tests for core (10/10 passed)
- [x] **Milestone 4: Responsive App Shell & Home Screen** (Completed 2026-10-03)
  - [x] Breakpoint-based responsive shell (`shell.rs`, desktop >= 700px vs mobile < 700px)
  - [x] Desktop sidebar navigation vs Mobile bottom navigation bar (focus mode hiding bar in quiz)
  - [x] Home screen (`home.rs`) with 3 mode cards, stats tiles, resume banner, weak practice card
- [x] **Milestone 5: Easy Mode (Layout A)** (Completed 2026-10-03)
  - [x] Instant feedback (green/red highlights, check/x icons, result explanation line)
  - [x] Free navigation (previous, next, skip), star/bookmark toggle
  - [x] Answer locking after first tap
- [x] **Milestone 6: Medium Mode (Layout A - Exam Simulation)** (Completed 2026-10-03)
  - [x] Countdown timer (turns red in last 2 mins), auto-submit at 0
  - [x] 20-cell question grid (desktop inline, mobile bottom sheet)
  - [x] Flags, answer changing, finish confirmation dialog listing unanswered questions
- [x] **Milestone 7: Hard Mode (Layout B - Strict Mode)** (Completed 2026-10-03)
  - [x] Shorter countdown, image question weighting
  - [x] Segmented progress bar, two-column desktop vs single-column mobile
  - [x] Strict navigation (no back, no skip, no pause, no grid)
  - [x] Selection validation before confirmation
- [x] **Milestone 8: Results Screen** (Completed 2026-10-03)
  - [x] Circular score badge, pass/fail status, filter chips
  - [x] Expandable question cards comparing user answer vs correct answer
  - [x] Retry mode & "Subiramo ibyo nakosheje" (retry wrong)
  - [x] Attempt history persistence
- [x] **Milestone 9: Ibibazo Screen (Question Bank / Study)** (Completed 2026-10-03)
  - [x] Search by text or question number
  - [x] Filter chips (all, has image, mistakes, starred)
  - [x] "Hisha ibisubizo" (hide answers) switch & flashcard interaction
  - [x] Smooth scrolling list of 404 questions, star persistence
- [x] **Milestone 10: Imibare Screen (Statistics)** (Completed 2026-10-03)
  - [x] Metric tiles (attempts, average, high score, pass rate)
  - [x] 10-attempt bar chart with pass mark threshold line
  - [x] Most-missed questions card + practice launcher
  - [x] Mode breakdown & questions-seen progress
- [x] **Milestone 11: Igenamiterere Screen (Settings)** (Completed 2026-10-03)
  - [x] Mode timer steppers (Medium 10..40m, Hard 5..20m), pass mark stepper (10..20)
  - [x] Theme segmented control (Light / Dark / System) with live GPUI Kit theme synchronization
  - [x] Font size scaler (0.85x .. 1.25x) with live preview card
  - [x] Feature switches (desktop shortcuts, image weighting, easy timer)
  - [x] Danger zone history reset with confirmation dialog
- [x] **Milestone 12: Polish, Mobile Optimization & Documentation** (Completed 2026-10-03)
  - [x] Mobile touch targets (>= 44px) & safe-area insets verified
  - [x] Android NativeActivity & library name placeholder alignment verified
  - [x] App lifecycle pause/resume save with deadline-based monotonicity
  - [x] Android ARM64 APK compilation verified (`app-debug.apk` built successfully)
  - [x] Comprehensive README updated with Windows & Android build/install guide
  - [x] Add branded Windows executable and Android launcher icons (2026-10-04)
  - [x] Restore the saved theme during desktop and mobile startup (2026-10-04)
  - [x] Refresh the quiz layout and add fullscreen focus mode and keyboard shortcuts (2026-10-04)
  - [x] Hide quiz focus mode on Android/iOS APK only (desktop OS keeps it, including narrow windows) (2026-10-05)
  - [x] Wire soft keyboard IME bridge for Questions search and hide desktop shortcut hints on Android/iOS APK (2026-10-06)
  - [x] Implement Medium and Hard timer warning states (2026-10-04: level-aware pill, remaining-time bar, dismissible alerts, reduced-motion-safe pulse, and expiry submission)
  - [x] Desktop keyboard shortcut registry (`crates/app/src/shortcuts.rs`) with help dialog (`?` / `F1`), button badges, input focus detection, mode restrictions (Hard strict mode disables navigation/back; Easy blocks Next before answering; Medium Ctrl+Enter finish confirmation), and desktop-only Settings switch (Cmd on macOS, Ctrl elsewhere). Note: Stats 1–4 filters are registered in registry but their action is deferred until stats screen filter controls are built. (2026-10-04)
  - [x] Add responsive author credit footer to Home and Settings (2026-10-04)
  - [x] Fix long question and option wrapping, sidebar overflow, and Easy elapsed-time defaults and resume behavior (2026-10-04)
  - [x] Fast development loop (auto rebuild + restore state) (2026-10-04: justfile / scripts/dev.ps1 with watchexec, Cargo dev profile optimization [opt-level=1, line-tables-only, dependencies opt-level=3 reducing rebuilds from 34.02s to 11.75s], debug-only dev_state.json screen/filter/scroll restoration, and --timer 30s flag)
  - [x] Add GPUI Kit scrollbars to every vertical app viewport, with responsive visibility, draggable desktop thumbs, reserved gutters, and persistent per-screen offsets (2026-10-04)
- [ ] **Milestone 13: Optional iOS Verification** (Requires macOS + Xcode 15+)

## Releases

- [x] **Version 1.0.0** (2026-10-06): aligned desktop and Android version metadata and prepared downloadable Windows and Android ARM64 release artifacts.
- [x] **Version 1.1.0** (2026-10-06): recovered all verifiable questions from the supplied PDF, corrected answer extraction, and prepared updated Windows and Android ARM64 builds.
- [x] **Version 1.8.0** (2026-10-08): added signed, user-confirmed self-updates from GitHub Releases for Windows and Android, including automatic checks, release notes, download verification, and native installer handoff. Fixed the Android release startup crash by preserving the Java/JNI activity bridge during R8 minification.
