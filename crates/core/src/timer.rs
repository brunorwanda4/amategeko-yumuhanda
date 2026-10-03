use crate::models::{Attempt, QuizMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// No timer active or not applicable (e.g. Easy mode without elapsed display)
    None,
    /// Timer showing elapsed seconds since start (e.g. Easy mode with elapsed timer)
    Elapsed(u32),
    /// Countdown timer with remaining seconds and urgent flag (<= 120s)
    Countdown {
        remaining_seconds: u32,
        is_urgent: bool,
    },
    /// Countdown has expired
    Expired,
}

pub struct QuizTimer;

impl QuizTimer {
    /// Computes the timer state for an attempt at the current timestamp.
    pub fn state(attempt: &Attempt, current_time_secs: u64, easy_show_timer: bool) -> TimerState {
        if attempt.completed {
            return TimerState::None;
        }

        match attempt.mode {
            QuizMode::Hagati | QuizMode::Bikomeye => {
                match attempt.deadline_secs {
                    Some(deadline) => {
                        if current_time_secs >= deadline {
                            TimerState::Expired
                        } else {
                            let remaining = (deadline - current_time_secs) as u32;
                            TimerState::Countdown {
                                remaining_seconds: remaining,
                                is_urgent: remaining <= 120, // last 2 minutes
                            }
                        }
                    }
                    None => TimerState::None,
                }
            }
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
