use crate::state::AppState;
use amategeko_core::{Attempt, QuizMode, QuizTimer, Strings, TimerState};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct QuizView;

impl QuizView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        _is_desktop: bool,
        cx: &mut Context<V>,
        on_select_option: impl Fn(&mut V, &str, &mut Window, &mut Context<V>) + 'static + Copy,
        on_next: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_prev: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_skip: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_hard_confirm: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_jump_to: impl Fn(&mut V, usize, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_flag: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_star: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_finish_request: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_abandon_request: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let attempt = match &state.current_attempt {
            Some(att) => att,
            None => {
                return div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .child(
                        div()
                            .text_base()
                            .text_color(colors.foreground)
                            .child("Nta kizamini kiri gukorwa."),
                    );
            }
        };

        let now = state.clock.now_seconds();
        let timer_state = QuizTimer::state(attempt, now, state.settings.easy_show_timer);

        let current_idx = attempt.current_index;
        let total_questions = attempt.total_questions();
        let current_q = match attempt.current_question() {
            Some(q) => q,
            None => return div().child("Ikibazo ntikibonetse"),
        };

        let user_ans = attempt.current_answer().cloned();
        let is_locked = attempt.is_current_locked();
        let is_flagged = attempt.is_current_flagged();
        let is_starred = state.progress.is_starred(current_q.id);

        let mode_color = match attempt.mode {
            QuizMode::Byoroshye => colors.success,
            QuizMode::Hagati => colors.warning,
            QuizMode::Bikomeye => colors.danger,
            QuizMode::WeakPractice | QuizMode::RetryWrong => colors.primary,
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.background)
            // Top Bar: Back / Mode Badge / Counter / Timer / Quick Action
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    // Back / Abandon
                    .child(
                        div()
                            .id("quiz_back_btn")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|el| el.bg(colors.accent))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_abandon_request(this, window, cx);
                            }))
                            .child(
                                Icon::new(IconName::ArrowLeft)
                                    .size(px(18.0))
                                    .text_color(colors.foreground),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(colors.foreground)
                                    .child("Sohoka"),
                            ),
                    )
                    // Mode Badge & Progress Counter
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .px_2()
                                    .py_0p5()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(mode_color)
                                    .text_color(mode_color)
                                    .child(attempt.mode.title_kinyarwanda()),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_bold()
                                    .text_color(colors.foreground)
                                    .child(format!("{}/{}", current_idx + 1, total_questions)),
                            ),
                    )
                    // Timer & Star / Flag Actions
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            // Timer display
                            .child(match timer_state {
                                TimerState::Countdown {
                                    remaining_seconds,
                                    is_urgent,
                                } => {
                                    let timer_color = if is_urgent {
                                        colors.danger
                                    } else {
                                        colors.foreground
                                    };
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            Icon::new(IconName::Bell)
                                                .size(px(16.0))
                                                .text_color(timer_color),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_bold()
                                                .text_color(timer_color)
                                                .child(QuizTimer::format_duration(
                                                    remaining_seconds,
                                                )),
                                        )
                                }
                                TimerState::Elapsed(secs) => div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        Icon::new(IconName::Bell)
                                            .size(px(16.0))
                                            .text_color(colors.muted_foreground),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child(QuizTimer::format_duration(secs)),
                                    ),
                                _ => div(),
                            })
                            // Star / Bookmark toggle (Easy modes)
                            .when(
                                matches!(
                                    attempt.mode,
                                    QuizMode::Byoroshye
                                        | QuizMode::WeakPractice
                                        | QuizMode::RetryWrong
                                ),
                                |el| {
                                    let q_id = current_q.id;
                                    let star_icon = if is_starred {
                                        IconName::StarFill
                                    } else {
                                        IconName::Star
                                    };
                                    let star_color = if is_starred {
                                        colors.warning
                                    } else {
                                        colors.muted_foreground
                                    };
                                    el.child(
                                        div()
                                            .id("star_btn")
                                            .p_1()
                                            .cursor_pointer()
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_toggle_star(this, q_id, window, cx);
                                            }))
                                            .child(
                                                Icon::new(star_icon)
                                                    .size(px(18.0))
                                                    .text_color(star_color),
                                            ),
                                    )
                                },
                            )
                            // Flag toggle (Medium mode)
                            .when(attempt.mode == QuizMode::Hagati, |el| {
                                let flag_color = if is_flagged {
                                    colors.warning
                                } else {
                                    colors.muted_foreground
                                };
                                el.child(
                                    div()
                                        .id("flag_btn")
                                        .p_1()
                                        .cursor_pointer()
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_toggle_flag(this, window, cx);
                                        }))
                                        .child(
                                            Icon::new(IconName::TriangleAlert)
                                                .size(px(18.0))
                                                .text_color(flag_color),
                                        ),
                                )
                            }),
                    ),
            )
            // Segmented Progress Bar (Hard mode)
            .when(attempt.mode == QuizMode::Bikomeye, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_row()
                        .w_full()
                        .h(px(4.0))
                        .children((0..total_questions).map(|i| {
                            let seg_color = if i < current_idx {
                                colors.primary
                            } else if i == current_idx {
                                colors.accent
                            } else {
                                colors.border
                            };
                            div()
                                .flex_1()
                                .h_full()
                                .bg(seg_color)
                                .when(i > 0, |seg| seg.border_l_1().border_color(colors.background))
                        })),
                )
            })
            // Question Grid (Medium mode)
            .when(attempt.mode == QuizMode::Hagati, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .justify_center()
                        .gap_1p5()
                        .px_4()
                        .py_2()
                        .border_b_1()
                        .border_color(colors.border)
                        .bg(colors.background)
                        .children((0..total_questions).map(|i| {
                            let is_cur = i == current_idx;
                            let is_ans = attempt.answers.contains_key(&i);
                            let has_flag = attempt.flags.contains(&i);

                            let cell_bg = if is_cur {
                                colors.primary
                            } else if is_ans {
                                colors.secondary
                            } else {
                                colors.background
                            };

                            let cell_text_color = if is_cur {
                                colors.primary_foreground
                            } else if is_ans {
                                colors.foreground
                            } else {
                                colors.muted_foreground
                            };

                            div()
                                .id(format!("grid_cell_{i}"))
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(28.0))
                                .h(px(28.0))
                                .rounded_md()
                                .border_1()
                                .border_color(if is_cur {
                                    colors.primary
                                } else if has_flag {
                                    colors.warning
                                } else {
                                    colors.border
                                })
                                .bg(cell_bg)
                                .cursor_pointer()
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_jump_to(this, i, window, cx);
                                }))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_semibold()
                                        .text_color(cell_text_color)
                                        .child(format!("{}", i + 1)),
                                )
                        })),
                )
            })
            // Main Question Body (Scrollable)
            .child(
                div()
                    .id("quiz_content_scroll")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .p_4()
                    .gap_4()
                    // Question text
                    .child(
                        div()
                            .text_lg()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child(format!("{}. {}", current_q.id, current_q.text)),
                    )
                    // Sign image (if question has an image)
                    .when(current_q.has_image, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .p_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(colors.border)
                                .bg(colors.secondary)
                                .child(
                                    img(format!("assets/images/q{}.png", current_q.id))
                                        .max_h(px(180.0))
                                        .rounded_lg(),
                                ),
                        )
                    })
                    // Options list
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .children(current_q.options.iter().map(|(letter, opt_text)| {
                                Self::render_option_card(
                                    attempt,
                                    letter,
                                    opt_text,
                                    &current_q.correct,
                                    user_ans.as_deref(),
                                    is_locked,
                                    cx,
                                    on_select_option,
                                )
                            })),
                    )
                    // Easy mode explanation / hint
                    .when(attempt.mode.provides_instant_feedback(), |el| {
                        if let Some(ans) = &user_ans {
                            let is_correct = ans.eq_ignore_ascii_case(&current_q.correct);
                            let explanation_text = if is_correct {
                                Strings::QUIZ_EASY_CORRECT
                                    .replace("{option}", &current_q.correct.to_uppercase())
                            } else {
                                Strings::QUIZ_EASY_WRONG
                                    .replace("{option}", &current_q.correct.to_uppercase())
                            };
                            let exp_color = if is_correct {
                                colors.success
                            } else {
                                colors.danger
                            };

                            el.child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .p_3()
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(exp_color)
                                    .bg(colors.secondary)
                                    .child(
                                        Icon::new(if is_correct {
                                            IconName::Check
                                        } else {
                                            IconName::CircleX
                                        })
                                        .size(px(18.0))
                                        .text_color(exp_color),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(exp_color)
                                            .child(explanation_text),
                                    ),
                            )
                        } else {
                            el.child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .p_2()
                                    .child(
                                        Icon::new(IconName::Info)
                                            .size(px(16.0))
                                            .text_color(colors.muted_foreground),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child(Strings::QUIZ_EASY_HINT),
                                    ),
                            )
                        }
                    }),
            )
            // Bottom Action Navigation Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .border_t_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    .child(match attempt.mode {
                        QuizMode::Bikomeye => {
                            // Hard mode: Single large confirm button
                            div()
                                .flex()
                                .w_full()
                                .child(
                                    Button::new("hard_confirm_btn")
                                        .primary()
                                        .label(Strings::QUIZ_CONFIRM_ANSWER)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_hard_confirm(this, window, cx);
                                        })),
                                )
                        }
                        QuizMode::Hagati => {
                            // Medium mode: Prev, Next, Finish
                            div()
                                .flex()
                                .flex_row()
                                .w_full()
                                .items_center()
                                .justify_between()
                                .child(
                                    Button::new("med_prev_btn")
                                        .outline()
                                        .label(Strings::QUIZ_PREVIOUS)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_prev(this, window, cx);
                                        })),
                                )
                                .child(
                                    Button::new("med_next_btn")
                                        .primary()
                                        .label(Strings::QUIZ_NEXT)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_next(this, window, cx);
                                        })),
                                )
                                .child(
                                    Button::new("med_finish_btn")
                                        .primary()
                                        .label(Strings::QUIZ_FINISH)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_finish_request(this, window, cx);
                                        })),
                                )
                        }
                        _ => {
                            // Easy / Practice mode: Prev, Skip, Next / Finish
                            let is_last = current_idx + 1 == total_questions;
                            div()
                                .flex()
                                .flex_row()
                                .w_full()
                                .items_center()
                                .justify_between()
                                .child(
                                    Button::new("easy_prev_btn")
                                        .outline()
                                        .label(Strings::QUIZ_PREVIOUS)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_prev(this, window, cx);
                                        })),
                                )
                                .child(
                                    Button::new("easy_skip_btn")
                                        .outline()
                                        .label(Strings::QUIZ_SKIP)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_skip(this, window, cx);
                                        })),
                                )
                                .child(
                                    Button::new("easy_next_btn")
                                        .primary()
                                        .label(if is_last {
                                            Strings::QUIZ_FINISH
                                        } else {
                                            Strings::QUIZ_NEXT
                                        })
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            if is_last {
                                                on_finish_request(this, window, cx);
                                            } else {
                                                on_next(this, window, cx);
                                            }
                                        })),
                                )
                        }
                    }),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_option_card<V: 'static>(
        attempt: &Attempt,
        letter: &str,
        text: &str,
        correct_letter: &str,
        user_selection: Option<&str>,
        is_locked: bool,
        cx: &mut Context<V>,
        on_select_option: impl Fn(&mut V, &str, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let is_selected = user_selection
            .map(|s| s.eq_ignore_ascii_case(letter))
            .unwrap_or(false);
        let is_correct_option = letter.eq_ignore_ascii_case(correct_letter);

        // Styling computation based on mode and feedback
        let (border_color, bg_color, text_color, icon_element) =
            if attempt.mode.provides_instant_feedback() {
                if is_locked {
                    if is_selected && is_correct_option {
                        // User chose correct
                        (
                            colors.success,
                            colors.secondary,
                            colors.success,
                            Some(IconName::Check),
                        )
                    } else if is_selected && !is_correct_option {
                        // User chose wrong
                        (
                            colors.danger,
                            colors.secondary,
                            colors.danger,
                            Some(IconName::CircleX),
                        )
                    } else if is_correct_option {
                        // Show what was the right answer
                        (
                            colors.success,
                            colors.secondary,
                            colors.success,
                            Some(IconName::Check),
                        )
                    } else {
                        (
                            colors.border,
                            colors.background,
                            colors.muted_foreground,
                            None,
                        )
                    }
                } else {
                    (
                        colors.border,
                        colors.secondary,
                        colors.foreground,
                        None,
                    )
                }
            } else {
                // Medium or Hard mode
                if is_selected {
                    (
                        colors.primary,
                        colors.sidebar_accent,
                        colors.primary,
                        None,
                    )
                } else {
                    (
                        colors.border,
                        colors.secondary,
                        colors.foreground,
                        None,
                    )
                }
            };

        let letter_owned = letter.to_string();

        div()
            .id(format!("option_card_{letter}"))
            .flex()
            .flex_row()
            .items_center()
            .min_h(px(48.0))
            .p_3()
            .rounded_xl()
            .border_1()
            .border_color(border_color)
            .bg(bg_color)
            .when(!is_locked, |el| el.cursor_pointer().hover(|e| e.bg(colors.sidebar_accent)))
            .on_click(cx.listener(move |this, _, window, cx| {
                if !is_locked {
                    on_select_option(this, &letter_owned, window, cx);
                }
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded_md()
                    .border_1()
                    .border_color(border_color)
                    .bg(if is_selected {
                        colors.primary
                    } else {
                        colors.background
                    })
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(if is_selected {
                                colors.primary_foreground
                            } else {
                                text_color
                            })
                            .child(letter.to_uppercase()),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .px_3()
                    .text_sm()
                    .text_color(text_color)
                    .child(text.to_string()),
            )
            .children(icon_element.map(|icon| {
                Icon::new(icon)
                    .size(px(18.0))
                    .text_color(text_color)
            }))
    }
}
