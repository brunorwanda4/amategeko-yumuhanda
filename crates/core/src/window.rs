//! Desktop window size and position rules.
//!
//! Pure geometry with no GPUI or OS types, so it can be unit-tested. All values are
//! logical pixels (physical pixels divided by the display scale factor), which keeps
//! the saved state correct when display scaling changes.

use serde::{Deserialize, Serialize};

/// File (inside the storage directory) holding the last window state.
pub const WINDOW_FILE: &str = "window.json";

/// Backup copy written when `window.json` cannot be parsed.
pub const WINDOW_BACKUP_FILE: &str = "window.json.bak";

/// Preferred window size when nothing is saved.
pub const DEFAULT_WINDOW_SIZE: Size = Size {
    width: 1100.0,
    height: 750.0,
};

/// Preferred minimum window size. Narrow windows use the mobile layout (< 700 px).
pub const MIN_WINDOW_SIZE: Size = Size {
    width: 480.0,
    height: 520.0,
};

/// Space kept free between the window and the display edges.
pub const SCREEN_MARGIN: f32 = 16.0;

/// Largest share of the primary display used by the default window size.
pub const DEFAULT_SCREEN_FRACTION: f32 = 0.8;

/// Pause after the last move or resize before the window state is saved.
pub const SAVE_DELAY_MS: u64 = 500;

/// A width and height in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// A rectangle in logical pixels (origin at the top-left corner).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && other.x < self.x + self.width
            && self.y < other.y + other.height
            && other.y < self.y + self.height
    }
}

/// A connected display: its usable area (excluding the taskbar) and whether it is primary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayRect {
    pub bounds: Rect,
    pub primary: bool,
}

/// Saved window placement. When `maximized` is true, the rectangle is the size the
/// window returns to when it is un-maximized.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowState {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub maximized: bool,
}

impl WindowState {
    /// The window rectangle (restore rectangle when maximized).
    pub fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    fn is_valid(&self) -> bool {
        [self.x, self.y, self.width, self.height]
            .iter()
            .all(|v| v.is_finite())
            && self.width > 0.0
            && self.height > 0.0
    }
}

/// Used only when the platform reports no displays at all.
fn fallback_display(default: Size) -> Rect {
    Rect {
        x: 0.0,
        y: 0.0,
        width: default.width + 2.0 * SCREEN_MARGIN,
        height: default.height + 2.0 * SCREEN_MARGIN,
    }
}

/// The primary display, or the first one if none is flagged primary.
fn primary_display(displays: &[DisplayRect], default: Size) -> Rect {
    displays
        .iter()
        .find(|d| d.primary)
        .or_else(|| displays.first())
        .map(|d| d.bounds)
        .unwrap_or_else(|| fallback_display(default))
}

/// Largest window that fits on `display` with the margin on every side.
fn max_size_on(display: &Rect) -> Size {
    Size {
        width: (display.width - 2.0 * SCREEN_MARGIN).max(1.0),
        height: (display.height - 2.0 * SCREEN_MARGIN).max(1.0),
    }
}

/// Minimum window size for `display`: `min`, or the display minus the margin if smaller.
pub fn min_window_size(display: &Rect, min: Size) -> Size {
    let max = max_size_on(display);
    Size {
        width: min.width.min(max.width),
        height: min.height.min(max.height),
    }
}

fn clamp_size(size: Size, display: &Rect, min: Size) -> Size {
    let max = max_size_on(display);
    let min = min_window_size(display, min);
    Size {
        width: size.width.clamp(min.width, max.width),
        height: size.height.clamp(min.height, max.height),
    }
}

fn centered_on(display: &Rect, size: Size, maximized: bool) -> WindowState {
    WindowState {
        x: display.x + (display.width - size.width) / 2.0,
        y: display.y + (display.height - size.height) / 2.0,
        width: size.width,
        height: size.height,
        maximized,
    }
}

/// Index of the display whose area contains the centre of `rect`, if any.
fn display_containing_center(rect: &Rect, displays: &[DisplayRect]) -> Option<usize> {
    let (cx, cy) = rect.center();
    displays
        .iter()
        .position(|d| d.bounds.contains_point(cx, cy))
}

/// Index of the display a fitted window belongs to: the one containing its centre,
/// otherwise the primary display. Returns `None` only when `displays` is empty.
pub fn display_index_for(state: &WindowState, displays: &[DisplayRect]) -> Option<usize> {
    display_containing_center(&state.rect(), displays)
        .or_else(|| displays.iter().position(|d| d.primary))
        .or_else(|| (!displays.is_empty()).then_some(0))
}

/// True when any part of `rect` is on a connected display. A minimized window on
/// Windows is moved far off-screen, so this is false while it is minimized.
pub fn is_on_any_display(rect: &Rect, displays: &[DisplayRect]) -> bool {
    displays.iter().any(|d| d.bounds.intersects(rect))
}

/// Decide where the window opens.
///
/// - No (or invalid) saved state: the smaller of `default` and 80% of the primary
///   display, centred on the primary display.
/// - Saved state whose centre is on a connected display: kept, with its size clamped to
///   that display (minus the margin) and its position moved just enough to stay inside.
/// - Saved state that is off-screen (monitor unplugged, resolution changed): the saved
///   size clamped to the primary display, centred on it.
/// - `maximized` is always kept.
pub fn fit_window(
    saved: Option<WindowState>,
    displays: &[DisplayRect],
    default: Size,
    min: Size,
) -> WindowState {
    let primary = primary_display(displays, default);

    let Some(saved) = saved.filter(WindowState::is_valid) else {
        let preferred = Size {
            width: default.width.min(primary.width * DEFAULT_SCREEN_FRACTION),
            height: default.height.min(primary.height * DEFAULT_SCREEN_FRACTION),
        };
        return centered_on(&primary, clamp_size(preferred, &primary, min), false);
    };

    let wanted = Size {
        width: saved.width,
        height: saved.height,
    };

    match display_containing_center(&saved.rect(), displays) {
        Some(index) => {
            let display = displays[index].bounds;
            let size = clamp_size(wanted, &display, min);
            let max_x = display.x + display.width - size.width;
            let max_y = display.y + display.height - size.height;
            WindowState {
                x: saved.x.clamp(display.x, max_x.max(display.x)),
                y: saved.y.clamp(display.y, max_y.max(display.y)),
                width: size.width,
                height: size.height,
                maximized: saved.maximized,
            }
        }
        None => centered_on(
            &primary,
            clamp_size(wanted, &primary, min),
            saved.maximized,
        ),
    }
}
