//! Shared page layout so Home, Questions, Statistics and Settings line up.
//!
//! The Quiz and Results screens keep their own layout on purpose.

use gpui::{div, px, Div, Pixels, Styled};

/// Widest the content column of a page may grow on large windows.
pub const PAGE_MAX_WIDTH: f32 = 720.0;

/// `PAGE_MAX_WIDTH` as `Pixels`, for components that take a width (footer).
pub fn page_max_width() -> Pixels {
    px(PAGE_MAX_WIDTH)
}

/// Outer page padding: 16 px on mobile, 24 px on desktop.
pub trait PagePadding: Styled + Sized {
    fn page_padding(self, is_desktop: bool) -> Self {
        if is_desktop {
            self.p_6()
        } else {
            self.p_4()
        }
    }
}

impl<E: Styled + Sized> PagePadding for E {}

/// Centered content column shared by all standard pages.
pub fn page_column() -> Div {
    div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(page_max_width())
        .mx_auto()
}
