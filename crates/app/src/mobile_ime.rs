//! Soft-keyboard (IME) bridge for the questions search field on Android/iOS.
//!
//! The search UI is a custom focused field (not a native EditText). gpui-mobile
//! shows the IME via [`show_keyboard`] and delivers typed text through
//! [`set_text_input_callback`]. Desktop builds are no-ops.

use std::cell::RefCell;

thread_local! {
    static PENDING_TEXT: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Show the soft keyboard and install the text callback (Android/iOS only).
pub fn begin_search_ime() {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        PENDING_TEXT.with(|pending| pending.borrow_mut().clear());
        gpui_mobile::set_text_input_callback(Some(Box::new(|text: &str| {
            PENDING_TEXT.with(|pending| {
                pending.borrow_mut().push(text.to_string());
            });
        })));
        gpui_mobile::TEXT_INPUT_DIRTY.store(true, std::sync::atomic::Ordering::Release);
        gpui_mobile::show_keyboard();
    }
}

/// Hide the soft keyboard and clear the text callback (Android/iOS only).
pub fn end_search_ime() {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        gpui_mobile::hide_keyboard();
        gpui_mobile::set_text_input_callback(None);
        PENDING_TEXT.with(|pending| pending.borrow_mut().clear());
    }
}

/// Apply pending soft-keyboard text into `search`, returning true if it changed.
///
/// Soft keyboard sends `"\x08"` for backspace (same as gpui-mobile form example).
pub fn drain_pending_into_search(search: &mut String) -> bool {
    let texts = PENDING_TEXT.with(|pending| pending.borrow_mut().drain(..).collect::<Vec<_>>());
    if texts.is_empty() {
        return false;
    }

    let backspace_count = texts.iter().filter(|t| t.as_str() == "\x08").count();
    if backspace_count >= 6 {
        if search.is_empty() {
            return false;
        }
        search.clear();
        return true;
    }

    let mut changed = false;
    for text in texts {
        match text.as_str() {
            "\x08" => {
                if search.pop().is_some() {
                    changed = true;
                }
            }
            "\n" | "\r" | "\r\n" => {
                // Search submits / dismisses via shell Escape / Enter; ignore newline.
            }
            other => {
                let clean = other.replace('\n', " ").replace('\r', "");
                if !clean.is_empty() {
                    search.push_str(&clean);
                    changed = true;
                }
            }
        }
    }
    changed
}
