use crate::state::AppState;
use amategeko_core::{QuestionResult, QuizMode, Strings};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultFilter {
    All,
    Correct,
    Wrong,
}

pub struct ResultsView;

impl ResultsView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        _is_desktop: bool,
        filter: ResultFilter,
        expanded: &HashSet<usize>,
        cx: &mut Context<V>,
        on_set_filter: impl Fn(&mut V, ResultFilter, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_expand: impl Fn(&mut V, usize, &mut Window, &mut Context<V>) + 'static + Copy,
        on_retry_wrong: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_new_quiz: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_home: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let result = match &state.last_result {
            Some(res) => res,
            None => {
                return div()
                    .id("results_empty_scroll")
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .p_6()
                    .gap_4()
                    .child(
                        div()
                            .text_lg()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child("Nta bisubizo by'ikizamini bihari."),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(colors.muted_foreground)
                            .child("Tangira ikizamini gishya kugira ngo ubone ibisubizo byawe hano."),
                    )
                    .child(
                        Button::new("home_empty_btn")
                            .primary()
                            .label("Gusubira Ahabanza")
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_home(this, window, cx);
                            })),
                    );
            }
        };

        let score = result.score;
        let total = result.total;
        let pass_mark = result.pass_mark;
        let is_passed = result.passed;
        let percentage = (score * 100).checked_div(total).unwrap_or(0);

        let mode_label = match result.mode {
            QuizMode::Byoroshye => Strings::MODE_EASY_TITLE,
            QuizMode::Hagati => Strings::MODE_MEDIUM_TITLE,
            QuizMode::Bikomeye => Strings::MODE_HARD_TITLE,
            QuizMode::WeakPractice => Strings::HOME_WEAK_TITLE,
            QuizMode::RetryWrong => Strings::RESULTS_RETRY_WRONG,
        };

        let mins = result.duration_seconds / 60;
        let secs = result.duration_seconds % 60;
        let duration_str = format!("{:02}:{:02}", mins, secs);

        let wrong_count = total.saturating_sub(score);
        let correct_count = score;

        // Filter question results
        let filtered_questions: Vec<(usize, &QuestionResult)> = result
            .question_results
            .iter()
            .enumerate()
            .filter(|(_, qr)| match filter {
                ResultFilter::All => true,
                ResultFilter::Correct => qr.is_correct,
                ResultFilter::Wrong => !qr.is_correct,
            })
            .collect();

        div()
            .id("results_scroll_view")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .p_4()
            .gap_4()
            // Top Header: Score Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p_6()
                    .rounded_2xl()
                    .border_1()
                    .border_color(if is_passed { colors.success } else { colors.danger })
                    .bg(colors.secondary)
                    .gap_3()
                    // Score circular badge
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .w(px(110.0))
                            .h(px(110.0))
                            .rounded_full()
                            .border_4()
                            .border_color(if is_passed { colors.success } else { colors.danger })
                            .bg(colors.background)
                            .child(
                                div()
                                    .text_2xl()
                                    .font_extrabold()
                                    .text_color(if is_passed { colors.success } else { colors.danger })
                                    .child(format!("{}/{}", score, total)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(colors.muted_foreground)
                                    .child(format!("{}%", percentage)),
                            ),
                    )
                    // Status text
                    .child(
                        div()
                            .text_xl()
                            .font_bold()
                            .text_color(if is_passed { colors.success } else { colors.danger })
                            .child(if is_passed { Strings::RESULTS_PASSED } else { Strings::RESULTS_FAILED }),
                    )
                    // Mode + Time + Pass threshold metadata row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .justify_center()
                            .gap_3()
                            .text_xs()
                            .text_color(colors.muted_foreground)
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(format!("Uburyo: {}", mode_label)),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(format!("Igihe: {}", duration_str)),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(format!("Gutsinda: {}/{}", pass_mark, total)),
                            ),
                    )
                    // Action Buttons Row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .justify_center()
                            .gap_3()
                            .pt_2()
                            // Retry wrong button (if any mistakes)
                            .when(wrong_count > 0, |el| {
                                el.child(
                                    Button::new("retry_wrong_btn")
                                        .primary()
                                        .label(Strings::RESULTS_RETRY_WRONG)
                                        .icon(IconName::RotateCw)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_retry_wrong(this, window, cx);
                                        })),
                                )
                            })
                            // New quiz button
                            .child(
                                Button::new("new_quiz_btn")
                                    .secondary()
                                    .label(Strings::RESULTS_NEW_QUIZ)
                                    .icon(IconName::Play)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_new_quiz(this, window, cx);
                                    })),
                            )
                            // Home button
                            .child(
                                Button::new("home_btn")
                                    .outline()
                                    .label("Ahabanza")
                                    .icon(IconName::LayoutDashboard)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_home(this, window, cx);
                                    })),
                            ),
                    ),
            )
            // Filter Chips Section
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        // All filter chip
                        div()
                            .id("filter_all_chip")
                            .px_3()
                            .py_1p5()
                            .rounded_full()
                            .cursor_pointer()
                            .border_1()
                            .border_color(if filter == ResultFilter::All { colors.primary } else { colors.border })
                            .bg(if filter == ResultFilter::All { colors.primary } else { colors.secondary })
                            .text_xs()
                            .font_semibold()
                            .text_color(if filter == ResultFilter::All { colors.primary_foreground } else { colors.foreground })
                            .child(format!("Byose ({})", total))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_set_filter(this, ResultFilter::All, window, cx);
                            })),
                    )
                    .child(
                        // Correct filter chip
                        div()
                            .id("filter_correct_chip")
                            .px_3()
                            .py_1p5()
                            .rounded_full()
                            .cursor_pointer()
                            .border_1()
                            .border_color(if filter == ResultFilter::Correct { colors.success } else { colors.border })
                            .bg(if filter == ResultFilter::Correct { colors.success } else { colors.secondary })
                            .text_xs()
                            .font_semibold()
                            .text_color(if filter == ResultFilter::Correct { colors.primary_foreground } else { colors.foreground })
                            .child(format!("Iby'ukuri ({})", correct_count))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_set_filter(this, ResultFilter::Correct, window, cx);
                            })),
                    )
                    .child(
                        // Wrong filter chip
                        div()
                            .id("filter_wrong_chip")
                            .px_3()
                            .py_1p5()
                            .rounded_full()
                            .cursor_pointer()
                            .border_1()
                            .border_color(if filter == ResultFilter::Wrong { colors.danger } else { colors.border })
                            .bg(if filter == ResultFilter::Wrong { colors.danger } else { colors.secondary })
                            .text_xs()
                            .font_semibold()
                            .text_color(if filter == ResultFilter::Wrong { colors.primary_foreground } else { colors.foreground })
                            .child(format!("Ibyakosheje ({})", wrong_count))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_set_filter(this, ResultFilter::Wrong, window, cx);
                            })),
                    ),
            )
            // Question Review List
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(filtered_questions.into_iter().map(|(orig_idx, qr)| {
                        let is_expanded = expanded.contains(&orig_idx);
                        let q = &qr.question;
                        let is_correct = qr.is_correct;
                        let user_ans = qr.user_answer.as_deref();
                        let correct_ans = &qr.correct_answer;

                        let status_color = if is_correct { colors.success } else { colors.danger };
                        let status_icon = if is_correct { IconName::Check } else { IconName::CircleX };

                        let mut sorted_keys: Vec<&String> = q.options.keys().collect();
                        sorted_keys.sort();

                        div()
                            .id(format!("result_card_{}", orig_idx))
                            .flex()
                            .flex_col()
                            .rounded_xl()
                            .border_1()
                            .border_color(if is_expanded { status_color } else { colors.border })
                            .bg(colors.secondary)
                            .p_3()
                            .gap_3()
                            // Card Header Row (Tappable to toggle expand)
                            .child(
                                div()
                                    .id(format!("result_header_{}", orig_idx))
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_toggle_expand(this, orig_idx, window, cx);
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_2()
                                            .flex_1()
                                            // Status Icon (Check or Cross)
                                            .child(
                                                Icon::new(status_icon)
                                                    .size(px(18.0))
                                                    .text_color(status_color),
                                            )
                                            // Question Number + Text Snippet
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_bold()
                                                            .text_color(colors.foreground)
                                                            .child(format!("{}. Ikibazo cya {}", orig_idx + 1, q.id)),
                                                    )
                                                    .when(!is_expanded, |snip| {
                                                        let snippet = if q.text.len() > 65 {
                                                            format!("{}...", &q.text[..60])
                                                        } else {
                                                            q.text.clone()
                                                        };
                                                        snip.child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(colors.muted_foreground)
                                                                .child(snippet),
                                                        )
                                                    }),
                                            ),
                                    )
                                    // Expand / Collapse Chevron
                                    .child(
                                        Icon::new(if is_expanded {
                                            IconName::ChevronUp
                                        } else {
                                            IconName::ChevronDown
                                        })
                                        .size(px(16.0))
                                        .text_color(colors.muted_foreground),
                                    ),
                            )
                            // Card Body (Visible when expanded)
                            .when(is_expanded, |body| {
                                body.child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_3()
                                        .pt_2()
                                        .border_t_1()
                                        .border_color(colors.border)
                                        // Full question text
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_medium()
                                                .text_color(colors.foreground)
                                                .child(q.text.clone()),
                                        )
                                        // Sign image (if present)
                                        .when(q.has_image, |el| {
                                            el.child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .p_2()
                                                    .rounded_lg()
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .bg(colors.background)
                                                    .child(
                                                        img(format!("assets/images/q{}.png", q.id))
                                                            .max_h(px(140.0))
                                                            .rounded_md(),
                                                    ),
                                            )
                                        })
                                        // Unanswered notice
                                        .when(user_ans.is_none(), |el| {
                                            el.child(
                                                div()
                                                    .px_3()
                                                    .py_2()
                                                    .rounded_lg()
                                                    .bg(colors.background)
                                                    .border_1()
                                                    .border_color(colors.warning)
                                                    .text_xs()
                                                    .font_semibold()
                                                    .text_color(colors.warning)
                                                    .child("Ntiwasubije iki kibazo."),
                                            )
                                        })
                                        // Options review
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_2()
                                                .children(sorted_keys.into_iter().map(|key| {
                                                    let opt_text = q.options.get(key).cloned().unwrap_or_default();
                                                    let is_user_choice = user_ans == Some(key.as_str());
                                                    let is_correct_choice = correct_ans == key;

                                                    let (opt_bg, opt_border, badge_text, badge_color) = if is_user_choice && is_correct_choice {
                                                        (colors.success, colors.success, Some("Igisubizo cyawe (Cy'ukuri)"), colors.success)
                                                    } else if is_user_choice && !is_correct_choice {
                                                        (colors.danger, colors.danger, Some("Igisubizo cyawe (Siko)"), colors.danger)
                                                    } else if is_correct_choice {
                                                        (colors.success, colors.success, Some("Igisubizo cy'ukuri"), colors.success)
                                                    } else {
                                                        (colors.background, colors.border, None, colors.muted_foreground)
                                                    };

                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .p_2p5()
                                                        .rounded_lg()
                                                        .border_1()
                                                        .border_color(opt_border)
                                                        .bg(if is_user_choice || is_correct_choice { colors.background } else { opt_bg })
                                                        .gap_1()
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_row()
                                                                .items_start()
                                                                .gap_2()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .font_bold()
                                                                        .text_color(if is_correct_choice {
                                                                            colors.success
                                                                        } else if is_user_choice {
                                                                            colors.danger
                                                                        } else {
                                                                            colors.foreground
                                                                        })
                                                                        .child(format!("{})", key)),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .text_color(colors.foreground)
                                                                        .flex_1()
                                                                        .child(opt_text),
                                                                ),
                                                        )
                                                        .when_some(badge_text, |b, txt| {
                                                            b.child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_semibold()
                                                                    .text_color(badge_color)
                                                                    .child(txt),
                                                            )
                                                        })
                                                })),
                                        ),
                                )
                            })
                    })),
            )
    }
}
