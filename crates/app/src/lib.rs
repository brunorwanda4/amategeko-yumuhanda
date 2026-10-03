//! Shared UI for Amategeko y'Umuhanda (Desktop & Mobile).

pub mod state;
pub mod ui;

pub use state::{AppState, Screen};
pub use ui::home::HomeView;
pub use ui::quiz::QuizView;
pub use ui::shell::ShellView;
