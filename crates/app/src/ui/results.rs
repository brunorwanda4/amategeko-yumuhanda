use crate::state::AppState;
use amategeko_core::{t, Language, QuestionResult, QuizMode};
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

        let lang = state.settings.language;
        let q_lang = state.settings.question_language;
        let show_both = state.settings.show_both_languages;

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
        let pass_mark = result.pass_mark;
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
                    .border_color(if is_passed {
                        colors.success
                    } else {
                        colors.danger
                    })
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
                                    .text_color(if is_passed {
                                        colors.success
                                    } else {
                                        colors.danger
                                    })
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
                            .text_color(if is_passed {
                                colors.success
                            } else {
                                colors.danger
                            })
                            .child(if is_passed {
                                t("results.passed", lang)
                            } else {
                                t("results.failed", lang)
                            }),
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
                                    .child(format!(
                                        "{}: {}",
                                        if lang == Language::En { "Mode" } else { "Uburyo" },
                                        mode_label
                                    )),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(
                                        t("results.duration", lang)
                                            .replace("{duration}", &duration_str),
                                    ),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(
                                        t("results.pass_mark", lang)
                                            .replace("{pass_mark}", &pass_mark.to_string())
                                            .replace("{total}", &total.to_string()),
                                    ),
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
                                        .label(t("results.retry_wrong", lang))
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
                                    .label(t("results.new_quiz", lang))
                                    .icon(IconName::Play)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_new_quiz(this, window, cx);
                                    })),
                            )
                            // Home button
                            .child(
                                Button::new("home_btn")
                                    .outline()
                                    .label(t("nav.home", lang))
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
                            .border_color(if filter == ResultFilter::All {
                                colors.primary
                            } else {
                                colors.border
                            })
                            .bg(if filter == ResultFilter::All {
                                colors.primary
                            } else {
                                colors.secondary
                            })
                            .text_xs()
                            .font_semibold()
                            .text_color(if filter == ResultFilter::All {
                                colors.primary_foreground
                            } else {
                                colors.foreground
                            })
                            .child(
                                t("results.filter_all", lang).replace("{count}", &total.to_string()),
                            )
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
                            .border_color(if filter == ResultFilter::Correct {
                                colors.success
                            } else {
                                colors.border
                            })
                            .bg(if filter == ResultFilter::Correct {
                                colors.success
                            } else {
                                colors.secondary
                            })
                            .text_xs()
                            .font_semibold()
                            .text_color(if filter == ResultFilter::Correct {
                                colors.primary_foreground
                            } else {
                                colors.foreground
                            })
                            .child(
                                t("results.filter_correct", lang)
                                    .replace("{count}", &correct_count.to_string()),
                            )
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
                            .border_color(if filter == ResultFilter::Wrong {
                                colors.danger
                            } else {
                                colors.border
                            })
                            .bg(if filter == ResultFilter::Wrong {
                                colors.danger
                            } else {
                                colors.secondary
                            })
                            .text_xs()
                            .font_semibold()
                            .text_color(if filter == ResultFilter::Wrong {
                                colors.primary_foreground
                            } else {
                                colors.foreground
                            })
                            .child(
                                t("results.filter_wrong", lang)
                                    .replace("{count}", &wrong_count.to_string()),
                            )
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

                        let status_color = if is_correct {
                            colors.success
                        } else {
                            colors.danger
                        };
                        let status_icon = if is_correct {
                            IconName::Check
                        } else {
                            IconName::CircleX
                        };

                        let opts = q.options_for(q_lang);
                        let mut sorted_keys: Vec<&String> = opts.keys().collect();
                        sorted_keys.sort();

                        div()
                            .id(format!("result_card_{}", orig_idx))
                            .flex()
                            .flex_col()
                            .rounded_xl()
                            .border_1()
                            .border_color(if is_expanded {
                                status_color
                            } else {
                                colors.border
                            })
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
                                            // Question Number + Badges + Text Snippet
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
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
                                                                    .text_color(colors.foreground)
                                                                    .child(if lang == Language::En {
                                                                        format!("{}. Question #{}", orig_idx + 1, q.id)
                                                                    } else {
                                                                        format!("{}. Ikibazo cya {}", orig_idx + 1, q.id)
                                                                    }),
                                                            )
                                                            // Language badge if q_lang != lang (SPEC 15.4)
                                                            .when(q_lang != lang, |b| {
                                                                b.child(
                                                                    div()
                                                                        .px_1p5()
                                                                        .py_0p5()
                                                                        .rounded_md()
                                                                        .bg(colors.accent)
                                                                        .child(
                                                                            div()
                                                                                .text_xs()
                                                                                .font_bold()
                                                                                .text_color(colors.foreground)
                                                                                .child(match q_lang {
                                                                                    Language::En => t("badge.lang.en", lang),
                                                                                    Language::Rw => t("badge.lang.rw", lang),
                                                                                }),
                                                                        ),
                                                                )
                                                            })
                                                            // Draft badge if status is draft (SPEC 15.2)
                                                            .when(q.is_draft_translation() && q_lang == Language::En, |b| {
                                                                b.child(
                                                                    div()
                                                                        .px_1p5()
                                                                        .py_0p5()
                                                                        .rounded_md()
                                                                        .bg(colors.warning)
                                                                        .child(
                                                                            div()
                                                                                .text_xs()
                                                                                .font_bold()
                                                                                .text_color(colors.primary_foreground)
                                                                                .child(t("badge.unofficial_translation", lang)),
                                                                        ),
                                                                )
                                                            }),
                                                    )
                                                    .when(!is_expanded, |snip| {
                                                        let q_text = q.text_for(q_lang);
                                                        let snippet = if q_text.len() > 65 {
                                                            format!("{}...", &q_text[..60])
                                                        } else {
                                                            q_text.to_string()
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
                                        // Full question text + optional secondary
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_medium()
                                                        .text_color(colors.foreground)
                                                        .child(q.text_for(q_lang).to_string()),
                                                )
                                                .when(show_both, |bilingual| {
                                                    if let Some(sec_text) = q.secondary_text_for(q_lang) {
                                                        bilingual.child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(colors.muted_foreground)
                                                                .child(sec_text.to_string()),
                                                        )
                                                    } else {
                                                        bilingual
                                                    }
                                                }),
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
                                                    .child(t("results.unanswered", lang)),
                                            )
                                        })
                                        // Options review
                                        .child(div().flex().flex_col().gap_2().children(
                                            sorted_keys.into_iter().map(|key| {
                                                let opt_text =
                                                    opts.get(key).cloned().unwrap_or_default();
                                                let sec_opt = if show_both {
                                                    q.secondary_option_for(q_lang, key).map(|s| s.to_string())
                                                } else {
                                                    None
                                                };
                                                let is_user_choice = user_ans == Some(key.as_str());
                                                let is_correct_choice = correct_ans == key;

                                                let your_ans_correct = if lang == Language::En {
                                                    "Your answer (Correct)"
                                                } else {
                                                    "Igisubizo cyawe (Cy'ukuri)"
                                                };
                                                let your_ans_wrong = if lang == Language::En {
                                                    "Your answer (Incorrect)"
                                                } else {
                                                    "Igisubizo cyawe (Siko)"
                                                };
                                                let right_ans = if lang == Language::En {
                                                    "Correct answer"
                                                } else {
                                                    "Igisubizo cy'ukuri"
                                                };

                                                let (opt_bg, opt_border, badge_text, badge_color) =
                                                    if is_user_choice && is_correct_choice {
                                                        (
                                                            colors.success,
                                                            colors.success,
                                                            Some(your_ans_correct),
                                                            colors.success,
                                                        )
                                                    } else if is_user_choice && !is_correct_choice {
                                                        (
                                                            colors.danger,
                                                            colors.danger,
                                                            Some(your_ans_wrong),
                                                            colors.danger,
                                                        )
                                                    } else if is_correct_choice {
                                                        (
                                                            colors.success,
                                                            colors.success,
                                                            Some(right_ans),
                                                            colors.success,
                                                        )
                                                    } else {
                                                        (
                                                            colors.background,
                                                            colors.border,
                                                            None,
                                                            colors.muted_foreground,
                                                        )
                                                    };

                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .p_2p5()
                                                    .rounded_lg()
                                                    .border_1()
                                                    .border_color(opt_border)
                                                    .bg(if is_user_choice || is_correct_choice {
                                                        colors.background
                                                    } else {
                                                        opt_bg
                                                    })
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
                                                                    .text_color(
                                                                        if is_correct_choice {
                                                                            colors.success
                                                                        } else if is_user_choice {
                                                                            colors.danger
                                                                        } else {
                                                                            colors.foreground
                                                                        },
                                                                    )
                                                                    .child(format!("{})", key)),
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .flex_col()
                                                                    .gap_0p5()
                                                                    .child(
                                                                        div()
                                                                            .text_xs()
                                                                            .text_color(colors.foreground)
                                                                            .child(opt_text),
                                                                    )
                                                                    .when_some(sec_opt, |el, sec| {
                                                                        el.child(
                                                                            div()
                                                                                .text_xs()
                                                                                .text_color(colors.muted_foreground)
                                                                                .child(sec),
                                                                        )
                                                                    }),
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
                                            }),
                                        )),
                                )
                            })
                    })),
            )
    }
}
