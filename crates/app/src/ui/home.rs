use crate::font::DISPLAY_FONT_FAMILY;
use crate::state::AppState;
use crate::ui::layout::PagePadding as _;
use crate::ui::scroll::{vertical_scrollbar, DragScroll, ScrollbarContext};

use amategeko_core::{
    effective_length, t, tf, time_for, Language, QuestionSet, QuizMode, StatsCalculator,
};
use gpui::InteractiveElement as _;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::slider::{Slider, SliderState};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

const DISPLAY_FONT: &str = DISPLAY_FONT_FAMILY;
const BODY_FONT: &str = "Instrument Sans";

/// Theme tokens strictly matching the redesign spec for dark and light modes.
#[derive(Clone, Copy, Debug)]
pub struct HomeTokens {
    pub bg: gpui::Hsla,
    pub surface: gpui::Hsla,
    pub surface2: gpui::Hsla,
    pub line: gpui::Hsla,
    pub line2: gpui::Hsla,
    pub fg: gpui::Hsla,
    pub muted: gpui::Hsla,
    pub inv_bg: gpui::Hsla,
    pub inv_fg: gpui::Hsla,
    pub c_mis: gpui::Hsla,
    pub c_oth: gpui::Hsla,
    pub c_un: gpui::Hsla,
}

impl HomeTokens {
    pub fn new(is_dark: bool) -> Self {
        if is_dark {
            Self {
                bg: gpui::rgb(0x000000).into(),
                surface: gpui::rgb(0x0f0f0f).into(),
                surface2: gpui::rgb(0x1a1a1a).into(),
                line: gpui::rgb(0x2a2a2a).into(),
                line2: gpui::rgb(0x3a3a3a).into(),
                fg: gpui::rgb(0xffffff).into(),
                muted: gpui::rgb(0x9a9a9a).into(),
                inv_bg: gpui::rgb(0xffffff).into(),
                inv_fg: gpui::rgb(0x000000).into(),
                c_mis: gpui::rgb(0x6c6c6c).into(),
                c_oth: gpui::rgb(0x2e2e2e).into(),
                c_un: gpui::rgb(0x4a4a4a).into(),
            }
        } else {
            Self {
                bg: gpui::rgb(0xffffff).into(),
                surface: gpui::rgb(0xf5f5f5).into(),
                surface2: gpui::rgb(0xebebeb).into(),
                line: gpui::rgb(0xdcdcdc).into(),
                line2: gpui::rgb(0xc4c4c4).into(),
                fg: gpui::rgb(0x000000).into(),
                muted: gpui::rgb(0x666666).into(),
                inv_bg: gpui::rgb(0x000000).into(),
                inv_fg: gpui::rgb(0xffffff).into(),
                c_mis: gpui::rgb(0x8e8e8e).into(),
                c_oth: gpui::rgb(0xdedede).into(),
                c_un: gpui::rgb(0xb8b8b8).into(),
            }
        }
    }
}

/// Everything the quiz page can ask its host to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeAction {
    /// Select a mode card (updates tiles and chips, does not start).
    SelectMode(QuizMode),
    /// Pick the question set used by Easy.
    SelectSet(QuestionSet),
    SetQuizLength(usize),
    ActivateCustomLength,
    /// Start the given mode now.
    Play(QuizMode),
    /// Start Easy with the Mistakes set (the "My mistakes" card).
    PlayMistakes,
    /// Start or continue the ordered full-bank study mode.
    PlayStudy,
    /// Ask the host to confirm clearing the current study round.
    RestartStudy,
    OpenQuestions,
    OpenQuestion(u32),
    OpenResults,
    OpenStats,
    Resume,
    Discard,
    UpdateWhatsNew,
    UpdateDownload,
    UpdateLater,
    UpdateSkip,
}

/// UI state owned by the host view.
#[derive(Clone)]
pub struct HomeProps {
    pub selected_mode: QuizMode,
    pub selected_set: QuestionSet,
    pub window_width: Option<f32>,
    pub custom_length_active: bool,
    pub quiz_length_slider: Entity<SliderState>,
}

struct ModeInfo {
    mode: QuizMode,
    key: &'static str,
    name: &'static str,
    tag: &'static str,
    lv: usize,
    desc: &'static str,
    id: &'static str,
    play_id: &'static str,
}

pub struct HomeView;

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
        let is_dark = cx.theme().mode == gpui_kit::component::ThemeMode::Dark;
        let colors = cx.theme().colors;
        let tokens = HomeTokens::new(is_dark);
        let lang = state.settings.language;
        let is_desktop = scroll.is_desktop;

        let window_w = props
            .window_width
            .unwrap_or(if is_desktop { 1280.0 } else { 390.0 });
        let content_w = if is_desktop {
            (window_w - 228.0).min(720.0)
        } else {
            window_w
        };
        let is_mid = is_desktop && content_w < 960.0;
        let is_two_col = is_desktop && !is_mid;

        let selected_mode = props.selected_mode;
        let is_easy = selected_mode == QuizMode::Byoroshye;
        let is_medium = selected_mode == QuizMode::Hagati;
        let set = Self::effective_set(state, props.selected_set);
        let set_count = state.question_set_count(set);
        let requested_length = state.settings.quiz_length.clamp(5, 100);
        let available = if is_easy { set_count } else { state.bank.len() };
        let attempt_length = effective_length(selected_mode, requested_length, available);

        // Resume banner if an in-progress quiz attempt exists
        let banner = state.current_attempt.as_ref().map(|att| {
            let detail = format!(
                "{}: {}/{}",
                att.mode.title(lang),
                att.answered_count(),
                att.total_questions()
            );
            Self::resume_banner(detail, lang, is_desktop, tokens, cx, on_action)
        });
        let update_banner = match crate::updater::status() {
            crate::updater::UpdateStatus::Available(update) => Some(Self::update_banner(
                &update.release.tag_name,
                lang,
                is_desktop,
                tokens,
                cx,
                on_action,
            )),
            _ => None,
        };

        // 1. Hero Card: ordered full-bank study
        let mut study_ids: Vec<u32> = state
            .bank
            .all()
            .iter()
            .map(|question| question.id)
            .collect();
        study_ids.sort_unstable();
        let total_study = study_ids.len();
        let answered_study = state.progress.study.answers.len().min(total_study);
        let study_pct = if total_study > 0 {
            ((answered_study as f32 / total_study as f32) * 100.0).round() as usize
        } else {
            0
        };
        let resume_number = state
            .progress
            .study
            .last_id
            .and_then(|id| study_ids.iter().position(|&x| x == id))
            .map(|pos| (pos + 1).min(total_study))
            .unwrap_or(1);

        let hero_card = div()
            .id("study_all_questions_card")
            .border_1()
            .border_color(colors.primary.opacity(0.35))
            .bg(tokens.surface)
            .text_color(tokens.fg)
            .rounded(px(24.0))
            .p(px(if is_desktop { 28.0 } else { 16.0 }))
            .w_full()
            .flex()
            .when(is_desktop, |el| el.flex_row().items_end().gap(px(30.0)))
            .when(!is_desktop, |el| el.flex_col().gap(px(12.0)))
            .child(
                // Big number "3 muri 404"
                div()
                    .flex()
                    .when(is_desktop, |el| el.flex_col().gap(px(12.0)))
                    .when(!is_desktop, |el| el.flex_row().items_center().gap(px(12.0)))
                    .child(
                        div()
                            .font_family(DISPLAY_FONT)
                            .font_extrabold()
                            .text_size(px(if is_desktop { 112.0 } else { 56.0 }))
                            .line_height(px(if is_desktop { 96.0 } else { 50.0 }))
                            .text_color(colors.primary)
                            .child(resume_number.to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .min_w_0()
                            .gap_0p5()
                            .child(
                                div()
                                    .font_family(BODY_FONT)
                                    .font_medium()
                                    .text_sm()
                                    .text_color(tokens.muted)
                                    .child(tf(
                                        "study.of_total",
                                        lang,
                                        &[("total", &total_study.to_string())],
                                    )),
                            )
                            .when(!is_desktop, |el| {
                                el.child(
                                    div()
                                        .font_family(DISPLAY_FONT)
                                        .font_bold()
                                        .text_lg()
                                        .whitespace_normal()
                                        .text_color(tokens.fg)
                                        .child(t("study.title", lang)),
                                )
                            }),
                    ),
            )
            .child(
                // Center text + progress
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .when(is_desktop, |el| {
                        el.child(
                            div()
                                .font_family(DISPLAY_FONT)
                                .font_bold()
                                .text_size(px(26.0))
                                .line_height(px(30.0))
                                .mb(px(6.0))
                                .text_color(tokens.fg)
                                .child(t("study.title", lang)),
                        )
                        .child(
                            div()
                                .font_family(BODY_FONT)
                                .text_sm()
                                .mb(px(18.0))
                                .text_color(tokens.muted)
                                .child(t("study.sub", lang)),
                        )
                    })
                    .child(
                        // Thin progress bar
                        div()
                            .h(px(6.0))
                            .rounded_full()
                            .bg(colors.primary.opacity(0.16))
                            .overflow_hidden()
                            .w_full()
                            .child(
                                div()
                                    .h_full()
                                    .w(relative((study_pct.max(2) as f32) / 100.0))
                                    .rounded_full()
                                    .bg(colors.primary),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .mt(px(8.0))
                            .text_xs()
                            .font_family(BODY_FONT)
                            .text_color(tokens.muted)
                            .child(tf(
                                "study.continue",
                                lang,
                                &[
                                    ("n", &resume_number.to_string()),
                                    ("total", &total_study.to_string()),
                                ],
                            ))
                            .child(tf("study.done", lang, &[("pct", &study_pct.to_string())])),
                    ),
            )
            .child(
                // Action buttons
                div()
                    .flex()
                    .when(is_desktop, |el| el.flex_col().gap(px(8.0)).items_stretch())
                    .when(!is_desktop, |el| {
                        el.flex_row().justify_center().gap(px(8.0)).items_center()
                    })
                    .child(
                        div()
                            .id("study_continue_btn")
                            .cursor_pointer()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_center()
                            .gap(px(10.0))
                            .rounded_full()
                            .px_5()
                            .py_3()
                            .when(!is_desktop, |el| {
                                el.min_h(px(44.0)).px_4().py_2().gap_2().text_xs()
                            })
                            .font_semibold()
                            .text_sm()
                            .bg(colors.primary)
                            .text_color(colors.primary_foreground)
                            .hover(|el| el.opacity(0.9))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_action(this, HomeAction::PlayStudy, window, cx);
                            }))
                            .child(
                                Icon::new(IconName::Play)
                                    .size(px(if is_desktop { 16.0 } else { 14.0 }))
                                    .text_color(colors.primary_foreground),
                            )
                            .child(t("resume.go", lang)),
                    )
                    .child(
                        div()
                            .id("study_restart_btn")
                            .cursor_pointer()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_center()
                            .gap(px(10.0))
                            .rounded_full()
                            .px_5()
                            .py_3()
                            .when(!is_desktop, |el| {
                                el.min_h(px(44.0)).px_4().py_2().gap_2().text_xs()
                            })
                            .font_semibold()
                            .text_sm()
                            .border_1()
                            .border_color(colors.primary.opacity(0.5))
                            .bg(gpui::transparent_black())
                            .text_color(colors.primary)
                            .hover(|el| el.bg(colors.primary.opacity(0.1)))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_action(this, HomeAction::RestartStudy, window, cx);
                            }))
                            .child(t("study.restart", lang)),
                    ),
            );

        // 2. Modes list (3 radio rows)
        let modes_data = [
            ModeInfo {
                mode: QuizMode::Byoroshye,
                key: "1",
                name: t("mode.easy", lang),
                tag: t("tag.learning", lang),
                lv: 1,
                desc: t("mode.easy.desc", lang),
                id: "quiz_mode_easy",
                play_id: "quiz_play_easy",
            },
            ModeInfo {
                mode: QuizMode::Hagati,
                key: "2",
                name: t("mode.med", lang),
                tag: t("tag.practice", lang),
                lv: 2,
                desc: t("mode.med.desc", lang),
                id: "quiz_mode_medium",
                play_id: "quiz_play_medium",
            },
            ModeInfo {
                mode: QuizMode::Bikomeye,
                key: "3",
                name: t("mode.hard", lang),
                tag: t("tag.strict", lang),
                lv: 3,
                desc: t("mode.hard.desc", lang),
                id: "quiz_mode_hard",
                play_id: "quiz_play_hard",
            },
        ];

        let mode_rows: Vec<_> = modes_data
            .iter()
            .map(|info| {
                let is_sel = info.mode == selected_mode;
                let mode = info.mode;
                let mode_accent = match info.mode {
                    QuizMode::Byoroshye => colors.success,
                    QuizMode::Hagati => colors.warning,
                    QuizMode::Bikomeye => colors.danger,
                    _ => colors.primary,
                };
                div()
                    .id(info.id)
                    .debug_selector(move || info.id.to_string())
                    .cursor_pointer()
                    .rounded(px(14.0))
                    .p(px(if is_desktop { 14.0 } else { 12.0 }))
                    .border_1()
                    .when(is_sel, |el| {
                        el.border_color(mode_accent).bg(mode_accent.opacity(0.08))
                    })
                    .when(!is_sel, |el| el.border_color(tokens.line).bg(tokens.bg))
                    .hover(move |el| el.border_color(mode_accent))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action(this, HomeAction::SelectMode(mode), window, cx);
                    }))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(14.0))
                    .child(
                        // Dot
                        div()
                            .size(px(18.0))
                            .rounded_full()
                            .border_1()
                            .border_color(if is_sel { mode_accent } else { tokens.line2 })
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(is_sel, |el| {
                                el.child(div().size(px(8.0)).rounded_full().bg(mode_accent))
                            }),
                    )
                    .child(
                        // Content
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap_0p5()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_baseline()
                                    .gap_2()
                                    .child(
                                        div()
                                            .font_family(BODY_FONT)
                                            .font_semibold()
                                            .text_sm()
                                            .text_color(tokens.fg)
                                            .child(info.name),
                                    )
                                    .child(
                                        div()
                                            .font_family(BODY_FONT)
                                            .text_xs()
                                            .text_color(tokens.muted)
                                            .child(info.tag),
                                    ),
                            )
                            .when(is_desktop, |el| {
                                el.child(
                                    div()
                                        .font_family(BODY_FONT)
                                        .text_xs()
                                        .text_color(tokens.muted)
                                        .child(info.desc),
                                )
                            }),
                    )
                    .child(
                        // Side: meter + kbd + direct play
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_end()
                                    .gap(px(3.0))
                                    .h(px(16.0))
                                    .child(div().w(px(5.0)).h(px(7.0)).rounded(px(1.5)).bg(
                                        if info.lv >= 1 {
                                            mode_accent
                                        } else {
                                            tokens.line2
                                        },
                                    ))
                                    .child(div().w(px(5.0)).h(px(11.0)).rounded(px(1.5)).bg(
                                        if info.lv >= 2 {
                                            mode_accent
                                        } else {
                                            tokens.line2
                                        },
                                    ))
                                    .child(div().w(px(5.0)).h(px(16.0)).rounded(px(1.5)).bg(
                                        if info.lv >= 3 {
                                            mode_accent
                                        } else {
                                            tokens.line2
                                        },
                                    )),
                            )
                            .when(is_desktop, |el| {
                                el.child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(6.0))
                                        .border_1()
                                        .border_color(tokens.line2)
                                        .bg(tokens.bg)
                                        .text_xs()
                                        .text_color(tokens.muted)
                                        .child(info.key),
                                )
                            })
                            .child(
                                div()
                                    .id(info.play_id)
                                    .debug_selector(move || info.play_id.to_string())
                                    .size(px(if is_desktop { 38.0 } else { 44.0 }))
                                    .rounded_full()
                                    .border_1()
                                    .border_color(tokens.line2)
                                    .bg(tokens.bg)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(move |el| {
                                        el.border_color(mode_accent).bg(mode_accent.opacity(0.1))
                                    })
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        cx.stop_propagation();
                                        on_action(this, HomeAction::Play(mode), window, cx);
                                    }))
                                    .child(
                                        Icon::new(IconName::Play)
                                            .size(px(16.0))
                                            .text_color(mode_accent),
                                    ),
                            ),
                    )
            })
            .collect();

        // 3. Source chips
        let chips: Vec<_> = QuestionSet::ALL_SETS
            .iter()
            .enumerate()
            .map(|(idx, kind)| {
                let chip_id = format!("quiz_set_{idx}");
                let count = state.question_set_count(*kind);
                let selected = *kind == set;
                let disabled = count == 0;
                let label = set_label(*kind, lang);
                let kind_val = *kind;

                div()
                    .id(chip_id.clone())
                    .debug_selector(move || chip_id)
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(7.0))
                    .px(px(13.0))
                    .min_h(px(if is_desktop { 32.0 } else { 44.0 }))
                    .rounded_full()
                    .border_1()
                    .border_color(if selected { tokens.fg } else { tokens.line2 })
                    .bg(if selected {
                        tokens.fg
                    } else {
                        gpui::transparent_black()
                    })
                    .when(disabled, |el| el.opacity(0.4).cursor_not_allowed())
                    .when(!disabled, |el| {
                        el.cursor_pointer()
                            .hover(|e| e.border_color(tokens.fg))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_action(this, HomeAction::SelectSet(kind_val), window, cx);
                            }))
                    })
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_sm()
                            .font_medium()
                            .text_color(if selected { tokens.bg } else { tokens.fg })
                            .child(label),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(if selected {
                                tokens.bg.opacity(0.6)
                            } else {
                                tokens.muted
                            })
                            .child(count.to_string()),
                    )
            })
            .collect();

        let length_chips: Vec<_> = [10usize, 20, 30, 40, 50]
            .into_iter()
            .map(|length| {
                let selected = !props.custom_length_active && requested_length == length;
                div()
                    .id(format!("quiz_length_{length}"))
                    .cursor_pointer()
                    .min_h(px(if is_desktop { 32.0 } else { 44.0 }))
                    .px_3()
                    .rounded_full()
                    .border_1()
                    .border_color(if selected { tokens.fg } else { tokens.line2 })
                    .bg(if selected {
                        tokens.fg
                    } else {
                        gpui::transparent_black()
                    })
                    .text_color(if selected { tokens.bg } else { tokens.fg })
                    .flex()
                    .items_center()
                    .justify_center()
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action(this, HomeAction::SetQuizLength(length), window, cx);
                    }))
                    .child(length.to_string())
            })
            .collect();

        let custom_selected = props.custom_length_active;
        let custom_length_slider = props.quiz_length_slider.clone();

        // 4. Start Button CTA
        let (cta_count, cta_source) = if is_easy {
            (attempt_length, set_label(set, lang))
        } else {
            (attempt_length, t("set.all", lang))
        };
        let cta_title = match selected_mode {
            QuizMode::Byoroshye => t("mode.easy", lang),
            QuizMode::Hagati => t("mode.med", lang),
            QuizMode::Bikomeye => t("mode.hard", lang),
            _ => t("mode.easy", lang),
        };
        let cta_sub = tf(
            "home.questions_from",
            lang,
            &[("count", &cta_count.to_string()), ("source", cta_source)],
        );
        let cta_accent = match selected_mode {
            QuizMode::Byoroshye => colors.success,
            QuizMode::Hagati => colors.warning,
            QuizMode::Bikomeye => colors.danger,
            _ => colors.primary,
        };

        let start_cta = div()
            .id("quiz_cta_start_btn")
            .cursor_pointer()
            .w_full()
            .rounded(px(18.0))
            .border_1()
            .border_color(cta_accent.opacity(0.55))
            .bg(tokens.surface2)
            .text_color(tokens.fg)
            .p(px(12.0))
            .pl(px(20.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(14.0))
            .hover(move |el| el.border_color(cta_accent).bg(cta_accent.opacity(0.08)))
            .on_click(cx.listener(move |this, _, window, cx| {
                on_action(this, HomeAction::Play(selected_mode), window, cx);
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .font_family(DISPLAY_FONT)
                            .font_bold()
                            .text_size(px(18.0))
                            .line_height(px(22.0))
                            .text_color(tokens.fg)
                            .child(cta_title),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(cta_sub),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .when(is_desktop, |el| {
                        el.child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(tokens.line2)
                                .bg(tokens.bg)
                                .text_xs()
                                .font_medium()
                                .text_color(tokens.fg)
                                .child("Enter"),
                        )
                    })
                    .child(
                        div()
                            .size(px(44.0))
                            .rounded_full()
                            .border_1()
                            .border_color(cta_accent)
                            .bg(cta_accent.opacity(0.1))
                            .text_color(cta_accent)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(IconName::Play)
                                    .size(px(18.0))
                                    .text_color(cta_accent),
                            ),
                    ),
            );

        // Play panel wrapper
        let few_note = (is_easy && set_count > 0 && set_count < requested_length)
            .then(|| tf("quiz.few", lang, &[("n", &set_count.to_string())]));

        let play_panel = div()
            .bg(tokens.surface)
            .border_1()
            .border_color(tokens.line)
            .rounded(px(20.0))
            .p(px(if is_desktop { 22.0 } else { 18.0 }))
            .w_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .mb(px(14.0))
                    .font_family(DISPLAY_FONT)
                    .font_bold()
                    .text_size(px(17.0))
                    .line_height(px(22.0))
                    .text_color(tokens.fg)
                    .child(t("quiz.title", lang)),
            )
            .child(div().flex().flex_col().gap_2().children(mode_rows))
            .when(selected_mode != QuizMode::Bikomeye, |el| {
                el.child(
                    div()
                        .mt(px(20.0))
                        .mb(px(10.0))
                        .font_family(BODY_FONT)
                        .text_xs()
                        .text_color(tokens.muted)
                        .child(t("quiz.length", lang)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap_2()
                        .children(length_chips)
                        .child(
                            div()
                                .id("quiz_length_custom")
                                .cursor_pointer()
                                .min_h(px(if is_desktop { 32.0 } else { 44.0 }))
                                .px_3()
                                .rounded_full()
                                .border_1()
                                .border_color(if custom_selected {
                                    tokens.fg
                                } else {
                                    tokens.line2
                                })
                                .bg(if custom_selected {
                                    tokens.fg
                                } else {
                                    gpui::transparent_black()
                                })
                                .text_color(if custom_selected {
                                    tokens.bg
                                } else {
                                    tokens.fg
                                })
                                .flex()
                                .items_center()
                                .justify_center()
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_action(this, HomeAction::ActivateCustomLength, window, cx);
                                }))
                                .child(t("quiz.length.custom", lang)),
                        ),
                )
                .when(custom_selected, |el| {
                    el.child(
                        div().mt_2().flex().flex_col().gap_2().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_3()
                                .child(Slider::new(&custom_length_slider).w(px(if is_desktop {
                                    240.0
                                } else {
                                    200.0
                                })))
                                .child(
                                    div()
                                        .w(px(32.0))
                                        .text_sm()
                                        .font_bold()
                                        .text_right()
                                        .text_color(tokens.fg)
                                        .child(requested_length.to_string()),
                                ),
                        ),
                    )
                })
                .when(is_medium, |el| {
                    el.child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(t("quiz.length.time", lang)),
                    )
                })
            })
            .when(selected_mode == QuizMode::Bikomeye, |el| {
                el.child(
                    div()
                        .mt_4()
                        .text_sm()
                        .text_color(tokens.muted)
                        .child(t("quiz.length.hard", lang)),
                )
            })
            .when(is_medium || selected_mode == QuizMode::Bikomeye, |el| {
                let seconds = if is_medium {
                    time_for(state.settings.medium_duration_mins, attempt_length)
                } else {
                    u64::from(state.settings.hard_duration_mins) * 60
                };
                el.child(
                    div().mt_4().child(
                        div()
                            .rounded_lg()
                            .border_1()
                            .border_color(tokens.line)
                            .bg(tokens.bg)
                            .px_3()
                            .py_2()
                            .text_xs()
                            .child(format!(
                                "{}: {}",
                                if is_medium {
                                    t("settings.medium_time", lang)
                                } else {
                                    t("settings.hard_time", lang)
                                },
                                seconds.div_ceil(60)
                            )),
                    ),
                )
            })
            .when(is_easy, |el| {
                el.child(
                    div()
                        .mt(px(20.0))
                        .mb(px(10.0))
                        .font_family(BODY_FONT)
                        .text_xs()
                        .text_color(tokens.muted)
                        .child(t("quiz.from", lang)),
                )
                .child(div().flex().flex_row().flex_wrap().gap_2().children(chips))
            })
            .when(is_easy && few_note.is_some(), |el| {
                el.child(
                    div()
                        .mt(px(8.0))
                        .text_xs()
                        .text_color(tokens.muted)
                        .children(few_note),
                )
            })
            .when(!is_easy, |el| {
                el.child(
                    div()
                        .mt(px(16.0))
                        .font_family(BODY_FONT)
                        .text_xs()
                        .text_color(tokens.muted)
                        .child(t("quiz.examnote", lang)),
                )
            })
            .child(div().mt(px(20.0)).w_full().child(start_cta));

        // 5. Question Map (404 cells)
        let all_questions = state.bank.all();
        let mut ok_c = 0usize;
        let mut mis_c = 0usize;
        let mut un_c = 0usize;
        let mut oth_c = 0usize;

        let current_study_id = state
            .progress
            .study
            .last_id
            .unwrap_or_else(|| all_questions.first().map(|q| q.id).unwrap_or(1));

        let focus_mis = props.selected_set == QuestionSet::Mistakes;
        let focus_un = props.selected_set == QuestionSet::NotSeen;
        let map_cell_size = if !is_desktop && window_w < 380.0 {
            10.0
        } else {
            11.0
        };
        let map_gap = if !is_desktop && window_w < 380.0 {
            2.0
        } else {
            3.0
        };
        let tooltip_width = if is_desktop { 320.0 } else { 260.0 };

        let map_cells: Vec<_> = all_questions
            .iter()
            .map(|q| {
                let q_id = q.id;
                let stat = state.progress.question_stats.get(&q.id);
                let seen = stat.map_or(0, |s| s.seen_count);
                let wrong = stat.map_or(0, |s| s.wrong_count);
                let correct = stat.map_or(0, |s| s.correct_count);

                let (cell_state, is_mis, is_un) = if seen == 0 {
                    un_c += 1;
                    (0, false, true)
                } else if wrong > 0 {
                    mis_c += 1;
                    (1, true, false)
                } else if correct > 0 {
                    ok_c += 1;
                    (2, false, false)
                } else {
                    oth_c += 1;
                    (3, false, false)
                };

                let is_current = q.id == current_study_id;
                let dimmed = (focus_mis && !is_mis) || (focus_un && !is_un);
                let question_text = q.text_for(state.settings.question_language).to_string();
                let correct_label = tf(
                    "home.correct_count",
                    lang,
                    &[("count", &correct.to_string())],
                );
                let wrong_label = tf("home.wrong_count", lang, &[("count", &wrong.to_string())]);

                div()
                    .id(SharedString::from(format!("question_map_cell_{q_id}")))
                    .size(px(map_cell_size))
                    .rounded(px(2.5))
                    .cursor_pointer()
                    .hover(|el| el.border_1().border_color(tokens.fg))
                    .tooltip(move |window, cx| {
                        let question_text = question_text.clone();
                        let correct_label = correct_label.clone();
                        let wrong_label = wrong_label.clone();
                        Tooltip::element(move |_, cx| {
                            let colors = cx.theme().colors;
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .max_w(px(tooltip_width))
                                .py_1()
                                .child(
                                    div()
                                        .flex()
                                        .items_start()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex_none()
                                                .rounded_md()
                                                .bg(colors.muted)
                                                .px_1p5()
                                                .py_0p5()
                                                .text_xs()
                                                .font_semibold()
                                                .child(format!("#{q_id}")),
                                        )
                                        .child(
                                            div()
                                                .min_w_0()
                                                .whitespace_normal()
                                                .font_medium()
                                                .child(question_text.clone()),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .items_center()
                                        .gap_x_3()
                                        .gap_y_1()
                                        .text_xs()
                                        .text_color(colors.muted_foreground)
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .size(px(7.0))
                                                        .rounded_full()
                                                        .bg(colors.success),
                                                )
                                                .child(correct_label.clone()),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .size(px(7.0))
                                                        .rounded_full()
                                                        .bg(colors.danger),
                                                )
                                                .child(wrong_label.clone()),
                                        ),
                                )
                        })
                        .build(window, cx)
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action(this, HomeAction::OpenQuestion(q_id), window, cx);
                    }))
                    .when(dimmed, |el| el.opacity(0.2))
                    .when(cell_state == 0, |el| {
                        el.border_1().border_color(tokens.c_un)
                    })
                    .when(cell_state == 1, |el| el.bg(colors.danger))
                    .when(cell_state == 2, |el| el.bg(colors.success))
                    .when(cell_state == 3, |el| el.bg(colors.primary))
                    .when(is_current, |el| el.border_2().border_color(tokens.fg))
            })
            .collect();

        let map_panel = div()
            .bg(tokens.surface)
            .border_1()
            .border_color(tokens.line)
            .rounded(px(20.0))
            .p(px(if is_desktop { 22.0 } else { 18.0 }))
            .w_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .justify_between()
                    .items_baseline()
                    .gap_2()
                    .mb(px(14.0))
                    .child(
                        div()
                            .font_family(DISPLAY_FONT)
                            .font_bold()
                            .text_size(px(17.0))
                            .line_height(px(22.0))
                            .text_color(tokens.fg)
                            .child(t("home.progress_title", lang)),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(t("home.progress_hint", lang)),
                    ),
            )
            .child(
                div()
                    .id("question_map")
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(map_gap))
                    .w_full()
                    .children(map_cells),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_x_4()
                    .gap_y_1p5()
                    .mt(px(16.0))
                    .text_xs()
                    .font_family(BODY_FONT)
                    .text_color(tokens.muted)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(div().size(px(10.0)).rounded(px(2.5)).bg(colors.success))
                            .child(tf(
                                "home.correct_count",
                                lang,
                                &[("count", &ok_c.to_string())],
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(div().size(px(10.0)).rounded(px(2.5)).bg(colors.danger))
                            .child(tf(
                                "home.wrong_count",
                                lang,
                                &[("count", &mis_c.to_string())],
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .size(px(10.0))
                                    .rounded(px(2.5))
                                    .border_1()
                                    .border_color(colors.warning)
                                    .bg(colors.warning.opacity(0.12)),
                            )
                            .child(tf(
                                "home.unseen_count",
                                lang,
                                &[("count", &un_c.to_string())],
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(div().size(px(10.0)).rounded(px(2.5)).bg(colors.primary))
                            .child(tf(
                                "home.other_count",
                                lang,
                                &[("count", &oth_c.to_string())],
                            )),
                    ),
            );

        // 6. Stat Tiles (Byose, Exam score, Pass rate)
        let stats_summary = StatsCalculator::compute_summary(&state.progress);
        let last_attempt = state
            .progress
            .attempts
            .last()
            .or(state.last_result.as_ref());
        let last_score_num = last_attempt.map(|last| last.score);
        let last_total_num = last_attempt
            .map(|last| last.total_questions())
            .unwrap_or(20);
        let last_score_str = last_score_num
            .map(|s| s.to_string())
            .unwrap_or_else(|| "—".to_string());
        let pass_rate_num = if stats_summary.total_attempts > 0 {
            Some(stats_summary.pass_rate_percentage.round() as usize)
        } else {
            None
        };
        let pass_rate_str = pass_rate_num
            .map(|p| format!("{}%", p))
            .unwrap_or_else(|| "—".to_string());

        let tile_byose = div()
            .id("home_all_questions_stat")
            .cursor_pointer()
            .flex_1()
            .bg(tokens.surface)
            .border_1()
            .border_color(tokens.line)
            .rounded(px(20.0))
            .p(px(if is_desktop { 20.0 } else { 16.0 }))
            .hover(|el| el.border_color(colors.primary))
            .on_click(cx.listener(move |this, _, window, cx| {
                on_action(this, HomeAction::OpenQuestions, window, cx);
            }))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .size(px(30.0))
                            .rounded(px(9.0))
                            .bg(colors.primary.opacity(0.12))
                            .text_color(colors.primary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(IconName::BookOpen)
                                    .size(px(16.0))
                                    .text_color(colors.primary),
                            ),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(t("questions.filter_all", lang)),
                    ),
            )
            .child(
                div()
                    .font_family(DISPLAY_FONT)
                    .font_extrabold()
                    .text_size(px(if is_desktop { 44.0 } else { 36.0 }))
                    .line_height(px(if is_desktop { 48.0 } else { 40.0 }))
                    .text_color(tokens.fg)
                    .my(px(if is_desktop { 16.0 } else { 12.0 }))
                    .child(state.bank.len().to_string()),
            )
            .child(
                div()
                    .h(px(6.0))
                    .rounded_full()
                    .bg(tokens.line2)
                    .overflow_hidden()
                    .w_full()
                    .child(div().h_full().w_full().bg(colors.primary)),
            );

        let score_val = last_score_num.unwrap_or(0);
        let segs: Vec<_> = (0..last_total_num)
            .map(|i| {
                div()
                    .flex_1()
                    .h(px(6.0))
                    .rounded(px(2.0))
                    .bg(if i < score_val {
                        colors.warning
                    } else {
                        tokens.line2
                    })
            })
            .collect();

        let tile_results = div()
            .id("home_results_stat")
            .cursor_pointer()
            .flex_1()
            .bg(tokens.surface)
            .border_1()
            .border_color(tokens.line)
            .rounded(px(20.0))
            .p(px(if is_desktop { 20.0 } else { 16.0 }))
            .hover(|el| el.border_color(colors.warning))
            .on_click(cx.listener(move |this, _, window, cx| {
                on_action(this, HomeAction::OpenResults, window, cx);
            }))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .size(px(30.0))
                            .rounded(px(9.0))
                            .bg(colors.warning.opacity(0.12))
                            .text_color(colors.warning)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(IconName::Target)
                                    .size(px(16.0))
                                    .text_color(colors.warning),
                            ),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(t("results.title", lang)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .gap_1p5()
                    .my(px(if is_desktop { 16.0 } else { 12.0 }))
                    .child(
                        div()
                            .font_family(DISPLAY_FONT)
                            .font_extrabold()
                            .text_size(px(if is_desktop { 44.0 } else { 36.0 }))
                            .line_height(px(if is_desktop { 48.0 } else { 40.0 }))
                            .text_color(tokens.fg)
                            .child(last_score_str),
                    )
                    .child(
                        div()
                            .font_family(DISPLAY_FONT)
                            .font_medium()
                            .text_lg()
                            .text_color(tokens.muted)
                            .child(format!("/ {last_total_num}")),
                    ),
            )
            .child(div().flex().flex_row().gap(px(2.0)).w_full().children(segs));

        let pr_pct = pass_rate_num.unwrap_or(0);
        let tile_pass_rate = div()
            .id("home_pass_rate_stat")
            .cursor_pointer()
            .flex_1()
            .bg(tokens.surface)
            .border_1()
            .border_color(tokens.line)
            .rounded(px(20.0))
            .p(px(if is_desktop { 20.0 } else { 16.0 }))
            .hover(|el| el.border_color(colors.success))
            .on_click(cx.listener(move |this, _, window, cx| {
                on_action(this, HomeAction::OpenStats, window, cx);
            }))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .size(px(30.0))
                            .rounded(px(9.0))
                            .bg(colors.success.opacity(0.12))
                            .text_color(colors.success)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(IconName::Check)
                                    .size(px(16.0))
                                    .text_color(colors.success),
                            ),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(t("stats.pass_rate", lang)),
                    ),
            )
            .child(
                div()
                    .font_family(DISPLAY_FONT)
                    .font_extrabold()
                    .text_size(px(if is_desktop { 44.0 } else { 36.0 }))
                    .line_height(px(if is_desktop { 48.0 } else { 40.0 }))
                    .text_color(tokens.fg)
                    .my(px(if is_desktop { 16.0 } else { 12.0 }))
                    .child(pass_rate_str),
            )
            .child(
                div()
                    .h(px(6.0))
                    .rounded_full()
                    .bg(tokens.line2)
                    .overflow_hidden()
                    .w_full()
                    .child(
                        div()
                            .h_full()
                            .w(relative((pr_pct as f32) / 100.0))
                            .bg(colors.success),
                    ),
            );

        let stat_section = if is_desktop {
            div()
                .flex()
                .flex_row()
                .gap(px(18.0))
                .w_full()
                .child(tile_byose)
                .child(tile_results)
                .child(tile_pass_rate)
        } else {
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .w_full()
                .child(tile_byose)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(10.0))
                        .w_full()
                        .child(tile_results)
                        .child(tile_pass_rate),
                )
        };

        // 7. Mistakes Banner
        let weak_banner = div()
            .bg(tokens.bg)
            .border_1()
            .border_color(colors.danger.opacity(0.35))
            .rounded(px(20.0))
            .p(px(if is_desktop { 20.0 } else { 18.0 }))
            .w_full()
            .flex()
            .when(is_desktop, |el| el.flex_row().items_center().gap(px(20.0)))
            .when(!is_desktop, |el| {
                el.flex_col().items_stretch().gap(px(14.0))
            })
            .when(is_desktop, |el| {
                el.child(
                    div()
                        .size(px(48.0))
                        .rounded(px(14.0))
                        .border_1()
                        .border_color(colors.danger.opacity(0.4))
                        .bg(colors.danger.opacity(0.1))
                        .flex()
                        .items_center()
                        .justify_center()
                        .flex_none()
                        .child(
                            Icon::new(IconName::Target)
                                .size(px(24.0))
                                .text_color(colors.danger),
                        ),
                )
            })
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .font_family(DISPLAY_FONT)
                            .font_bold()
                            .text_size(px(17.0))
                            .line_height(px(22.0))
                            .text_color(tokens.fg)
                            .child(t("home.weak_title", lang)),
                    )
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .text_xs()
                            .text_color(tokens.muted)
                            .child(t("home.weak_desc", lang)),
                    ),
            )
            .child(
                div()
                    .id("start_weak_practice_btn")
                    .cursor_pointer()
                    .rounded_full()
                    .border_1()
                    .border_color(colors.danger.opacity(0.5))
                    .bg(tokens.bg)
                    .hover(|el| {
                        el.border_color(colors.danger)
                            .bg(colors.danger.opacity(0.1))
                    })
                    .px_5()
                    .py_3()
                    .flex()
                    .items_center()
                    .justify_center()
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action(this, HomeAction::PlayMistakes, window, cx);
                    }))
                    .child(
                        div()
                            .font_family(BODY_FONT)
                            .font_semibold()
                            .text_sm()
                            .text_color(colors.danger)
                            .child(t("home.weak_btn", lang)),
                    ),
            );

        // Footer
        let footer = div()
            .flex()
            .flex_row()
            .justify_between()
            .w_full()
            .pt(px(6.0))
            .text_xs()
            .font_family(BODY_FONT)
            .text_color(tokens.muted)
            .child(format!("{} Rwanda Bruno", t("about.built_by", lang)))
            .child(format!("v{}", env!("CARGO_PKG_VERSION")));

        // Layout container
        let content_grid = div()
            .w_full()
            .max_w(px(720.0))
            .mx_auto()
            .flex()
            .flex_col()
            .gap(px(18.0));

        let assembled = if is_two_col {
            // Desktop 2-column grid:
            // row 1: hero card (full width)
            // row 2: "Tangira ikizamini" panel (left) + question map "Aho ugeze" (right)
            // row 3: 3 stat tiles
            // row 4: "Ibibazo nakosheje kenshi" banner
            content_grid
                .children(banner)
                .when_some(update_banner, |el, banner| el.child(banner))
                .child(hero_card)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(18.0))
                        .w_full()
                        .items_start()
                        .child(div().flex_1().min_w_0().child(play_panel))
                        .child(div().flex_1().min_w_0().child(map_panel)),
                )
                .child(stat_section)
                .child(weak_banner)
                .child(footer)
        } else if is_mid {
            // Below 960px content width: collapse to single column
            // row 1: hero
            // row 2: play panel
            // row 3: map
            // row 4: stats
            // row 5: mistakes banner
            content_grid
                .children(banner)
                .when_some(update_banner, |el, banner| el.child(banner))
                .child(hero_card)
                .child(play_panel)
                .child(map_panel)
                .child(stat_section)
                .child(weak_banner)
                .child(footer)
        } else {
            // Mobile (<= 720px): single column order: hero, play panel, stats, map, mistakes banner
            content_grid
                .children(banner)
                .when_some(update_banner, |el, banner| el.child(banner))
                .child(hero_card)
                .child(play_panel)
                .child(stat_section)
                .child(map_panel)
                .child(weak_banner)
                .child(footer)
        };

        div().flex().flex_col().size_full().bg(tokens.bg).child(
            div()
                .id("home_scroll")
                .track_scroll(scroll.handle)
                .drag_scroll(scroll.handle)
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .page_padding(is_desktop)
                .child(vertical_scrollbar(
                    "home_scrollbar",
                    scroll.handle,
                    is_desktop,
                    scroll.reveal_on_open,
                ))
                .child(assembled),
        )
    }

    fn resume_banner<V: 'static>(
        detail: String,
        lang: Language,
        is_desktop: bool,
        tokens: HomeTokens,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
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
            .border_color(tokens.fg)
            .bg(tokens.surface)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .min_w_0()
                    .child(
                        div()
                            .text_base()
                            .font_family(DISPLAY_FONT)
                            .font_semibold()
                            .text_color(tokens.fg)
                            .child(t("resume.title", lang)),
                    )
                    .child(div().text_xs().text_color(tokens.muted).child(detail)),
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
                            .border_color(tokens.line)
                            .bg(tokens.surface2)
                            .cursor_pointer()
                            .hover(|el| el.border_color(tokens.fg))
                            .when(!is_desktop, |el| el.flex_1())
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_action(this, HomeAction::Discard, window, cx);
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .text_color(tokens.fg)
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
                            .bg(tokens.fg)
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
                                    .text_color(tokens.bg)
                                    .child(t("resume.go", lang)),
                            ),
                    ),
            )
    }

    fn update_banner<V: 'static>(
        version: &str,
        lang: Language,
        is_desktop: bool,
        tokens: HomeTokens,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, HomeAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let title = tf(
            "update.available",
            lang,
            &[("v", version.trim_start_matches('v'))],
        );
        let button = |id: &'static str,
                      label: &'static str,
                      action: HomeAction,
                      primary: bool,
                      cx: &mut Context<V>| {
            Button::new(id)
                .when(primary, |button| button.primary())
                .when(!primary, |button| button.outline())
                .label(label)
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_action(this, action, window, cx);
                }))
        };

        div()
            .flex()
            .when(is_desktop, |el| {
                el.flex_row().items_center().justify_between()
            })
            .when(!is_desktop, |el| el.flex_col().items_start())
            .gap_3()
            .p_4()
            .rounded_xl()
            .border_1()
            .border_color(tokens.line)
            .bg(tokens.surface)
            .child(
                div()
                    .min_w_0()
                    .font_family(DISPLAY_FONT)
                    .font_semibold()
                    .text_color(tokens.fg)
                    .child(title),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .when(!is_desktop, |el| el.w_full())
                    .child(button(
                        "update_whats_new",
                        t("update.whatsnew", lang),
                        HomeAction::UpdateWhatsNew,
                        false,
                        cx,
                    ))
                    .child(button(
                        "update_download",
                        t("update.download", lang),
                        HomeAction::UpdateDownload,
                        true,
                        cx,
                    ))
                    .child(button(
                        "update_later",
                        t("update.later", lang),
                        HomeAction::UpdateLater,
                        false,
                        cx,
                    ))
                    .child(button(
                        "update_skip",
                        t("update.skip", lang),
                        HomeAction::UpdateSkip,
                        false,
                        cx,
                    )),
            )
    }
}
