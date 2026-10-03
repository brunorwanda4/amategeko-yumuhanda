use crate::state::AppState;
use amategeko_core::{QuizMode, StatsCalculator, Strings};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct HomeView;

impl HomeView {
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
        on_resume: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_discard: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let stats_summary = StatsCalculator::compute_summary(&state.progress);

        div()
            .id("home_scroll")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .p_4()
            .when(is_desktop, |el| el.p_6())
            // Max width wrapper for clean desktop centering
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(1080.0))
                    .mx_auto()
                    .gap_6()
                    // Header / Hero
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
                                    .child(Strings::APP_TITLE),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.muted_foreground)
                                    .child(Strings::HOME_WELCOME),
                            ),
                    )
                    // In-Progress Attempt Banner (if present)
                    .children(state.current_attempt.as_ref().map(|att| {
                        let answered = att.answered_count();
                        let total = att.total_questions();
                        let mode_title = att.mode.title_kinyarwanda();

                        div()
                            .flex()
                            .flex_col()
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
                                    .when(is_desktop, |el| el.flex_row().items_center().justify_between())
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_bold()
                                            .text_color(colors.primary)
                                            .child(Strings::HOME_RESUME_TITLE),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(colors.background)
                                            .text_color(colors.primary)
                                            .child(format!("{mode_title}: {answered}/{total} ibibazo")),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap_3()
                                    .child(
                                        Button::new("resume_quiz_btn")
                                            .primary()
                                            .label(Strings::HOME_RESUME_BTN)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_resume(this, window, cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("discard_quiz_btn")
                                            .outline()
                                            .label(Strings::HOME_DISCARD_BTN)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_discard(this, window, cx);
                                            })),
                                    ),
                            )
                    }))
                    // Mode Cards Section
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_semibold()
                                    .text_color(colors.foreground)
                                    .child("Hitamo uburyo bwo kwitoza:"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .when(is_desktop, |el| el.flex_row())
                                    // Card 1: Byoroshye (Easy - Green)
                                    .child(
                                        Self::render_mode_card(
                                            QuizMode::Byoroshye,
                                            Strings::MODE_EASY_TITLE,
                                            "Kwiga & Gusobanukirwa",
                                            Strings::MODE_EASY_DESC,
                                            "20 ibibazo • Nta gihe kigabanyuka • Ibisubizo byihuse",
                                            colors.success,
                                            cx,
                                            on_start_mode,
                                        ),
                                    )
                                    // Card 2: Hagati (Medium - Amber/Warning)
                                    .child(
                                        Self::render_mode_card(
                                            QuizMode::Hagati,
                                            Strings::MODE_MEDIUM_TITLE,
                                            "Kwimenyereza ikizamini",
                                            Strings::MODE_MEDIUM_DESC,
                                            "20 ibibazo • Iminota 20 • Guhindura igisubizo • Gusimbuka",
                                            colors.warning,
                                            cx,
                                            on_start_mode,
                                        ),
                                    )
                                    // Card 3: Bikomeye (Hard - Red/Danger)
                                    .child(
                                        Self::render_mode_card(
                                            QuizMode::Bikomeye,
                                            Strings::MODE_HARD_TITLE,
                                            "Uburyo bukaze",
                                            Strings::MODE_HARD_DESC,
                                            "20 ibibazo • Iminota 12 • Ibyapa byinshi • Nta gusimbuka",
                                            colors.danger,
                                            cx,
                                            on_start_mode,
                                        ),
                                    ),
                            ),
                    )
                    // Weak Questions Practice Card
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .when(is_desktop, |el| el.flex_row().items_center().justify_between())
                            .gap_3()
                            .p_4()
                            .rounded_xl()
                            .border_1()
                            .border_color(colors.border)
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
                                            .text_color(colors.foreground)
                                            .child("Ibibazo nakosheje kenshi"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child("Itoze ibibazo ugiramo amakosa cyane kugira ngo ubyumve neza"),
                                    ),
                            )
                            .child(
                                Button::new("start_weak_practice_btn")
                                    .primary()
                                    .label("Tangira kwitoza ibyo nakosheje")
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_start_mode(this, QuizMode::WeakPractice, window, cx);
                                    })),
                            ),
                    )
                    // Quick Statistics Preview (Responsive 4 in a row on Desktop, 2x2 grid on Mobile)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_semibold()
                                    .text_color(colors.foreground)
                                    .child("Incamake y'imibare"),
                            )
                            .child(
                                div()
                                    .when(is_desktop, |el| {
                                        // 4 in a single horizontal row on Desktop
                                        el.flex()
                                            .flex_row()
                                            .gap_3()
                                            .child(Self::render_stat_tile(
                                                Strings::STATS_TOTAL_ATTEMPTS,
                                                &stats_summary.total_attempts.to_string(),
                                                cx,
                                            ))
                                            .child(Self::render_stat_tile(
                                                Strings::STATS_AVERAGE_SCORE,
                                                &format!("{:.1}/20", stats_summary.average_score),
                                                cx,
                                            ))
                                            .child(Self::render_stat_tile(
                                                Strings::STATS_PASS_RATE,
                                                &format!("{:.0}%", stats_summary.pass_rate_percentage),
                                                cx,
                                            ))
                                            .child(Self::render_stat_tile(
                                                "Ibibazo wamaze kubona",
                                                &format!("{}/390", stats_summary.questions_seen_count),
                                                cx,
                                            ))
                                    })
                                    .when(!is_desktop, |el| {
                                        // 2x2 grid on Mobile for clean touch readability
                                        el.flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap_2()
                                                    .child(Self::render_stat_tile(
                                                        Strings::STATS_TOTAL_ATTEMPTS,
                                                        &stats_summary.total_attempts.to_string(),
                                                        cx,
                                                    ))
                                                    .child(Self::render_stat_tile(
                                                        Strings::STATS_AVERAGE_SCORE,
                                                        &format!("{:.1}/20", stats_summary.average_score),
                                                        cx,
                                                    )),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap_2()
                                                    .child(Self::render_stat_tile(
                                                        Strings::STATS_PASS_RATE,
                                                        &format!("{:.0}%", stats_summary.pass_rate_percentage),
                                                        cx,
                                                    ))
                                                    .child(Self::render_stat_tile(
                                                        "Ibibazo wamaze kubona",
                                                        &format!("{}/390", stats_summary.questions_seen_count),
                                                        cx,
                                                    )),
                                            )
                                    }),
                            ),
                    ),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_mode_card<V: 'static>(
        mode: QuizMode,
        title: &'static str,
        badge_text: &'static str,
        desc: &'static str,
        features: &'static str,
        accent_color: gpui::Hsla,
        cx: &mut Context<V>,
        on_start_mode: impl Fn(&mut V, QuizMode, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap_3()
            .p_4()
            .rounded_xl()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            // Header with title and badge
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_lg()
                            .font_bold()
                            .text_color(accent_color)
                            .child(title),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .px_2()
                            .py_0p5()
                            .rounded_md()
                            .border_1()
                            .border_color(accent_color)
                            .text_color(accent_color)
                            .child(badge_text),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(colors.foreground)
                    .child(desc),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(colors.muted_foreground)
                    .child(features),
            )
            .child(
                div()
                    .pt_2()
                    .child(
                        Button::new(format!("btn_mode_{title}"))
                            .primary()
                            .label(format!("Tangira {title}"))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_start_mode(this, mode, window, cx);
                            })),
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
            .gap_1()
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .child(
                div()
                    .text_xs()
                    .text_color(colors.muted_foreground)
                    .child(label),
            )
            .child(
                div()
                    .text_base()
                    .font_bold()
                    .text_color(colors.foreground)
                    .child(value.to_string()),
            )
    }
}
