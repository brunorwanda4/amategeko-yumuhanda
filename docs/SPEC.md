# How to update the project files

Follow these steps once. Everything marked **PASTE** goes into your files as it is.

## Step 1. Replace the files that I updated
- Replace `AGENTS.md` in the repo root with the new `AGENTS.md`.
- Replace `.gitignore` in the repo root with the new `.gitignore`. It now includes a Website section (Bun, Next.js, Fumadocs).

## Step 2. Save the website prompt
Save the website prompt (the "TASK: Build the project website..." prompt from the earlier answer) as `docs/WEBSITE_SPEC.md`.
Then add this block at the end of that file, as its section 8:

**PASTE (end of docs/WEBSITE_SPEC.md):**
```markdown
## 8. Content and licensing (must follow)
- The source PDF is marked "RESTRICTED". Do not publish the PDF, `questions.json`, `assets/images/`, or any real question from them.
- Sample questions come only from `website/content/samples/` (original questions written by the owner).
- Sign images are self-drawn SVGs, or Wikimedia Commons files with a recorded licence.
- Every image, icon, font and block of text is recorded in `website/ASSETS.md`: path, source URL, author, licence, credit required (yes/no).
- If a licence is unclear, do not add the asset. Ask the owner.
```

## Step 3. Update `docs/SPEC.md` (the big app prompt)

### 3a. In section 8 (Architecture), replace the folder tree with this
**PASTE:**
```markdown
road-rules-trainer/
  Cargo.toml                // workspace
  README.md, AGENTS.md, .gitignore
  docs/SPEC.md, docs/WEBSITE_SPEC.md, docs/PROGRESS.md
  assets/questions.json, assets/images/, assets/overrides.json
  tools/extract_questions.py, tools/requirements.txt
  crates/
    core/      // NO gpui dependency: models, quiz engine, stats, data loading, i18n, Storage/Clock traits
    app/       // shared GPUI + GPUI Kit UI used by desktop AND mobile
    desktop/   // bin: Windows/Linux/macOS entry point
    mobile/    // Android + iOS entry points (copied structure from gpui-mobile example)
  website/     // Next.js + Fumadocs + shadcn + Bun (landing page and docs); see docs/WEBSITE_SPEC.md
```

### 3b. Add these new sections at the end of the file
**PASTE:**
```markdown
## 12. Project website
The repository also contains a project website in `website/` (landing page + documentation). It is built with Next.js, Fumadocs, shadcn/ui and Bun, as a static export. Its full specification is `docs/WEBSITE_SPEC.md`.
- The website is separate from the app. It shares no code with the Rust crates.
- Downloads are links to the latest GitHub Release. The website never hosts the APK, the exe, the PDF, or the question bank.
- No analytics, cookies, trackers, forms, or backend on the website.
- Website pages must describe the real behavior of the app (quiz rules, screens, settings). If a rule in this file changes, update the website docs in the same task or tell the owner.

## 13. Content and licensing
- The source PDF is marked "RESTRICTED". Never publish it, `questions.json` or `assets/images/` (website, release assets, screenshots) unless the owner confirms in writing that sharing is allowed.
- For public material (website, README screenshots, store listing) use ONLY original sample questions written by the owner (`website/content/samples/`) and self-drawn or properly licensed images.
- Every public asset has a recorded licence in `website/ASSETS.md`.
- If the repository is made public, the owner decides whether to keep `assets/questions.json` and `assets/images/` out of git (see the commented lines in `.gitignore`).

## 14. Optional: browser build of the app (NOT part of the first release)
Only start this when the owner asks. Goal: the same Rust app also runs in a browser (`wasm32-unknown-unknown`), so it works on any device including iPhone.
- Read the GPUI Kit web/WebAssembly documentation first. If it disagrees with this section, the docs win.
- Add `crates/web`, build `crates/app` for wasm, output a static `dist/`. No JavaScript app code and no npm packages for it.
- Add a browser `Storage` implementation (localStorage or IndexedDB). Use a wasm-safe `Clock`. Never use `std::time::Instant`, `SystemTime`, `std::fs` or threads in code that must run in the browser.
- Timers stay deadline-based and must be correct after the tab was in the background.
- Do not deploy it publicly until the owner confirms the question bank may be shared (see section 13).
```

### 3c. In the build order (section 10), add these milestones after the last one
**PASTE:**
```markdown
14. Project website: follow `docs/WEBSITE_SPEC.md` (scaffold, landing page, docs, SEO, static export, deploy guide).
15. (Optional) Browser build of the app (section 14).
```

### 3d. In the acceptance criteria (section 11), add
**PASTE:**
```markdown
- The website builds with `bun run build`, passes lint and type checks, and works in light and dark themes on a phone-sized screen.
- The website contains no PDF content, no `questions.json` content, and no `assets/images/` files. Every asset is listed in `website/ASSETS.md` with its licence.
```

## Step 4. Update `README.md`
In the project structure block, add this line:
```
├── website/               landing page and docs (Next.js + Fumadocs + shadcn, Bun)
```
In the Development section, add:
```markdown
### Website

```bash
cd website
bun install
bun run dev
bun run build
```

See `docs/WEBSITE_SPEC.md` for the full website specification.
```

## Step 5. Create `docs/PROGRESS.md`
Create it with the milestone list, so the agent has something to tick off.

**PASTE:**
```markdown
# Progress

Update this file at the end of every task: tick the box, add the date and a one-line note.

## App
- [ ] 1. Extraction tool: questions.json, images, validation report
- [ ] 2. Workspace skeleton, GPUI Kit + theme, mobile spike on Android (MOBILE_NOTES.md)
- [ ] 3. core: models, quiz engine, deadline timer, Storage/Clock traits, in-progress save, tests
- [ ] 4. Shared shell (sidebar / bottom nav) + Home
- [ ] 5. Easy mode
- [ ] 6. Medium mode
- [ ] 7. Hard mode
- [ ] 8. Results screen
- [ ] 9. Ibibazo (question bank) screen
- [ ] 10. Imibare (stats) screen + weak-questions practice
- [ ] 11. Igenamiterere (settings) screen
- [ ] 12. Polish: dark theme, shortcuts, Back button, lifecycle, tests, README
- [ ] 13. (Optional) iOS build

## Website
- [ ] 14.1 Scaffold Fumadocs + Next.js + Bun + shadcn
- [ ] 14.2 Landing page
- [ ] 14.3 Docs content, search, edit-on-GitHub links
- [ ] 14.4 SEO and polish
- [ ] 14.5 Static export and deploy guide
- [ ] 14.6 (Optional) Kinyarwanda version of the user guide

## Optional
- [ ] 15. Browser build of the app (SPEC section 14)
```

## Step 6. Check
- `git status` shows only: `AGENTS.md`, `.gitignore`, `README.md`, `docs/SPEC.md`, `docs/WEBSITE_SPEC.md`, `docs/PROGRESS.md`.
- Commit them: `git add AGENTS.md .gitignore README.md docs/` then `git commit -m "docs: add website, licensing rules and progress tracker"`.
- Start the agent with: "Read AGENTS.md, docs/SPEC.md and docs/PROGRESS.md, then do milestone 1."

## 15. Languages (Kinyarwanda + English)
The app supports Kinyarwanda (`rw`, default) and English (`en`).

### 15.1 Interface language
- All user-visible text is in `assets/i18n/rw.json` and `assets/i18n/en.json` (same keys in both, embedded in the app). No text is written directly in UI code.
- Placeholders use named parameters, for example "Ikibazo {n} / {total}" and "Question {n} / {total}". Do not build sentences by joining pieces.
- A unit test fails if the two files have different keys, or an empty value.
- If a key is missing at runtime, fall back to Kinyarwanda and log a warning. Never crash.
- Settings: "Ururimi / Language" (Kinyarwanda, English). Changes apply immediately and are saved in `settings.json` (`language`).

### 15.2 Question language
- `assets/questions.json` (Kinyarwanda) is the source of truth for: id, correct letter, option order, image, difficulty data.
- `assets/questions.en.json` contains ONLY translated text: `{ "id", "text", "options": {a,b,c,d}, "status": "draft" | "reviewed" }`.
- At load time, merge by id. Validation: same ids, same option letters, no empty strings; the English file never defines `correct` or `image`.
- Settings: "Ururimi rw'ibibazo / Question language" (Kinyarwanda, English), independent from the interface language.
- Optional switch "Erekana indimi zombi / Show both languages": shows the second language under the question and each option. Default off.
- If an English question is missing, show the Kinyarwanda one.
- Attempts, mistakes, stars and stats store question ids only, never text, so they work in both languages.
- Browse search (Ibibazo screen) searches the text of the selected question language. A number search works in both.
- English questions with status "draft" show a small "Unofficial translation" note on the Browse and Results screens, which disappears when status is "reviewed".

### 15.3 Translation tooling (offline, not part of the app)
- `tools/glossary.json`: fixed translations for key terms (so "icyapa" is always "sign", and so on).
- `tools/check_translation.py`: checks ids, option letters, empty values, glossary terms, and that numbers in the text (speeds, weights, distances) are identical in both languages.
- The agent must NOT invent translations. Drafts come from the owner's AI translation step; the agent only builds the loader, the checks and the UI.

### 15.4 Screens
- Settings: add a "Ururimi / Language" group with two segmented controls (interface, questions) and the "show both languages" switch.
- Home, Quiz, Results, Browse, Stats: every label comes from the i18n files. Check that English text fits in buttons and chips on mobile (English is often shorter, but some labels are longer).
- Results and Browse: show a small language badge (RW / EN) if the question language differs from the interface language.

### 15.5 Milestones
- Build the i18n system in milestone 3 (core), with both languages from the start.
- Add the English question loader and the language settings in milestone 11.
- The English translation itself is done by the owner and loaded when ready.

### 15.6 Acceptance criteria
- Switching the interface language changes every screen instantly, with no leftover Kinyarwanda or English text.
- Switching the question language keeps the same correct answers, images and stats.
- Missing English questions fall back to Kinyarwanda.
- All i18n tests pass.

## 16. Timer warning states (Medium and Hard only)

- Thresholds are fractions of the attempt's total time: WARNING at ≤ 25% remaining, ERROR at ≤ 10% remaining (constants in `core`). Example: 20 min → 05:00 and 02:00; 12 min → 03:00 and 01:12.
- Timer pill: normal (neutral, clock icon) → warning (warning colour, triangle icon) → error (danger colour, circle-alert icon, bold, subtle pulse that respects reduced-motion) → done (danger colour, 00:00).
- A thin time-remaining bar under the header follows the same colours.
- When the level gets worse (normal→warning, warning→error, any→done), show ONE dismissible banner (auto-hides after 5 s, except the "time is up" one):
  - warning: "Igihe kiregereje. Hasigaye {time}."
  - error: "Igihe kigiye kurangira! Hasigaye {time}."
  - done: "Igihe kirarangiye. Ikizamini cyoherejwe."
- Never rely on colour alone: icon + text + accessibility announcement (assertive) on each level change.
- Levels are computed from the deadline-based remaining time (so they stay correct after backgrounding). If the app resumes already past a threshold, show the current level's banner once.
- On mobile, an optional vibration on level change if the platform supports it (setting off by default).
- Unit tests in `core`: threshold levels for 20 min and 12 min totals; level never goes back to normal unless the attempt restarts; resume past a threshold reports the correct level.
