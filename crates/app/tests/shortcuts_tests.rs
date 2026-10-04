use amategeko_app::shortcuts::{ShortcutAction, ShortcutRegistry, ShortcutScope};
use amategeko_core::{Attempt, QuizEngine, QuizMode};

#[test]
fn test_registry_has_no_duplicate_keys_per_screen() {
    let registry = ShortcutRegistry::new();
    let all = registry.all();

    let all_scopes = [
        ShortcutScope::Home,
        ShortcutScope::QuizAll,
        ShortcutScope::QuizEasy,
        ShortcutScope::QuizMedium,
        ShortcutScope::QuizHard,
        ShortcutScope::Results,
        ShortcutScope::Questions,
        ShortcutScope::Stats,
        ShortcutScope::Settings,
    ];

    for scope in all_scopes {
        let mut seen = std::collections::HashSet::new();
        for def in all {
            if def.scopes.contains(&scope) || def.scopes.contains(&ShortcutScope::Global) {
                // In Hard quiz mode, global navigation shortcuts are deactivated,
                // but for non-Hard scopes, check that no two shortcuts share the exact same key combo
                if scope == ShortcutScope::QuizHard && def.action.is_navigation() {
                    continue;
                }
                let key_str = format!("{:?}", def.key);
                assert!(
                    seen.insert(key_str.clone()),
                    "Duplicate shortcut key combo '{key_str}' found in scope {scope:?} for action {:?}",
                    def.action
                );
            }
        }
    }
}

#[test]
fn test_hard_mode_exposes_no_navigation_shortcuts() {
    let registry = ShortcutRegistry::new();
    let hard_shortcuts =
        registry.shortcuts_for_context(amategeko_app::Screen::Quiz, Some(QuizMode::Bikomeye));

    for def in hard_shortcuts {
        assert!(
            !def.action.is_navigation(),
            "Hard mode must not expose navigation shortcut: {:?}",
            def.action
        );
        assert!(
            def.action.is_allowed_in_hard(),
            "Action {:?} is not allowed in Hard mode",
            def.action
        );
        // Only A-D, 1-4, Enter (and Help ?/F1)
        match def.action {
            ShortcutAction::ChooseOption(_)
            | ShortcutAction::NextOrConfirm
            | ShortcutAction::ShowHelp => {}
            other => panic!("Unexpected action in Hard mode: {other:?}"),
        }
    }
}

#[test]
fn test_easy_next_is_blocked_before_answering() {
    let q1 = amategeko_core::Question {
        id: 1,
        text: "Ikibazo 1".to_string(),
        options: std::collections::BTreeMap::from([
            ("a".to_string(), "Yego".to_string()),
            ("b".to_string(), "Oya".to_string()),
        ]),
        correct: "a".to_string(),
        image: None,
        has_image: false,
        text_en: None,
        options_en: None,
        text_rw: None,
        options_rw: None,
        status_en: None,
    };
    let mut attempt = Attempt::new(
        "test-session".to_string(),
        QuizMode::Byoroshye,
        vec![q1],
        0,
        Some(100),
    );

    // 1. Before answering: Next is blocked
    assert!(!ShortcutRegistry::can_advance_easy_next(&attempt));

    // 2. After answering: Next is allowed
    let _ = QuizEngine::select_option(&mut attempt, "a");
    assert!(ShortcutRegistry::can_advance_easy_next(&attempt));
}

#[test]
fn test_shortcuts_ignored_while_typing_and_when_disabled() {
    let registry = ShortcutRegistry::new();

    // Helper to make a KeyDownEvent
    let make_event = |key: &str, modifiers: gpui::Modifiers| gpui::KeyDownEvent {
        keystroke: gpui::Keystroke {
            modifiers,
            key: key.to_string(),
            key_char: None,
        },
        is_held: false,
        prefer_character_input: false,
    };

    // 1. When disabled in Settings, all shortcuts are ignored
    let ev_a = make_event("a", gpui::Modifiers::default());
    let res_disabled = registry.resolve_event(
        &ev_a,
        amategeko_app::Screen::Quiz,
        Some(QuizMode::Hagati),
        false, // disabled!
        false,
    );
    assert_eq!(res_disabled, None);

    // 2. When enabled, normal shortcut works
    let res_enabled = registry.resolve_event(
        &ev_a,
        amategeko_app::Screen::Quiz,
        Some(QuizMode::Hagati),
        true, // enabled
        false,
    );
    assert_eq!(res_enabled, Some(ShortcutAction::ChooseOption('a')));

    // 3. When text input is focused (typing), single-key shortcuts are ignored
    let res_typing_single = registry.resolve_event(
        &ev_a,
        amategeko_app::Screen::Questions,
        None,
        true,
        true, // typing!
    );
    assert_eq!(res_typing_single, None);

    // But Esc and Ctrl combos work while typing
    let ev_esc = make_event("escape", gpui::Modifiers::default());
    let res_typing_esc = registry.resolve_event(
        &ev_esc,
        amategeko_app::Screen::Questions,
        None,
        true,
        true, // typing
    );
    assert_eq!(res_typing_esc, Some(ShortcutAction::CloseOrBack));

    let mut ctrl_mod = gpui::Modifiers::default();
    if cfg!(target_os = "macos") {
        ctrl_mod.platform = true;
    } else {
        ctrl_mod.control = true;
    }
    let ev_ctrl_f = make_event("f", ctrl_mod);
    let res_typing_ctrl_f = registry.resolve_event(
        &ev_ctrl_f,
        amategeko_app::Screen::Questions,
        None,
        true,
        true, // typing
    );
    assert_eq!(res_typing_ctrl_f, Some(ShortcutAction::FocusSearch));
}
