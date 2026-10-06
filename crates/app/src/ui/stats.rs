use crate::state::AppState;
use crate::ui::layout::{page_column, PagePadding as _};
use crate::ui::scroll::vertical_scrollbar;
use amategeko_core::{t, tf, StatsCalculator, StatsFilter};
use gpui::InteractiveElement as _;
use gpui_kit::base::Disableable as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct StatsView;

impl StatsView {
    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        scroll_handle: &gpui::ScrollHandle,
        reveal_scrollbar: bool,
        active_filter: StatsFilter,
        confirm_clear: bool,
        cx: &mut Context<V>,
        on_set_filter: impl Fn(&mut V, StatsFilter, &mut Window, &mut Context<V>) + 'static + Copy,
        on_set_confirm_clear: impl Fn(&mut V, bool, &mut Window, &mut Context<V>) + 'static + Copy,
        on_clear_history: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_start_weak_practice: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let lang = state.settings.language;
        let q_lang = state.settings.question_language;
        let pass_mark = state.settings.pass_mark;

        // Compute filtered summary for tiles and chart
        let summary = StatsCalculator::compute_summary_filtered(&state.progress, active_filter);
        let total_bank = state.bank.len() as u32;
        let seen_count = summary.questions_seen_count;

        let has_history = !state.progress.attempts.is_empty();

        div()
            .id("stats_scroll_view")
            .track_scroll(scroll_handle)
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .page_padding(is_desktop)
            .child(vertical_scrollbar(
                "stats_scrollbar",
                scroll_handle,
                is_desktop,
                reveal_scrollbar,
            ))
            .child(
                page_column()
                    .gap_4()
                    // 1. Title + Clear History Header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .flex_wrap()
                            .child(
                                div()
                                    .text_xl()
                                    .font_bold()
                                    .text_color(colors.foreground)
                                    .child(t("stats.title", lang)),
                            )
                            .child(
                                Button::new("stats_clear_history_btn")
                                    .outline()
                                    .icon(IconName::Delete)
                                    .label(t("stats.clear", lang))
                                    .disabled(!has_history)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_set_confirm_clear(this, true, window, cx);
                                    })),
                            ),
                    )
                    // Inline Confirmation Banner
                    .when(confirm_clear, |el| {
                        el.child(
                            div()
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .items_center()
                                .justify_between()
                                .p_3()
                                .rounded_xl()
                                .border_1()
                                .border_color(colors.danger.opacity(0.4))
                                .bg(colors.danger.opacity(0.1))
                                .gap_3()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(colors.danger)
                                        .child(t("stats.sure", lang)),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            Button::new("stats_cancel_clear_btn")
                                                .outline()
                                                .label(t("stats.cancel", lang))
                                                .on_click(cx.listener(
                                                    move |this, _, window, cx| {
                                                        on_set_confirm_clear(
                                                            this, false, window, cx,
                                                        );
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("stats_confirm_clear_btn")
                                                .danger()
                                                .label(t("stats.yes", lang))
                                                .on_click(cx.listener(
                                                    move |this, _, window, cx| {
                                                        on_set_confirm_clear(
                                                            this, false, window, cx,
                                                        );
                                                        on_clear_history(this, window, cx);
                                                    },
                                                )),
                                        ),
                                ),
                        )
                    })
                    // 2. Filter Chips: All / Easy / Medium / Hard
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap_1p5()
                            .children(
                                [
                                    (StatsFilter::All, "stats.filter_all"),
                                    (StatsFilter::Easy, "stats.filter_easy"),
                                    (StatsFilter::Medium, "stats.filter_medium"),
                                    (StatsFilter::Hard, "stats.filter_hard"),
                                ]
                                .into_iter()
                                .map(
                                    |(filter_variant, label_key)| {
                                        let is_active = active_filter == filter_variant;
                                        let chip_bg = if is_active {
                                            colors.primary
                                        } else {
                                            colors.secondary
                                        };
                                        let chip_fg = if is_active {
                                            colors.primary_foreground
                                        } else {
                                            colors.muted_foreground
                                        };
                                        let chip_border = if is_active {
                                            colors.primary
                                        } else {
                                            colors.border
                                        };

                                        div()
                                            .id(format!("stats_filter_{}", filter_variant.as_str()))
                                            .cursor_pointer()
                                            .px_3()
                                            .py_1p5()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(chip_border)
                                            .bg(chip_bg)
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(chip_fg)
                                            .hover(|s| {
                                                if is_active {
                                                    s
                                                } else {
                                                    s.bg(colors.secondary_hover)
                                                }
                                            })
                                            .child(t(label_key, lang))
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_set_filter(this, filter_variant, window, cx);
                                            }))
                                    },
                                ),
                            ),
                    )
                    // 3. 4 Tiles: Exams, Average, Best score, Pass rate
                    .child({
                        let has_attempts = summary.total_attempts > 0;
                        let exams_val = if has_attempts {
                            format!("{}", summary.total_attempts)
                        } else {
                            "-".to_string()
                        };
                        let avg_val = if has_attempts {
                            format!("{:.1} / 20", summary.average_score)
                        } else {
                            "-".to_string()
                        };
                        let best_val = if has_attempts {
                            format!("{} / 20", summary.high_score)
                        } else {
                            "-".to_string()
                        };
                        let pass_val = if has_attempts {
                            format!("{:.0}%", summary.pass_rate_percentage)
                        } else {
                            "-".to_string()
                        };

                        let tiles = vec![
                            (t("stats.exams", lang), exams_val),
                            (t("stats.average", lang), avg_val),
                            (t("stats.best_score", lang), best_val),
                            (t("stats.pass_rate", lang), pass_val),
                        ];

                        div().flex().flex_row().flex_wrap().gap_2p5().children(
                            tiles.into_iter().map(|(label, value)| {
                                div()
                                    .flex()
                                    .flex_col()
                                    .justify_center()
                                    .p_3()
                                    .rounded_xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    .when(is_desktop, |el| el.flex_1().min_w(px(140.0)))
                                    .when(!is_desktop, |el| el.flex_1().min_w(px(130.0)))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child(label),
                                    )
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(value),
                                    )
                            }),
                        )
                    })
                    // 4. Bar Chart Card (Scores of the last 10 attempts)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_3p5()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .gap_2p5()
                            .child(
                                // Header: Title + Pass mark legend
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(t("stats.chart_title", lang)),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1p5()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child(
                                                div()
                                                    .font_bold()
                                                    .text_color(colors.success)
                                                    .child("- - -"),
                                            )
                                            .child(div().child(tf(
                                                "stats.pass_mark_legend",
                                                lang,
                                                &[("pass_mark", &pass_mark.to_string())],
                                            ))),
                                    ),
                            )
                            // Chart Canvas
                            .child({
                                let last_10 = &summary.last_10_attempts;
                                if last_10.is_empty() {
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .h(px(150.0))
                                        .border_b_1()
                                        .border_color(colors.border)
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(colors.muted_foreground)
                                                .child(t("stats.no_exams", lang)),
                                        )
                                        .into_any_element()
                                } else {
                                    let plot_usable_height = 110.0;
                                    let pass_fraction = (pass_mark as f32 / 20.0).clamp(0.0, 1.0);
                                    let pass_line_bottom = pass_fraction * plot_usable_height;
                                    let col_w = if is_desktop { 30.0 } else { 18.0 };

                                    div()
                                        .flex()
                                        .flex_col()
                                        .w_full()
                                        .child(
                                            // Plot area with relative container for bars + dashed pass line
                                            div()
                                                .relative()
                                                .h(px(150.0))
                                                .w_full()
                                                .border_b_1()
                                                .border_color(colors.border)
                                                // Dashed pass line
                                                .child(
                                                    div()
                                                        .absolute()
                                                        .left_0()
                                                        .right_0()
                                                        .bottom(px(pass_line_bottom))
                                                        .h(px(1.0))
                                                        .border_t_1()
                                                        .border_dashed()
                                                        .border_color(colors.success),
                                                )
                                                // Bars columns
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_end()
                                                        .justify_around()
                                                        .size_full()
                                                        .children(last_10.iter().map(|att| {
                                                            let is_pass = att.score >= pass_mark;
                                                            let bar_color = if is_pass {
                                                                colors.primary
                                                            } else {
                                                                colors.danger
                                                            };
                                                            let bar_h = ((att.score as f32 / 20.0)
                                                                * plot_usable_height)
                                                                .max(3.0);

                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .items_center()
                                                                .justify_end()
                                                                .h_full()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .font_bold()
                                                                        .text_color(
                                                                            colors.muted_foreground,
                                                                        )
                                                                        .child(format!(
                                                                            "{}",
                                                                            att.score
                                                                        )),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .w(px(col_w))
                                                                        .h(px(bar_h))
                                                                        .rounded_t_sm()
                                                                        .bg(bar_color),
                                                                )
                                                        })),
                                                ),
                                        )
                                        // Date labels row below the plot
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .justify_around()
                                                .w_full()
                                                .pt_1()
                                                .children(last_10.iter().map(|att| {
                                                    div()
                                                        .w(px(col_w))
                                                        .flex()
                                                        .justify_center()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(colors.muted_foreground)
                                                                .child(att.date_str.clone()),
                                                        )
                                                })),
                                        )
                                        .into_any_element()
                                }
                            }),
                    )
                    // 5. Cards Row (Most Missed Questions + By Mode)
                    // On desktop: 2 cols side-by-side; On phone: stacked vertically
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .when(is_desktop, |el| el.flex_row())
                            .when(!is_desktop, |el| el.flex_col())
                            // Card 1: Most Missed Questions
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .p_3p5()
                                    .rounded_xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    .gap_2p5()
                                    .when(is_desktop, |el| el.flex_1().min_w(px(0.0)))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(t("stats.top_missed_title", lang)),
                                    )
                                    // Questions list or empty message
                                    .child({
                                        if summary.most_missed.is_empty() {
                                            div()
                                                .text_xs()
                                                .text_color(colors.muted_foreground)
                                                .py_3()
                                                .child(t("stats.no_mistakes", lang))
                                                .into_any_element()
                                        } else {
                                            div()
                                                .flex()
                                                .flex_col()
                                                .children(summary.most_missed.iter().map(|mq| {
                                                    let q_opt = state.bank.get(mq.question_id);
                                                    let text_snip = if let Some(q) = q_opt {
                                                        let full_text = q.text_for(q_lang);
                                                        if full_text.chars().count() > 45 {
                                                            format!(
                                                                "{}...",
                                                                full_text
                                                                    .chars()
                                                                    .take(42)
                                                                    .collect::<String>()
                                                            )
                                                        } else {
                                                            full_text.to_string()
                                                        }
                                                    } else {
                                                        format!("#{}", mq.question_id)
                                                    };

                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap_2p5()
                                                        .py_2()
                                                        .border_t_1()
                                                        .border_color(colors.border)
                                                        // ID
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_medium()
                                                                .text_color(colors.muted_foreground)
                                                                .min_w(px(20.0))
                                                                .child(format!(
                                                                    "{}",
                                                                    mq.question_id
                                                                )),
                                                        )
                                                        // One-line text with ellipsis
                                                        .child(
                                                            div()
                                                                .flex_1()
                                                                .min_w(px(0.0))
                                                                .overflow_hidden()
                                                                .text_xs()
                                                                .text_color(colors.foreground)
                                                                .child(text_snip),
                                                        )
                                                        // Wrong/Seen Pill Badge
                                                        .child(
                                                            div()
                                                                .px_2()
                                                                .py_0p5()
                                                                .rounded_full()
                                                                .bg(colors.danger.opacity(0.12))
                                                                .text_xs()
                                                                .font_bold()
                                                                .text_color(colors.danger)
                                                                .child(format!(
                                                                    "{}/{}",
                                                                    mq.wrong_count, mq.seen_count
                                                                )),
                                                        )
                                                }))
                                                .into_any_element()
                                        }
                                    })
                                    // "Practice these" button
                                    .child(
                                        Button::new("stats_practice_missed_btn")
                                            .primary()
                                            .w_full()
                                            .disabled(summary.most_missed.is_empty())
                                            .icon(IconName::Play)
                                            .label(t("stats.practice_missed", lang))
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_start_weak_practice(this, window, cx);
                                            })),
                                    ),
                            )
                            // Card 2: By Mode
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .p_3p5()
                                    .rounded_xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    .gap_2p5()
                                    .when(is_desktop, |el| el.flex_1().min_w(px(0.0)))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(t("stats.by_mode", lang)),
                                    )
                                    // Easy, Medium, Hard breakdown
                                    .child(div().flex().flex_col().gap_2().children(
                                        summary.mode_summaries.iter().map(|ms| {
                                            let mode_title = match ms.mode {
                                                amategeko_core::QuizMode::Byoroshye => {
                                                    t("mode.easy.title", lang)
                                                }
                                                amategeko_core::QuizMode::Hagati => {
                                                    t("mode.medium.title", lang)
                                                }
                                                amategeko_core::QuizMode::Bikomeye => {
                                                    t("mode.hard.title", lang)
                                                }
                                                _ => "",
                                            };

                                            let avg_str = if ms.attempts_count > 0 {
                                                format!("{:.1} / 20", ms.average_score)
                                            } else {
                                                "-".to_string()
                                            };

                                            let progress_frac = if ms.attempts_count > 0 {
                                                (ms.average_score / 20.0).clamp(0.0, 1.0)
                                            } else {
                                                0.0
                                            };

                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .justify_between()
                                                        .text_xs()
                                                        .child(
                                                            div()
                                                                .font_medium()
                                                                .text_color(colors.foreground)
                                                                .child(mode_title),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_color(colors.muted_foreground)
                                                                .child(avg_str),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .h(px(6.0))
                                                        .rounded_full()
                                                        .bg(colors.background)
                                                        .border_1()
                                                        .border_color(colors.border)
                                                        .child(
                                                            div()
                                                                .h_full()
                                                                .rounded_full()
                                                                .bg(colors.primary)
                                                                .w(relative(progress_frac)),
                                                        ),
                                                )
                                        }),
                                    ))
                                    // Questions Seen of Total
                                    .child({
                                        let seen_str = tf(
                                            "stats.questions_seen_of",
                                            lang,
                                            &[
                                                ("seen", &seen_count.to_string()),
                                                ("total", &total_bank.to_string()),
                                            ],
                                        );
                                        let seen_frac = if total_bank > 0 {
                                            (seen_count as f32 / total_bank as f32).clamp(0.0, 1.0)
                                        } else {
                                            0.0
                                        };

                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .pt_1()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(colors.muted_foreground)
                                                    .child(seen_str),
                                            )
                                            .child(
                                                div()
                                                    .w_full()
                                                    .h(px(6.0))
                                                    .rounded_full()
                                                    .bg(colors.background)
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .child(
                                                        div()
                                                            .h_full()
                                                            .rounded_full()
                                                            .bg(colors.primary)
                                                            .w(relative(seen_frac)),
                                                    ),
                                            )
                                    }),
                            ),
                    ),
            )
    }
}
