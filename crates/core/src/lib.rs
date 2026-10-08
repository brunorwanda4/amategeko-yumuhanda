//! Core data models, quiz rules, stats, and platform traits for Amategeko y'Umuhanda.
//!
//! This crate contains NO GPUI dependencies, making it pure Rust, 100% unit-testable
//! and trivial to cross-compile for all targets.

pub mod about;
pub mod data;
#[cfg(debug_assertions)]
pub mod dev;
pub mod error;
pub mod font;
pub mod i18n;
pub mod links;
pub mod models;
pub mod platform;
pub mod quiz;
pub mod stats;
pub mod timer;
pub mod update;
pub mod window;

pub use about::{ProjectCredit, APP_VERSION, PROJECT_CREDIT};
pub use data::QuestionBank;
#[cfg(debug_assertions)]
pub use dev::*;
pub use error::{AppError, Result};
pub use font::rem_px;
pub use i18n::{t, tf, I18n, Strings};
pub use models::*;
pub use platform::{Clock, InMemoryStorage, MockClock, Storage, SystemClock};
pub use quiz::{effective_length, pass_threshold, time_for, QuizEngine};
pub use stats::{
    MostMissedQuestion, RecentAttemptStat, StatsCalculator, StatsFilter, StatsSummary,
};
pub use timer::{
    level, QuizTimer, TimerLevel, TimerState, TimerTracker, ERROR_THRESHOLD_PERCENT,
    WARNING_THRESHOLD_PERCENT,
};
