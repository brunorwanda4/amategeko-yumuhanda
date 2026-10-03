use crate::state::AppState;
use amategeko_core::{Question, Strings};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionsFilter {
    All,
    HasImage,
    Mistakes,
    Starred,
}

pub struct QuestionsView;

impl QuestionsView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        _is_desktop: bool,
        filter: QuestionsFilter,
        search_query: &str,
        hide_answers: bool,
        revealed: &HashMap<u32, String>,
        cx: &mut Context<V>,
        on_set_filter: impl Fn(&mut V, QuestionsFilter, &mut Window, &mut Context<V>) + 'static + Copy,
        on_set_search: impl Fn(&mut V, String, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_hide_answers: impl Fn(&mut V, bool, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_star: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_reveal_option: impl Fn(&mut V, u32, String, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let has_image_only = filter == QuestionsFilter::HasImage;
        let starred_only = filter == QuestionsFilter::Starred;
        let mistakes_only = filter == QuestionsFilter::Mistakes;

        let filtered_questions: Vec<&Question> = state.bank.search(
            search_query,
            has_image_only,
            starred_only,
            mistakes_only,
            &state.progress.starred_questions,
            &state.progress.question_stats,
        );

        let total_questions = state.bank.len();
        let visible_count = filtered_questions.len();
        let display_search = if search_query.is_empty() {
            Strings::QUESTIONS_SEARCH_PLACEHOLDER.to_string()
        } else {
            search_query.to_string()
        };

        div()
            .id("questions_view_root")
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.background)
            // Top Controls Bar (Search + Filter Chips + Switch)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_4()
                    .border_b_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    // Search Bar Row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_2()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.background)
                            .child(
                                Icon::new(IconName::Search)
                                    .size(px(18.0))
                                    .text_color(colors.muted_foreground),
                            )
                            // Search Query Display / Quick Number Search
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(if search_query.is_empty() {
                                        colors.muted_foreground
                                    } else {
                                        colors.foreground
                                    })
                                    .child(display_search),
                            )
                            // Clear search button if active
                            .when(!search_query.is_empty(), |el| {
                                el.child(
                                    div()
                                        .id("clear_search_btn")
                                        .cursor_pointer()
                                        .p_1()
                                        .child(
                                            Icon::new(IconName::CircleX)
                                                .size(px(16.0))
                                                .text_color(colors.muted_foreground),
                                        )
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_set_search(this, String::new(), window, cx);
                                        })),
                                )
                            }),
                    )
                    // Quick Search Shortcut Chips (e.g. Sign Questions, Mistakes, or Number ranges)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap_2()
                            // All chip
                            .child(
                                div()
                                    .id("q_filter_all")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_full()
                                    .cursor_pointer()
                                    .border_1()
                                    .border_color(if filter == QuestionsFilter::All {
                                        colors.primary
                                    } else {
                                        colors.border
                                    })
                                    .bg(if filter == QuestionsFilter::All {
                                        colors.primary
                                    } else {
                                        colors.background
                                    })
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(if filter == QuestionsFilter::All {
                                        colors.primary_foreground
                                    } else {
                                        colors.foreground
                                    })
                                    .child(format!("{} ({})", Strings::QUESTIONS_FILTER_ALL, total_questions))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_set_filter(this, QuestionsFilter::All, window, cx);
                                    })),
                            )
                            // Image questions chip
                            .child(
                                div()
                                    .id("q_filter_image")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_full()
                                    .cursor_pointer()
                                    .border_1()
                                    .border_color(if filter == QuestionsFilter::HasImage {
                                        colors.primary
                                    } else {
                                        colors.border
                                    })
                                    .bg(if filter == QuestionsFilter::HasImage {
                                        colors.primary
                                    } else {
                                        colors.background
                                    })
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(if filter == QuestionsFilter::HasImage {
                                        colors.primary_foreground
                                    } else {
                                        colors.foreground
                                    })
                                    .child(Strings::QUESTIONS_FILTER_IMAGE)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_set_filter(this, QuestionsFilter::HasImage, window, cx);
                                    })),
                            )
                            // Mistakes chip
                            .child(
                                div()
                                    .id("q_filter_mistakes")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_full()
                                    .cursor_pointer()
                                    .border_1()
                                    .border_color(if filter == QuestionsFilter::Mistakes {
                                        colors.danger
                                    } else {
                                        colors.border
                                    })
                                    .bg(if filter == QuestionsFilter::Mistakes {
                                        colors.danger
                                    } else {
                                        colors.background
                                    })
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(if filter == QuestionsFilter::Mistakes {
                                        colors.primary_foreground
                                    } else {
                                        colors.foreground
                                    })
                                    .child(Strings::QUESTIONS_FILTER_MISTAKES)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_set_filter(this, QuestionsFilter::Mistakes, window, cx);
                                    })),
                            )
                            // Starred chip
                            .child(
                                div()
                                    .id("q_filter_starred")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_full()
                                    .cursor_pointer()
                                    .border_1()
                                    .border_color(if filter == QuestionsFilter::Starred {
                                        colors.warning
                                    } else {
                                        colors.border
                                    })
                                    .bg(if filter == QuestionsFilter::Starred {
                                        colors.warning
                                    } else {
                                        colors.background
                                    })
                                    .text_xs()
                                    .font_semibold()
                                    .text_color(if filter == QuestionsFilter::Starred {
                                        colors.primary_foreground
                                    } else {
                                        colors.foreground
                                    })
                                    .child(format!(
                                        "{} ({})",
                                        Strings::QUESTIONS_FILTER_STARRED,
                                        state.progress.starred_questions.len()
                                    ))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_set_filter(this, QuestionsFilter::Starred, window, cx);
                                    })),
                            ),
                    )
                    // Bottom Row of Controls: Count info + Hide Answers Toggle
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .pt_1()
                            // Question count badge
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(colors.muted_foreground)
                                    .child(format!("Ibibazo {} kuri {}", visible_count, total_questions)),
                            )
                            // Hide answers flashcard switch
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(colors.foreground)
                                            .child(Strings::QUESTIONS_HIDE_ANSWERS),
                                    )
                                    .child(
                                        Switch::new("hide_answers_switch")
                                            .checked(hide_answers)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_toggle_hide_answers(this, !hide_answers, window, cx);
                                            })),
                                    ),
                            ),
                    ),
            )
            // Scrollable Question List
            .child(
                div()
                    .id("questions_scroll_view")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .p_4()
                    .gap_4()
                    // Empty state when no questions match
                    .when(filtered_questions.is_empty(), |el| {
                        el.child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .p_8()
                                .gap_3()
                                .child(
                                    Icon::new(IconName::BookOpen)
                                        .size(px(40.0))
                                        .text_color(colors.muted_foreground),
                                )
                                .child(
                                    div()
                                        .text_base()
                                        .font_bold()
                                        .text_color(colors.foreground)
                                        .child("Nta kibazo kigaragaye"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(colors.muted_foreground)
                                        .child("Hindura amashakiro cyangwa kanda 'Byose' kugira ngo ubone ibibazo byose."),
                                )
                                .child(
                                    Button::new("reset_filter_btn")
                                        .secondary()
                                        .label("Kora Reset")
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_set_filter(this, QuestionsFilter::All, window, cx);
                                            on_set_search(this, String::new(), window, cx);
                                        })),
                                ),
                        )
                    })
                    // Render Question Cards
                    .children(filtered_questions.into_iter().map(|q| {
                        let q_id = q.id;
                        let is_starred = state.progress.is_starred(q_id);
                        let wrong_count = state
                            .progress
                            .question_stats
                            .get(&q_id)
                            .map(|s| s.wrong_count)
                            .unwrap_or(0);

                        let revealed_choice = revealed.get(&q_id).cloned();

                        let mut sorted_keys: Vec<&String> = q.options.keys().collect();
                        sorted_keys.sort();

                        div()
                            .id(format!("q_card_{}", q_id))
                            .flex()
                            .flex_col()
                            .p_4()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .gap_3()
                            // Card Header: Question Number + Badges + Star button
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_bold()
                                                    .text_color(colors.primary)
                                                    .child(format!("Ikibazo cya {}", q_id)),
                                            )
                                            .when(wrong_count > 0, |w| {
                                                w.child(
                                                    div()
                                                        .px_2()
                                                        .py_0p5()
                                                        .rounded_full()
                                                        .bg(colors.danger)
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(colors.primary_foreground)
                                                        .child(format!("Byakoswe {}x", wrong_count)),
                                                )
                                            }),
                                    )
                                    // Star Toggle Button
                                    .child(
                                        div()
                                            .id(format!("star_q_{}", q_id))
                                            .p_1p5()
                                            .cursor_pointer()
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_toggle_star(this, q_id, window, cx);
                                            }))
                                            .child(
                                                Icon::new(if is_starred {
                                                    IconName::StarFill
                                                } else {
                                                    IconName::Star
                                                })
                                                .size(px(20.0))
                                                .text_color(if is_starred {
                                                    colors.warning
                                                } else {
                                                    colors.muted_foreground
                                                }),
                                            ),
                                    ),
                            )
                            // Question Body
                            .child(
                                div()
                                    .text_base()
                                    .font_semibold()
                                    .text_color(colors.foreground)
                                    .child(q.text.clone()),
                            )
                            // Image (if present)
                            .when(q.has_image, |el| {
                                el.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .p_3()
                                        .rounded_xl()
                                        .border_1()
                                        .border_color(colors.border)
                                        .bg(colors.background)
                                        .child(
                                            img(format!("assets/images/q{}.png", q_id))
                                                .max_h(px(160.0))
                                                .rounded_lg(),
                                        ),
                                )
                            })
                            // Options List
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .children(sorted_keys.into_iter().map(|key| {
                                        let opt_text = q.options.get(key).cloned().unwrap_or_default();
                                        let is_correct_key = q.correct == *key;

                                        let opt_letter = key.clone();

                                        let (border_c, bg_c, badge, icon_opt) = if !hide_answers {
                                            // Answers are shown: highlight correct answer
                                            if is_correct_key {
                                                (
                                                    colors.success,
                                                    colors.background,
                                                    Some("Igisubizo cy'ukuri".to_string()),
                                                    Some((IconName::Check, colors.success)),
                                                )
                                            } else {
                                                (colors.border, colors.background, None, None)
                                            }
                                        } else {
                                            // Answers are hidden (Flashcard mode): reveal only when user tapped
                                            if let Some(chosen) = &revealed_choice {
                                                if chosen == key {
                                                    if is_correct_key {
                                                        (
                                                            colors.success,
                                                            colors.background,
                                                            Some("Ni byo! Igisubizo cy'ukuri".to_string()),
                                                            Some((IconName::Check, colors.success)),
                                                        )
                                                    } else {
                                                        (
                                                            colors.danger,
                                                            colors.background,
                                                            Some(format!("Siko! Igisubizo cy'ukuri ni {}", q.correct)),
                                                            Some((IconName::CircleX, colors.danger)),
                                                        )
                                                    }
                                                } else if is_correct_key {
                                                    (
                                                        colors.success,
                                                        colors.background,
                                                        Some("Igisubizo cy'ukuri".to_string()),
                                                        Some((IconName::Check, colors.success)),
                                                    )
                                                } else {
                                                    (colors.border, colors.background, None, None)
                                                }
                                            } else {
                                                (colors.border, colors.background, None, None)
                                            }
                                        };

                                        div()
                                            .id(format!("q_{}_opt_{}", q_id, key))
                                            .flex()
                                            .flex_col()
                                            .p_3()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(border_c)
                                            .bg(bg_c)
                                            .cursor_pointer()
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                if hide_answers {
                                                    on_reveal_option(this, q_id, opt_letter.clone(), window, cx);
                                                }
                                            }))
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
                                                            .text_color(if is_correct_key && !hide_answers {
                                                                colors.success
                                                            } else {
                                                                colors.foreground
                                                            })
                                                            .child(format!("{})", key)),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(colors.foreground)
                                                            .flex_1()
                                                            .child(opt_text),
                                                    )
                                                    .when_some(icon_opt, |row, (ic, col)| {
                                                        row.child(
                                                            Icon::new(ic)
                                                                .size(px(16.0))
                                                                .text_color(col),
                                                        )
                                                    }),
                                            )
                                            .when_some(badge, |b, txt| {
                                                b.child(
                                                    div()
                                                        .text_xs()
                                                        .font_semibold()
                                                        .text_color(if is_correct_key {
                                                            colors.success
                                                        } else {
                                                            colors.danger
                                                        })
                                                        .child(txt),
                                                )
                                            })
                                    })),
                            )
                    })),
            )
    }
}
