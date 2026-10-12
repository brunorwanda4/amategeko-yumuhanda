use crate::state::AppState;
use crate::ui::layout::{page_column, PagePadding as _, SwipeGuard};
use crate::ui::scroll::{vertical_scrollbar, DragScroll};
use ::gpui::InteractiveElement as _;
use amategeko_core::{t, Language, Question};
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{ActiveTheme, Icon, IconName, Sizable};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionsFilter {
    All,
    HasImage,
    Mistakes,
    Starred,
}

pub struct QuestionsView;

impl QuestionsView {
    fn render_empty_state<V: 'static>(
        lang: Language,
        search_input: &Entity<InputState>,
        cx: &mut Context<V>,
        on_set_filter: impl Fn(&mut V, QuestionsFilter, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let reset_input = search_input.clone();

        div()
            .flex_shrink_0()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .min_w_0()
            .p_8()
            .gap_3()
            .child(
                Icon::new(IconName::BookOpen)
                    .size(px(40.0))
                    .text_color(colors.muted_foreground),
            )
            .child(
                div()
                    .min_w_0()
                    .text_base()
                    .font_family(crate::font::DISPLAY_FONT_FAMILY)
                    .font_bold()
                    .text_color(colors.foreground)
                    .whitespace_normal()
                    .child(if lang == Language::En {
                        "No questions found"
                    } else {
                        "Nta kibazo kigaragaye"
                    }),
            )
            .child(
                div()
                    .min_w_0()
                    .text_sm()
                    .text_color(colors.muted_foreground)
                    .whitespace_normal()
                    .child(if lang == Language::En {
                        "Try changing your search terms or select 'All' to view all questions."
                    } else {
                        "Hindura amashakiro cyangwa kanda 'Byose' kugira ngo ubone ibibazo byose."
                    }),
            )
            .child(
                Button::new("reset_filter_btn")
                    .secondary()
                    .label(if lang == Language::En {
                        "Reset Filters"
                    } else {
                        "Kora Reset"
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_set_filter(this, QuestionsFilter::All, window, cx);
                        reset_input.update(cx, |input, cx| {
                            input.set_value("", window, cx);
                        });
                    })),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_question_card<V: 'static>(
        q: &Question,
        state: &AppState,
        is_desktop: bool,
        lang: Language,
        q_lang: Language,
        show_both: bool,
        shortcuts_active: bool,
        icon_button_size: f32,
        min_touch_h: f32,
        hide_answers: bool,
        expanded: &HashSet<u32>,
        revealed: &HashMap<u32, String>,
        cx: &mut Context<V>,
        on_toggle_star: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_expand: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_reveal_option: impl Fn(&mut V, u32, String, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let q_id = q.id;
        let is_starred = state.progress.is_starred(q_id);
        let is_expanded = expanded.contains(&q_id);
        let wrong_count = state
            .progress
            .question_stats
            .get(&q_id)
            .map(|s| s.wrong_count)
            .unwrap_or(0);

        let revealed_choice = revealed.get(&q_id).cloned();

        let opts = q.options_for(q_lang);
        let mut sorted_keys: Vec<&String> = opts.keys().collect();
        sorted_keys.sort();

        div()
            .id(format!("q_card_{}", q_id))
            .debug_selector(move || format!("q_card_{}", q_id))
            .flex_shrink_0()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .overflow_hidden()
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .p_3p5()
            .gap_3()
            // Question Card Header / Summary Row.
            // Desktop: single row (number+text left, actions right).
            // Mobile:  two rows  (number+text full width, then actions).
            .child(
                div()
                    .id(format!("q_header_{}", q_id))
                    .flex()
                    .when(is_desktop, |el| {
                        el.flex_row().items_center().justify_between()
                    })
                    .when(!is_desktop, |el| el.flex_col().items_start())
                    .gap_3()
                    .w_full()
                    .min_w_0()
                    .overflow_hidden()
                    .cursor_pointer()
                    .when(shortcuts_active, |el| {
                        let hint = if is_expanded {
                            "Collapse (Enter)"
                        } else {
                            "Expand (Enter)"
                        };
                        el.tooltip(move |window, cx| Tooltip::new(hint).build(window, cx))
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_toggle_expand(this, q_id, window, cx);
                    }))
                    // Left (desktop) / Top (mobile): Question Number + Text
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_baseline()
                            .gap_3()
                            .when(is_desktop, |el| el.flex_1().min_w_0())
                            .when(!is_desktop, |el| el.w_full().min_w_0())
                            .overflow_hidden()
                            .child(
                                div()
                                    .flex_none()
                                    .text_sm()
                                    .text_color(colors.muted_foreground)
                                    .child(format!("{}.", q_id)),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .overflow_hidden()
                                    .whitespace_normal()
                                    .text_sm()
                                    .font_normal()
                                    .text_color(colors.foreground)
                                    .child(q.text_for(q_lang).to_string()),
                            ),
                    )
                    // Right (desktop) / Bottom (mobile): Badges, Correct Letter,
                    // Star, Chevron.
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .flex_wrap()
                            .gap_2p5()
                            .when(is_desktop, |el| {
                                el.flex_shrink_0().flex_none().justify_end()
                            })
                            .when(!is_desktop, |el| el.w_full().justify_end())
                            // Image badge (blue pill)
                            .when(q.has_image, |b| {
                                b.child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_md()
                                        .bg(::gpui::Hsla {
                                            h: 215.0 / 360.0,
                                            s: 0.70,
                                            l: 0.16,
                                            a: 1.0,
                                        })
                                        .text_xs()
                                        .font_medium()
                                        .text_color(::gpui::Hsla {
                                            h: 215.0 / 360.0,
                                            s: 0.90,
                                            l: 0.65,
                                            a: 1.0,
                                        })
                                        .child(Icon::new(IconName::Frame).size(px(13.0)))
                                        .child(t("badge.image", lang)),
                                )
                            })
                            // Missed badge (dark red pill)
                            .when(wrong_count > 0, |b| {
                                b.child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_md()
                                        .bg(::gpui::Hsla {
                                            h: 0.0,
                                            s: 0.55,
                                            l: 0.18,
                                            a: 1.0,
                                        })
                                        .text_xs()
                                        .font_medium()
                                        .text_color(::gpui::Hsla {
                                            h: 0.0,
                                            s: 0.85,
                                            l: 0.65,
                                            a: 1.0,
                                        })
                                        .child(
                                            t("questions.missed_count", lang)
                                                .replace("{count}", &wrong_count.to_string()),
                                        ),
                                )
                            })
                            // Correct Answer Letter (shown when collapsed and not hiding answers)
                            .when(!is_expanded && !hide_answers, |el| {
                                el.child(
                                    div()
                                        .text_sm()
                                        .font_medium()
                                        .text_color(colors.success)
                                        .child(format!("{})", q.correct)),
                                )
                            })
                            // Star Button
                            .child(
                                div()
                                    .id(format!("star_q_{}", q_id))
                                    .w(px(icon_button_size))
                                    .h(px(icon_button_size))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .flex_none()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(if is_starred {
                                        colors.accent
                                    } else {
                                        colors.secondary
                                    })
                                    .cursor_pointer()
                                    .hover(|el| el.bg(colors.accent))
                                    .when(shortcuts_active, |el| {
                                        let hint =
                                            if is_starred { "Unstar (K)" } else { "Star (K)" };
                                        el.tooltip(move |window, cx| {
                                            Tooltip::new(hint).build(window, cx)
                                        })
                                    })
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_toggle_star(this, q_id, window, cx);
                                    }))
                                    .child(
                                        Icon::new(if is_starred {
                                            IconName::StarFill
                                        } else {
                                            IconName::Star
                                        })
                                        .size(px(15.0))
                                        .text_color(
                                            if is_starred {
                                                colors.warning
                                            } else {
                                                colors.muted_foreground
                                            },
                                        ),
                                    ),
                            )
                            // Expand / Collapse Chevron
                            .child(
                                div()
                                    .w(px(20.0))
                                    .h(px(icon_button_size))
                                    .flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(if is_expanded {
                                            IconName::ChevronUp
                                        } else {
                                            IconName::ChevronDown
                                        })
                                        .size(px(16.0))
                                        .text_color(colors.muted_foreground),
                                    ),
                            ),
                    ),
            )
            // Expanded Card Details
            .when(is_expanded, |body| {
                body.child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .min_w_0()
                        .overflow_hidden()
                        .gap_3()
                        .pt_1()
                        // Sign Image Container with dashed border
                        .when(q.has_image, |img_box| {
                            img_box.child(
                                div()
                                    .w_full()
                                    .min_w_0()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .p_3()
                                    .rounded_lg()
                                    .border_1()
                                    .border_dashed()
                                    .border_color(colors.border)
                                    .bg(colors.background)
                                    .gap_2()
                                    .overflow_hidden()
                                    .child(
                                        img(format!("assets/images/q{}.png", q_id))
                                            .max_w_full()
                                            .max_h(px(140.0))
                                            .rounded_lg(),
                                    ),
                            )
                        })
                        // Bilingual secondary text (if enabled)
                        .when(show_both, |bilingual| {
                            if let Some(sec_text) = q.secondary_text_for(q_lang) {
                                bilingual.child(
                                    div()
                                        .w_full()
                                        .min_w_0()
                                        .overflow_hidden()
                                        .whitespace_normal()
                                        .text_xs()
                                        .text_color(colors.muted_foreground)
                                        .child(sec_text.to_string()),
                                )
                            } else {
                                bilingual
                            }
                        })
                        // Answer Options List
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .w_full()
                                .min_w_0()
                                .overflow_hidden()
                                .gap_2()
                                .children(sorted_keys.into_iter().map(|key| {
                                    let opt_text = opts.get(key).cloned().unwrap_or_default();
                                    let is_correct_key = q.correct == *key;
                                    let opt_letter = key.clone();

                                    let (border_c, bg_c, text_c, icon_opt) = if !hide_answers {
                                        if is_correct_key {
                                            (
                                                colors.success,
                                                ::gpui::Hsla {
                                                    h: 130.0 / 360.0,
                                                    s: 0.50,
                                                    l: 0.10,
                                                    a: 1.0,
                                                },
                                                colors.success,
                                                Some(IconName::Check),
                                            )
                                        } else {
                                            (
                                                colors.border,
                                                colors.background,
                                                colors.foreground,
                                                None,
                                            )
                                        }
                                    } else if let Some(chosen) = &revealed_choice {
                                        if chosen == key {
                                            if is_correct_key {
                                                (
                                                    colors.success,
                                                    ::gpui::Hsla {
                                                        h: 130.0 / 360.0,
                                                        s: 0.50,
                                                        l: 0.10,
                                                        a: 1.0,
                                                    },
                                                    colors.success,
                                                    Some(IconName::Check),
                                                )
                                            } else {
                                                (
                                                    colors.danger,
                                                    ::gpui::Hsla {
                                                        h: 0.0,
                                                        s: 0.50,
                                                        l: 0.12,
                                                        a: 1.0,
                                                    },
                                                    colors.danger,
                                                    Some(IconName::CircleX),
                                                )
                                            }
                                        } else if is_correct_key {
                                            (
                                                colors.success,
                                                ::gpui::Hsla {
                                                    h: 130.0 / 360.0,
                                                    s: 0.50,
                                                    l: 0.10,
                                                    a: 1.0,
                                                },
                                                colors.success,
                                                Some(IconName::Check),
                                            )
                                        } else {
                                            (
                                                colors.border,
                                                colors.background,
                                                colors.foreground,
                                                None,
                                            )
                                        }
                                    } else {
                                        (colors.border, colors.background, colors.foreground, None)
                                    };

                                    div()
                                        .id(format!("q_{}_opt_{}", q_id, key))
                                        .flex()
                                        .flex_row()
                                        .items_start()
                                        .w_full()
                                        .min_w_0()
                                        .when(!is_desktop, |el| el.min_h(px(min_touch_h)))
                                        .gap_2p5()
                                        .p_3()
                                        .rounded_lg()
                                        .border_1()
                                        .border_color(border_c)
                                        .bg(bg_c)
                                        .cursor_pointer()
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            if hide_answers {
                                                on_reveal_option(
                                                    this,
                                                    q_id,
                                                    opt_letter.clone(),
                                                    window,
                                                    cx,
                                                );
                                            }
                                        }))
                                        .child(
                                            div()
                                                .size(px(28.0))
                                                .flex_none()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded_md()
                                                .border_1()
                                                .border_color(border_c)
                                                .bg(colors.background)
                                                .text_sm()
                                                .font_semibold()
                                                .text_color(text_c)
                                                .child(format!("{})", key)),
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .min_w_0()
                                                .overflow_hidden()
                                                .whitespace_normal()
                                                .text_sm()
                                                .text_color(text_c)
                                                .child(opt_text),
                                        )
                                        .when_some(icon_opt, |row, ic| {
                                            row.child(
                                                Icon::new(ic)
                                                    .size(px(16.0))
                                                    .flex_none()
                                                    .text_color(text_c),
                                            )
                                        })
                                })),
                        ),
                )
            })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        scroll_handle: &::gpui::ScrollHandle,
        reveal_scrollbar: bool,
        filter: QuestionsFilter,
        search_query: &str,
        search_input: &Entity<InputState>,
        hide_answers: bool,
        expanded: &HashSet<u32>,
        revealed: &HashMap<u32, String>,
        show_filters_sheet: bool,
        cx: &mut Context<V>,
        on_set_filter: impl Fn(&mut V, QuestionsFilter, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_hide_answers: impl Fn(&mut V, bool, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_filters_sheet: impl Fn(&mut V, bool, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_star: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_toggle_expand: impl Fn(&mut V, u32, &mut Window, &mut Context<V>) + 'static + Copy,
        on_reveal_option: impl Fn(&mut V, u32, String, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let lang = state.settings.language;
        let q_lang = state.settings.question_language;
        let show_both = state.settings.show_both_languages;
        let shortcuts_active =
            crate::shortcuts_ui_active(is_desktop, state.settings.desktop_shortcuts_enabled);

        // Touch-target sizing. On desktop the original compact controls are
        // preserved; on compact/mobile layouts we guarantee a 44px
        // interactive height for every touch target.
        let icon_button_size: f32 = if is_desktop { 32.0 } else { 44.0 };
        let min_touch_h: f32 = if is_desktop { 0.0 } else { 44.0 };

        let has_image_only = filter == QuestionsFilter::HasImage;
        let starred_only = filter == QuestionsFilter::Starred;
        let mistakes_only = filter == QuestionsFilter::Mistakes;

        let filtered_questions: Vec<&Question> = state.bank.search_for_lang(
            search_query,
            has_image_only,
            starred_only,
            mistakes_only,
            &state.progress.starred_questions,
            &state.progress.question_stats,
            q_lang,
        );

        let total_questions = state.bank.len();
        let visible_count = filtered_questions.len();

        let filter_buttons = [
            (
                QuestionsFilter::All,
                "filter_all",
                t("questions.filter_all", lang),
            ),
            (
                QuestionsFilter::HasImage,
                "filter_image",
                t("questions.filter_image", lang),
            ),
            (
                QuestionsFilter::Mistakes,
                "filter_mistakes",
                t("questions.filter_mistakes", lang),
            ),
            (
                QuestionsFilter::Starred,
                "filter_starred",
                t("questions.filter_starred", lang),
            ),
        ];

        if is_desktop {
            // Desktop layout (100% unchanged from original)
            div()
                .id("questions_view_root")
                .flex()
                .flex_col()
                .size_full()
                .min_w_0()
                .min_h_0()
                .overflow_hidden()
                .bg(colors.background)
                .page_padding(is_desktop)
                .child(
                    page_column()
                        .flex_1()
                        .min_w_0()
                        .min_h_0()
                        .gap_3p5()
                        // Top Bar: page title and a count computed from the loaded bank.
                        // Wraps on narrow widths so the counter drops below the title.
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .items_center()
                                .justify_between()
                                .gap_2()
                                .w_full()
                                .min_w_0()
                                .flex_shrink_0()
                                .child(
                                    div()
                                        .min_w_0()
                                        .text_2xl()
                                        .font_family(crate::font::DISPLAY_FONT_FAMILY)
                                        .font_bold()
                                        .text_color(colors.foreground)
                                        .whitespace_normal()
                                        .child(t("nav.questions", lang)),
                                )
                                .child(
                                    div()
                                        .min_w_0()
                                        .text_sm()
                                        .text_color(colors.muted_foreground)
                                        .whitespace_normal()
                                        .child(
                                            t("questions.count_info", lang)
                                                .replace("{visible}", &visible_count.to_string())
                                                .replace("{total}", &total_questions.to_string()),
                                        ),
                                ),
                        )
                        // Search Input Row
                        .child({
                            let focus_input = search_input.clone();
                            div()
                                .w_full()
                                .min_w_0()
                                .flex_shrink_0()
                                .cursor_text()
                                .capture_any_mouse_down(move |_, window, cx| {
                                    SwipeGuard::claim();
                                    focus_input.update(cx, |input, cx| input.focus(window, cx));
                                })
                                .child(
                                    Input::new(search_input)
                                        .id("questions_search_input")
                                        .large()
                                        .w_full()
                                        .min_w_0()
                                        .aria_label(t("questions.search_placeholder", lang))
                                        .prefix(
                                            Icon::new(IconName::Search)
                                                .size(px(16.0))
                                                .text_color(colors.muted_foreground),
                                        )
                                        .cleanable(true),
                                )
                        })
                        // Filter Buttons Row (Byose, Ibifite ishushyo, Ibyo nakosheje, Inyenyeri)
                        // Wraps cleanly when there isn't enough horizontal room.
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .items_center()
                                .gap_2p5()
                                .w_full()
                                .min_w_0()
                                .flex_shrink_0()
                                .children(filter_buttons.into_iter().map(
                                    |(item_filter, id, label)| {
                                        let is_active = filter == item_filter;
                                        div()
                                            .id(id)
                                            .px_4()
                                            .py_2()
                                            .when(!is_desktop, |el| el.min_h(px(min_touch_h)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded_lg()
                                            .cursor_pointer()
                                            .border_1()
                                            .border_color(if is_active {
                                                colors.muted_foreground
                                            } else {
                                                colors.border
                                            })
                                            .bg(if is_active {
                                                colors.secondary
                                            } else {
                                                colors.background
                                            })
                                            .text_sm()
                                            .font_medium()
                                            .whitespace_normal()
                                            .text_color(if is_active {
                                                colors.foreground
                                            } else {
                                                colors.muted_foreground
                                            })
                                            .hover(move |el| {
                                                if !is_active {
                                                    el.border_color(colors.muted_foreground)
                                                        .text_color(colors.foreground)
                                                } else {
                                                    el
                                                }
                                            })
                                            .child(label)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_set_filter(this, item_filter, window, cx);
                                            }))
                                    },
                                )),
                        )
                        // Flashcard Toggle Row: "Hisha ibisubizo" + Switch
                        .child(
                            div()
                                .id("hide_answers_toggle_row")
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .items_center()
                                .gap_3()
                                .w_full()
                                .min_w_0()
                                .flex_shrink_0()
                                .when(!is_desktop, |el| el.min_h(px(min_touch_h)))
                                .when(shortcuts_active, |el| {
                                    let hint = format!("{} (H)", t("questions.hide_answers", lang));
                                    el.tooltip(move |window, cx| {
                                        Tooltip::new(hint.clone()).build(window, cx)
                                    })
                                })
                                .child(
                                    div()
                                        .min_w_0()
                                        .text_sm()
                                        .text_color(colors.foreground)
                                        .whitespace_normal()
                                        .child(t("questions.hide_answers", lang)),
                                )
                                .child(
                                    Switch::new("hide_answers_switch")
                                        .checked(hide_answers)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_toggle_hide_answers(this, !hide_answers, window, cx);
                                        })),
                                ),
                        )
                        // Scrollable Question Cards List
                        .child(
                            div()
                                .id("questions_scroll_view")
                                .debug_selector(|| "questions_scroll_view".into())
                                .track_scroll(scroll_handle)
                                .drag_scroll(scroll_handle)
                                .relative()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .min_w_0()
                                .min_h_0()
                                .overflow_y_scroll()
                                .overflow_x_hidden()
                                .pr_2()
                                .gap_2p5()
                                .child(vertical_scrollbar(
                                    "questions_scrollbar",
                                    scroll_handle,
                                    is_desktop,
                                    reveal_scrollbar,
                                ))
                                // Empty state when search or filter returns no questions
                                .when(filtered_questions.is_empty(), |el| {
                                    el.child(Self::render_empty_state(
                                        lang,
                                        search_input,
                                        cx,
                                        on_set_filter,
                                    ))
                                })
                                // Render Question Cards
                                .children(filtered_questions.into_iter().map(|q| {
                                    Self::render_question_card(
                                        q,
                                        state,
                                        is_desktop,
                                        lang,
                                        q_lang,
                                        show_both,
                                        shortcuts_active,
                                        icon_button_size,
                                        min_touch_h,
                                        hide_answers,
                                        expanded,
                                        revealed,
                                        cx,
                                        on_toggle_star,
                                        on_toggle_expand,
                                        on_reveal_option,
                                    )
                                })),
                        ),
                )
        } else {
            // Mobile layout (Phones only)
            let active_filter_count = (if filter != QuestionsFilter::All { 1 } else { 0 })
                + (if hide_answers { 1 } else { 0 });

            div()
                .id("questions_view_root")
                .relative()
                .flex()
                .flex_col()
                .size_full()
                .min_w_0()
                .min_h_0()
                .overflow_hidden()
                .bg(colors.background)
                // 1. Pinned compact search row (~48px)
                .child(
                    div()
                        .id("mobile_pinned_search_row")
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .w_full()
                        .min_w_0()
                        .px_3()
                        .py_1p5()
                        .border_b_1()
                        .border_color(colors.border)
                        .bg(colors.background)
                        .child({
                            let focus_input = search_input.clone();
                            div()
                                .flex_1()
                                .min_w_0()
                                .cursor_text()
                                .capture_any_mouse_down(move |_, window, cx| {
                                    SwipeGuard::claim();
                                    focus_input.update(cx, |input, cx| input.focus(window, cx));
                                })
                                .child(
                                    Input::new(search_input)
                                        .id("mobile_questions_search_input")
                                        .large()
                                        .w_full()
                                        .min_w_0()
                                        .aria_label(t("questions.search_placeholder", lang))
                                        .prefix(
                                            Icon::new(IconName::Search)
                                                .size(px(16.0))
                                                .text_color(colors.muted_foreground),
                                        )
                                        .cleanable(true),
                                )
                        })
                        // Filters icon button with active filter badge
                        .child(
                            div()
                                .id("mobile_filters_btn")
                                .size(px(44.0))
                                .min_w(px(44.0))
                                .min_h(px(44.0))
                                .flex_none()
                                .relative()
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_lg()
                                .border_1()
                                .border_color(if active_filter_count > 0 {
                                    colors.foreground
                                } else {
                                    colors.border
                                })
                                .bg(if active_filter_count > 0 {
                                    colors.secondary
                                } else {
                                    colors.background
                                })
                                .cursor_pointer()
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_toggle_filters_sheet(this, true, window, cx);
                                }))
                                .child(Icon::new(IconName::Settings2).size(px(20.0)).text_color(
                                    if active_filter_count > 0 {
                                        colors.foreground
                                    } else {
                                        colors.muted_foreground
                                    },
                                ))
                                .when(active_filter_count > 0, |el| {
                                    el.child(
                                        div()
                                            .absolute()
                                            .top(px(-3.0))
                                            .right(px(-3.0))
                                            .size(px(18.0))
                                            .rounded_full()
                                            .bg(colors.primary)
                                            .text_color(colors.background)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_xs()
                                            .font_bold()
                                            .child(active_filter_count.to_string()),
                                    )
                                }),
                        ),
                )
                // 2. Single scroll area for the whole content (title + counter, chips + switch, and question list)
                .child(
                    div()
                        .id("questions_scroll_view")
                        .debug_selector(|| "questions_scroll_view".into())
                        .track_scroll(scroll_handle)
                        .drag_scroll(scroll_handle)
                        .relative()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .min_h_0()
                        .overflow_y_scroll()
                        .overflow_x_hidden()
                        .px_3()
                        .pb_6()
                        .gap_3()
                        .child(vertical_scrollbar(
                            "questions_scrollbar",
                            scroll_handle,
                            is_desktop,
                            reveal_scrollbar,
                        ))
                        // Title + counter (scrolls away with the list)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .w_full()
                                .min_w_0()
                                .pt_2()
                                .child(
                                    div()
                                        .min_w_0()
                                        .text_2xl()
                                        .font_family(crate::font::DISPLAY_FONT_FAMILY)
                                        .font_bold()
                                        .text_color(colors.foreground)
                                        .whitespace_normal()
                                        .child(t("nav.questions", lang)),
                                )
                                .child(
                                    div()
                                        .min_w_0()
                                        .text_sm()
                                        .text_color(colors.muted_foreground)
                                        .whitespace_normal()
                                        .child(
                                            t("questions.count_info", lang)
                                                .replace("{visible}", &visible_count.to_string())
                                                .replace("{total}", &total_questions.to_string()),
                                        ),
                                ),
                        )
                        // Horizontal scrolling chip row + Hide answers switch (hidden while Filters sheet is open)
                        .when(!show_filters_sheet, |parent| {
                            parent.child(
                                div()
                                    .id("mobile_filters_horizontal_scroll")
                                    .capture_any_mouse_down(|_, _, _| {
                                        SwipeGuard::claim();
                                    })
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .w_full()
                                    .min_w_0()
                                    .overflow_x_scroll()
                                    .pb_1()
                                    .children(filter_buttons.into_iter().map(
                                        |(item_filter, id, label)| {
                                            let is_active = filter == item_filter;
                                            div()
                                                .id(id)
                                                .px_4()
                                                .h(px(44.0))
                                                .flex_none()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded_lg()
                                                .cursor_pointer()
                                                .border_1()
                                                .border_color(if is_active {
                                                    colors.muted_foreground
                                                } else {
                                                    colors.border
                                                })
                                                .bg(if is_active {
                                                    colors.secondary
                                                } else {
                                                    colors.background
                                                })
                                                .text_sm()
                                                .font_medium()
                                                .whitespace_nowrap()
                                                .text_color(if is_active {
                                                    colors.foreground
                                                } else {
                                                    colors.muted_foreground
                                                })
                                                .child(label)
                                                .on_click(cx.listener(
                                                    move |this, _, window, cx| {
                                                        on_set_filter(
                                                            this,
                                                            item_filter,
                                                            window,
                                                            cx,
                                                        );
                                                    },
                                                ))
                                        },
                                    ))
                                    .child(
                                        div()
                                            .id("mobile_hide_answers_horizontal")
                                            .h(px(44.0))
                                            .px_3()
                                            .flex_none()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_2()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(colors.border)
                                            .bg(colors.background)
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_medium()
                                                    .whitespace_nowrap()
                                                    .text_color(colors.foreground)
                                                    .child(t("questions.hide_answers", lang)),
                                            )
                                            .child(
                                                Switch::new("mobile_hide_answers_switch")
                                                    .checked(hide_answers)
                                                    .on_click(cx.listener(
                                                        move |this, _, window, cx| {
                                                            on_toggle_hide_answers(
                                                                this,
                                                                !hide_answers,
                                                                window,
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    ),
                            )
                        })
                        // Empty state
                        .when(filtered_questions.is_empty(), |el| {
                            el.child(Self::render_empty_state(
                                lang,
                                search_input,
                                cx,
                                on_set_filter,
                            ))
                        })
                        // Question cards
                        .children(filtered_questions.into_iter().map(|q| {
                            Self::render_question_card(
                                q,
                                state,
                                is_desktop,
                                lang,
                                q_lang,
                                show_both,
                                shortcuts_active,
                                icon_button_size,
                                min_touch_h,
                                hide_answers,
                                expanded,
                                revealed,
                                cx,
                                on_toggle_star,
                                on_toggle_expand,
                                on_reveal_option,
                            )
                        })),
                )
                // 3. Filters bottom sheet
                .when(show_filters_sheet, |root| {
                    root.child(
                        div()
                            .id("questions_filters_backdrop")
                            .absolute()
                            .inset_0()
                            .flex()
                            .flex_col()
                            .justify_end()
                            .bg(::gpui::Rgba {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.65,
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_toggle_filters_sheet(this, false, window, cx);
                            }))
                            .child(
                                div()
                                    .id("questions_filters_sheet")
                                    .w_full()
                                    .max_h(px(480.0))
                                    .overflow_y_scroll()
                                    .p_5()
                                    .gap_4()
                                    .rounded_t_2xl()
                                    .border_t_1()
                                    .border_color(colors.border)
                                    .bg(colors.background)
                                    .on_mouse_down(::gpui::MouseButton::Left, |_, _, _| {})
                                    .flex()
                                    .flex_col()
                                    // Sheet Header
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .w_full()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_family(crate::font::DISPLAY_FONT_FAMILY)
                                                    .font_bold()
                                                    .text_color(colors.foreground)
                                                    .child(t("questions.filters", lang)),
                                            )
                                            .child(
                                                div()
                                                    .id("close_filters_sheet_btn")
                                                    .size(px(44.0))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .rounded_md()
                                                    .cursor_pointer()
                                                    .child(
                                                        Icon::new(IconName::CircleX)
                                                            .size(px(20.0))
                                                            .text_color(colors.muted_foreground),
                                                    )
                                                    .on_click(cx.listener(
                                                        move |this, _, window, cx| {
                                                            on_toggle_filters_sheet(
                                                                this, false, window, cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    )
                                    // Filter Chips
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .flex_wrap()
                                            .gap_2p5()
                                            .w_full()
                                            .children(filter_buttons.into_iter().map(
                                                |(item_filter, id, label)| {
                                                    let is_active = filter == item_filter;
                                                    div()
                                                        .id(format!("{id}_sheet"))
                                                        .min_h(px(44.0))
                                                        .px_4()
                                                        .py_2()
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .rounded_lg()
                                                        .cursor_pointer()
                                                        .border_1()
                                                        .border_color(if is_active {
                                                            colors.muted_foreground
                                                        } else {
                                                            colors.border
                                                        })
                                                        .bg(if is_active {
                                                            colors.secondary
                                                        } else {
                                                            colors.background
                                                        })
                                                        .text_sm()
                                                        .font_medium()
                                                        .text_color(if is_active {
                                                            colors.foreground
                                                        } else {
                                                            colors.muted_foreground
                                                        })
                                                        .child(label)
                                                        .on_click(cx.listener(
                                                            move |this, _, window, cx| {
                                                                on_set_filter(
                                                                    this,
                                                                    item_filter,
                                                                    window,
                                                                    cx,
                                                                );
                                                            },
                                                        ))
                                                },
                                            )),
                                    )
                                    // "Hide answers" switch row
                                    .child(
                                        div()
                                            .min_h(px(44.0))
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .py_2()
                                            .border_t_1()
                                            .border_color(colors.border)
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(colors.foreground)
                                                    .child(t("questions.hide_answers", lang)),
                                            )
                                            .child(
                                                Switch::new("sheet_hide_answers_switch")
                                                    .checked(hide_answers)
                                                    .on_click(cx.listener(
                                                        move |this, _, window, cx| {
                                                            on_toggle_hide_answers(
                                                                this,
                                                                !hide_answers,
                                                                window,
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    )
                                    // Reset and Done actions
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .gap_3()
                                            .pt_2()
                                            .border_t_1()
                                            .border_color(colors.border)
                                            .child(
                                                div().flex_1().min_h(px(44.0)).child(
                                                    Button::new("sheet_reset_filters_btn")
                                                        .secondary()
                                                        .w_full()
                                                        .label(t("questions.reset", lang))
                                                        .on_click(cx.listener(
                                                            move |this, _, window, cx| {
                                                                on_set_filter(
                                                                    this,
                                                                    QuestionsFilter::All,
                                                                    window,
                                                                    cx,
                                                                );
                                                                if hide_answers {
                                                                    on_toggle_hide_answers(
                                                                        this, false, window, cx,
                                                                    );
                                                                }
                                                            },
                                                        )),
                                                ),
                                            )
                                            .child(
                                                div().flex_1().min_h(px(44.0)).child(
                                                    Button::new("sheet_done_filters_btn")
                                                        .primary()
                                                        .w_full()
                                                        .label(t("questions.done", lang))
                                                        .on_click(cx.listener(
                                                            move |this, _, window, cx| {
                                                                on_toggle_filters_sheet(
                                                                    this, false, window, cx,
                                                                );
                                                            },
                                                        )),
                                                ),
                                            ),
                                    ),
                            ),
                    )
                })
        }
    }
}
