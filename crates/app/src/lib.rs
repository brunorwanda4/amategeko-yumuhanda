//! Shared UI for Amategeko y'Umuhanda (Desktop & Mobile).

pub mod assets;
pub mod font;
pub mod mobile_ime;
pub mod platform_ui;
pub mod shortcuts;
pub mod state;

pub use assets::AppAssets;
pub use font::{init_fonts, set_font_scale, FONT_FAMILY};
pub mod ui;

pub use platform_ui::{is_native_mobile, shortcuts_ui_active};
pub use shortcuts::{ShortcutAction, ShortcutDef, ShortcutRegistry};
pub use state::{AppState, Screen};
pub use ui::home::HomeView;
pub use ui::quiz::QuizView;
pub use ui::shell::{get_safe_area, set_safe_area, SafeArea, ShellView};

use gpui_kit::component::{Theme, ThemeMode as GpuiThemeMode};

/// Applies the given theme mode to GPUI Kit's theme registry and optionally refreshes the window.
pub fn apply_theme(
    theme: amategeko_core::ThemeMode,
    window: Option<&mut gpui::Window>,
    cx: &mut gpui::App,
) {
    match theme {
        amategeko_core::ThemeMode::System => {
            Theme::sync_system_appearance(window, cx);
        }
        amategeko_core::ThemeMode::Light => {
            Theme::change(GpuiThemeMode::Light, window, cx);
        }
        amategeko_core::ThemeMode::Dark => {
            Theme::change(GpuiThemeMode::Dark, window, cx);
        }
    }
    // Theme::change resets fonts from the theme config; re-apply ours.
    font::apply_font_to_theme(cx);
    ui::scroll::configure_scrollbar_motion(cx);
}
