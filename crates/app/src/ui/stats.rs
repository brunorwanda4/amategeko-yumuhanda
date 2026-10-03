use crate::state::AppState;
use amategeko_core::{QuizMode, StatsCalculator, Strings};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct StatsView;

impl StatsView {
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_start_weak_practice: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_start_quiz: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let summary = StatsCalculator::compute_summary(&state.progress);
        let total_bank = state.bank.len() as u32;
        let seen_count = summary.questions_seen_count;
        let seen_percentage = (seen_count * 100).checked_div(total_bank).unwrap_or(0);

        div()
            .id("stats_scroll_view")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .p_4()
            .gap_4()
            // Title Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        Icon::new(IconName::ChartPie)
                            .size(px(24.0))
                            .text_color(colors.primary),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_extrabold()
                            .text_color(colors.foreground)
                            .child(Strings::STATS_TITLE),
                    ),
            )
            // Empty State (if no attempts yet)
            .when(summary.total_attempts == 0, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .p_8()
                        .rounded_2xl()
                        .border_1()
                        .border_color(colors.border)
                        .bg(colors.secondary)
                        .gap_4()
                        .child(
                            Icon::new(IconName::ChartPie)
                                .size(px(48.0))
                                .text_color(colors.muted_foreground),
                        )
                        .child(
                            div()
                                .text_lg()
                                .font_bold()
                                .text_color(colors.foreground)
                                .child("Nta mibare iraboneka"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(colors.muted_foreground)
                                .child(Strings::STATS_EMPTY),
                        )
                        .child(
                            Button::new("stats_start_quiz_btn")
                                .primary()
                                .label("Tangira Ikizamini")
                                .icon(IconName::Play)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_start_quiz(this, window, cx);
                                })),
                        ),
                )
            })
            // Populated State (when attempts exist)
            .when(summary.total_attempts > 0, |el| {
                el
                    // 4 Key Summary Tiles (2x2 on mobile, 4x1 on desktop)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap_3()
                            .child(Self::render_stat_tile(
                                "Ibizamini byakozwe",
                                format!("{}", summary.total_attempts),
                                IconName::Play,
                                colors.primary,
                                is_desktop,
                                cx,
                            ))
                            .child(Self::render_stat_tile(
                                "Impuzandengo",
                                format!("{:.1} / 20", summary.average_score),
                                IconName::ChartPie,
                                colors.accent,
                                is_desktop,
                                cx,
                            ))
                            .child(Self::render_stat_tile(
                                "Amanota yo hejuru",
                                format!("{}/20", summary.high_score),
                                IconName::Star,
                                colors.warning,
                                is_desktop,
                                cx,
                            ))
                            .child(Self::render_stat_tile(
                                "Ijanisha ryo gutsinda",
                                format!("{:.0}%", summary.pass_rate_percentage),
                                IconName::Check,
                                colors.success,
                                is_desktop,
                                cx,
                            )),
                    )
                    // 10-Attempt Score Bar Chart Card
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_4()
                            .rounded_2xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .gap_4()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(Strings::STATS_RECENT_CHART_TITLE),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_medium()
                                            .text_color(colors.muted_foreground)
                                            .child("Amanota asabwa: 12/20"),
                                    ),
                            )
                            // Bar Chart Visual Canvas
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .h(px(160.0))
                                    .w_full()
                                    .p_2()
                                    .rounded_xl()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_end()
                                            .justify_around()
                                            .size_full()
                                            .children(summary.last_10_scores.iter().enumerate().map(|(idx, &(sc, pm))| {
                                                let is_pass = sc >= pm;
                                                let bar_color = if is_pass { colors.success } else { colors.danger };
                                                let bar_h = (sc as f32 / 20.0) * 110.0;

                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .items_center()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_bold()
                                                            .text_color(bar_color)
                                                            .child(format!("{}", sc)),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(px(if is_desktop { 32.0 } else { 20.0 }))
                                                            .h(px(bar_h.max(6.0)))
                                                            .rounded_t_md()
                                                            .bg(bar_color),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_medium()
                                                            .text_color(colors.muted_foreground)
                                                            .child(format!("#{}", idx + 1)),
                                                    )
                                            })),
                                    ),
                            ),
                    )
                    // Questions Seen Progress Card
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_4()
                            .rounded_2xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child("Ibibazo byamaze kuboneka"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_bold()
                                            .text_color(colors.primary)
                                            .child(format!("{} kuri {} ({}%)", seen_count, total_bank, seen_percentage)),
                                    ),
                            )
                            // Progress bar track
                            .child(
                                div()
                                    .w_full()
                                    .h(px(10.0))
                                    .rounded_full()
                                    .bg(colors.background)
                                    .border_1()
                                    .border_color(colors.border)
                                    .child(
                                        div()
                                            .h_full()
                                            .rounded_full()
                                            .bg(colors.primary)
                                            .w(px(((seen_percentage as f32 / 100.0) * 300.0).max(8.0))),
                                    ),
                            ),
                    )
                    // Mode Breakdown Section
                    .when(!summary.mode_summaries.is_empty(), |m_el| {
                        m_el.child(
                            div()
                                .flex()
                                .flex_col()
                                .p_4()
                                .rounded_2xl()
                                .border_1()
                                .border_color(colors.border)
                                .bg(colors.secondary)
                                .gap_3()
                                .child(
                                    div()
                                        .text_base()
                                        .font_bold()
                                        .text_color(colors.foreground)
                                        .child("Imibare hakurikijwe Uburyo"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .children(summary.mode_summaries.iter().map(|ms| {
                                            let mode_title = match ms.mode {
                                                QuizMode::Byoroshye => Strings::MODE_EASY_TITLE,
                                                QuizMode::Hagati => Strings::MODE_MEDIUM_TITLE,
                                                QuizMode::Bikomeye => Strings::MODE_HARD_TITLE,
                                                QuizMode::WeakPractice => Strings::HOME_WEAK_TITLE,
                                                QuizMode::RetryWrong => Strings::RESULTS_RETRY_WRONG,
                                            };

                                            let pass_pct = (ms.passed_count * 100).checked_div(ms.attempts_count).unwrap_or(0);

                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .p_3()
                                                .rounded_xl()
                                                .bg(colors.background)
                                                .border_1()
                                                .border_color(colors.border)
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_bold()
                                                                .text_color(colors.foreground)
                                                                .child(mode_title),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(colors.muted_foreground)
                                                                .child(format!("{} ibizamini \u{2022} Wagize {:.1}/20 impuzandengo", ms.attempts_count, ms.average_score)),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_bold()
                                                        .text_color(if pass_pct >= 60 { colors.success } else { colors.danger })
                                                        .child(format!("{}% yatsinze", pass_pct)),
                                                )
                                        })),
                                ),
                        )
                    })
                    // Most-Missed Questions Card
                    .when(!summary.most_missed.is_empty(), |mm_el| {
                        mm_el.child(
                            div()
                                .flex()
                                .flex_col()
                                .p_4()
                                .rounded_2xl()
                                .border_1()
                                .border_color(colors.border)
                                .bg(colors.secondary)
                                .gap_4()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_base()
                                                .font_bold()
                                                .text_color(colors.foreground)
                                                .child(Strings::STATS_TOP_MISSED_TITLE),
                                        )
                                        .child(
                                            Button::new("stats_weak_practice_btn")
                                                .primary()
                                                .label(Strings::HOME_WEAK_BTN)
                                                .icon(IconName::Play)
                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                    on_start_weak_practice(this, window, cx);
                                                })),
                                        ),
                                )
                                // List of top missed questions
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2p5()
                                        .children(summary.most_missed.iter().map(|mq| {
                                            let q_opt = state.bank.get(mq.question_id);
                                            let text_snip = if let Some(q) = q_opt {
                                                if q.text.len() > 70 {
                                                    format!("{}...", &q.text[..65])
                                                } else {
                                                    q.text.clone()
                                                }
                                            } else {
                                                format!("Ikibazo cya {}", mq.question_id)
                                            };

                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .p_3()
                                                .rounded_xl()
                                                .bg(colors.background)
                                                .border_1()
                                                .border_color(colors.border)
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .flex_1()
                                                        .gap_0p5()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_bold()
                                                                .text_color(colors.primary)
                                                                .child(format!("Ikibazo cya {}", mq.question_id)),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(colors.foreground)
                                                                .child(text_snip),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .px_2p5()
                                                        .py_1()
                                                        .rounded_full()
                                                        .bg(colors.danger)
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(colors.primary_foreground)
                                                        .child(format!("{}x byakoswe", mq.wrong_count)),
                                                )
                                        })),
                                ),
                        )
                    })
            })
    }

    fn render_stat_tile<V: 'static>(
        label: &'static str,
        value: String,
        icon: IconName,
        accent_color: gpui::Hsla,
        is_desktop: bool,
        cx: &mut Context<V>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let width_val = if is_desktop { px(160.0) } else { px(150.0) };

        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w(width_val)
            .p_4()
            .rounded_2xl()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .gap_2()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(colors.muted_foreground)
                            .child(label),
                    )
                    .child(Icon::new(icon).size(px(18.0)).text_color(accent_color)),
            )
            .child(
                div()
                    .text_xl()
                    .font_extrabold()
                    .text_color(colors.foreground)
                    .child(value),
            )
    }
}
