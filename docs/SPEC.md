# Product Specification: Amategeko y'Umuhanda

## 1. Product Overview
"Amategeko y'Umuhanda" is a standalone, 100% offline study app for the Rwandan driving-test theory exam in Kinyarwanda. It targets Windows desktop and Android (iOS optional) from a single shared codebase using Rust, GPUI, GPUI Kit, and gpui-mobile.

## 2. Tech Stack & Architecture
- Rust stable
- GPUI (gpui-pre =0.3.4) + GPUI Kit (rev 7d9efcd2069f9eaa6eb3ba6345aac4aa7d87c9f7) + gpui-mobile (gpui-pre-mobile)
- Crates: `serde`, `serde_json`, `rand`, `thiserror`, `directories` (desktop only)
- Single responsive codebase:
  - `crates/core`: data models, quiz engine, stats, timer, i18n, platform traits (no GPUI dependency)
  - `crates/app`: shared UI views, responsive shell (sidebar on desktop >=700px, bottom nav on mobile <700px)
  - `crates/desktop`: Windows entry point, desktop storage
  - `crates/mobile`: Android/iOS entry points, mobile private storage

## 3. Question Bank & Data Schema
Source: 433 questions from `ibibazo_byamategeko_y_umuhanda.pdf`.
One-time offline extraction produces:
- `assets/questions.json`:
  ```json
  {
    "id": 1,
    "text": "Ikinyabiziga cyose cyangwa ibinyabiziga bigenda bigomba kugira:",
    "options": {
      "a": "Umuyobozi",
      "b": "Umuherekeza",
      "c": "A na B ni ibisubizo by'ukuri",
      "d": "Nta gisubizo cy'ukuri kirimo"
    },
    "correct": "a",
    "image": null,
    "has_image": false
  }
  ```
- `assets/images/q{id}.png`: cropped, downscaled, and optimized sign/marking images.
- `assets/overrides.json`: manual errata and fixes.
- `tools/needs_review.json`: extractor validation report.

## 4. Quiz Rules
- 20 random unique questions per attempt.
- Options stay in original a–d order.
- Pass mark: default 12/20 (customizable in settings).
- **Byoroshye (Easy, Green)**:
  - Instant feedback on first tap (correct green check, wrong red x).
  - Feedback explanation line: "Ni byo. Igisubizo ni X." or "Siko. Igisubizo cy'ukuri ni X."
  - Answer locked after first tap.
  - Free navigation (Previous, Next, Skip), star toggle, optional elapsed time.
- **Hagati (Medium, Amber)**:
  - Exam simulation: countdown (default 20 min).
  - No feedback during quiz; selection highlighted blue and changeable.
  - 20-cell question grid (desktop inline, mobile bottom sheet).
  - Previous / Next / Flag. Last 2 minutes timer turns red. Auto-submit at 0.
  - Finish confirmation dialog listing unanswered questions.
- **Bikomeye (Hard, Red)**:
  - Strict mode: shorter countdown (default 12 min), weighted toward image and complex questions.
  - No going back, no skipping, no pause, no grid.
  - Explicit confirm button "Emeza igisubizo". Error if nothing selected. Auto-submit at 0.
- **Special Practice Modes**:
  - Weak-questions practice: draws up to 20 most-missed questions (uses Easy rules).
  - Retry-wrong: from Results, launches Easy rules with missed questions from that attempt.
- **Deadline-Based Timers**:
  - Remaining time = deadline - now (monotonic/wall clock). Resilient to app backgrounding and phone locks.
- **In-Progress Persistence**:
  - Running attempt saved to `in_progress.json` after every answer. Reopening offers resume or discard.

## 5. Screens
- **Ahabanza (Home)**: Murakaza neza, 3 mode cards (Tangira), 3 stat tiles, weak questions card, unfinished quiz banner.
- **Ikizamini (Quiz Layout A - Easy/Medium)**: mode badge, question counter, timer/star, progress bar, question text & image, option cards, footer buttons.
- **Ikizamini (Quiz Layout B - Hard)**: segmented progress bar, 2-column desktop (image panel left, question & options right) vs stacked mobile, strict confirm.
- **Ibisubizo (Results)**: circular score badge, pass/fail badge, filter chips (All, Missed, Unanswered), expandable question review list, retry actions.
- **Ibibazo (Browse)**: counter, search input (number or text), filters, hide answers toggle, 433-item virtualized list, star toggle.
- **Imibare (Stats)**: metric tiles, 10-attempt bar chart with pass mark line, most-missed questions card, per-mode performance breakdown.
- **Igenamiterere (Settings)**: mode duration sliders, pass mark slider, theme mode (Light/Dark/System), font size slider with preview, behavior switches, clear history.

## 6. Mobile Requirements
- Touch targets >= 44x44 px (buttons >= 48px high).
- Safe-area insets handled (notch, status bar, navigation bar).
- Sticky bottom action buttons.
- Android system back handled gracefully (confirm exit during quiz).
- Fully offline operation.
