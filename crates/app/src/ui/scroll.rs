use std::time::Duration;

use gpui_kit::component::scroll::{Scrollbar, ScrollbarMode, ScrollbarMotion, ScrollbarStyles};
use gpui_kit::{px, App, ElementId, ScrollHandle};

#[derive(Clone, Copy)]
pub struct ScrollbarContext<'a> {
    pub handle: &'a ScrollHandle,
    pub is_desktop: bool,
    pub reveal_on_open: bool,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use amategeko_core::Question;
    use gpui::{
        div, point, Context, InteractiveElement as _, IntoElement, ParentElement as _, Render,
        ScrollDelta, ScrollWheelEvent, StatefulInteractiveElement as _, Styled as _,
        TestAppContext, VisualTestContext, Window,
    };

    struct ScrollHarness {
        handle: ScrollHandle,
        width: f32,
        longest_question: String,
    }

    impl Render for ScrollHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("scroll-harness")
                .w(px(self.width))
                .h(px(240.0))
                .relative()
                .track_scroll(&self.handle)
                .overflow_y_scroll()
                .pr_3()
                .child(vertical_scrollbar(
                    "scroll-harness-bar",
                    &self.handle,
                    true,
                    false,
                ))
                .child(div().text_sm().child(self.longest_question.clone()))
                .children((0..433).map(|row| {
                    div()
                        .h(px(28.0))
                        .flex_shrink_0()
                        .child(format!("Question {}", row + 1))
                }))
        }
    }

    fn draw(cx: &mut VisualTestContext) {
        cx.run_until_parked();
        cx.update(|window, cx| {
            _ = window.draw(cx);
        });
    }

    #[gpui::test]
    fn full_question_list_scrolls_and_keeps_offset_across_rerenders(cx: &mut TestAppContext) {
        cx.update(gpui_kit::init);
        let questions: Vec<Question> =
            serde_json::from_str(include_str!("../../../../assets/questions.json"))
                .expect("bundled questions must parse in the UI regression test");
        let longest_question = questions
            .iter()
            .max_by_key(|question| {
                question.text.chars().count()
                    + question
                        .options
                        .values()
                        .map(|option| option.chars().count())
                        .sum::<usize>()
            })
            .expect("the bundled question bank must not be empty")
            .text
            .clone();

        assert_eq!(questions.len(), 390);

        let (view, cx) = cx.add_window_view(move |_, _| ScrollHarness {
            handle: ScrollHandle::default(),
            width: 320.0,
            longest_question,
        });
        let cx: &mut VisualTestContext = cx;
        draw(cx);

        cx.simulate_event(ScrollWheelEvent {
            position: point(px(100.0), px(100.0)),
            delta: ScrollDelta::Pixels(point(px(0.0), px(-600.0))),
            ..Default::default()
        });
        draw(cx);
        let offset_before = view.read_with(cx, |view, _| view.handle.offset().y);
        assert!(offset_before < px(0.0));

        view.update(cx, |view, cx| {
            view.width = 260.0;
            view.longest_question = format!("EN: {}", view.longest_question);
            cx.notify();
        });
        draw(cx);

        let offset_after = view.read_with(cx, |view, _| view.handle.offset().y);
        assert_eq!(offset_after, offset_before);
    }
}
