//! Fixed app font and font-size scaling.
//!
//! The font family is not user-configurable. Only the size scale is.

use gpui::{px, App, Global, Window};
use gpui_kit::component::Theme;
use std::borrow::Cow;

/// The one and only app font family (real family name from the font's name table).
pub const FONT_FAMILY: &str = "Inter";

/// Font family used for display titles and large values.
pub(crate) const DISPLAY_FONT_FAMILY: &str = "Bricolage Grotesque 14pt";

/// Static Inter weights for body text, plus Bricolage Grotesque for display text.
static FONT_FILES: [&[u8]; 8] = [
    include_bytes!("../../../assets/fonts/Inter-Regular.ttf"),
    include_bytes!("../../../assets/fonts/Inter-Medium.ttf"),
    include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf"),
    include_bytes!("../../../assets/fonts/Inter-Bold.ttf"),
    include_bytes!("../../../assets/fonts/Inter-ExtraBold.ttf"),
    include_bytes!("../../../assets/fonts/BricolageGrotesque-Regular.ttf"),
    include_bytes!("../../../assets/fonts/BricolageGrotesque-Bold.ttf"),
    include_bytes!("../../../assets/fonts/BricolageGrotesque-ExtraBold.ttf"),
];

#[derive(Clone, Copy)]
struct FontPrefs {
    family_loaded: bool,
    scale: f32,
}

impl Global for FontPrefs {}

/// Registers the embedded font with GPUI's text system and remembers the saved
/// scale. Call after `gpui_kit::init` and before the first `apply_theme`.
/// If registration fails, a warning is logged and the system font stays in use.
pub fn init_fonts(scale: f32, cx: &mut App) {
    let family_loaded = match cx
        .text_system()
        .add_fonts(FONT_FILES.iter().map(|b| Cow::Borrowed(*b)).collect())
    {
        Ok(()) => true,
        Err(err) => {
            log::warn!("could not load app font {FONT_FAMILY}: {err}; using system font");
            false
        }
    };
    cx.set_global(FontPrefs {
        family_loaded,
        scale,
    });
    apply_font_to_theme(cx);
}

/// Pushes the font family and scaled size into the GPUI Kit theme.
/// No-op until `init_fonts` ran or the theme exists.
pub(crate) fn apply_font_to_theme(cx: &mut App) {
    let Some(prefs) = cx.try_global::<FontPrefs>().copied() else {
        return;
    };
    if !cx.has_global::<Theme>() {
        return;
    }
    let theme = Theme::global_mut(cx);
    theme.font_size = px(amategeko_core::rem_px(prefs.scale));
    if prefs.family_loaded {
        theme.font_family = FONT_FAMILY.into();
    }
    Theme::sync_base(cx);
}

/// Applies a new font-size scale everywhere and redraws the window.
pub fn set_font_scale(scale: f32, window: &mut Window, cx: &mut App) {
    if let Some(prefs) = cx.try_global::<FontPrefs>().copied() {
        cx.set_global(FontPrefs { scale, ..prefs });
    } else {
        cx.set_global(FontPrefs {
            family_loaded: false,
            scale,
        });
    }
    apply_font_to_theme(cx);
    window.set_rem_size(px(amategeko_core::rem_px(scale)));
    window.refresh();
}
