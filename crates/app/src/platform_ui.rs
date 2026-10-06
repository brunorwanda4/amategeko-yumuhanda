//! Compile-target helpers for desktop OS vs Android/iOS APK.
//!
//! Window width (`is_desktop` layout) is separate: a narrow desktop window is
//! still desktop OS; a wide tablet APK is still mobile.

/// True when this binary targets Android or iOS (APK / mobile app), not window width.
#[inline]
pub const fn is_native_mobile() -> bool {
    cfg!(any(target_os = "android", target_os = "ios"))
}

/// Whether keyboard shortcut badges, tooltips, and the help dialog may appear.
/// Never on APK; on desktop OS only when the wide layout and setting allow it.
#[inline]
pub fn shortcuts_ui_active(is_desktop_layout: bool, desktop_shortcuts_enabled: bool) -> bool {
    !is_native_mobile() && is_desktop_layout && desktop_shortcuts_enabled
}
