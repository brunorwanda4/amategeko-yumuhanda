use crate::state::AppState;
use crate::ui::scroll::vertical_scrollbar;
use amategeko_core::{t, Language, QuestionResult, QuizMode};
use gpui::InteractiveElement as _;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultFilter {
    All,
    Mistakes,
    Unanswered,
    Correct,
    Wrong,
}

pub struct ResultsView;

impl ResultsView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        scroll_handle: &ScrollHandle,
        reveal_scrollbar: bool,
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

        let lang = state.settings.language;
        let q_lang = state.settings.question_language;
        let show_both = state.settings.show_both_languages;

        let result = match &state.last_result {
            Some(res) => res,
            None => {
                return div()
                    .id("results_empty_scroll")
                    .track_scroll(scroll_handle)
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .p_6()
                    .gap_4()
                    .child(vertical_scrollbar(
                        "results_empty_scrollbar",
                        scroll_handle,
                        is_desktop,
                        reveal_scrollbar,
                    ))
                    .child(
                        div()
                            .text_lg()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child(if lang == Language::En {
                                "No quiz results available."
                            } else {
                                "Nta bisubizo by'ikizamini bihari."
                            }),
                    )
                    .child(div().text_sm().text_color(colors.muted_foreground).child(
                        if lang == Language::En {
                            "Start a new quiz to view your results here."
                        } else {
                            "Tangira ikizamini gishya kugira ngo ubone ibisubizo byawe hano."
                        },
                    ))
                    .child(
                        Button::new("home_empty_btn")
                            .primary()
                            .label(t("nav.home", lang))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_home(this, window, cx);
                            })),
                    );
            }
        };

        let score = result.score;
        let total = result.total;
        let is_passed = result.passed;
        let percentage = (score * 100).checked_div(total).unwrap_or(0);

        let mode_label = match result.mode {
            QuizMode::Byoroshye => t("mode.easy.title", lang),
            QuizMode::Hagati => t("mode.medium.title", lang),
            QuizMode::Bikomeye => t("mode.hard.title", lang),
            QuizMode::WeakPractice => t("home.weak_title", lang),
            QuizMode::RetryWrong => t("results.retry_wrong", lang),
        };

        let mins = result.duration_seconds / 60;
        let secs = result.duration_seconds % 60;
        let duration_str = format!("{:02}:{:02}", mins, secs);

        let correct_count = result
            .question_results
            .iter()
            .filter(|qr| qr.is_correct)
            .count();
        let unanswered_count = result
            .question_results
            .iter()
            .filter(|qr| qr.user_answer.is_none())
            .count();
        let wrong_answered_count = result
            .question_results
            .iter()
            .filter(|qr| !qr.is_correct && qr.user_answer.is_some())
            .count();
        let total_mistakes = (total as usize).saturating_sub(score as usize);
        let shortcuts_active =
            crate::shortcuts_ui_active(is_desktop, state.settings.desktop_shortcuts_enabled);

        // Filter question results
        let filtered_questions: Vec<(usize, &QuestionResult)> = result
            .question_results
            .iter()
            .enumerate()
            .filter(|(_, qr)| match filter {
                ResultFilter::All => true,
                ResultFilter::Mistakes | ResultFilter::Wrong => {
                    !qr.is_correct && qr.user_answer.is_some()
                }
                ResultFilter::Unanswered => qr.user_answer.is_none(),
                ResultFilter::Correct => qr.is_correct,
            })
            .collect();

        div()
            .id("results_view_root")
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.background)
            .p_6()
            .gap_4()
            // 1. Page Heading
            .child(
                div()
                    .flex_none()
                    .text_2xl()
                    .font_bold()
                    .text_color(colors.foreground)
                    .child(if lang == Language::En {
                        "Results"
                    } else {
                        "Ibisubizo"
                    }),
            )
            // 2. Score Summary Card
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .items_center()
                    .p_6()
                    .rounded_2xl()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    .gap_6()
                    // Circular score gauge
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .size(px(88.0))
                            .rounded_full()
                            .border_4()
                            .border_color(if is_passed {
                                colors.success
                            } else {
                                colors.danger
                            })
                            .bg(colors.background)
                            .child(
                                div()
                                    .text_2xl()
                                    .font_extrabold()
                                    .text_color(colors.foreground)
                                    .child(format!("{score}")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(colors.muted_foreground)
                                    .child(format!("/ {total}")),
                            ),
                    )
                    // Score metadata info
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            // Score percentage + Passed / Failed pill
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_3xl()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(format!("{percentage}%")),
                                    )
                                    .child(
                                        div()
                                            .px_2p5()
                                            .py_0p5()
                                            .rounded_full()
                                            .bg(if is_passed {
                                                colors.success.opacity(0.15)
                                            } else {
                                                colors.danger.opacity(0.15)
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_semibold()
                                                    .text_color(if is_passed {
                                                        colors.success
                                                    } else {
                                                        colors.danger
                                                    })
                                                    .child(if is_passed {
                                                        if lang == Language::En {
                                                            "Passed"
                                                        } else {
                                                            "Watsinze"
                                                        }
                                                    } else if lang == Language::En {
                                                        "Failed"
                                                    } else {
                                                        "Ntiwatsinze"
                                                    }),
                                            ),
                                    ),
                            )
                            // Subtitle: Mode · Time used: 16:48
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.muted_foreground)
                                    .child(if lang == Language::En {
                                        format!("{mode_label} · Time used: {duration_str}")
                                    } else {
                                        format!("{mode_label} · Igihe cyakoreshejwe: {duration_str}")
                                    }),
                            )
                            // Badges row: Correct, Wrong, Unanswered
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .pt_1()
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_full()
                                            .bg(colors.success.opacity(0.12))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_medium()
                                                    .text_color(colors.success)
                                                    .child(if lang == Language::En {
                                                        format!("Correct {correct_count}")
                                                    } else {
                                                        format!("Iby'ukuri {correct_count}")
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_full()
                                            .bg(colors.danger.opacity(0.12))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_medium()
                                                    .text_color(colors.danger)
                                                    .child(if lang == Language::En {
                                                        format!("Wrong {wrong_answered_count}")
                                                    } else {
                                                        format!("Ibyakosheje {wrong_answered_count}")
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_full()
                                            .bg(colors.muted_foreground.opacity(0.12))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_medium()
                                                    .text_color(colors.muted_foreground)
                                                    .child(if lang == Language::En {
                                                        format!("Unanswered {unanswered_count}")
                                                    } else {
                                                        format!("Ibitashubijwe {unanswered_count}")
                                                    }),
                                            ),
                                    ),
                            ),
                    ),
            )
            // 3. Filter Pills Row
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    // All 20
                    .child({
                        let is_active = filter == ResultFilter::All;
                        let hint = format!(
                            "{} (1/A)",
                            if lang == Language::En {
                                "All"
                            } else {
                                "Byose"
                            }
                        );
                        div()
                            .id("filter_all_chip")
                            .cursor_pointer()
                            .px_4()
                            .py_2()
                            .rounded_xl()
                            .border_1()
                            .border_color(if is_active {
                                colors.border
                            } else {
                                colors.border.opacity(0.3)
                            })
                            .bg(if is_active {
                                colors.secondary
                            } else {
                                colors.background
                            })
                            .hover(|h| h.bg(colors.accent.opacity(0.3)))
                            .when(shortcuts_active, |el| {
                                el.tooltip(move |window, cx| {
                                    Tooltip::new(hint.clone()).build(window, cx)
                                })
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_set_filter(this, ResultFilter::All, window, cx);
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(if is_active {
                                        colors.foreground
                                    } else {
                                        colors.muted_foreground
                                    })
                                    .child(format!(
                                        "{} {}",
                                        if lang == Language::En {
                                            "All"
                                        } else {
                                            "Byose"
                                        },
                                        total
                                    )),
                            )
                    })
                    // Mistakes 5
                    .child({
                        let is_active =
                            filter == ResultFilter::Mistakes || filter == ResultFilter::Wrong;
                        let hint = format!(
                            "{} (2/X)",
                            if lang == Language::En {
                                "Mistakes"
                            } else {
                                "Amakosa"
                            }
                        );
                        div()
                            .id("filter_wrong_chip")
                            .cursor_pointer()
                            .px_4()
                            .py_2()
                            .rounded_xl()
                            .border_1()
                            .border_color(if is_active {
                                colors.border
                            } else {
                                colors.border.opacity(0.3)
                            })
                            .bg(if is_active {
                                colors.secondary
                            } else {
                                colors.background
                            })
                            .hover(|h| h.bg(colors.accent.opacity(0.3)))
                            .when(shortcuts_active, |el| {
                                el.tooltip(move |window, cx| {
                                    Tooltip::new(hint.clone()).build(window, cx)
                                })
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_set_filter(this, ResultFilter::Mistakes, window, cx);
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(if is_active {
                                        colors.foreground
                                    } else {
                                        colors.muted_foreground
                                    })
                                    .child(format!(
                                        "{} {}",
                                        if lang == Language::En {
                                            "Mistakes"
                                        } else {
                                            "Amakosa"
                                        },
                                        wrong_answered_count
                                    )),
                            )
                    })
                    // Unanswered 1
                    .child({
                        let is_active = filter == ResultFilter::Unanswered;
                        let hint = format!(
                            "{} (3)",
                            if lang == Language::En {
                                "Unanswered"
                            } else {
                                "Ibitashubijwe"
                            }
                        );
                        div()
                            .id("filter_unanswered_chip")
                            .cursor_pointer()
                            .px_4()
                            .py_2()
                            .rounded_xl()
                            .border_1()
                            .border_color(if is_active {
                                colors.border
                            } else {
                                colors.border.opacity(0.3)
                            })
                            .bg(if is_active {
                                colors.secondary
                            } else {
                                colors.background
                            })
                            .hover(|h| h.bg(colors.accent.opacity(0.3)))
                            .when(shortcuts_active, |el| {
                                el.tooltip(move |window, cx| {
                                    Tooltip::new(hint.clone()).build(window, cx)
                                })
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_set_filter(this, ResultFilter::Unanswered, window, cx);
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(if is_active {
                                        colors.foreground
                                    } else {
                                        colors.muted_foreground
                                    })
                                    .child(format!(
                                        "{} {}",
                                        if lang == Language::En {
                                            "Unanswered"
                                        } else {
                                            "Ibitashubijwe"
                                        },
                                        unanswered_count
                                    )),
                            )
                    }),
            )
            // 4. Question Review List Container - SCROLLABLE with its own vertical scrollbar
            .child(
                div()
                    .id("results_scroll_view")
                    .track_scroll(scroll_handle)
                    .relative()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h(px(180.0))
                    .overflow_y_scroll()
                    .rounded_2xl()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    .child(vertical_scrollbar(
                        "results_scrollbar",
                        scroll_handle,
                        is_desktop,
                        reveal_scrollbar,
                    ))
                    .when(filtered_questions.is_empty(), |container| {
                        container.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .p_8()
                                .text_sm()
                                .text_color(colors.muted_foreground)
                                .child(if lang == Language::En {
                                    "No questions match this filter."
                                } else {
                                    "Nta bibazo bihuye n'iri yungurura."
                                }),
                        )
                    })
                    .children(filtered_questions.into_iter().enumerate().map(
                        |(list_idx, (orig_idx, qr))| {
                            let is_expanded = expanded.contains(&orig_idx);
                            let q = &qr.question;
                            let is_correct = qr.is_correct;
                            let user_ans = qr.user_answer.as_deref();
                            let correct_ans = &qr.correct_answer;

                            let opts = q.options_for(q_lang);
                            let mut sorted_keys: Vec<&String> = opts.keys().collect();
                            sorted_keys.sort();

                            div()
                                .id(format!("result_card_{orig_idx}"))
                                .flex()
                                .flex_col()
                                .w_full()
                                .when(list_idx > 0, |el| {
                                    el.border_t_1().border_color(colors.border)
                                })
                                // Card Header (Click to toggle expand)
                                .child(
                                    div()
                                        .id(format!("result_header_{orig_idx}"))
                                        .flex()
                                        .flex_row()
                                        .items_start()
                                        .justify_between()
                                        .p_4()
                                        .gap_4()
                                        .cursor_pointer()
                                        .hover(|h| h.bg(colors.accent.opacity(0.15)))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_toggle_expand(this, orig_idx, window, cx);
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_start()
                                                .gap_3()
                                                .flex_1()
                                                .min_w_0()
                                                // Question Index Number (e.g. "1.")
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_medium()
                                                        .text_color(colors.muted_foreground)
                                                        .child(format!("{}.", orig_idx + 1)),
                                                )
                                                // Main details column
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .gap_2()
                                                        .flex_1()
                                                        .min_w_0()
                                                        // Question Text
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_medium()
                                                                .text_color(colors.foreground)
                                                                .child(
                                                                    q.text_for(q_lang).to_string(),
                                                                ),
                                                        )
                                                        // Badges Row
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_row()
                                                                .flex_wrap()
                                                                .items_center()
                                                                .gap_2()
                                                                // User answer badge
                                                                .when_some(user_ans, |row, ans| {
                                                                    row.child(
                                                                        div()
                                                                            .px_2p5()
                                                                            .py_0p5()
                                                                            .rounded_md()
                                                                            .bg(if is_correct {
                                                                                colors.success.opacity(0.15)
                                                                            } else {
                                                                                colors.danger.opacity(0.15)
                                                                            })
                                                                            .child(
                                                                                div()
                                                                                    .text_xs()
                                                                                    .font_medium()
                                                                                    .text_color(
                                                                                        if is_correct
                                                                                        {
                                                                                            colors.success
                                                                                        } else {
                                                                                            colors.danger
                                                                                        },
                                                                                    )
                                                                                    .child(if lang == Language::En {
                                                                                        format!("Your answer: {ans}")
                                                                                    } else {
                                                                                        format!("Igisubizo cyawe: {ans}")
                                                                                    }),
                                                                            ),
                                                                    )
                                                                })
                                                                // Unanswered badge
                                                                .when(user_ans.is_none(), |row| {
                                                                    row.child(
                                                                        div()
                                                                            .px_2p5()
                                                                            .py_0p5()
                                                                            .rounded_md()
                                                                            .bg(colors.muted_foreground.opacity(0.15))
                                                                            .child(
                                                                                div()
                                                                                    .text_xs()
                                                                                    .font_medium()
                                                                                    .text_color(
                                                                                        colors
                                                                                            .muted_foreground,
                                                                                    )
                                                                                    .child(if lang == Language::En {
                                                                                        "Unanswered"
                                                                                    } else {
                                                                                        "Ntiwasubije"
                                                                                    }),
                                                                            ),
                                                                    )
                                                                })
                                                                // Correct answer badge (when mistake/unanswered)
                                                                .when(!is_correct, |row| {
                                                                    row.child(
                                                                        div()
                                                                            .px_2p5()
                                                                            .py_0p5()
                                                                            .rounded_md()
                                                                            .bg(colors.success.opacity(0.15))
                                                                            .child(
                                                                                div()
                                                                                    .text_xs()
                                                                                    .font_medium()
                                                                                    .text_color(
                                                                                        colors.success,
                                                                                    )
                                                                                    .child(if lang == Language::En {
                                                                                        format!("Correct: {correct_ans}")
                                                                                    } else {
                                                                                        format!("Icy'ukuri: {correct_ans}")
                                                                                    }),
                                                                            ),
                                                                    )
                                                                })
                                                                // Image badge
                                                                .when(q.has_image, |row| {
                                                                    row.child(
                                                                        div()
                                                                            .px_2p5()
                                                                            .py_0p5()
                                                                            .rounded_md()
                                                                            .bg(colors.primary.opacity(0.15))
                                                                            .child(
                                                                                div()
                                                                                    .text_xs()
                                                                                    .font_medium()
                                                                                    .text_color(
                                                                                        colors.primary,
                                                                                    )
                                                                                    .child(if lang == Language::En {
                                                                                        "Image"
                                                                                    } else {
                                                                                        "Ishusho"
                                                                                    }),
                                                                            ),
                                                                    )
                                                                }),
                                                        ),
                                                ),
                                        )
                                        // Right: Status circle icon + chevron
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_3()
                                                .flex_none()
                                                .child(if is_correct {
                                                    Icon::new(IconName::CircleCheck)
                                                        .size(px(20.0))
                                                        .text_color(colors.success)
                                                } else if user_ans.is_some() {
                                                    Icon::new(IconName::CircleX)
                                                        .size(px(20.0))
                                                        .text_color(colors.danger)
                                                } else {
                                                    Icon::new(IconName::CircleAlert)
                                                        .size(px(20.0))
                                                        .text_color(colors.muted_foreground)
                                                })
                                                .child(
                                                    Icon::new(if is_expanded {
                                                        IconName::ChevronUp
                                                    } else {
                                                        IconName::ChevronDown
                                                    })
                                                    .size(px(18.0))
                                                    .text_color(colors.muted_foreground),
                                                ),
                                        ),
                                )
                                // Expanded content
                                .when(is_expanded, |body| {
                                    body.child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .px_4()
                                            .pb_4()
                                            .gap_3()
                                            // Sign image if available
                                            .when(q.has_image, |img_el| {
                                                img_el.child(
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
                                                            img(format!(
                                                                "assets/images/q{}.png",
                                                                q.id
                                                            ))
                                                            .max_h(px(160.0))
                                                            .rounded_lg(),
                                                        ),
                                                )
                                            })
                                            // Secondary translation if enabled
                                            .when(show_both, |bilingual| {
                                                if let Some(sec_text) =
                                                    q.secondary_text_for(q_lang)
                                                {
                                                    bilingual.child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(colors.muted_foreground)
                                                            .child(sec_text.to_string()),
                                                    )
                                                } else {
                                                    bilingual
                                                }
                                            })
                                            // Options list
                                            .child(
                                                div().flex().flex_col().gap_2().children(
                                                    sorted_keys.into_iter().map(|key| {
                                                        let opt_text = opts
                                                            .get(key)
                                                            .cloned()
                                                            .unwrap_or_default();
                                                        let is_user_choice =
                                                            user_ans == Some(key.as_str());
                                                        let is_correct_choice = correct_ans == key;

                                                        let (
                                                            opt_bg,
                                                            opt_border,
                                                            text_color,
                                                            right_icon,
                                                        ) = if is_user_choice && !is_correct_choice
                                                        {
                                                            (
                                                                colors.danger.opacity(0.12),
                                                                colors.danger,
                                                                colors.danger,
                                                                Some(
                                                                    Icon::new(IconName::CircleX)
                                                                        .size(px(18.0))
                                                                        .text_color(colors.danger),
                                                                ),
                                                            )
                                                        } else if is_correct_choice {
                                                            (
                                                                colors.success.opacity(0.12),
                                                                colors.success,
                                                                colors.success,
                                                                Some(
                                                                    Icon::new(IconName::CircleCheck)
                                                                        .size(px(18.0))
                                                                        .text_color(colors.success),
                                                                ),
                                                            )
                                                        } else {
                                                            (
                                                                colors.background.opacity(0.4),
                                                                colors.border,
                                                                colors.foreground,
                                                                None,
                                                            )
                                                        };

                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .items_center()
                                                            .justify_between()
                                                            .w_full()
                                                            .px_4()
                                                            .py_3()
                                                            .rounded_xl()
                                                            .border_1()
                                                            .border_color(opt_border)
                                                            .bg(opt_bg)
                                                            .gap_3()
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_row()
                                                                    .items_start()
                                                                    .gap_3()
                                                                    .flex_1()
                                                                    .min_w_0()
                                                                    .child(
                                                                        div()
                                                                            .text_sm()
                                                                            .font_bold()
                                                                            .text_color(text_color)
                                                                            .child(format!(
                                                                                "{key})"
                                                                            )),
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .flex_1()
                                                                            .min_w_0()
                                                                            .text_sm()
                                                                            .text_color(text_color)
                                                                            .child(opt_text),
                                                                    ),
                                                            )
                                                            .when_some(right_icon, |row, icon| {
                                                                row.child(icon)
                                                            })
                                                    }),
                                                ),
                                            ),
                                    )
                                })
                        },
                    )),
            )
            // 5. Bottom Action Buttons
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_3()
                    .pt_1()
                    // Try again button
                    .child({
                        let label = if lang == Language::En {
                            "Try again"
                        } else {
                            "Subiramo ikizamini"
                        };
                        let tooltip_txt = if shortcuts_active {
                            format!("{label} (R)")
                        } else {
                            label.to_string()
                        };
                        div()
                            .id("new_quiz_btn")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2p5()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .cursor_pointer()
                            .hover(|h| h.bg(colors.accent))
                            .tooltip(move |window, cx| {
                                Tooltip::new(tooltip_txt.clone()).build(window, cx)
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_new_quiz(this, window, cx);
                            }))
                            .child(
                                Icon::new(IconName::RotateCw)
                                    .size(px(16.0))
                                    .text_color(colors.foreground),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(colors.foreground)
                                    .child(label),
                            )
                    })
                    // Retry mistakes button
                    .when(total_mistakes > 0, |row| {
                        let label = if lang == Language::En {
                            format!("Retry my mistakes ({total_mistakes})")
                        } else {
                            format!("Subiramo amakosa ({total_mistakes})")
                        };
                        let tooltip_txt = if shortcuts_active {
                            format!("{label} (W)")
                        } else {
                            label.clone()
                        };
                        row.child(
                            div()
                                .id("retry_wrong_btn")
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .px_4()
                                .py_2p5()
                                .rounded_xl()
                                .border_1()
                                .border_color(colors.border)
                                .bg(colors.secondary)
                                .cursor_pointer()
                                .hover(|h| h.bg(colors.accent))
                                .tooltip(move |window, cx| {
                                    Tooltip::new(tooltip_txt.clone()).build(window, cx)
                                })
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_retry_wrong(this, window, cx);
                                }))
                                .child(
                                    Icon::new(IconName::Target)
                                        .size(px(16.0))
                                        .text_color(colors.foreground),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_medium()
                                        .text_color(colors.foreground)
                                        .child(label),
                                ),
                        )
                    })
                    // Home button
                    .child({
                        let label = t("nav.home", lang);
                        let tooltip_txt = if shortcuts_active {
                            format!("{label} (Esc)")
                        } else {
                            label.to_string()
                        };
                        div()
                            .id("home_btn")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2p5()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .cursor_pointer()
                            .hover(|h| h.bg(colors.accent))
                            .tooltip(move |window, cx| {
                                Tooltip::new(tooltip_txt.clone()).build(window, cx)
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_home(this, window, cx);
                            }))
                            .child(
                                Icon::new(IconName::House)
                                    .size(px(16.0))
                                    .text_color(colors.foreground),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(colors.foreground)
                                    .child(label),
                            )
                    }),
            )
    }
}
