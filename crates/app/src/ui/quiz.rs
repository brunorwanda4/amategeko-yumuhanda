use crate::state::AppState;
use amategeko_core::{Attempt, QuizMode, QuizTimer, Strings, TimerState};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct QuizView;

impl QuizView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        is_focus_mode: bool,
        cx: &mut Context<V>,
        on_select_option: impl Fn(&mut V, &str, &mut Window, &mut Context<V>) + 'static + Copy,
        on_next: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_prev: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_skip: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_hard_confirm: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_jump_to: impl Fn(&mut V, usize, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_flag: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_star: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_focus: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
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

        let mode_label = attempt.mode.title_kinyarwanda();

        let mode_text_color = match attempt.mode {
            QuizMode::Byoroshye => colors.success,
            QuizMode::Hagati => colors.warning,
            QuizMode::Bikomeye => colors.danger,
            QuizMode::WeakPractice | QuizMode::RetryWrong => colors.primary,
        };

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
        let q_id = current_q.id;

        let is_first = current_idx == 0;
        let is_last = current_idx + 1 == total_questions;
        let is_hard = attempt.mode == QuizMode::Bikomeye;

        let next_btn_label = if is_hard {
            Strings::QUIZ_CONFIRM_ANSWER
        } else if is_last {
            Strings::QUIZ_FINISH
        } else {
            Strings::QUIZ_NEXT
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.background)
            // Top Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px_6()
                    .py_3()
                    .border_b_1()
                    .border_color(colors.border)
                    .bg(colors.background)
                    // Left: Exit Button + Mode Badge + Question Counter
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            // Exit / Sohoka button
                            .child(
                                div()
                                    .id("quiz_back_btn")
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .px_2p5()
                                    .py_1p5()
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    .cursor_pointer()
                                    .hover(|el| el.bg(colors.accent))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_abandon_request(this, window, cx);
                                    }))
                                    .child(
                                        Icon::new(IconName::ArrowLeft)
                                            .size(px(14.0))
                                            .text_color(colors.muted_foreground),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_medium()
                                            .text_color(colors.muted_foreground)
                                            .child(Strings::DIALOG_ABANDON_CONFIRM),
                                    ),
                            )
                            // Mode Pill Badge (e.g. Byoroshye in green)
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded_full()
                                    .bg(colors.secondary)
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(mode_text_color)
                                            .child(mode_label),
                                    ),
                            )
                            // Question Counter: Ikibazo 7 / 20
                            .child(
                                div()
                                    .text_base()
                                    .font_bold()
                                    .text_color(colors.foreground)
                                    .child(
                                        Strings::QUIZ_QUESTION_PROGRESS
                                            .replace("{current}", &(current_idx + 1).to_string())
                                            .replace("{total}", &total_questions.to_string()),
                                    ),
                            ),
                    )
                    // Right: Timer + Flag (medium) + Focus Toggle + Star Button
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2p5()
                            // Timer Pill (if active)
                            .when(
                                matches!(
                                    timer_state,
                                    TimerState::Countdown { .. } | TimerState::Elapsed(_)
                                ),
                                |el| match timer_state {
                                    TimerState::Countdown {
                                        remaining_seconds,
                                        is_urgent,
                                        ..
                                    } => {
                                        let mins = remaining_seconds / 60;
                                        let secs = remaining_seconds % 60;
                                        let t_color = if is_urgent {
                                            colors.danger
                                        } else {
                                            colors.foreground
                                        };
                                        el.child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1p5()
                                                .px_3()
                                                .py_1p5()
                                                .rounded_lg()
                                                .border_1()
                                                .border_color(if is_urgent {
                                                    colors.danger
                                                } else {
                                                    colors.border
                                                })
                                                .bg(colors.secondary)
                                                .child(
                                                    Icon::new(IconName::Bell)
                                                        .size(px(14.0))
                                                        .text_color(t_color),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(t_color)
                                                        .child(format!("{:02}:{:02}", mins, secs)),
                                                ),
                                        )
                                    }
                                    TimerState::Elapsed(elapsed) => {
                                        let mins = elapsed / 60;
                                        let secs = elapsed % 60;
                                        el.child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1p5()
                                                .px_3()
                                                .py_1p5()
                                                .rounded_lg()
                                                .border_1()
                                                .border_color(colors.border)
                                                .bg(colors.secondary)
                                                .child(
                                                    Icon::new(IconName::Bell)
                                                        .size(px(14.0))
                                                        .text_color(colors.foreground),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(colors.foreground)
                                                        .child(format!("{:02}:{:02}", mins, secs)),
                                                ),
                                        )
                                    }
                                    _ => el,
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
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(36.0))
                                        .h(px(36.0))
                                        .rounded_lg()
                                        .border_1()
                                        .border_color(if is_flagged {
                                            colors.warning
                                        } else {
                                            colors.border
                                        })
                                        .bg(colors.secondary)
                                        .cursor_pointer()
                                        .hover(|btn| btn.bg(colors.accent))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_toggle_flag(this, window, cx);
                                        }))
                                        .child(
                                            Icon::new(IconName::TriangleAlert)
                                                .size(px(17.0))
                                                .text_color(flag_color),
                                        ),
                                )
                            })
                            // Focus / Fullscreen mode toggle
                            .child(
                                div()
                                    .id("focus_mode_btn")
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(if is_focus_mode {
                                        colors.primary
                                    } else {
                                        colors.border
                                    })
                                    .bg(if is_focus_mode {
                                        colors.accent
                                    } else {
                                        colors.secondary
                                    })
                                    .cursor_pointer()
                                    .hover(|el| el.bg(colors.accent))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_toggle_focus(this, window, cx);
                                    }))
                                    .child(
                                        Icon::new(if is_focus_mode {
                                            IconName::Minimize
                                        } else {
                                            IconName::Maximize
                                        })
                                        .size(px(16.0))
                                        .text_color(
                                            if is_focus_mode {
                                                colors.primary
                                            } else {
                                                colors.foreground
                                            },
                                        ),
                                    ),
                            )
                            // Star / Bookmark toggle
                            .child(
                                div()
                                    .id("star_btn")
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(if is_starred {
                                        colors.warning
                                    } else {
                                        colors.border
                                    })
                                    .bg(colors.secondary)
                                    .cursor_pointer()
                                    .hover(|el| el.bg(colors.accent))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_toggle_star(this, q_id, window, cx);
                                    }))
                                    .child(
                                        Icon::new(star_icon).size(px(18.0)).text_color(star_color),
                                    ),
                            ),
                    ),
            )
            // Segmented Progress Bar (Matches mockup across all modes!)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .w_full()
                    .px_6()
                    .py_2p5()
                    .gap_1p5()
                    .children((0..total_questions).map(|i| {
                        let is_cur = i == current_idx;
                        let ans = attempt.answers.get(&i);
                        let seg_color = if is_cur {
                            // Current question is a thick bright white highlight
                            colors.foreground
                        } else if let Some(user_a) = ans {
                            if attempt.mode.provides_instant_feedback() {
                                let is_correct =
                                    user_a.eq_ignore_ascii_case(&attempt.questions[i].correct);
                                if is_correct {
                                    colors.success
                                } else {
                                    colors.danger
                                }
                            } else {
                                colors.primary
                            }
                        } else {
                            colors.border
                        };

                        div()
                            .id(format!("progress_seg_{i}"))
                            .flex_1()
                            .h(px(4.0))
                            .rounded_full()
                            .bg(seg_color)
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_jump_to(this, i, window, cx);
                            }))
                    })),
            )
            // Main Question Body (Scrollable, Two Columns on Desktop)
            .child(
                div()
                    .id("quiz_content_scroll")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .px_6()
                    .py_4()
                    .child(
                        div()
                            .w_full()
                            .max_w(px(1120.0))
                            .mx_auto()
                            .flex()
                            .when(is_desktop, |el| el.flex_row().items_start().gap_8())
                            .when(!is_desktop, |el| el.flex_col().items_stretch().gap_6())
                            // LEFT COLUMN: Sign Image Card + Keyboard Shortcut Hint
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .when(is_desktop, |el| el.w(px(420.0)).flex_shrink_0())
                                    .when(!is_desktop, |el| el.w_full())
                                    // Dashed container for sign image
                                    .child(
                                        div()
                                            .w_full()
                                            .min_h(px(320.0))
                                            .h(px(340.0))
                                            .rounded_2xl()
                                            .border_1()
                                            .border_dashed()
                                            .border_color(colors.border)
                                            .bg(colors.secondary)
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .justify_center()
                                            .p_4()
                                            .when(current_q.has_image, |card| {
                                                card.child(
                                                    img(format!(
                                                        "assets/images/q{}.png",
                                                        current_q.id
                                                    ))
                                                    .max_h(px(270.0))
                                                    .max_w(px(380.0))
                                                    .rounded_xl(),
                                                )
                                            })
                                            .when(!current_q.has_image, |card| {
                                                card.gap_3()
                                                    .child(
                                                        svg()
                                                            .path("icons/image.svg")
                                                            .size(px(46.0))
                                                            .text_color(colors.muted_foreground),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_medium()
                                                            .text_color(colors.foreground)
                                                            .child(Strings::QUIZ_NO_IMAGE),
                                                    )
                                            }),
                                    )
                                    // Shortcut navigation indicator
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_2()
                                            .mt_3()
                                            .px_1()
                                            .child(
                                                svg()
                                                    .path("icons/keyboard.svg")
                                                    .size(px(14.0))
                                                    .text_color(colors.muted_foreground),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(colors.muted_foreground)
                                                    .child(Strings::QUIZ_SHORTCUT_HINT),
                                            ),
                                    ),
                            )
                            // RIGHT COLUMN: Question Text, Hint / Feedback, Options, Bottom Actions
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .gap_4()
                                    // Question text
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(current_q.text.clone()),
                                    )
                                    // Subtitle / Feedback
                                    .child({
                                        if attempt.mode.provides_instant_feedback() {
                                            if let Some(ans) = &user_ans {
                                                let is_correct =
                                                    ans.eq_ignore_ascii_case(&current_q.correct);
                                                let exp_color = if is_correct {
                                                    colors.success
                                                } else {
                                                    colors.danger
                                                };
                                                let exp_icon = if is_correct {
                                                    IconName::Check
                                                } else {
                                                    IconName::CircleX
                                                };
                                                let exp_text = if is_correct {
                                                    Strings::QUIZ_EASY_CORRECT.replace(
                                                        "{option}",
                                                        &current_q.correct.to_uppercase(),
                                                    )
                                                } else {
                                                    Strings::QUIZ_EASY_WRONG.replace(
                                                        "{option}",
                                                        &current_q.correct.to_uppercase(),
                                                    )
                                                };
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_2()
                                                    .child(
                                                        Icon::new(exp_icon)
                                                            .size(px(16.0))
                                                            .text_color(exp_color),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_medium()
                                                            .text_color(exp_color)
                                                            .child(exp_text),
                                                    )
                                            } else {
                                                div()
                                                    .text_sm()
                                                    .text_color(colors.muted_foreground)
                                                    .child(Strings::QUIZ_EASY_HINT)
                                            }
                                        } else {
                                            div()
                                                .text_sm()
                                                .text_color(colors.muted_foreground)
                                                .child(Strings::QUIZ_HARD_NO_SELECTION)
                                        }
                                    })
                                    // Options list
                                    .child(div().flex().flex_col().gap_3().children(
                                        current_q.options.iter().map(|(letter, opt_text)| {
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
                                        }),
                                    ))
                                    // Bottom Navigation inside Right Column
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .mt_4()
                                            .pt_2()
                                            // Previous button (Left)
                                            .child(
                                                div()
                                                    .id("quiz_prev_btn")
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_2()
                                                    .px_4()
                                                    .py_2()
                                                    .rounded_xl()
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .bg(colors.secondary)
                                                    .when(!is_first, |el| {
                                                        el.cursor_pointer()
                                                            .hover(|h| h.bg(colors.accent))
                                                            .on_click(cx.listener(
                                                                move |this, _, window, cx| {
                                                                    on_prev(this, window, cx);
                                                                },
                                                            ))
                                                    })
                                                    .when(is_first, |el| el.opacity(0.35))
                                                    .child(
                                                        Icon::new(IconName::ArrowLeft)
                                                            .size(px(16.0))
                                                            .text_color(colors.foreground),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_medium()
                                                            .text_color(colors.foreground)
                                                            .child(Strings::QUIZ_PREVIOUS),
                                                    ),
                                            )
                                            // Right Actions: Simbuka (Skip) & Ibikurikira (Next) / Soza (Finish)
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_3()
                                                    // Skip button
                                                    .when(!is_hard && !is_last, |row| {
                                                        row.child(
                                                            div()
                                                                .id("quiz_skip_btn")
                                                                .flex()
                                                                .flex_row()
                                                                .items_center()
                                                                .px_4()
                                                                .py_2()
                                                                .rounded_xl()
                                                                .border_1()
                                                                .border_color(colors.border)
                                                                .bg(colors.secondary)
                                                                .cursor_pointer()
                                                                .hover(|h| h.bg(colors.accent))
                                                                .on_click(cx.listener(
                                                                    move |this, _, window, cx| {
                                                                        on_skip(this, window, cx);
                                                                    },
                                                                ))
                                                                .child(
                                                                    div()
                                                                        .text_sm()
                                                                        .font_medium()
                                                                        .text_color(
                                                                            colors.foreground,
                                                                        )
                                                                        .child(Strings::QUIZ_SKIP),
                                                                ),
                                                        )
                                                    })
                                                    // Next / Finish / Confirm button
                                                    .child(
                                                        div()
                                                            .id("quiz_next_btn")
                                                            .flex()
                                                            .flex_row()
                                                            .items_center()
                                                            .gap_2()
                                                            .px_4()
                                                            .py_2()
                                                            .rounded_xl()
                                                            .border_1()
                                                            .border_color(colors.border)
                                                            .bg(colors.secondary)
                                                            .cursor_pointer()
                                                            .hover(|h| h.bg(colors.accent))
                                                            .on_click(cx.listener(
                                                                move |this, _, window, cx| {
                                                                    if is_hard {
                                                                        on_hard_confirm(
                                                                            this, window, cx,
                                                                        );
                                                                    } else if is_last {
                                                                        on_finish_request(
                                                                            this, window, cx,
                                                                        );
                                                                    } else {
                                                                        on_next(this, window, cx);
                                                                    }
                                                                },
                                                            ))
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .font_medium()
                                                                    .text_color(colors.foreground)
                                                                    .child(next_btn_label),
                                                            )
                                                            .child(
                                                                Icon::new(IconName::ArrowRight)
                                                                    .size(px(16.0))
                                                                    .text_color(colors.foreground),
                                                            ),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
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

        let (border_color, bg_color, text_color, badge_bg, badge_border, badge_text, icon_element) =
            if attempt.mode.provides_instant_feedback() {
                if is_locked {
                    if is_selected && is_correct_option {
                        (
                            colors.success,
                            colors.secondary,
                            colors.foreground,
                            colors.success,
                            colors.success,
                            colors.primary_foreground,
                            Some((IconName::Check, colors.success)),
                        )
                    } else if is_selected && !is_correct_option {
                        (
                            colors.danger,
                            colors.secondary,
                            colors.foreground,
                            colors.danger,
                            colors.danger,
                            colors.primary_foreground,
                            Some((IconName::CircleX, colors.danger)),
                        )
                    } else if is_correct_option {
                        (
                            colors.success,
                            colors.secondary,
                            colors.foreground,
                            colors.success,
                            colors.success,
                            colors.primary_foreground,
                            Some((IconName::Check, colors.success)),
                        )
                    } else {
                        (
                            colors.border,
                            colors.secondary,
                            colors.muted_foreground,
                            colors.background,
                            colors.border,
                            colors.muted_foreground,
                            None,
                        )
                    }
                } else {
                    (
                        colors.border,
                        colors.secondary,
                        colors.foreground,
                        colors.background,
                        colors.border,
                        colors.muted_foreground,
                        None,
                    )
                }
            } else {
                // Medium or Hard mode
                if is_selected {
                    (
                        colors.primary,
                        colors.secondary,
                        colors.foreground,
                        colors.primary,
                        colors.primary,
                        colors.primary_foreground,
                        None,
                    )
                } else {
                    (
                        colors.border,
                        colors.secondary,
                        colors.foreground,
                        colors.background,
                        colors.border,
                        colors.muted_foreground,
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
            .min_h(px(52.0))
            .px_4()
            .py_3()
            .rounded_xl()
            .border_1()
            .border_color(border_color)
            .bg(bg_color)
            .when(!is_locked, |el| {
                el.cursor_pointer()
                    .hover(|e| e.bg(colors.accent).border_color(colors.primary))
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                if !is_locked {
                    on_select_option(this, &letter_owned, window, cx);
                }
            }))
            // Badge containing letter: A, B, C, D
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded_lg()
                    .border_1()
                    .border_color(badge_border)
                    .bg(badge_bg)
                    .child(
                        div()
                            .text_sm()
                            .font_bold()
                            .text_color(badge_text)
                            .child(letter.to_uppercase()),
                    ),
            )
            // Option text
            .child(
                div()
                    .flex_1()
                    .px_3()
                    .text_sm()
                    .font_medium()
                    .text_color(text_color)
                    .child(text.to_string()),
            )
            // Feedback icon
            .children(
                icon_element.map(|(icon, col)| Icon::new(icon).size(px(18.0)).text_color(col)),
            )
    }
}
