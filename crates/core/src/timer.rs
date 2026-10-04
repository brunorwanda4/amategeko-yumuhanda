use crate::models::{Attempt, QuizMode};
use serde::{Deserialize, Serialize};

/// Warning threshold fraction (25% remaining).
pub const WARNING_THRESHOLD_PERCENT: u32 = 25;

/// Error / urgent threshold fraction (10% remaining).
pub const ERROR_THRESHOLD_PERCENT: u32 = 10;

/// Urgency level of a quiz countdown timer.
///
/// Levels progress in severity: Normal < Warning < Error < Done.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum TimerLevel {
    /// Normal remaining time (> 25% total).
    #[default]
    Normal = 0,
    /// Warning level (<= 25% total remaining).
    Warning = 1,
    /// Error / urgent level (<= 10% total remaining).
    Error = 2,
    /// Time is expired (00:00).
    Done = 3,
}

impl TimerLevel {
    pub const WARNING_THRESHOLD_PERCENT: u32 = WARNING_THRESHOLD_PERCENT;
    pub const ERROR_THRESHOLD_PERCENT: u32 = ERROR_THRESHOLD_PERCENT;

    /// Pure evaluation of timer level given remaining seconds and total attempt duration in seconds.
    pub fn level(remaining_seconds: u32, total_seconds: u32) -> Self {
        level(remaining_seconds, total_seconds)
    }

    /// Returns `true` if this level is strictly more severe (worse) than `other`.
    pub fn is_worse_than(&self, other: &Self) -> bool {
        self > other
    }
}

/// Pure calculation of the timer warning level given remaining seconds and total duration.
///
/// Thresholds:
/// - `Done` at 0 remaining (or 0 total)
/// - `Error` at <= 10% remaining
/// - `Warning` at <= 25% remaining
/// - `Normal` otherwise
pub fn level(remaining_seconds: u32, total_seconds: u32) -> TimerLevel {
    if total_seconds == 0 || remaining_seconds == 0 {
        return TimerLevel::Done;
    }

    let remaining_scaled = (remaining_seconds as u64) * 100;
    let total_scaled = total_seconds as u64;

    if remaining_scaled <= total_scaled * (ERROR_THRESHOLD_PERCENT as u64) {
        TimerLevel::Error
    } else if remaining_scaled <= total_scaled * (WARNING_THRESHOLD_PERCENT as u64) {
        TimerLevel::Warning
    } else {
        TimerLevel::Normal
    }
}

/// Tracks timer progression for an active attempt.
///
/// Only reports a transition when the timer level gets strictly worse (more severe).
/// Level never goes back to normal unless the attempt restarts via `reset()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerTracker {
    pub current_level: TimerLevel,
}

impl Default for TimerTracker {
    fn default() -> Self {
        Self {
            current_level: TimerLevel::Normal,
        }
    }
}

impl TimerTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates the new level from remaining and total seconds.
    /// If the new level is strictly worse than `current_level`, updates `current_level`
    /// and returns `Some(new_level)`. Otherwise returns `None`.
    pub fn update(&mut self, remaining_seconds: u32, total_seconds: u32) -> Option<TimerLevel> {
        let new_level = level(remaining_seconds, total_seconds);
        self.update_level(new_level)
    }

    /// Evaluates a specific `new_level`.
    /// If `new_level > self.current_level`, records it and returns `Some(new_level)`.
    /// Otherwise returns `None`.
    pub fn update_level(&mut self, new_level: TimerLevel) -> Option<TimerLevel> {
        if new_level > self.current_level {
            self.current_level = new_level;
            Some(new_level)
        } else {
            None
        }
    }

    /// Resets the tracker back to `Normal` (used when an attempt restarts or a new quiz begins).
    pub fn reset(&mut self) {
        self.current_level = TimerLevel::Normal;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// No timer active or not applicable (e.g. Easy mode without elapsed display)
    None,
    /// Timer showing elapsed seconds since start (e.g. Easy mode with elapsed timer)
    Elapsed(u32),
    /// Countdown timer with remaining seconds, total seconds, warning level, and urgency flag
    Countdown {
        remaining_seconds: u32,
        total_seconds: u32,
        level: TimerLevel,
        is_urgent: bool,
    },
    /// Countdown has expired
    Expired,
}

pub struct QuizTimer;

impl QuizTimer {
    pub const WARNING_THRESHOLD_PERCENT: u32 = WARNING_THRESHOLD_PERCENT;
    pub const ERROR_THRESHOLD_PERCENT: u32 = ERROR_THRESHOLD_PERCENT;

    /// Computes the timer state for an attempt at the current timestamp.
    pub fn state(attempt: &Attempt, current_time_secs: u64, easy_show_timer: bool) -> TimerState {
        if attempt.completed {
            return TimerState::None;
        }

        match attempt.mode {
            QuizMode::Hagati | QuizMode::Bikomeye => match attempt.deadline_secs {
                Some(deadline) => {
                    let total = attempt.total_duration_secs().unwrap_or(0);
                    if current_time_secs >= deadline {
                        TimerState::Expired
                    } else {
                        let remaining = (deadline - current_time_secs) as u32;
                        let lvl = level(remaining, total);
                        TimerState::Countdown {
                            remaining_seconds: remaining,
                            total_seconds: total,
                            level: lvl,
                            is_urgent: lvl >= TimerLevel::Error,
                        }
                    }
                }
                None => TimerState::None,
            },
            QuizMode::Byoroshye | QuizMode::WeakPractice | QuizMode::RetryWrong => {
                if easy_show_timer {
                    let elapsed = current_time_secs.saturating_sub(attempt.start_time_secs) as u32;
                    TimerState::Elapsed(elapsed)
                } else {
                    TimerState::None
                }
            }
        }
    }

    /// Pure calculation of timer level.
    pub fn level(remaining_seconds: u32, total_seconds: u32) -> TimerLevel {
        level(remaining_seconds, total_seconds)
    }

    /// Reports a change only when the new level gets strictly worse than the previous level.
    pub fn report_level_change(prev: TimerLevel, new: TimerLevel) -> Option<TimerLevel> {
        if new > prev {
            Some(new)
        } else {
            None
        }
    }

    /// Checks if a deadline has expired.
    pub fn is_expired(attempt: &Attempt, current_time_secs: u64) -> bool {
        if let Some(deadline) = attempt.deadline_secs {
            current_time_secs >= deadline
        } else {
            false
        }
    }

    /// Formats seconds as "MM:SS"
    pub fn format_duration(seconds: u32) -> String {
        let mins = seconds / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}", mins, secs)
    }
}
