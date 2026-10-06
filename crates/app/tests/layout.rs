use amategeko_app::{AppState, QuizView, Screen, ShellView};
use amategeko_core::{InMemoryStorage, Language, MockClock, QuizEngine, QuizMode};
use gpui::{px, size, Context, IntoElement, Render, TestAppContext, VisualTestContext, Window};
use std::sync::Arc;

struct QuizLayoutHarness {
    state: AppState,
    is_desktop: bool,
    scroll_handle: gpui::ScrollHandle,
}

impl Render for QuizLayoutHarness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        QuizView::render(
            &self.state,
            self.is_desktop,
            false,
            &self.scroll_handle,
            false,
            cx,
            |_, _, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _, _| {},
            |_, _, _| {},
            |_, _, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _, _| {},
        )
    }
}

fn state_with_longest_text(language: Language) -> AppState {
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.settings.language = language;
    state.settings.question_language = language;
    state.start_quiz(QuizMode::Byoroshye);

    let mut question = state
        .bank
        .all()
        .iter()
        .max_by_key(|q| q.text_for(language).chars().count())
        .expect("question bank should not be empty")
        .clone();
    let option = state
        .bank
        .all()
        .iter()
        .flat_map(|q| q.options_for(language).values())
        .max_by_key(|text| text.chars().count())
        .expect("question bank should contain options")
        .clone();
    match language {
        Language::Rw => {
            question
                .options_rw
                .get_or_insert_with(|| question.options.clone())
                .insert("a".into(), option);
        }
        Language::En => {
            question
                .options_en
                .get_or_insert_with(|| question.options.clone())
                .insert("a".into(), option);
        }
    }
    state
        .current_attempt
        .as_mut()
        .expect("attempt should be active")
        .questions[0] = question;
    state
}

fn draw_context(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|window, cx| {
        _ = window.draw(cx);
    });
}

#[gpui::test]
fn longest_question_and_option_fit_minimum_and_desktop_widths(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    for language in [Language::Rw, Language::En] {
        for width in [480.0, 900.0] {
            let window =
                cx.open_window(size(px(width), px(900.0)), move |_, _| QuizLayoutHarness {
                    state: state_with_longest_text(language),
                    is_desktop: width >= 900.0,
                    scroll_handle: gpui::ScrollHandle::default(),
                });
            cx.run_until_parked();
            let mut visual = VisualTestContext::from_window(window.into(), cx);
            visual.update(|window, cx| window.draw(cx).clear(cx));

            let row = visual
                .debug_bounds("quiz-option-row-a")
                .expect("option row should render");
            let badge = visual
                .debug_bounds("quiz-option-badge-a")
                .expect("option badge should render");
            let text = visual
                .debug_bounds("quiz-option-text-a")
                .expect("option text should render");
            let question = visual
                .debug_bounds("quiz-question-text")
                .expect("question text should render");

            assert!(row.right() <= px(width));
            assert!(text.right() <= row.right());
            assert!(question.right() <= px(width));
            assert_eq!(badge.size, size(px(32.0), px(32.0)));
            if width == 480.0 {
                assert!(text.size.height > px(32.0));
            }
        }
    }
}

#[gpui::test]
fn questions_view_in_shell_renders_cards(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.navigate(Screen::Questions);

    let (view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
    let cx: &mut VisualTestContext = cx;
    draw_context(cx);

    let scroll_view = cx
        .debug_bounds("questions_scroll_view")
        .expect("questions_scroll_view should exist");
    assert!(
        scroll_view.size.height > px(100.0),
        "scroll view must have height"
    );

    let card1 = cx.debug_bounds("q_card_1").expect("q_card_1 should render");
    assert!(
        card1.size.height >= px(40.0),
        "q_card_1 must have visible height, got {:?}",
        card1.size.height
    );

    // Switch to "With image" filter and verify image question cards render with visible height
    view.update(cx, |this, cx| {
        this.questions_filter = amategeko_app::ui::questions::QuestionsFilter::HasImage;
        cx.notify();
    });
    draw_context(cx);

    let img_card = cx
        .debug_bounds("q_card_173")
        .expect("image question card should render");
    assert!(
        img_card.size.height >= px(40.0),
        "image card must have visible height, got {:?}",
        img_card.size.height
    );
}

#[gpui::test]
fn segmented_progress_bar_shown_only_on_easy_and_medium(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    for mode in [QuizMode::Byoroshye, QuizMode::Hagati, QuizMode::Bikomeye] {
        let storage = Arc::new(InMemoryStorage::new());
        let clock = Arc::new(MockClock::new(1_000));
        let mut state = AppState::new(storage, clock);
        state.start_quiz(mode);

        let (_view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
        let cx: &mut VisualTestContext = cx;
        draw_context(cx);

        let seg0 = cx.debug_bounds("progress_seg_0");
        if mode == QuizMode::Byoroshye || mode == QuizMode::Hagati {
            assert!(
                seg0.is_some(),
                "progress_seg_0 should exist for mode {:?}",
                mode
            );
        } else {
            assert!(
                seg0.is_none(),
                "progress_seg_0 should NOT exist for mode {:?}",
                mode
            );
        }
    }
}

#[gpui::test]
fn hard_mode_quiz_finishes_before_time_and_shows_results(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.start_quiz(QuizMode::Bikomeye);

    let total = state
        .current_attempt
        .as_ref()
        .expect("attempt should be active")
        .total_questions();
    for _ in 0..total - 1 {
        let att = state.current_attempt.as_mut().unwrap();
        att.answers.insert(att.current_index, "a".into());
        let _ = QuizEngine::confirm_and_advance_hard(att);
    }
    state
        .current_attempt
        .as_mut()
        .unwrap()
        .answers
        .insert(total - 1, "c".into());

    let (view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
    let cx: &mut VisualTestContext = cx;
    draw_context(cx);

    assert_eq!(
        view.read_with(cx, |this, _| this.state.active_screen.clone()),
        Screen::Quiz
    );

    cx.simulate_event(gpui::KeyDownEvent {
        keystroke: gpui::Keystroke {
            modifiers: gpui::Modifiers::default(),
            key: "enter".to_string(),
            key_char: None,
        },
        is_held: false,
        prefer_character_input: false,
    });
    draw_context(cx);

    assert_eq!(
        view.read_with(cx, |this, _| this.state.active_screen.clone()),
        Screen::Results
    );
    let has_result = view.read_with(cx, |this, _| this.state.last_result.is_some());
    assert!(has_result, "last_result must be populated");
}

#[gpui::test]
fn stats_view_in_shell_renders_empty_and_switches_filters(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.navigate(Screen::Stats);

    let (view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
    let cx: &mut VisualTestContext = cx;
    draw_context(cx);

    assert_eq!(
        view.read_with(cx, |this, _| this.state.active_screen.clone()),
        Screen::Stats
    );

    // Test switching filter via shortcuts 1, 2, 3, 4
    for (key, expected_filter) in [("2", "easy"), ("3", "medium"), ("4", "hard"), ("1", "all")] {
        cx.simulate_event(gpui::KeyDownEvent {
            keystroke: gpui::Keystroke {
                modifiers: gpui::Modifiers::default(),
                key: key.to_string(),
                key_char: None,
            },
            is_held: false,
            prefer_character_input: false,
        });
        draw_context(cx);
        assert_eq!(
            view.read_with(cx, |this, _| this.stats_filter.clone()),
            expected_filter
        );
    }
}

#[gpui::test]
fn quiz_view_empty_state_renders_mode_cards_and_starts_quiz(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.navigate(Screen::Quiz);

    let (view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
    let cx: &mut VisualTestContext = cx;
    draw_context(cx);

    assert_eq!(
        view.read_with(cx, |this, _| this.state.active_screen.clone()),
        Screen::Quiz
    );
    assert!(view.read_with(cx, |this, _| this.state.current_attempt.is_none()));

    // Verify empty state mode buttons exist
    assert!(cx.debug_bounds("empty_quiz_btn_easy").is_some());
    assert!(cx.debug_bounds("empty_quiz_btn_medium").is_some());
    assert!(cx.debug_bounds("empty_quiz_btn_hard").is_some());

    // Click easy mode button to start quiz
    let easy_btn = cx.debug_bounds("empty_quiz_btn_easy").unwrap();
    cx.simulate_click(easy_btn.center(), gpui::Modifiers::default());
    draw_context(cx);

    // Verify quiz has started in Easy mode
    assert!(view.read_with(cx, |this, _| this.state.current_attempt.is_some()));
    let mode = view.read_with(cx, |this, _| {
        this.state.current_attempt.as_ref().unwrap().mode
    });
    assert_eq!(mode, QuizMode::Byoroshye);
}

fn home_state(language: Language) -> AppState {
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.settings.language = language;
    state.navigate(Screen::Home);
    state
}

#[gpui::test]
fn home_cards_and_chips_fit_minimum_and_desktop_widths(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    for language in [Language::Rw, Language::En] {
        for width in [480.0, 900.0] {
            let state = home_state(language);
            let window = cx.open_window(size(px(width), px(900.0)), move |_, cx| {
                ShellView::new(state, cx)
            });
            cx.run_until_parked();
            let mut visual = VisualTestContext::from_window(window.into(), cx);
            visual.update(|window, cx| window.draw(cx).clear(cx));

            let cards = ["quiz_mode_easy", "quiz_mode_medium", "quiz_mode_hard"];
            let plays = ["quiz_play_easy", "quiz_play_medium", "quiz_play_hard"];
            for (card_id, play_id) in cards.into_iter().zip(plays) {
                let card = visual
                    .debug_bounds(card_id)
                    .expect("mode card should render");
                let play = visual
                    .debug_bounds(play_id)
                    .expect("play button should render");
                assert!(card.right() <= px(width), "{card_id} overflows at {width}");
                assert!(play.right() <= card.right(), "{play_id} leaves its card");
                assert!(play.size.width >= px(38.0));
                if width < 700.0 {
                    assert!(play.size.height >= px(44.0), "touch target too small");
                }
            }

            for chip_id in [
                "quiz_set_0",
                "quiz_set_1",
                "quiz_set_2",
                "quiz_set_3",
                "quiz_set_4",
                "quiz_set_5",
            ] {
                let chip = visual.debug_bounds(chip_id).expect("chip should render");
                assert!(chip.right() <= px(width), "{chip_id} overflows at {width}");
            }
        }
    }
}

#[gpui::test]
fn home_card_selects_and_play_starts_that_mode(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let state = home_state(Language::En);
    let (view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
    let cx: &mut VisualTestContext = cx;
    draw_context(cx);

    // Easy is selected by default, so the set chips are visible.
    assert!(cx.debug_bounds("quiz_set_0").is_some());

    // Tapping the Medium card only selects it: no quiz starts, chips go away.
    let medium = cx.debug_bounds("quiz_mode_medium").unwrap();
    cx.simulate_click(medium.center(), gpui::Modifiers::default());
    draw_context(cx);
    assert!(view.read_with(cx, |this, _| this.state.current_attempt.is_none()));
    assert!(cx.debug_bounds("quiz_set_0").is_none());

    // PLAY on the Hard card starts Hard at once, even though Medium is selected.
    let play = cx.debug_bounds("quiz_play_hard").unwrap();
    cx.simulate_click(play.center(), gpui::Modifiers::default());
    draw_context(cx);
    let mode = view.read_with(cx, |this, _| {
        this.state.current_attempt.as_ref().map(|a| a.mode)
    });
    assert_eq!(mode, Some(QuizMode::Bikomeye));
    assert_eq!(
        view.read_with(cx, |this, _| this.state.active_screen.clone()),
        Screen::Quiz
    );
}

#[gpui::test]
fn questions_search_focus_and_mobile_ime_drain(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let storage = Arc::new(InMemoryStorage::new());
    let clock = Arc::new(MockClock::new(1_000));
    let mut state = AppState::new(storage, clock);
    state.navigate(Screen::Questions);

    let (view, cx) = cx.add_window_view(move |_, cx| ShellView::new(state, cx));
    let cx: &mut VisualTestContext = cx;
    draw_context(cx);

    // Focus search
    view.update(cx, |this, cx| {
        this.set_questions_search_focused(true, None, cx);
    });
    assert!(view.read_with(cx, |this, _| this.questions_search_focused));

    // Simulate IME text arrival from soft keyboard
    amategeko_app::mobile_ime::push_pending_test("ihangane");
    draw_context(cx);

    assert_eq!(
        view.read_with(cx, |this, _| this.questions_search.clone()),
        "ihangane"
    );

    // Simulate IME submit (newline / Done key)
    amategeko_app::mobile_ime::push_pending_test("\n");
    draw_context(cx);

    // Should have dismissed search focus
    assert!(!view.read_with(cx, |this, _| this.questions_search_focused));
    assert_eq!(
        view.read_with(cx, |this, _| this.questions_search.clone()),
        "ihangane"
    );
}
