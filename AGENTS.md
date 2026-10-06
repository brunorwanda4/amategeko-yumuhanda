# AGENTS.md

Instructions for AI coding agents working in this repository. Read this file fully before doing anything. If this file and a chat message disagree, follow the chat message, then tell the owner about the conflict.

## 1. What this project is

"Amategeko y'Umuhanda" is an **offline** study app for the Rwandan driving-test theory exam. It runs on **Windows desktop** and **Android** (iOS optional), from one Rust codebase using **GPUI** and **GPUI Kit** (component library) with **gpui-mobile** as the mobile platform layer. The repository also contains a **project website** (landing page + docs) in `website/`, built with Next.js, Fumadocs, shadcn/ui and Bun.

- UI languages of the app: **Kinyarwanda** (`rw`, default) and **English** (`en`). All user-visible app strings live in `assets/i18n/rw.json` and `assets/i18n/en.json`.
- Data: 433 questions extracted once from a PDF into `assets/questions.json` (+ images). The app never reads the PDF at runtime.
- Product specification for the app: **`docs/SPEC.md`** (screens, quiz rules, data schema, architecture, milestones, acceptance criteria). It is the source of truth. Read the relevant section before building or changing a feature.
- Specification for the website: **`docs/WEBSITE_SPEC.md`**.
- Progress tracking: **`docs/PROGRESS.md`** (milestone checklist). Read it at the start of every session and update it at the end.

## 2. Session start checklist

1. Read `AGENTS.md` (this file), the relevant parts of `docs/SPEC.md` or `docs/WEBSITE_SPEC.md`, `docs/PROGRESS.md`, and `MOBILE_NOTES.md` if it exists.
2. Run `git status` and `git log --oneline -10`. Do not start on top of unrelated uncommitted changes. Tell the owner if the tree is dirty.
3. Work on a feature branch, never on `main` (see section 6).
4. Pick ONE milestone or task from `docs/PROGRESS.md`. Do not jump ahead.
5. Work in only one area per task: the Rust app (`crates/`, `tools/`, `assets/`) OR the website (`website/`). Do not mix them in one commit.

## 3. Tech rules

### Rust app
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

### Website (`website/`)
- **Bun** as package manager and runtime. **Next.js** (App Router) + **Fumadocs** + **TypeScript** + **Tailwind CSS** + **shadcn/ui**.
- Before writing website code, read the current docs and follow them exactly. Never guess their APIs:
  - https://fumadocs.dev
  - https://ui.shadcn.com/docs
  - https://bun.sh/docs
- Add UI with the shadcn CLI (`bunx --bun shadcn@latest add ...`). Do not hand-build components that shadcn already provides.
- Output is a **static export** (no server, no database, no backend code).

## 4. Commands

Run from the repository root. If a command's package name differs, check `Cargo.toml` / `package.json` and fix this file.

### Rust app
```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p desktop --release                      # run the desktop app
cargo check --target aarch64-linux-android -p mobile # Android compile check (needs Android NDK + cargo-ndk)
python tools/extract_questions.py                   # regenerate assets/questions.json from the PDF
```

Android and iOS build scripts are in `crates/mobile` (copied from the `gpui-mobile` example's `build.sh`). iOS needs a Mac and is optional.

### Website
```bash
cd website
bun install
bun run dev          # local preview
bun run lint
bunx tsc --noEmit
bun run build        # must pass before every commit that touches website/
```

## 5. Hard rules: NEVER do these

**Product and data**
- Never parse the PDF at runtime. Extraction is a one-time tool in `tools/`.
- Never invent, "correct" or reword a question, option or answer in `questions.json`. Fix errors only through `assets/overrides.json`, and tell the owner which question you changed and why. The correct answer comes from the letter in parentheses in the PDF.
- Never change the quiz rules for Easy / Medium / Hard (see `docs/SPEC.md` section 3) without the owner's approval. Examples: Easy locks an answer after the first tap; Hard has no back, skip, flag, grid or pause; Hard refuses to advance with no selection.
- Never change the JSON schema of `questions.json`, `settings.json`, `progress.json` or `in_progress.json` without a migration that keeps old files loading.
- Never hard-code sample numbers from mockups (14/20, 71%, etc.). Everything shown is computed from real data.
- Never write user-visible text in UI code. Add keys to BOTH `assets/i18n/rw.json` and `assets/i18n/en.json`.
- Never translate or edit questions yourself. Translations come only from the owner's reviewed files. Never change `correct`, option order or images in `questions.en.json`.
- Never remove or reorder a translation key without updating both language files and the parity test.

**Privacy and scope**
- Never add network access, analytics, telemetry, crash reporters, ads or accounts to the app or the website. The app is 100% offline (no `INTERNET` permission on Android). The website has no cookies, trackers, forms or backend.
- Never commit secrets, API keys, keystores, `.jks`, `.p12`, `local.properties`, `.env` files, or signing passwords.
- Never commit build output (`target/`, `build/`, `.gradle/`, `*.apk`, `*.aab`, `*.ipa`, `node_modules/`, `.next/`, `out/`, `.source/`) or IDE files.
- Never commit the source PDF or large binary files unless the owner explicitly says so. The PDF is marked "RESTRICTED". Check with the owner before committing `assets/questions.json` and `assets/images/` to any public remote.

**Website content and licensing**
- Never put the source PDF, `questions.json`, the images in `assets/images/`, or any real question from them on the website, in `website/public/`, in screenshots, or in docs, unless the owner confirms in writing that publishing is allowed.
- Sample questions on the website come only from `website/content/samples/` (original questions written by the owner). Never copy questions from the PDF, `questions.json`, or other websites.
- Every image, icon, font and block of text added to the website must be original or have a licence that allows public use. Record each one in `website/ASSETS.md` with: file path, source URL, author, licence, and whether credit is required. If the licence is unclear, do not add the asset and ask the owner.
- Sign images on the website must be self-drawn SVGs or Wikimedia Commons files with a recorded licence. Never use images from the PDF or from `assets/images/`.
- Website pages must match `docs/SPEC.md` and the real app behavior. If they differ, tell the owner instead of guessing.

**Code quality**
- No `unwrap()` / `expect()` / `panic!` on file, JSON, image or platform operations in app code. Return errors and show a friendly error state. Tests may use them.
- No hard-coded colours in the app. Use GPUI Kit theme tokens. On the website use shadcn theme tokens.
- No `unsafe` outside the platform glue in `crates/mobile` and `crates/desktop`, and every use needs a `// SAFETY:` comment.
- In the website, no `any` in TypeScript unless justified with a comment.
- Do not add a dependency (Cargo crate or Bun package) without a one-line justification in the commit message. Prefer the standard library and packages already in the project.
- Use **Bun only** for the website. Never use npm, yarn or pnpm, and never commit `package-lock.json`, `yarn.lock` or `pnpm-lock.yaml`. Commit Bun's lockfile.
- Do not delete, skip (`#[ignore]`), weaken or rewrite a failing test just to make it pass. Fix the code, or explain to the owner why the test is wrong.
- Do not silence warnings with `#[allow(...)]` or `// eslint-disable` as a shortcut. Fix them, or explain why it is justified.
- Do not leave `TODO` / `unimplemented!()` / dummy data in finished features. If something is not done, say so in `docs/PROGRESS.md`.
- - Never remove or change the "Built by Rwanda Bruno" credit or its GitHub link (`https://github.com/brunorwanda4`) without the owner's approval. It lives in one constant and is not shown on quiz, results or timed screens.
- Opening that link in the system browser is the only allowed external link in the app. It does not change the offline rule: the app makes no network requests itself.
- Never register a keyboard shortcut outside `shortcuts.rs`, and never add one that breaks a mode rule (for example Previous/Skip/Flag in Hard, or Next before answering in Easy).

**Git and environment**
- Never commit directly to `main`. Never force-push. Never rewrite published history (`rebase` / `reset --hard` / `commit --amend` on pushed commits).
- Never use `git add -A` / `git add .` blindly. Stage specific files and review `git diff --staged` first.
- Never run destructive commands outside the project folder, and never delete the user's files, app-data folder or git history.
- Never modify files under `/` system paths, global git config, or global Cargo/Rust/Bun settings.

## 6. Git workflow and committing

- **Branches:** one branch per milestone or task, named `feat/<short-name>`, `fix/<short-name>`, `docs/<short-name>`, or `chore/<short-name>` (examples: `feat/easy-mode`, `feat/website-landing`).
- **Commit after every finished, working task** (not only at the end of a milestone). A commit must leave the project in a state that compiles and passes checks.
- **Before every commit**, run the checks for the area you changed and make sure all pass:
  - Rust app: `cargo fmt --all`, then `cargo clippy --workspace --all-targets -- -D warnings`, then `cargo test --workspace`. If you touched `crates/mobile` or shared UI, also `cargo check --target aarch64-linux-android -p mobile`.
  - Website: `bun run lint`, then `bunx tsc --noEmit`, then `bun run build` (inside `website/`).
- If a check fails and you cannot fix it, do **not** commit broken code. Report the failure to the owner.
- **Commit messages** follow Conventional Commits, in English, imperative mood, subject <= 72 characters:
  - `feat(quiz): lock answer after first tap in Easy mode`
  - `fix(timer): recompute deadline after app resume`
  - `feat(ui): add bottom navigation for mobile layout`
  - `feat(website): add landing page hero and mode cards`
  - `docs(website): add install guide for Android`
  - `test(stats): cover most-missed questions ranking`
  - `docs: update PROGRESS after Medium mode`
  - `chore(deps): pin gpui-kit to version required by gpui-mobile`
  - Add a short body when the "why" is not obvious (what changed, why, anything the owner should check).
- Keep commits small and focused. Do not mix refactors, features and formatting in one commit.
- **Pushing:** after finishing a milestone, push the **feature branch** (never `main`) if a remote is configured, using a normal non-force push. Do not open or merge pull requests unless asked. The owner reviews and merges.
- Never commit work that is half-finished behind a comment. Use a branch and say it is unfinished in `docs/PROGRESS.md`.

## 7. Definition of done (for any task)

**Rust app tasks** are done only when ALL of these are true:

- It matches `docs/SPEC.md` (behavior, Kinyarwanda strings, layout on desktop AND mobile where relevant).
- `cargo fmt`, `clippy -D warnings`, and `cargo test --workspace` pass.
- New logic has unit tests in `crates/core` (quiz rules, timers, stats, search, persistence). UI features that change flows have a GPUI Kit headless test.
- Errors are handled without panics (missing image, bad question, corrupt or missing data file).
- User-visible text is in both `assets/i18n/rw.json` and `assets/i18n/en.json`; no hard-coded colours; touch targets are >= 44 px on mobile.
- Every new string exists in both `rw.json` and `en.json`, and the parity test passes.
- Timers are deadline-based and still correct after the app is backgrounded; the in-progress attempt is saved after each answer.

**Website tasks** are done only when ALL of these are true:

- It matches `docs/WEBSITE_SPEC.md` and the real behavior of the app.
- `bun run lint`, `bunx tsc --noEmit` and `bun run build` pass, with no console errors in the browser.
- Light and dark themes both look right; the page works on a phone-sized screen; semantic HTML, alt text and keyboard navigation are in place.
- Every new asset is original or licensed, and is recorded in `website/ASSETS.md`.
- No question bank content, PDF content or `assets/images/` files are used.

**For every task:**

- `docs/PROGRESS.md` is updated (checkbox, date, short note). `README.md`, `website/README.md` or `MOBILE_NOTES.md` is updated if setup steps or findings changed.
- Everything is committed on a feature branch with a clear message.

## 8. Mobile specifics

- gpui-mobile is **experimental** (text-input/IME, accessibility and lifecycle hooks are incomplete). Record what works and what does not in `MOBILE_NOTES.md`, with the device/emulator and Android version used.
- Target: Android arm64-v8a, minSdk 26. Do not promise armv7 or x86_64. iOS is optional and last.
- Android Back button must be handled: close dialogs and sheets first; in a running quiz ask for confirmation; never close the app by accident.
- Storage on mobile uses the app-private files directory through the `Storage` trait, not the `directories` crate. Questions and images are bundled in the app.
- Keep the APK small. Report its size in the README when it changes noticeably.

## 9. Browser build of the app (only if `docs/SPEC.md` section 14 is enabled)

- It is OPTIONAL and comes after Android. Do not start it unless the owner asks.
- Rust compiled to WebAssembly only. No JavaScript app code, no npm packages for it.
- Never use `std::time::Instant`, `SystemTime`, `std::fs`, or threads in code that must run in the browser; use the `Clock` and `Storage` traits.
- Never deploy it publicly, and never publish `questions.json` online, without the owner's approval.

## 10. When to stop and ask the owner

Stop and ask (do not guess) when:

- The spec is ambiguous or two sections conflict.
- A required GPUI / GPUI Kit / gpui-mobile / Fumadocs / shadcn API cannot be found in the docs or example.
- A change needs a new dependency, a new Android permission, or a version bump of GPUI-related crates.
- A question in the data looks wrong (do not edit it silently).
- The licence of an image, icon, font or text for the website is unclear.
- A test you did not write fails and the cause is unclear.
- The task would require breaking any rule in section 5.

## 11. End-of-task report

When you finish a task, reply with a short report:

1. **Done:** what you built or fixed, in 2-5 lines.
2. **Files:** the main files changed.
3. **Checks:** which commands you ran and their results (fmt, clippy, tests, Android check, or lint, tsc, build).
4. **Commit:** branch name and commit hash(es).
5. **How to try it:** exact command(s) or steps for the owner to test manually.
6. **Not done / risks / questions:** anything unfinished, any assumption you made, anything the owner must decide.

Do not claim something works unless you ran it. If you could not run it (for example no Android device), say so.

## 12. Style

- Rust: follow `rustfmt` defaults. Prefer small functions, clear names, and explicit error types (`thiserror`) in `core`. Comments explain why, not what. Public items in `core` have doc comments. Keep UI code thin: state and rules live in `core`; `app` only renders and forwards events.
- Website: TypeScript strict mode, small components, content in MDX under `website/content/`, links to the repo and releases from one constant file (`lib/config.ts`).
- Reply to the owner in simple English. Kinyarwanda is used only for in-app text and the Kinyarwanda parts of the website.

The app font is fixed (FONT_FAMILY). Never add a font family setting and never hard-code another font.
