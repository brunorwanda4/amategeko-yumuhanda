use amategeko_app::{AppState, QuizView};
use amategeko_core::{InMemoryStorage, Language, MockClock, QuizMode};
use gpui::{px, size, Context, IntoElement, Render, TestAppContext, VisualTestContext, Window};
use std::sync::Arc;

struct QuizLayoutHarness {
    state: AppState,
    is_desktop: bool,
}

impl Render for QuizLayoutHarness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        QuizView::render(
            &self.state,
            self.is_desktop,
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

#[gpui::test]
fn longest_question_and_option_fit_minimum_and_desktop_widths(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    for language in [Language::Rw, Language::En] {
        for width in [480.0, 900.0] {
            let window =
                cx.open_window(size(px(width), px(900.0)), move |_, _| QuizLayoutHarness {
                    state: state_with_longest_text(language),
                    is_desktop: width >= 900.0,
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
