use crate::state::AppState;
use crate::ui::footer::AppFooter;
use crate::ui::scroll::{vertical_scrollbar, ScrollbarContext};
use amategeko_core::stats::timestamp_to_day_month_str;
use amategeko_core::{t, tf, Language, QuestionSet, QuizMode, StatsCalculator};
use gpui::InteractiveElement as _;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// Number of questions in a full exam.
const EXAM_QUESTIONS: usize = 20;

/// Everything the quiz page can ask its host to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeAction {
    /// Select a mode card (updates tiles and chips, does not start).
    SelectMode(QuizMode),
    /// Pick the question set used by Easy.
    SelectSet(QuestionSet),
    /// Start the given mode now.
    Play(QuizMode),
    /// Start Easy with the Mistakes set (the "My mistakes" card).
    PlayMistakes,
    Resume,
    Discard,
}

/// UI state owned by the host view.
#[derive(Debug, Clone, Copy)]
pub struct HomeProps {
    pub selected_mode: QuizMode,
    pub selected_set: QuestionSet,
    /// Mode that was just started; shows "Starting {mode}…" in the bottom bar.
    pub starting: Option<QuizMode>,
}

struct ModeInfo {
    mode: QuizMode,
    key: &'static str,
    name: &'static str,
    desc: &'static str,
    short: &'static str,
    tag: &'static str,
    play_label: &'static str,
    icon: IconName,
    color: gpui::Hsla,
}

pub struct HomeView;

/// Localised name of a quiz mode as used on the quiz page.
pub fn mode_name(mode: QuizMode, lang: Language) -> &'static str {
    match mode {
        QuizMode::Byoroshye => t("mode.easy", lang),
        QuizMode::Hagati => t("mode.med", lang),
        QuizMode::Bikomeye => t("mode.hard", lang),
        other => other.title(lang),
    }
}

fn set_label(set: QuestionSet, lang: Language) -> &'static str {
    match set {
        QuestionSet::All => t("set.all", lang),
        QuestionSet::Signs => t("set.sign", lang),
        QuestionSet::Rules => t("set.rule", lang),
        QuestionSet::NotSeen => t("set.new", lang),
        QuestionSet::Starred => t("set.star", lang),
        QuestionSet::Mistakes => t("set.mis", lang),
    }
}

impl HomeView {
    /// The set Easy will really use: falls back to All if the picked set became empty.
    pub fn effective_set(state: &AppState, selected: QuestionSet) -> QuestionSet {
        if state.question_set_count(selected) == 0 {
            QuestionSet::All
        } else {
            selected
        }
    }

    pub fn render<V: 'static>(
        state: &AppState,
        props: HomeProps,
        scroll: ScrollbarContext<'_>,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let lang = state.settings.language;
        let is_desktop = scroll.is_desktop;
        let shortcuts_active =
            crate::shortcuts_ui_active(is_desktop, state.settings.desktop_shortcuts_enabled);

        let selected_mode = props.selected_mode;
        let is_easy = selected_mode == QuizMode::Byoroshye;
        let set = Self::effective_set(state, props.selected_set);
        let set_count = state.question_set_count(set);

        let modes = [
            ModeInfo {
                mode: QuizMode::Byoroshye,
                key: "easy",
                name: t("mode.easy", lang),
                desc: t("mode.easy.desc", lang),
                short: t("mode.easy.short", lang),
                tag: t("tag.learning", lang),
                play_label: t("mode.easy.play_label", lang),
                icon: IconName::GraduationCap,
                color: colors.success,
            },
            ModeInfo {
                mode: QuizMode::Hagati,
                key: "medium",
                name: t("mode.med", lang),
                desc: t("mode.med.desc", lang),
                short: t("mode.med.short", lang),
                tag: t("tag.practice", lang),
                play_label: t("mode.med.play_label", lang),
                icon: IconName::Clock,
                color: colors.warning,
            },
            ModeInfo {
                mode: QuizMode::Bikomeye,
                key: "hard",
                name: t("mode.hard", lang),
                desc: t("mode.hard.desc", lang),
                short: t("mode.hard.short", lang),
                tag: t("tag.strict", lang),
                play_label: t("mode.hard.play_label", lang),
                icon: IconName::Flame,
                color: colors.danger,
            },
        ];

        // "What to expect" tiles come from Settings and the chosen set.
        let exam_total = state.bank.len().min(EXAM_QUESTIONS);
        let minutes = |n: u32| tf("val.min", lang, &[("n", &n.to_string())]);
        let (q_val, time_val, answers_val, nav_val) = match selected_mode {
            QuizMode::Byoroshye => (
                set_count.min(EXAM_QUESTIONS).to_string(),
                t("val.nolimit", lang).to_string(),
                t("val.instant", lang).to_string(),
                t("val.free", lang).to_string(),
            ),
            QuizMode::Hagati => (
                exam_total.to_string(),
                minutes(state.settings.medium_duration_mins),
                t("val.end", lang).to_string(),
                t("val.freeflag", lang).to_string(),
            ),
            _ => (
                exam_total.to_string(),
                minutes(state.settings.hard_duration_mins),
                t("val.end", lang).to_string(),
                t("val.no", lang).to_string(),
            ),
        };
        let facts = [
            (t("tile.q", lang), q_val),
            (t("tile.time", lang), time_val),
            (t("tile.answers", lang), answers_val),
            (t("tile.nav", lang), nav_val),
        ];

        let few_note = (is_easy && set_count > 0 && set_count < EXAM_QUESTIONS)
            .then(|| tf("quiz.few", lang, &[("n", &set_count.to_string())]));

        let bar_text = match props.starting {
            Some(mode) => tf("quiz.starting", lang, &[("mode", mode_name(mode, lang))]),
            None => Self::last_attempt_text(state, lang),
        };

        let banner = state.current_attempt.as_ref().map(|att| {
            let detail = format!(
                "{}: {}/{}",
                att.mode.title(lang),
                att.answered_count(),
                att.total_questions()
            );
            Self::resume_banner(detail, lang, is_desktop, cx, on_action)
        });

        let chips: Vec<_> = QuestionSet::ALL_SETS
            .iter()
            .enumerate()
            .map(|(idx, kind)| {
                Self::chip(
                    idx,
                    *kind,
                    set_label(*kind, lang),
                    state.question_set_count(*kind),
                    *kind == set,
                    is_desktop,
                    cx,
                    on_action,
                )
            })
            .collect();

        let mode_cards: Vec<_> = modes
            .iter()
            .map(|info| {
                Self::mode_card(info, info.mode == selected_mode, is_desktop, cx, on_action)
            })
            .collect();

        let fact_tiles: Vec<_> = facts
            .into_iter()
            .map(|(label, value)| Self::fact_tile(label, value, cx))
            .collect();
        let mut fact_tiles = fact_tiles.into_iter();

        let facts_el = if is_desktop {
            div()
                .flex()
                .flex_row()
                .gap_2()
                .children(fact_tiles)
                .into_any_element()
        } else {
            let row1 = div()
                .flex()
                .flex_row()
                .gap_2()
                .children(fact_tiles.by_ref().take(2));
            let row2 = div().flex().flex_row().gap_2().children(fact_tiles);
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(row1)
                .child(row2)
                .into_any_element()
        };

        let stats_summary = StatsCalculator::compute_summary(&state.progress);
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
        let stat_tiles = div()
            .flex()
            .gap_3()
            .when(is_desktop, |el| el.flex_row())
            .when(!is_desktop, |el| el.flex_col())
            .child(Self::render_stat_tile(
                t("questions.filter_all", lang),
                &state.bank.len().to_string(),
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
            ));
        let weak_card = Self::render_weak_card(lang, is_desktop, cx, on_action);
        let footer = AppFooter::render(is_desktop, px(720.0), lang, cx);

        let label_el = |text: &'static str| {
            div()
                .text_xs()
                .text_color(colors.muted_foreground)
                .child(text)
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.background)
            .child(
                div()
                    .id("home_scroll")
                    .track_scroll(scroll.handle)
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p_4()
                    .when(is_desktop, |el| el.p_6())
                    .child(vertical_scrollbar(
                        "home_scrollbar",
                        scroll.handle,
                        is_desktop,
                        scroll.reveal_on_open,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w_full()
                            .max_w(px(720.0))
                            .mx_auto()
                            .gap_4()
                            .children(banner)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_semibold()
                                            .text_color(colors.foreground)
                                            .child(t("quiz.title", lang)),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(colors.muted_foreground)
                                            .child(t("quiz.sub", lang)),
                                    ),
                            )
                            .child(label_el(t("quiz.mode", lang)))
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .when(is_desktop, |el| el.flex_row())
                                    .when(!is_desktop, |el| el.flex_col())
                                    .children(mode_cards),
                            )
                            .when(is_easy, |el| {
                                el.child(label_el(t("quiz.from", lang))).child(
                                    div().flex().flex_row().flex_wrap().gap_2().children(chips),
                                )
                            })
                            .when(is_easy, |el| {
                                el.child(
                                    div()
                                        .min_h(px(16.0))
                                        .text_xs()
                                        .text_color(colors.warning)
                                        .children(few_note),
                                )
                            })
                            .when(!is_easy, |el| {
                                el.child(
                                    div()
                                        .min_h(px(16.0))
                                        .text_xs()
                                        .text_color(colors.muted_foreground)
                                        .child(t("quiz.examnote", lang)),
                                )
                            })
                            .child(label_el(t("quiz.expect", lang)))
                            .child(facts_el)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.muted_foreground)
                                    .child(tf(
                                        "quiz.pass",
                                        lang,
                                        &[("p", &state.settings.pass_mark.to_string())],
                                    )),
                            )
                            .child(stat_tiles)
                            .child(weak_card),
                    )
                    .child(footer),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_xs()
                            .text_color(colors.muted_foreground)
                            .when(!is_desktop, |el| el.text_center())
                            .child(bar_text),
                    )
                    .when(is_desktop && shortcuts_active, |el| {
                        el.child(
                            div()
                                .text_xs()
                                .text_color(colors.muted_foreground)
                                .child(t("quiz.keys", lang)),
                        )
                    }),
            )
    }

    fn render_weak_card<V: 'static>(
        lang: Language,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

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
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .min_w_0()
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .size(px(42.0))
                            .rounded_xl()
                            .bg(colors.muted.opacity(0.4))
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
                            .min_w_0()
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
            .child(
                div()
                    .id("start_weak_practice_btn")
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .px_4()
                    .min_h(px(if is_desktop { 40.0 } else { 44.0 }))
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.muted.opacity(0.4))
                    .cursor_pointer()
                    .hover(|el| el.bg(colors.muted))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action(this, HomeAction::PlayMistakes, window, cx);
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

    fn last_attempt_text(state: &AppState, lang: Language) -> String {
        match state.progress.attempts.last() {
            Some(last) => {
                let result = if last.passed {
                    t("quiz.passed", lang)
                } else {
                    t("quiz.failed", lang)
                };
                tf(
                    "quiz.last",
                    lang,
                    &[
                        ("mode", mode_name(last.mode, lang)),
                        ("score", &last.score.to_string()),
                        ("result", result),
                        ("date", &timestamp_to_day_month_str(last.timestamp_secs)),
                    ],
                )
            }
            None => t("quiz.none", lang).to_string(),
        }
    }

    fn resume_banner<V: 'static>(
        detail: String,
        lang: Language,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let min_h = if is_desktop { 36.0 } else { 44.0 };

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
                    .gap_0p5()
                    .min_w_0()
                    .child(
                        div()
                            .text_base()
                            .font_semibold()
                            .text_color(colors.primary)
                            .child(t("resume.title", lang)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors.muted_foreground)
                            .child(detail),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .id("quiz_discard_btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .min_h(px(min_h))
                            .px_4()
                            .rounded_lg()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.secondary)
                            .cursor_pointer()
                            .hover(|el| el.bg(colors.muted))
                            .when(!is_desktop, |el| el.flex_1())
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_action(this, HomeAction::Discard, window, cx);
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(colors.foreground)
                                    .child(t("resume.discard", lang)),
                            ),
                    )
                    .child(
                        div()
                            .id("quiz_resume_btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .min_h(px(min_h))
                            .px_4()
                            .rounded_lg()
                            .bg(colors.primary)
                            .cursor_pointer()
                            .hover(|el| el.opacity(0.9))
                            .when(!is_desktop, |el| el.flex_1())
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_action(this, HomeAction::Resume, window, cx);
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.primary_foreground)
                                    .child(t("resume.go", lang)),
                            ),
                    ),
            )
    }

    fn mode_card<V: 'static>(
        info: &ModeInfo,
        selected: bool,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let mode = info.mode;

        let tile = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(32.0))
            .rounded_lg()
            .bg(info.color.opacity(0.15))
            .child(
                Icon::new(info.icon.clone())
                    .size(px(18.0))
                    .text_color(info.color),
            );

        let play = Self::play_button(info, is_desktop, cx, on_action);

        let card_id = format!("quiz_mode_{}", info.key);
        let base = div()
            .id(card_id.clone())
            .debug_selector(move || card_id)
            .flex()
            .min_w_0()
            .rounded_xl()
            .bg(colors.secondary)
            .cursor_pointer()
            .hover(|el| el.bg(colors.muted.opacity(0.4)))
            .when(selected, |el| {
                el.border_2().border_color(colors.foreground).p(px(11.0))
            })
            .when(!selected, |el| {
                el.border_1().border_color(colors.border).p(px(12.0))
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                on_action(this, HomeAction::SelectMode(mode), window, cx);
            }));

        if is_desktop {
            base.flex_1()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(tile)
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_base()
                                .font_semibold()
                                .text_color(colors.foreground)
                                .child(info.name),
                        )
                        .child(play),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(colors.muted_foreground)
                        .child(info.desc),
                )
                .child(
                    div().flex().flex_row().child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_md()
                            .bg(colors.muted.opacity(0.4))
                            .text_xs()
                            .text_color(colors.muted_foreground)
                            .child(info.tag),
                    ),
                )
        } else {
            base.flex_row()
                .items_center()
                .gap_3()
                .w_full()
                .child(tile)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .gap_0p5()
                        .child(
                            div()
                                .text_base()
                                .font_semibold()
                                .text_color(colors.foreground)
                                .child(info.name),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.muted_foreground)
                                .child(info.short),
                        ),
                )
                .child(play)
        }
    }

    fn play_button<V: 'static>(
        info: &ModeInfo,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let mode = info.mode;
        // The tooltip doubles as the button's accessible name ("Start Easy").
        let label = info.play_label;
        let size = if is_desktop { 38.0 } else { 44.0 };

        let play_id = format!("quiz_play_{}", info.key);
        div()
            .id(play_id.clone())
            .debug_selector(move || play_id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(size))
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .cursor_pointer()
            .hover(|el| el.bg(colors.muted))
            .tooltip(move |window, cx| Tooltip::new(label).build(window, cx))
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                on_action(this, HomeAction::Play(mode), window, cx);
            }))
            .child(
                Icon::new(IconName::Play)
                    .size(px(16.0))
                    .text_color(colors.foreground),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn chip<V: 'static>(
        idx: usize,
        kind: QuestionSet,
        label: &'static str,
        count: usize,
        selected: bool,
        is_desktop: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let disabled = count == 0;
        let listener = cx.listener(move |this, _, window, cx| {
            on_action(this, HomeAction::SelectSet(kind), window, cx);
        });

        let chip_id = format!("quiz_set_{idx}");
        div()
            .id(chip_id.clone())
            .debug_selector(move || chip_id)
            .flex()
            .flex_row()
            .items_center()
            .gap_1p5()
            .px_3()
            .min_h(px(if is_desktop { 32.0 } else { 44.0 }))
            .rounded_lg()
            .border_1()
            .border_color(if selected {
                colors.foreground
            } else {
                colors.border
            })
            .when(selected, |el| el.bg(colors.muted.opacity(0.4)))
            .when(disabled, |el| el.opacity(0.4).cursor_not_allowed())
            .when(!disabled, move |el| {
                el.cursor_pointer()
                    .hover(|el| el.bg(colors.muted.opacity(0.4)))
                    .on_click(listener)
            })
            .child(div().text_sm().text_color(colors.foreground).child(label))
            .child(
                div()
                    .text_xs()
                    .text_color(colors.muted_foreground)
                    .child(count.to_string()),
            )
    }

    fn fact_tile<V: 'static>(
        label: &'static str,
        value: String,
        cx: &mut Context<V>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap_0p5()
            .px_3()
            .py_2p5()
            .rounded_lg()
            .bg(colors.muted.opacity(0.4))
            .child(
                div()
                    .text_xs()
                    .text_color(colors.muted_foreground)
                    .child(label),
            )
            .child(
                div()
                    .text_sm()
                    .font_semibold()
                    .text_color(colors.foreground)
                    .child(value),
            )
    }
}
