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

/// Result of draining soft-keyboard text into the search buffer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ImeDrainResult {
    pub changed: bool,
    pub submitted: bool,
}

/// Apply pending soft-keyboard text into `search`, returning an [`ImeDrainResult`].
///
/// Soft keyboard sends `"\x08"` for backspace (same as gpui-mobile form example).
pub fn drain_pending_into_search(search: &mut String) -> ImeDrainResult {
    let texts = PENDING_TEXT.with(|pending| pending.borrow_mut().drain(..).collect::<Vec<_>>());
    if texts.is_empty() {
        return ImeDrainResult::default();
    }

    let backspace_count = texts.iter().filter(|t| t.as_str() == "\x08").count();
    if backspace_count >= 6 {
        if search.is_empty() {
            return ImeDrainResult::default();
        }
        search.clear();
        return ImeDrainResult {
            changed: true,
            submitted: false,
        };
    }

    let mut changed = false;
    let mut submitted = false;
    for text in texts {
        match text.as_str() {
            "\x08" => {
                if search.pop().is_some() {
                    changed = true;
                }
            }
            "\n" | "\r" | "\r\n" => {
                // Done/Enter on soft keyboard submits and dismisses keyboard
                submitted = true;
            }
            other => {
                if other.starts_with('\x1b') {
                    // Ignore cursor / arrow escape sequences (e.g. \x1b[D)
                    continue;
                }
                let clean = other.replace('\n', " ").replace('\r', "");
                if !clean.is_empty() {
                    search.push_str(&clean);
                    changed = true;
                }
            }
        }
    }
    ImeDrainResult { changed, submitted }
}

#[doc(hidden)]
pub fn push_pending_test(text: &str) {
    PENDING_TEXT.with(|pending| pending.borrow_mut().push(text.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drain_pending_text_appends_and_backspaces() {
        let mut search = String::new();

        push_pending_test("ih");
        push_pending_test("angane");
        let res = drain_pending_into_search(&mut search);
        assert!(res.changed);
        assert!(!res.submitted);
        assert_eq!(search, "ihangane");

        push_pending_test("\x08");
        push_pending_test("\x08");
        let res = drain_pending_into_search(&mut search);
        assert!(res.changed);
        assert_eq!(search, "ihanga");
    }

    #[test]
    fn test_drain_pending_consecutive_backspaces_clears() {
        let mut search = "umuhanda".to_string();
        for _ in 0..6 {
            push_pending_test("\x08");
        }
        let res = drain_pending_into_search(&mut search);
        assert!(res.changed);
        assert_eq!(search, "");
    }

    #[test]
    fn test_drain_pending_submits_on_newline() {
        let mut search = "icyapa".to_string();
        push_pending_test("\n");
        let res = drain_pending_into_search(&mut search);
        assert!(!res.changed);
        assert!(res.submitted);
        assert_eq!(search, "icyapa");
    }

    #[test]
    fn test_drain_pending_ignores_escape_sequences() {
        let mut search = "test".to_string();
        push_pending_test("\x1b[D");
        push_pending_test("\x1b[C");
        let res = drain_pending_into_search(&mut search);
        assert!(!res.changed);
        assert_eq!(search, "test");
    }
}
