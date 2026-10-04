use crate::state::AppState;
use amategeko_core::{t, QuizMode, StatsCalculator};
use gpui::InteractiveElement as _;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct HomeView;

impl HomeView {
    pub fn render<V: 'static>(
        state: &AppState,
        window_width: Pixels,
        cx: &mut Context<V>,
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
        on_resume: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_discard: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let stats_summary = StatsCalculator::compute_summary(&state.progress);
        let lang = state.settings.language;

        let is_desktop = window_width >= px(700.0);

        // Stats values matching the design
        let total_questions = state.bank.len();
        let last_score_display = if let Some(last) = state.progress.attempts.last() {
            format!("{} / {}", last.score, last.total)
        } else if let Some(last) = &state.last_result {
            format!("{} / {}", last.score, last.total)
        } else {
            "— / 20".to_string()
        };

        let pass_rate_display = if stats_summary.total_attempts > 0 {
            format!("{:.0}%", stats_summary.pass_rate_percentage)
        } else {
            "—".to_string()
        };

        div()
            .id("home_view_root")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .p_4()
            .when(is_desktop, |el| el.p_8())
            // Max width wrapper for clean desktop centering
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(1040.0))
                    .mx_auto()
                    .gap_6()
                    // Header / Greeting
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_bold()
                                    .text_color(colors.foreground)
                                    .child(t("home.welcome", lang)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.muted_foreground)
                                    .child(t("home.choose_mode", lang)),
                            ),
                    )
                    // In-Progress Attempt Banner (if an attempt is active)
                    .children(state.current_attempt.as_ref().map(|att| {
                        let answered = att.answered_count();
                        let total = att.total_questions();
                        let mode_title = match att.mode {
                            QuizMode::Byoroshye => t("mode.easy.title", lang),
                            QuizMode::Hagati => t("mode.medium.title", lang),
                            QuizMode::Bikomeye => t("mode.hard.title", lang),
                            QuizMode::WeakPractice => t("home.weak_title", lang),
                            QuizMode::RetryWrong => t("results.retry_wrong", lang),
                        };

                        div()
                            .flex()
                            .flex_col()
                            .when(is_desktop, |el| {
                                el.flex_row().items_center().justify_between()
                            })
                            .gap_3()
                            .p_4()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.primary)
                            .bg(colors.secondary)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_bold()
                                            .text_color(colors.primary)
                                            .child(t("home.resume_title", lang)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child(format!("{mode_title}: {answered}/{total}")),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .id("resume_quiz_btn")
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_center()
                                            .px_4()
                                            .py_2()
                                            .rounded_lg()
                                            .bg(colors.primary)
                                            .cursor_pointer()
                                            .hover(|el| el.opacity(0.9))
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_resume(this, window, cx);
                                            }))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_semibold()
                                                    .text_color(colors.primary_foreground)
                                                    .child(t("home.resume_btn", lang)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .id("discard_quiz_btn")
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_center()
                                            .px_4()
                                            .py_2()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(colors.border)
                                            .bg(colors.secondary)
                                            .cursor_pointer()
                                            .hover(|el| el.bg(colors.muted))
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_discard(this, window, cx);
                                            }))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_medium()
                                                    .text_color(colors.foreground)
                                                    .child(t("home.discard_btn", lang)),
                                            ),
                                    ),
                            )
                    }))
                    // 3 Mode Cards Row
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .when(is_desktop, |el| el.flex_row())
                            .when(!is_desktop, |el| el.flex_col())
                            // Card 1: Easy
                            .child(Self::render_mode_card(
                                QuizMode::Byoroshye,
                                t("mode.easy.title", lang),
                                t("mode.easy.desc", lang),
                                t("home.start", lang),
                                IconName::GraduationCap,
                                gpui::rgb(0x22c55e),
                                gpui::rgba(0x0e2f1bff),
                                is_desktop,
                                cx,
                                on_start_mode,
                            ))
                            // Card 2: Medium
                            .child(Self::render_mode_card(
                                QuizMode::Hagati,
                                t("mode.medium.title", lang),
                                t("mode.medium.desc", lang),
                                t("home.start", lang),
                                IconName::Clock,
                                gpui::rgb(0xf59e0b),
                                gpui::rgba(0x382405ff),
                                is_desktop,
                                cx,
                                on_start_mode,
                            ))
                            // Card 3: Hard
                            .child(Self::render_mode_card(
                                QuizMode::Bikomeye,
                                t("mode.hard.title", lang),
                                t("mode.hard.desc", lang),
                                t("home.start", lang),
                                IconName::Flame,
                                gpui::rgb(0xef4444),
                                gpui::rgba(0x3b1115ff),
                                is_desktop,
                                cx,
                                on_start_mode,
                            )),
                    )
                    // 3 Stat Tiles
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .when(is_desktop, |el| el.flex_row())
                            .when(!is_desktop, |el| el.flex_col())
                            .child(Self::render_stat_tile(
                                t("questions.filter_all", lang),
                                &total_questions.to_string(),
                                cx,
                            ))
                            .child(Self::render_stat_tile(
                                t("results.title", lang),
                                &last_score_display,
                                cx,
                            ))
                            .child(Self::render_stat_tile(
                                t("stats.pass_rate", lang),
                                &pass_rate_display,
                                cx,
                            )),
                    )
                    // Bottom Practice Card: Frequently Missed Questions
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .when(is_desktop, |el| {
                                el.flex_row().items_center().justify_between()
                            })
                            .gap_4()
                            .p_5()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            // Left side: target icon + titles
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .size(px(42.0))
                                            .rounded_xl()
                                            .bg(gpui::rgba(0x22262eff))
                                            .child(
                                                Icon::new(IconName::Target)
                                                    .size(px(22.0))
                                                    .text_color(colors.foreground),
                                            ),
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
                                                    .child(t("home.weak_title", lang)),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(colors.muted_foreground)
                                                    .child(t("home.weak_desc", lang)),
                                            ),
                                    ),
                            )
                            // Right side: Action Button
                            .child(
                                div()
                                    .id("start_weak_practice_btn")
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_center()
                                    .gap_2()
                                    .px_4()
                                    .py_2p5()
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(gpui::rgba(0x1a1e24ff))
                                    .cursor_pointer()
                                    .hover(|el| {
                                        el.bg(gpui::rgba(0x252a32ff))
                                            .border_color(gpui::rgba(0x3e4552ff))
                                    })
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_start_mode(this, QuizMode::WeakPractice, window, cx);
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(colors.foreground)
                                            .child(t("home.weak_btn", lang)),
                                    )
                                    .child(
                                        Icon::new(IconName::ArrowRight)
                                            .size(px(16.0))
                                            .text_color(colors.foreground),
                                    ),
                            ),
                    ),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_mode_card<V: 'static>(
        mode: QuizMode,
        title: &'static str,
        desc: &'static str,
        start_label: &'static str,
        icon: IconName,
        icon_color: impl Into<gpui::Hsla>,
        badge_bg: impl Into<gpui::Hsla>,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        div()
            .id(format!("card_mode_{title}"))
            .flex()
            .flex_col()
            .flex_1()
            .when(is_desktop, |el| el.min_w(px(240.0)))
            .p_5()
            .rounded_xl()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            // Header: Icon badge + Title
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
                            .bg(badge_bg.into())
                            .child(Icon::new(icon).size(px(20.0)).text_color(icon_color.into())),
                    )
                    .child(
                        div()
                            .text_lg()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child(title),
                    ),
            )
            // Card Description
            .child(
                div()
                    .mt_4()
                    .text_xs()
                    .text_color(colors.muted_foreground)
                    .min_h(px(44.0))
                    .child(desc),
            )
            // Bottom Action Button: Start
            .child(
                div().mt_6().w_full().child(
                    div()
                        .id(format!("btn_mode_{title}"))
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .w_full()
                        .py_2p5()
                        .px_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(colors.border)
                        .bg(gpui::rgba(0x1a1e24ff))
                        .cursor_pointer()
                        .hover(|el| {
                            el.bg(gpui::rgba(0x252a32ff))
                                .border_color(gpui::rgba(0x3e4552ff))
                        })
                        .on_click(cx.listener(move |this, _, window, cx| {
                            on_start_mode(this, mode, window, cx);
                        }))
                        .child(
                            Icon::new(IconName::Play)
                                .size(px(14.0))
                                .text_color(colors.foreground),
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_semibold()
                                .text_color(colors.foreground)
                                .child(start_label),
                        ),
                ),
            )
    }

    fn render_stat_tile<V: 'static>(
        label: &'static str,
        value: &str,
        cx: &mut Context<V>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap_2()
            .p_5()
            .rounded_xl()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .child(
                div()
                    .text_xs()
                    .font_medium()
                    .text_color(colors.muted_foreground)
                    .child(label),
            )
            .child(
                div()
                    .text_3xl()
                    .font_bold()
                    .text_color(colors.foreground)
                    .child(value.to_string()),
            )
    }
}
