//! Core data models, quiz rules, stats, and platform traits for Amategeko y'Umuhanda.
//!
//! This crate contains NO GPUI dependencies, making it pure Rust, 100% unit-testable
//! and trivial to cross-compile for all targets.

pub mod data;
pub mod error;
pub mod i18n;
pub mod models;
pub mod platform;
pub mod quiz;
pub mod stats;
pub mod timer;

pub use data::QuestionBank;
pub use error::{AppError, Result};
pub use i18n::Strings;
pub use models::*;
pub use platform::{Clock, InMemoryStorage, MockClock, Storage, SystemClock};
pub use quiz::QuizEngine;
pub use stats::{MostMissedQuestion, StatsCalculator, StatsSummary};
pub use timer::{QuizTimer, TimerState};
