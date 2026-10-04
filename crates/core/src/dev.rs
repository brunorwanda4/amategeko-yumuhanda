//! Development loop state persistence and test helpers.
//!
//! Available only in debug builds (#[cfg(debug_assertions)]).
//! Releases do not compile this module or link dev state files.

use crate::models::Attempt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Name of the dev state JSON file in storage.
pub const DEV_STATE_FILE: &str = "dev_state.json";

/// Top-level app screens for development restoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DevScreen {
    #[default]
    Home,
    Quiz,
    Results,
    #[serde(alias = "questions")]
    Browse,
    Stats,
    Settings,
}

/// Filter options for the Browse (Questions) screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BrowseFilter {
    #[default]
    All,
    HasImage,
    Mistakes,
    Starred,
}

/// Persisted developer state saved on every navigation, filter, or scroll change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DevState {
    #[serde(default)]
    pub screen: DevScreen,
    #[serde(default)]
    pub browse_filter: Option<BrowseFilter>,
    #[serde(default)]
    pub stats_filter: Option<String>,
    #[serde(default)]
    pub results_filter: Option<String>,
    #[serde(default)]
    pub scroll_position: f32,
    #[serde(default)]
    pub scroll_positions: HashMap<String, f32>,
}

impl DevState {
    pub fn new(screen: DevScreen) -> Self {
        Self {
            screen,
            browse_filter: None,
            stats_filter: None,
            results_filter: None,
            scroll_position: 0.0,
            scroll_positions: HashMap::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.scroll_position.is_finite() && self.scroll_position >= 0.0
    }
}

/// Parse a debug timer argument like "30s", "30S", or "30" into seconds.
pub fn parse_timer_arg(arg: &str) -> Option<u64> {
    let s = arg.trim();
    let s = s
        .strip_suffix('s')
        .or_else(|| s.strip_suffix('S'))
        .unwrap_or(s);
    s.parse::<u64>().ok().filter(|&secs| secs > 0)
}

/// Applies a remaining-time override to a countdown attempt (Medium / Hard).
///
/// Modifies `deadline_secs` and `start_time_secs` so that `deadline_secs - now_secs == remaining_secs`
/// while preserving the total configured duration of the quiz.
pub fn apply_timer_override(attempt: &mut Attempt, remaining_secs: u64, now_secs: u64) {
    if attempt.mode.has_countdown() {
        if let Some(deadline) = attempt.deadline_secs {
            let total = deadline.saturating_sub(attempt.start_time_secs);
            attempt.deadline_secs = Some(now_secs.saturating_add(remaining_secs));
            attempt.start_time_secs = now_secs.saturating_sub(total.saturating_sub(remaining_secs));
        }
    }
}
