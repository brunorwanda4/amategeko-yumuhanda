use std::time::Duration;

use gpui_kit::component::scroll::{Scrollbar, ScrollbarMode, ScrollbarMotion, ScrollbarStyles};
use gpui_kit::{px, App, ElementId, ScrollHandle};

/// Keeps the component library's theme colors while using the app's compact timing.
pub fn configure_scrollbar_motion(cx: &mut App) {
    let theme = gpui_kit::base::Theme::global_mut(cx);
    theme.scrollbar = theme.scrollbar.clone().with_motion(
        ScrollbarMotion::default()
            .with_idle(Duration::from_secs(1))
            .with_enter(Duration::from_millis(120))
            .with_exit(Duration::from_millis(220))
            .with_expand(Duration::from_millis(120)),
    );
}

/// Builds the shared vertical scrollbar without overriding theme colors.
pub fn vertical_scrollbar(
    id: impl Into<ElementId>,
    handle: &ScrollHandle,
    is_desktop: bool,
    reveal_on_open: bool,
) -> Scrollbar {
    let mode = if reveal_on_open {
        ScrollbarMode::Always
    } else if is_desktop {
        ScrollbarMode::Hover
    } else {
        ScrollbarMode::Scrolling
    };

    let styles = if is_desktop {
        ScrollbarStyles::default()
            .track(|style| style.width(px(10.0)))
            .track_hover(|style| style.width(px(10.0)))
            .track_active(|style| style.width(px(10.0)))
            .thumb(|style| style.width(px(4.0)).inset(px(3.0)))
            .thumb_hover(|style| style.width(px(6.0)).inset(px(2.0)))
            .thumb_active(|style| style.width(px(6.0)).inset(px(2.0)))
    } else {
        ScrollbarStyles::default()
            .track(|style| style.width(px(7.0)))
            .track_hover(|style| style.width(px(7.0)))
            .track_active(|style| style.width(px(7.0)))
            .thumb(|style| style.width(px(3.0)).inset(px(2.0)))
            .thumb_hover(|style| style.width(px(3.0)).inset(px(2.0)))
            .thumb_active(|style| style.width(px(3.0)).inset(px(2.0)))
    };

    Scrollbar::vertical(handle)
        .id(id)
        .mode(mode)
        .styles(|_| styles)
}
