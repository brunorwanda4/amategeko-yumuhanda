# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.8.3] - 2026-10-12

### Added
- Swipe gesture navigation for quiz questions on mobile (swipe left for next, swipe right for previous)
- Mode-specific swipe rules: Easy/Study require answer before advancing, Medium allows free navigation both ways, Hard mode disables swiping
- Nudge card animation when attempting to advance on an unanswered question
- Direction-aware slide transition animation between questions
- One-time swipe navigation tip on first answered question on mobile
- Dedicated `swipe_nav` toggle in Settings

### Fixed
- Gesture disambiguation between option tapping, vertical scrolling, header buttons, and system back edge zones

## [1.8.0] - 2026-10-11

### Added
- Study all questions mode, with saved position
- Choose the number of questions (Easy and Medium; Hard stays 20)
- Question bank corrected to 404 questions, with fixed answers
- Automatic updates from GitHub Releases, signed
- Screen stays awake during a quiz
- Timer warnings at 25% and 10% time left
- Window size and position remembered, with a minimum size
- Scrollbars
- Fixed app font and a working font-size setting
- About links (website, guide, source) in Settings
- "Built by Rwanda Bruno" credit
- English and Kinyarwanda interface and questions

### Fixed
- Quiz header no longer clips on small phones
- Results can scroll on phones
- the B key no longer also stars a question (star is K)
- window cannot be made tiny
