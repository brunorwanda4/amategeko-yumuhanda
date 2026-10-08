//! Shared UI for Amategeko y'Umuhanda (Desktop & Mobile).

pub mod assets;
pub mod font;
pub mod mobile_ime;
pub mod platform_ui;
pub mod shortcuts;
pub mod state;
pub mod updater;

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
use std::sync::atomic::{AtomicU8, Ordering};

const SYSTEM_THEME_UNSET: u8 = 0;
const SYSTEM_THEME_LIGHT: u8 = 1;
const SYSTEM_THEME_DARK: u8 = 2;
static SYSTEM_THEME_OVERRIDE: AtomicU8 = AtomicU8::new(SYSTEM_THEME_UNSET);

/// Supplies the system appearance on platforms where GPUI cannot query it directly.
pub fn set_system_theme_dark(is_dark: bool) {
    SYSTEM_THEME_OVERRIDE.store(
        if is_dark {
            SYSTEM_THEME_DARK
        } else {
            SYSTEM_THEME_LIGHT
        },
        Ordering::Relaxed,
    );
}

fn apply_app_palette(cx: &mut gpui::App) {
    let theme = Theme::global_mut(cx);
    let (background, surface, surface_2, border, border_strong, foreground, muted_foreground) =
        if theme.mode == GpuiThemeMode::Dark {
            (
                gpui::rgb(0x000000).into(),
                gpui::rgb(0x0f0f0f).into(),
                gpui::rgb(0x1a1a1a).into(),
                gpui::rgb(0x2a2a2a).into(),
                gpui::rgb(0x3a3a3a).into(),
                gpui::rgb(0xffffff).into(),
                gpui::rgb(0x9a9a9a).into(),
            )
        } else {
            (
                gpui::rgb(0xffffff).into(),
                gpui::rgb(0xf5f5f5).into(),
                gpui::rgb(0xebebeb).into(),
                gpui::rgb(0xdcdcdc).into(),
                gpui::rgb(0xc4c4c4).into(),
                gpui::rgb(0x000000).into(),
                gpui::rgb(0x666666).into(),
            )
        };

    let colors = &mut theme.colors;
    colors.background = background;
    colors.foreground = foreground;
    colors.popover = surface;
    colors.popover_foreground = foreground;
    colors.secondary = surface;
    colors.secondary_foreground = foreground;
    colors.secondary_hover = surface_2;
    colors.secondary_active = border;
    colors.muted = surface_2;
    colors.muted_foreground = muted_foreground;
    colors.accent = surface_2;
    colors.accent_foreground = foreground;
    colors.accordion = surface;
    colors.border = border;
    colors.input = border_strong;
    colors.button = surface_2;
    colors.button_foreground = foreground;
    colors.button_hover = border;
    colors.button_active = border_strong;
    colors.button_secondary = surface_2;
    colors.button_secondary_foreground = foreground;
    colors.button_secondary_hover = border;
    colors.button_secondary_active = border_strong;
    colors.group_box = surface;
    colors.group_box_foreground = foreground;
    colors.description_list_label = surface_2;
    colors.description_list_label_foreground = foreground;
    colors.list = surface;
    colors.list_even = surface_2;
    colors.list_head = surface_2;
    colors.list_hover = surface_2;
    colors.list_active = border;
    colors.list_active_border = border_strong;
    colors.table = surface;
    colors.table_even = surface_2;
    colors.table_head = surface_2;
    colors.table_head_foreground = foreground;
    colors.table_foot = surface_2;
    colors.table_foot_foreground = foreground;
    colors.table_hover = surface_2;
    colors.table_active = border;
    colors.table_active_border = border_strong;
    colors.table_row_border = border;
    colors.sidebar = background;
    colors.sidebar_foreground = foreground;
    colors.sidebar_accent = surface_2;
    colors.sidebar_accent_foreground = foreground;
    colors.sidebar_border = border;
    colors.title_bar = surface;
    colors.title_bar_border = border;
    colors.status_bar = surface;
    colors.status_bar_border = border;
    colors.tab = surface;
    colors.tab_bar = background;
    colors.tab_bar_segmented = surface_2;
    colors.tab_foreground = muted_foreground;
    colors.tab_active = surface_2;
    colors.tab_active_foreground = foreground;
    colors.switch = border_strong;
    colors.switch_thumb = background;
    colors.slider_bar = border_strong;
    colors.slider_thumb = foreground;
    colors.progress_bar = border_strong;
    colors.skeleton = surface_2;
    colors.scrollbar = background;
    colors.scrollbar_thumb = border_strong;
    colors.scrollbar_thumb_hover = muted_foreground;
    colors.window_border = border;

    theme.tokens = (&theme.colors).into();
}

/// Applies the given theme mode to GPUI Kit's theme registry and optionally refreshes the window.
pub fn apply_theme(
    theme: amategeko_core::ThemeMode,
    window: Option<&mut gpui::Window>,
    cx: &mut gpui::App,
) {
    match theme {
        amategeko_core::ThemeMode::System => match SYSTEM_THEME_OVERRIDE.load(Ordering::Relaxed) {
            SYSTEM_THEME_LIGHT => Theme::change(GpuiThemeMode::Light, window, cx),
            SYSTEM_THEME_DARK => Theme::change(GpuiThemeMode::Dark, window, cx),
            _ => Theme::sync_system_appearance(window, cx),
        },
        amategeko_core::ThemeMode::Light => {
            Theme::change(GpuiThemeMode::Light, window, cx);
        }
        amategeko_core::ThemeMode::Dark => {
            Theme::change(GpuiThemeMode::Dark, window, cx);
        }
    }
    apply_app_palette(cx);
    // Theme::change resets fonts from the theme config; re-apply ours.
    font::apply_font_to_theme(cx);
    ui::scroll::configure_scrollbar_motion(cx);
}
