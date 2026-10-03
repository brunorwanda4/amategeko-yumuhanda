# AGENTS.md

Instructions for AI coding agents working in this repository. Read this file fully before doing anything. If this file and a chat message disagree, follow the chat message, then tell the owner about the conflict.

## 1. What this project is

"Amategeko y'Umuhanda" is an **offline** study app for the Rwandan driving-test theory exam. It runs on **Windows desktop** and **Android** (iOS optional), from one Rust codebase using **GPUI** and **GPUI Kit** (component library) with **gpui-mobile** as the mobile platform layer.

- UI language: **Kinyarwanda**. All user-visible strings live in `crates/core/src/i18n.rs`.
- Data: 433 questions extracted once from a PDF into `assets/questions.json` (+ images). The app never reads the PDF at runtime.
- Full product specification: **`docs/SPEC.md`** (screens, quiz rules, data schema, architecture, milestones, acceptance criteria). It is the source of truth. Read the relevant section before building or changing a feature.
- Progress tracking: **`docs/PROGRESS.md`** (milestone checklist). Read it at the start of every session and update it at the end.

## 2. Session start checklist

1. Read `AGENTS.md` (this file), `docs/SPEC.md` (sections you need), `docs/PROGRESS.md`, and `MOBILE_NOTES.md` if it exists.
2. Run `git status` and `git log --oneline -10`. Do not start on top of unrelated uncommitted changes. Tell the owner if the tree is dirty.
3. Work on a feature branch, never on `main` (see section 6).
4. Pick ONE milestone or task from `docs/PROGRESS.md`. Do not jump ahead.

## 3. Tech rules

- Rust stable. Cargo workspace: `crates/core` (no GPUI dependency), `crates/app` (shared UI), `crates/desktop`, `crates/mobile`.
- UI components: **GPUI Kit only**. Do not add another UI library. Do not hand-build buttons, dialogs, sliders, switches, inputs or tabs if GPUI Kit has them.
- Before writing any GPUI / GPUI Kit / gpui-mobile code, read the docs and examples:
  - https://gpui-kit.com/llms-full.txt
  - https://gpui-kit.com/docs/getting-started
  - https://gpui-kit.com/docs/mobile
  - https://github.com/longbridge/gpui-mobile (README and `example/`)
- **Never guess APIs from memory.** GPUI changes quickly. Copy patterns from the docs and the example app. If code does not compile, re-read the docs; do not invent methods.
- **GPUI version alignment:** the app, `gpui-kit` and `gpui-mobile` must use the same GPUI version (the `gpui-pre` crates). Never mix a Zed git GPUI with the crates.io snapshots. Never bump GPUI-related versions without asking.
- Platform-specific code goes behind the traits in `crates/core/src/platform.rs` (`Storage`, `Clock`). `core` must stay free of GPUI and OS APIs so it can be unit-tested.
- Screens are written once in `crates/app` with a responsive layout (sidebar on desktop, bottom navigation on mobile). Do not duplicate screens per platform.

## 4. Commands

Run from the repository root. If a command's package name differs, check `Cargo.toml` and fix this file.

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p desktop --release                      # run the desktop app
cargo check --target aarch64-linux-android -p mobile # Android compile check (needs Android NDK + cargo-ndk)
python tools/extract_questions.py                   # regenerate assets/questions.json from the PDF
```

Android and iOS build scripts are in `crates/mobile` (copied from the `gpui-mobile` example's `build.sh`). iOS needs a Mac and is optional.

## 5. Hard rules: NEVER do these

**Product and data**
- Never parse the PDF at runtime. Extraction is a one-time tool in `tools/`.
- Never invent, "correct" or reword a question, option or answer in `questions.json`. Fix errors only through `assets/overrides.json`, and tell the owner which question you changed and why. The correct answer comes from the letter in parentheses in the PDF.
- Never change the quiz rules for Easy / Medium / Hard (see `docs/SPEC.md` section 3) without the owner's approval. Examples: Easy locks an answer after the first tap; Hard has no back, skip, flag, grid or pause; Hard refuses to advance with no selection.
- Never change the JSON schema of `questions.json`, `settings.json`, `progress.json` or `in_progress.json` without a migration that keeps old files loading.
- Never hard-code sample numbers from mockups (14/20, 71%, etc.). Everything shown is computed from real data.
- Never put user-visible text directly in UI code. Use `i18n.rs`.

**Privacy and scope**
- Never add network access, analytics, telemetry, crash reporters, ads or accounts. The app is 100% offline. No `INTERNET` permission on Android.
- Never commit secrets, API keys, keystores, `.jks`, `.p12`, `local.properties`, or signing passwords.
- Never commit build output (`target/`, `build/`, `.gradle/`, `*.apk`, `*.aab`, `*.ipa`) or IDE files.
- Never commit the source PDF or large binary files unless the owner explicitly says so. The PDF may be restricted material. Check with the owner before committing `assets/questions.json` and `assets/images/` to any public remote.

**Code quality**
- No `unwrap()` / `expect()` / `panic!` on file, JSON, image or platform operations in app code. Return errors and show a friendly error state. Tests may use them.
- No hard-coded colours. Use GPUI Kit theme tokens.
- No `unsafe` outside the platform glue in `crates/mobile` and `crates/desktop`, and every use needs a `// SAFETY:` comment.
- Do not add a dependency without a one-line justification in the commit message. Prefer the standard library and crates already in the workspace.
- Do not delete, skip (`#[ignore]`), weaken or rewrite a failing test just to make it pass. Fix the code, or explain to the owner why the test is wrong.
- Do not silence warnings with `#[allow(...)]` as a shortcut. Fix them, or explain why an allow is justified.
- Do not leave `TODO` / `unimplemented!()` / dummy data in finished features. If something is not done, say so in `docs/PROGRESS.md`.

**Git and environment**
- Never commit directly to `main`. Never force-push. Never rewrite published history (`rebase` / `reset --hard` / `commit --amend` on pushed commits).
- Never use `git add -A` / `git add .` blindly. Stage specific files and review `git diff --staged` first.
- Never run destructive commands outside the project folder, and never delete the user's files, app-data folder or git history.
- Never modify files under `/` system paths, global git config, or global Cargo/Rust settings.

## 6. Git workflow and committing

- **Branches:** one branch per milestone or task, named `feat/<short-name>`, `fix/<short-name>`, `docs/<short-name>`, or `chore/<short-name>` (example: `feat/easy-mode`).
- **Commit after every finished, working task** (not only at the end of a milestone). A commit must leave the project in a state that compiles and passes tests.
- **Before every commit**, run in order and make sure all pass:
  1. `cargo fmt --all`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. If you touched `crates/mobile` or shared UI: `cargo check --target aarch64-linux-android -p mobile`
- If a check fails and you cannot fix it, do **not** commit broken code. Report the failure to the owner.
- **Commit messages** follow Conventional Commits, in English, imperative mood, subject <= 72 characters:
  - `feat(quiz): lock answer after first tap in Easy mode`
  - `fix(timer): recompute deadline after app resume`
  - `feat(ui): add bottom navigation for mobile layout`
  - `test(stats): cover most-missed questions ranking`
  - `docs: update PROGRESS after Medium mode`
  - `chore(deps): pin gpui-kit to version required by gpui-mobile`
  - Add a short body when the "why" is not obvious (what changed, why, anything the owner should check).
- Keep commits small and focused. Do not mix refactors, features and formatting in one commit.
- **Pushing:** after finishing a milestone, push the **feature branch** (never `main`) if a remote is configured, using a normal non-force push. Do not open or merge pull requests unless asked. The owner reviews and merges.
- Never commit work that is half-finished behind a comment. Use a branch and say it is unfinished in `docs/PROGRESS.md`.

## 7. Definition of done (for any task)

A task is done only when ALL of these are true:

- It matches `docs/SPEC.md` (behavior, Kinyarwanda strings, layout on desktop AND mobile where relevant).
- `cargo fmt`, `clippy -D warnings`, and `cargo test --workspace` pass.
- New logic has unit tests in `crates/core` (quiz rules, timers, stats, search, persistence). UI features that change flows have a GPUI Kit headless test.
- Errors are handled without panics (missing image, bad question, corrupt or missing data file).
- User-visible text is in `i18n.rs`; no hard-coded colours; touch targets are >= 44 px on mobile.
- Timers are deadline-based and still correct after the app is backgrounded; the in-progress attempt is saved after each answer.
- `docs/PROGRESS.md` is updated (checkbox, date, short note). `README.md` or `MOBILE_NOTES.md` is updated if setup steps or platform findings changed.
- Everything is committed on a feature branch with a clear message.

## 8. Mobile specifics

- gpui-mobile is **experimental** (text-input/IME, accessibility and lifecycle hooks are incomplete). Record what works and what does not in `MOBILE_NOTES.md`, with the device/emulator and Android version used.
- Target: Android arm64-v8a, minSdk 26. Do not promise armv7 or x86_64. iOS is optional and last.
- Android Back button must be handled: close dialogs and sheets first; in a running quiz ask for confirmation; never close the app by accident.
- Storage on mobile uses the app-private files directory through the `Storage` trait, not the `directories` crate. Questions and images are bundled in the app.
- Keep the APK small. Report its size in the README when it changes noticeably.

## 9. When to stop and ask the owner

Stop and ask (do not guess) when:

- The spec is ambiguous or two sections conflict.
- A required GPUI / GPUI Kit / gpui-mobile API cannot be found in the docs or example.
- A change needs a new dependency, a new Android permission, or a version bump of GPUI-related crates.
- A question in the data looks wrong (do not edit it silently).
- A test you did not write fails and the cause is unclear.
- The task would require breaking any rule in section 5.

## 10. End-of-task report

When you finish a task, reply with a short report:

1. **Done:** what you built or fixed, in 2-5 lines.
2. **Files:** the main files changed.
3. **Checks:** which commands you ran and their results (fmt, clippy, tests, Android check).
4. **Commit:** branch name and commit hash(es).
5. **How to try it:** exact command(s) or steps for the owner to test manually.
6. **Not done / risks / questions:** anything unfinished, any assumption you made, anything the owner must decide.

Do not claim something works unless you ran it. If you could not run it (for example no Android device), say so.

## 11. Style

- Follow `rustfmt` defaults. Prefer small functions, clear names, and explicit error types (`thiserror`) in `core`.
- Comments explain why, not what. Public items in `core` have doc comments.
- Keep UI code thin: state and rules live in `core`; `app` only renders and forwards events.
- Reply to the owner in simple English. Kinyarwanda is used only for in-app text.
