use crate::state::AppState;
use crate::ui::scroll::{vertical_scrollbar, DragScroll};
use amategeko_core::{t, Attempt, Language, QuizMode, QuizTimer, TimerLevel, TimerState};
use gpui::InteractiveElement as _;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::time::Duration;

pub struct QuizView;

impl QuizView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        is_focus_mode: bool,
        scroll_handle: &ScrollHandle,
        reveal_scrollbar: bool,
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
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let lang = state.settings.language;
        let q_lang = state.settings.question_language;
        let show_both = state.settings.show_both_languages;

        let attempt = match &state.current_attempt {
            Some(att) => att,
            None => {
                return Self::render_empty_state(
                    lang,
                    is_desktop,
                    scroll_handle,
                    reveal_scrollbar,
                    cx,
                    on_start_mode,
                );
            }
        };

        let now = state.clock.now_seconds();
        let timer_state = QuizTimer::state(attempt, now, state.settings.easy_show_timer);

        let current_idx = attempt.current_index;
        let total_questions = attempt.total_questions();
        let current_q = match attempt.current_question() {
            Some(q) => q,
            None => return div().child("Question not found"),
        };
        let option_order = attempt.option_order(
            current_idx,
            q_lang,
            state.settings.shuffle_options,
        );
        let correct_badge = option_order
            .iter()
            .position(|option_id| option_id.eq_ignore_ascii_case(&current_q.correct))
            .map(|position| char::from(b'A' + position as u8).to_string())
            .unwrap_or_else(|| current_q.correct.to_uppercase());

        let user_ans = attempt.current_answer().cloned();
        let is_locked = attempt.is_current_locked();
        let is_flagged = attempt.is_current_flagged();
        let is_starred = state.progress.is_starred(current_q.id);

        let mode_label = match attempt.mode {
            QuizMode::Byoroshye => t("mode.easy.title", lang),
            QuizMode::Hagati => t("mode.medium.title", lang),
            QuizMode::Bikomeye => t("mode.hard.title", lang),
            QuizMode::WeakPractice => t("home.weak_title", lang),
            QuizMode::RetryWrong => t("results.retry_wrong", lang),
        };

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
            t("quiz.confirm_answer", lang)
        } else if is_last {
            t("quiz.finish", lang)
        } else {
            t("quiz.next", lang)
        };

        let shortcuts_active =
            crate::shortcuts_ui_active(is_desktop, state.settings.desktop_shortcuts_enabled);

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
                                    .gap_1p5()
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
                                    .when(is_desktop, |el| {
                                        el.child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(colors.muted_foreground)
                                                .child(t("dialog.abandon.confirm", lang)),
                                        )
                                    })
                                    .when(shortcuts_active && !is_hard, |el| {
                                        el.child(
                                            div()
                                                .text_xs()
                                                .font_semibold()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded_md()
                                                .bg(colors.muted.opacity(0.4))
                                                .border_1()
                                                .border_color(colors.border)
                                                .text_color(colors.muted_foreground)
                                                .child("Esc"),
                                        )
                                    }),
                            )
                            // Mode Pill Badge
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
                            // Question Counter
                            .child(
                                div()
                                    .text_base()
                                    .font_bold()
                                    .text_color(colors.foreground)
                                    .child(
                                        if is_desktop {
                                            t("quiz.question_progress", lang)
                                                .replace("{current}", &(current_idx + 1).to_string())
                                                .replace("{total}", &total_questions.to_string())
                                        } else {
                                            format!("{}/{}", current_idx + 1, total_questions)
                                        },
                                    ),
                            ),
                    )
                    // Right: Timer + Flag (medium) + Focus Toggle (desktop OS) + Star Button
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2p5()
                            // Timer Pill (if active)
                            .when(
                                !matches!(timer_state, TimerState::None),
                                |el| match timer_state {
                                    TimerState::Countdown {
                                        remaining_seconds,
                                        level,
                                        ..
                                    } => {
                                        let mins = remaining_seconds / 60;
                                        let secs = remaining_seconds % 60;
                                        let (timer_color, timer_icon) = match level {
                                            TimerLevel::Normal => {
                                                (colors.foreground, IconName::Clock)
                                            }
                                            TimerLevel::Warning => {
                                                (colors.warning, IconName::TriangleAlert)
                                            }
                                            TimerLevel::Error | TimerLevel::Done => {
                                                (colors.danger, IconName::CircleAlert)
                                            }
                                        };
                                        let pill = div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1p5()
                                            .px_3()
                                            .py_1p5()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(if level == TimerLevel::Normal {
                                                colors.border
                                            } else {
                                                timer_color
                                            })
                                            .bg(colors.secondary)
                                            .child(
                                                Icon::new(timer_icon)
                                                    .size(px(14.0))
                                                    .text_color(timer_color),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .when(level == TimerLevel::Error, |text| {
                                                        text.font_bold()
                                                    })
                                                    .when(level != TimerLevel::Error, |text| {
                                                        text.font_medium()
                                                    })
                                                    .text_color(timer_color)
                                                    .child(format!("{mins:02}:{secs:02}")),
                                            );

                                        if level == TimerLevel::Error {
                                            el.child(
                                                pill.with_animation(
                                                    "timer-error-pulse",
                                                    Animation::new(Duration::from_millis(1200))
                                                        .repeat(),
                                                    |pill, progress| {
                                                        let opacity = 0.9
                                                            + 0.1
                                                                * (progress
                                                                    * std::f32::consts::TAU)
                                                                    .cos();
                                                        pill.opacity(opacity)
                                                    },
                                                ),
                                            )
                                        } else {
                                            el.child(pill)
                                        }
                                    }
                                    TimerState::Expired => el.child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1p5()
                                            .px_3()
                                            .py_1p5()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(colors.danger)
                                            .bg(colors.secondary)
                                            .child(
                                                Icon::new(IconName::CircleAlert)
                                                    .size(px(14.0))
                                                    .text_color(colors.danger),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_bold()
                                                    .text_color(colors.danger)
                                                    .child("00:00"),
                                            ),
                                    ),
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
                                                    Icon::new(IconName::Clock)
                                                        .size(px(14.0))
                                                        .text_color(colors.foreground),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(colors.foreground)
                                                        .child(format!("{mins:02}:{secs:02}")),
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
                                let flag_tooltip = if shortcuts_active {
                                    format!("{} (M)", t("quiz.flag", lang))
                                } else {
                                    t("quiz.flag", lang).to_string()
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
                                        .tooltip(move |window, cx| {
                                            Tooltip::new(flag_tooltip.clone()).build(window, cx)
                                        })
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
                            // Focus / Fullscreen: desktop OS only (not Android/iOS APK)
                            .when(!crate::is_native_mobile(), |el| {
                                    let focus_tooltip = if shortcuts_active {
                                        if is_focus_mode {
                                            format!("{} (F)", t("quiz.focus_exit", lang))
                                        } else {
                                            format!("{} (F)", t("quiz.focus_mode", lang))
                                        }
                                    } else if is_focus_mode {
                                        t("quiz.focus_exit", lang).to_string()
                                    } else {
                                        t("quiz.focus_mode", lang).to_string()
                                    };
                                    el.child(
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
                                            .tooltip(move |window, cx| {
                                                Tooltip::new(focus_tooltip.clone())
                                                    .build(window, cx)
                                            })
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
                                                .text_color(if is_focus_mode {
                                                    colors.primary
                                                } else {
                                                    colors.foreground
                                                }),
                                            ),
                                    )
                                },
                            )
                            // Star / Bookmark toggle
                            .child({
                                let star_tooltip = if shortcuts_active && attempt.mode == QuizMode::Byoroshye {
                                    format!("{} (B)", t("quiz.star", lang))
                                } else {
                                    t("quiz.star", lang).to_string()
                                };
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
                                    .tooltip(move |window, cx| {
                                        Tooltip::new(star_tooltip.clone()).build(window, cx)
                                    })
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_toggle_star(this, q_id, window, cx);
                                    }))
                                    .child(
                                        Icon::new(star_icon).size(px(18.0)).text_color(star_color),
                                    )
                            }),
                    )
            )
            // Time Remaining Bar (Medium and Hard)
            .when(
                matches!(timer_state, TimerState::Countdown { .. } | TimerState::Expired),
                |el| {
                    let (remaining, total, level) = match timer_state {
                        TimerState::Countdown {
                            remaining_seconds,
                            total_seconds,
                            level,
                            ..
                        } => (remaining_seconds, total_seconds, level),
                        TimerState::Expired => (0, 1, TimerLevel::Done),
                        _ => (0, 1, TimerLevel::Normal),
                    };
                    let fraction = if total == 0 {
                        0.0
                    } else {
                        (remaining as f32 / total as f32).clamp(0.0, 1.0)
                    };
                    let bar_color = match level {
                        TimerLevel::Normal => colors.foreground,
                        TimerLevel::Warning => colors.warning,
                        TimerLevel::Error | TimerLevel::Done => colors.danger,
                    };

                    el.child(
                        div()
                            .w_full()
                            .h(px(3.0))
                            .bg(colors.secondary)
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(fraction))
                                    .bg(bar_color),
                            ),
                    )
                },
            )
            // Segmented Progress Bar (Easy and Medium modes only)
            .when(
                matches!(attempt.mode, QuizMode::Byoroshye | QuizMode::Hagati),
                |parent| {
                    parent.child(
                        div()
                            .id("quiz_segmented_progress")
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
                                    .debug_selector(move || format!("progress_seg_{i}"))
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
                },
            )
            // Main Question Body (Scrollable, Two Columns on Desktop)
            .child(
                div()
                    .id("quiz_content_scroll")
                    .track_scroll(scroll_handle)
.drag_scroll(scroll_handle)
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .px_6()
                    .pr_7()
                    .py_4()
                    .child(vertical_scrollbar(
                        "quiz_scrollbar",
                        scroll_handle,
                        is_desktop,
                        reveal_scrollbar,
                    ))
                    .child(
                        div()
                            .w_full()
                            .when(current_q.has_image, |el| el.max_w(px(1120.0)))
                                                        .mx_auto()
                            .flex()
                            .when(is_desktop && current_q.has_image, |el| {
                                el.flex_row().items_start().gap_8()
                            })
                            .when(!is_desktop || !current_q.has_image, |el| {
                                el.flex_col().items_stretch().gap_6()
                            })
                            // LEFT COLUMN: Sign Image Card + Keyboard Shortcut Hint (only shown if question has an image)
                            .when(current_q.has_image, |parent| {
                                parent.child(
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
                                                .child(
                                                    img(format!(
                                                        "assets/images/q{}.png",
                                                        current_q.id
                                                    ))
                                                    .max_h(px(270.0))
                                                    .max_w(px(380.0))
                                                    .rounded_xl(),
                                                ),
                                        )
                                )
                            })
                            // RIGHT / MAIN COLUMN: Question Text, Hint / Feedback, Options, Bottom Actions
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .flex_1()
                                    .min_w_0()
                                    .overflow_hidden()
                                    .gap_4()
                                    // Question text + optional draft badge + optional secondary translation
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .w_full()
                                            .min_w_0()
                                            .overflow_hidden()
                                            .gap_1p5()
                                            .when(
                                                current_q.is_draft_translation()
                                                    && q_lang == Language::En,
                                                |b| {
                                                    b.child(
                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .items_center()
                                                            .gap_1()
                                                            .px_2()
                                                            .py_0p5()
                                                            .rounded_md()
                                                            .bg(colors.warning)
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_bold()
                                                                    .text_color(
                                                                        colors.primary_foreground,
                                                                    )
                                                                    .child(t(
                                                                        "badge.unofficial_translation",
                                                                        lang,
                                                                    )),
                                                            ),
                                                    )
                                                },
                                            )
                                            .child(
                                                div()
                                                    .debug_selector(|| "quiz-question-text".into())
                                                    .w_full()
                                                    .min_w_0()
                                                    .overflow_hidden()
                                                    .whitespace_normal()
                                                    .text_xl()
                                                    .font_bold()
                                                    .text_color(colors.foreground)
                                                    .child(current_q.text_for(q_lang).to_string()),
                                            )
                                            .when(show_both, |bilingual| {
                                                if let Some(sec_text) =
                                                    current_q.secondary_text_for(q_lang)
                                                {
                                                    bilingual.child(
                                                        div()
                                                            .w_full()
                                                            .min_w_0()
                                                            .overflow_hidden()
                                                            .whitespace_normal()
                                                            .text_sm()
                                                            .font_normal()
                                                            .text_color(colors.muted_foreground)
                                                            .child(sec_text.to_string()),
                                                    )
                                                } else {
                                                    bilingual
                                                }
                                            }),
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
                                                    t("quiz.easy_correct", lang).replace(
                                                        "{option}",
                                                        &correct_badge,
                                                    )
                                                } else {
                                                    t("quiz.easy_wrong", lang).replace(
                                                        "{option}",
                                                        &correct_badge,
                                                    )
                                                };
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_start()
                                                    .w_full()
                                                    .min_w_0()
                                                    .overflow_hidden()
                                                    .gap_2()
                                                    .child(
                                                        Icon::new(exp_icon)
                                                            .size(px(16.0))
                                                            .flex_none()
                                                            .text_color(exp_color),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .min_w_0()
                                                            .overflow_hidden()
                                                            .whitespace_normal()
                                                            .text_sm()
                                                            .font_medium()
                                                            .text_color(exp_color)
                                                            .child(exp_text),
                                                    )
                                            } else {
                                                div()
                                                    .w_full()
                                                    .min_w_0()
                                                    .overflow_hidden()
                                                    .whitespace_normal()
                                                    .text_sm()
                                                    .text_color(colors.muted_foreground)
                                                    .child(t("quiz.easy_hint", lang))
                                            }
                                        } else {
                                            div()
                                                .w_full()
                                                .min_w_0()
                                                .overflow_hidden()
                                                .whitespace_normal()
                                                .text_sm()
                                                .text_color(colors.muted_foreground)
                                                .child(t("quiz.hard_no_selection", lang))
                                        }
                                    })
                                    // Options list
                                    .child(div().flex().flex_col().w_full().min_w_0().overflow_hidden().gap_3().children({
                                        let opts = current_q.options_for(q_lang);
                                        option_order.into_iter().enumerate().map(|(position, option_id)| {
                                            let option_id = option_id.as_str();
                                            // Badge shows the screen position, so the rows always read A, B, C, D.
                                            let badge = char::from(b'A' + position as u8).to_string();
                                            let opt_text =
                                                opts.get(option_id).cloned().unwrap_or_default();
                                            let sec_opt = if show_both {
                                                current_q
                                                    .secondary_option_for(q_lang, option_id)
                                                    .map(|s| s.to_string())
                                            } else {
                                                None
                                            };
                                            Self::render_option_card(
                                                attempt,
                                                option_id,
                                                &badge,
                                                &opt_text,
                                                sec_opt.as_deref(),
                                                &current_q.correct,
                                                user_ans.as_deref(),
                                                is_locked,
                                                lang,
                                                shortcuts_active,
                                                cx,
                                                on_select_option,
                                            )
                                        })
                                    }))
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
                                            .child({
                                                let prev_tooltip = if shortcuts_active && !is_hard {
                                                    format!("{} (←)", t("quiz.previous", lang))
                                                } else {
                                                    t("quiz.previous", lang).to_string()
                                                };
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
                                                    .tooltip(move |window, cx| {
                                                        Tooltip::new(prev_tooltip.clone()).build(window, cx)
                                                    })
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
                                                    .when(is_desktop, |el| {
                                                        el.child(
                                                            div()
                                                                .text_sm()
                                                                .font_medium()
                                                                .text_color(colors.foreground)
                                                                .child(t("quiz.previous", lang)),
                                                        )
                                                    })
                                            })
                                            // Right Actions: Simbuka (Skip) & Ibikurikira (Next) / Soza (Finish)
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_3()
                                                    // Skip button
                                                    .when(!is_hard && !is_last, |row| {
                                                        let skip_tooltip = if shortcuts_active
                                                            && attempt.mode == QuizMode::Byoroshye
                                                        {
                                                            format!("{} (S)", t("quiz.skip", lang))
                                                        } else {
                                                            t("quiz.skip", lang).to_string()
                                                        };
                                                        row.child(
                                                            div()
                                                                .id("quiz_skip_btn")
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
                                                                .tooltip(move |window, cx| {
                                                                    Tooltip::new(skip_tooltip.clone()).build(window, cx)
                                                                })
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
                                                                        .child(t(
                                                                            "quiz.skip",
                                                                            lang,
                                                                        )),
                                                                ),
                                                        )
                                                    })
                                                    // Next / Finish / Confirm button
                                                    .child({
                                                        let next_shortcut_hint = if is_last
                                                            && attempt.mode == QuizMode::Hagati
                                                        {
                                                            if cfg!(target_os = "macos") {
                                                                "Cmd+Enter"
                                                            } else {
                                                                "Ctrl+Enter"
                                                            }
                                                        } else {
                                                            "Enter"
                                                        };
                                                        let next_tooltip = if shortcuts_active {
                                                            format!("{next_btn_label} ({next_shortcut_hint})")
                                                        } else {
                                                            next_btn_label.to_string()
                                                        };
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
                                                            .tooltip(move |window, cx| {
                                                                Tooltip::new(next_tooltip.clone()).build(window, cx)
                                                            })
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
                                                            .when(is_desktop, |el| {
                                                                el.child(
                                                                    Icon::new(IconName::ArrowRight)
                                                                        .size(px(16.0))
                                                                        .text_color(colors.foreground),
                                                                )
                                                            })
                                                    }),
                                            ),
                                    )
                            ),
                    ),
            )
    }

    fn render_empty_state<V: 'static>(
        lang: Language,
        is_desktop: bool,
        scroll_handle: &ScrollHandle,
        reveal_scrollbar: bool,
        cx: &mut Context<V>,
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> gpui::Div {
        let theme = cx.theme();
        let colors = theme.colors;

        div().flex().flex_col().size_full().child(
            div()
                .id("quiz_empty_state_scroll")
                .track_scroll(scroll_handle)
                .drag_scroll(scroll_handle)
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .overflow_y_scroll()
                .bg(colors.background)
                .p_4()
                .when(is_desktop, |el| el.p_8())
                .child(vertical_scrollbar(
                    "quiz_empty_scrollbar",
                    scroll_handle,
                    is_desktop,
                    reveal_scrollbar,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .max_w(px(720.0))
                        .gap_6()
                        // Top Icon Badge
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .size(px(72.0))
                                .rounded_2xl()
                                .border_1()
                                .border_color(colors.border)
                                .bg(colors.secondary)
                                .child(
                                    Icon::new(IconName::BookOpen)
                                        .size(px(34.0))
                                        .text_color(colors.primary),
                                ),
                        )
                        // Title & Description
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .text_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .text_2xl()
                                        .font_bold()
                                        .text_color(colors.foreground)
                                        .child(t("quiz.no_active", lang)),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(colors.muted_foreground)
                                        .max_w(px(420.0))
                                        .child(t("quiz.no_active_desc", lang)),
                                ),
                        )
                        // Mode Cards: Easy, Medium, Hard
                        .child(
                            div()
                                .flex()
                                .w_full()
                                .gap_4()
                                .when(is_desktop, |el| el.flex_row())
                                .when(!is_desktop, |el| el.flex_col())
                                .child(Self::render_empty_mode_card(
                                    "empty_quiz_btn_easy",
                                    QuizMode::Byoroshye,
                                    t("mode.easy.title", lang),
                                    t("mode.easy.badge", lang),
                                    t("mode.easy.desc", lang),
                                    IconName::GraduationCap,
                                    gpui::rgb(0x22c55e),
                                    gpui::rgba(0x22c55e1a),
                                    is_desktop,
                                    t("home.start", lang),
                                    cx,
                                    on_start_mode,
                                ))
                                .child(Self::render_empty_mode_card(
                                    "empty_quiz_btn_medium",
                                    QuizMode::Hagati,
                                    t("mode.medium.title", lang),
                                    t("mode.medium.badge", lang),
                                    t("mode.medium.desc", lang),
                                    IconName::Clock,
                                    gpui::rgb(0xf59e0b),
                                    gpui::rgba(0xf59e0b1a),
                                    is_desktop,
                                    t("home.start", lang),
                                    cx,
                                    on_start_mode,
                                ))
                                .child(Self::render_empty_mode_card(
                                    "empty_quiz_btn_hard",
                                    QuizMode::Bikomeye,
                                    t("mode.hard.title", lang),
                                    t("mode.hard.badge", lang),
                                    t("mode.hard.desc", lang),
                                    IconName::Flame,
                                    gpui::rgb(0xef4444),
                                    gpui::rgba(0xef44441a),
                                    is_desktop,
                                    t("home.start", lang),
                                    cx,
                                    on_start_mode,
                                )),
                        ),
                ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_empty_mode_card<V: 'static>(
        id: &'static str,
        mode: QuizMode,
        title: &'static str,
        badge: &'static str,
        desc: &'static str,
        icon: IconName,
        icon_color: impl Into<gpui::Hsla>,
        badge_bg: impl Into<gpui::Hsla>,
        is_desktop: bool,
        start_label: &'static str,
        cx: &mut Context<V>,
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let icon_color: gpui::Hsla = icon_color.into();
        let badge_bg: gpui::Hsla = badge_bg.into();

        div()
            .id(id)
            .debug_selector(move || id.into())
            .cursor_pointer()
            .flex()
            .when(is_desktop, |el| {
                el.flex_col()
                    .items_start()
                    .justify_between()
                    .p_5()
                    .flex_1()
                    .min_w(px(200.0))
            })
            .when(!is_desktop, |el| {
                el.flex_row()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .w_full()
            })
            .rounded_xl()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .hover(move |el| el.bg(colors.muted).border_color(icon_color))
            .on_click(cx.listener(move |this, _, window, cx| {
                on_start_mode(this, mode, window, cx);
            }))
            // Main content
            .child(
                div()
                    .flex()
                    .when(is_desktop, |el| el.flex_col().items_start().w_full())
                    .when(!is_desktop, |el| el.flex_row().items_center().gap_3())
                    // Header row: Icon + Title & badge
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .size(px(40.0))
                                    .rounded_xl()
                                    .bg(badge_bg)
                                    .child(Icon::new(icon).size(px(20.0)).text_color(icon_color)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child(badge),
                                    ),
                            ),
                    )
                    // Description (on desktop only)
                    .when(is_desktop, |el| {
                        el.child(
                            div()
                                .mt_3()
                                .text_xs()
                                .text_color(colors.muted_foreground)
                                .min_h(px(38.0))
                                .child(desc),
                        )
                    }),
            )
            // Action button pill
            .child(
                div().when(is_desktop, |el| el.mt_4().w_full()).child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_center()
                        .gap_1p5()
                        .px_3p5()
                        .py_2()
                        .rounded_lg()
                        .bg(colors.primary)
                        .child(
                            Icon::new(IconName::Play)
                                .size(px(14.0))
                                .text_color(colors.primary_foreground),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_semibold()
                                .text_color(colors.primary_foreground)
                                .child(start_label),
                        ),
                ),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_option_card<V: 'static>(
        attempt: &Attempt,
        option_id: &str,
        badge_label: &str,
        text: &str,
        secondary_text: Option<&str>,
        correct_option_id: &str,
        user_selection: Option<&str>,
        is_locked: bool,
        lang: Language,
        shortcuts_active: bool,
        cx: &mut Context<V>,
        on_select_option: impl Fn(&mut V, &str, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let is_selected = user_selection
            .map(|s| s.eq_ignore_ascii_case(option_id))
            .unwrap_or(false);
        let is_correct_option = option_id.eq_ignore_ascii_case(correct_option_id);

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

        let option_id_owned = option_id.to_string();

        div()
            .id(format!("option_card_{option_id}"))
            .debug_selector(|| format!("quiz-option-row-{option_id}"))
            .flex()
            .flex_row()
            .items_start()
            .w_full()
            .min_w_0()
            .overflow_hidden()
            .gap_3()
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
            .when(shortcuts_active && !is_locked, |el| {
                let hint = badge_label.to_string();
                let tooltip = format!("{}: {hint}", t("shortcuts.choose_option", lang));
                el.tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                if !is_locked {
                    on_select_option(this, &option_id_owned, window, cx);
                }
            }))
            // Badge containing letter: A, B, C, D
            .child(
                div()
                    .debug_selector(|| format!("quiz-option-badge-{option_id}"))
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(32.0))
                    .flex_none()
                    .rounded_lg()
                    .border_1()
                    .border_color(badge_border)
                    .bg(badge_bg)
                    .child(
                        div()
                            .text_sm()
                            .font_bold()
                            .text_color(badge_text)
                            .child(badge_label.to_string()),
                    ),
            )
            // Option text (primary + optional secondary)
            .child(
                div()
                    .debug_selector(|| format!("quiz-option-text-{option_id}"))
                    .flex_1()
                    .flex_col()
                    .min_w_0()
                    .overflow_hidden()
                    .gap_0p5()
                    .child(
                        div()
                            .w_full()
                            .min_w_0()
                            .overflow_hidden()
                            .whitespace_normal()
                            .text_sm()
                            .font_medium()
                            .text_color(text_color)
                            .child(text.to_string()),
                    )
                    .when_some(secondary_text, |el, sec| {
                        el.child(
                            div()
                                .w_full()
                                .min_w_0()
                                .overflow_hidden()
                                .whitespace_normal()
                                .text_xs()
                                .text_color(colors.muted_foreground)
                                .child(sec.to_string()),
                        )
                    }),
            )
            // Feedback icon
            .children(
                icon_element
                    .map(|(icon, col)| Icon::new(icon).size(px(18.0)).flex_none().text_color(col)),
            )
    }
}
