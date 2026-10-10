use crate::state::AppState;
use crate::ui::footer::AppFooter;
use crate::ui::layout::{page_column, page_max_width, PagePadding as _};
use crate::ui::scroll::{vertical_scrollbar, DragScroll};
use amategeko_core::{t, tf, Language, ThemeMode};
use gpui::InteractiveElement as _;
use gpui_kit::base::Disableable as _;
use gpui_kit::base::Link;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::slider::{Slider, SliderEvent, SliderState};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::theme::ThemeColor;
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsAction {
    DecPassMark,
    IncPassMark,
    DecMediumTime,
    IncMediumTime,
    DecHardTime,
    IncHardTime,
    ToggleEasyTimer,
    ToggleHardWeightImages,
    ToggleDesktopShortcuts,
    ToggleShowAllAnswersAtEnd,
    SetTheme(ThemeMode),
    DecFontScale,
    IncFontScale,
    RequestClearHistory(bool),
    ConfirmClearHistory,
    ResetDefaults,
    SaveSettings,
    SetInterfaceLanguage(Language),
    SetQuestionLanguage(Language),
    ToggleShowBothLanguages,
    ToggleShuffleOptions,
    ToggleAutoUpdates,
    CheckUpdates,
}

pub struct SettingsView;

struct SliderControlState {
    slider: Entity<SliderState>,
    current_value: std::rc::Rc<std::cell::Cell<u32>>,
    subscription: Option<Subscription>,
}

impl SettingsView {
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        scroll_handle: &ScrollHandle,
        reveal_scrollbar: bool,
        confirm_clear: bool,
        window: &mut Window,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, SettingsAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let s = &state.settings;
        let lang = s.language;
        let update_status = crate::updater::status();
        let update_message = match &update_status {
            crate::updater::UpdateStatus::UpToDate => Some(t("update.uptodate", lang).to_string()),
            crate::updater::UpdateStatus::Available(update)
            | crate::updater::UpdateStatus::Downloading { update, .. }
            | crate::updater::UpdateStatus::Ready { update, .. }
            | crate::updater::UpdateStatus::PermissionRequired { update, .. } => Some(tf(
                "update.available",
                lang,
                &[("v", update.release.tag_name.trim_start_matches('v'))],
            )),
            crate::updater::UpdateStatus::Error(crate::updater::UpdateError::BadSignature) => {
                Some(t("update.badsig", lang).to_string())
            }
            crate::updater::UpdateStatus::Error(_) => Some(t("update.failed", lang).to_string()),
            _ => None,
        };

        div()
            .id("settings_scroll_view")
            .track_scroll(scroll_handle)
.drag_scroll(scroll_handle)
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .page_padding(is_desktop)
            .child(vertical_scrollbar(
                "settings_scrollbar",
                scroll_handle,
                is_desktop,
                reveal_scrollbar,
            ))
            .child(
                page_column()
                    .gap_6()
                    // Page Title Header
                    .child(
                        div()
                            .text_2xl()
                            .font_family(crate::font::DISPLAY_FONT_FAMILY)
                            .font_bold()
                            .text_color(colors.foreground)
                            .child(t("settings.title", lang)),
                    )
                    // SECTION 1: Language / Ururimi
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.muted_foreground)
                                    .child(t("settings.group_language", lang)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    // Interface Language
                                    .child(
                                        div()
                                            .flex()
                                            .when(is_desktop, |el| {
                                                    el.flex_row().items_center().justify_between()
                                                })
                                                .when(!is_desktop, |el| {
                                                    el.flex_col().items_start().gap_3()
                                                })
                                            .p_4()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_0p5()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_semibold()
                                                            .text_color(colors.foreground)
                                                            .child(t("settings.interface_language", lang)),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(colors.muted_foreground)
                                                            .child(t("settings.interface_language_desc", lang)),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .p_1()
                                                    .gap_1()
                                                    .rounded_xl()
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .bg(colors.background)
                                                    .child(Self::render_theme_tab(
                                                        "lang_en",
                                                        "English",
                                                        s.language == Language::En,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetInterfaceLanguage(Language::En), window, cx);
                                                        },
                                                    ))
                                                    .child(Self::render_theme_tab(
                                                        "lang_rw",
                                                        "Ikinyarwanda",
                                                        s.language == Language::Rw,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetInterfaceLanguage(Language::Rw), window, cx);
                                                        },
                                                    )),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Question Language
                                    .child(
                                        div()
                                            .flex()
                                            .when(is_desktop, |el| {
                                                el.flex_row().items_center().justify_between()
                                            })
                                            .when(!is_desktop, |el| {
                                                el.flex_col().items_start().gap_3()
                                            })
                                            .p_4()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_0p5()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_semibold()
                                                            .text_color(colors.foreground)
                                                            .child(t("settings.question_language", lang)),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(colors.muted_foreground)
                                                            .child(t("settings.question_language_desc", lang)),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .p_1()
                                                    .gap_1()
                                                    .rounded_xl()
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .bg(colors.background)
                                                    .child(Self::render_theme_tab(
                                                        "qlang_en",
                                                        "English",
                                                        s.question_language == Language::En,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetQuestionLanguage(Language::En), window, cx);
                                                        },
                                                    ))
                                                    .child(Self::render_theme_tab(
                                                        "qlang_rw",
                                                        "Ikinyarwanda",
                                                        s.question_language == Language::Rw,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetQuestionLanguage(Language::Rw), window, cx);
                                                        },
                                                    )),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Show both languages switch
                                    .child(Self::render_switch_row(
                                        t("settings.show_both_languages", lang),
                                        t("settings.show_both_languages_desc", lang),
                                        "sw_show_both_languages",
                                        s.show_both_languages,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleShowBothLanguages, window, cx);
                                        },
                                    ))
                                    .when(cfg!(target_arch = "wasm32"), |el| {
                                        el.child(Self::render_divider(&colors)).child(
                                            div()
                                                .p_4()
                                                .text_xs()
                                                .text_color(colors.muted_foreground)
                                                .child(t("web.storage_note", lang)),
                                        )
                                    }),
                            ),
                    )
                    // SECTION 2: Quiz Rules & Timing
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.muted_foreground)
                                    .child(t("settings.group_quiz", lang)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    // Row 1: Medium Time
                                    .child(Self::render_slider_row(
                                        t("settings.medium_time", lang),
                                        t("settings.medium_time_desc", lang),
                                        Self::render_slider(
                                            "slider_medium",
                                            s.medium_duration_mins,
                                            10,
                                            30,
                                            "min",
                                            &colors,
                                            window,
                                            cx,
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::DecMediumTime, window, cx);
                                            },
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::IncMediumTime, window, cx);
                                            },
                                        ),
                                        is_desktop,
                                    ))
                                    .child(Self::render_divider(&colors))
                                    // Row 2: Hard Time
                                    .child(Self::render_slider_row(
                                        t("settings.hard_time", lang),
                                        t("settings.hard_time_desc", lang),
                                        Self::render_slider(
                                            "slider_hard",
                                            s.hard_duration_mins,
                                            5,
                                            20,
                                            "min",
                                            &colors,
                                            window,
                                            cx,
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::DecHardTime, window, cx);
                                            },
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::IncHardTime, window, cx);
                                            },
                                        ),
                                        is_desktop,
                                    ))
                                    .child(Self::render_divider(&colors))
                                    // Row 3: Pass Mark
                                    .child(Self::render_slider_row(
                                        t("settings.pass_mark", lang),
                                        t("settings.pass_mark_desc", lang),
                                        Self::render_slider(
                                            "slider_pass",
                                            s.pass_mark,
                                            10,
                                            20,
                                            format!("/ 20 = {}%", s.pass_mark * 5),
                                            &colors,
                                            window,
                                            cx,
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::DecPassMark, window, cx);
                                            },
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::IncPassMark, window, cx);
                                            },
                                        ),
                                        is_desktop,
                                    )),
                            ),
                    )
                    // SECTION 3: Appearance & Controls
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.muted_foreground)
                                    .child(t("settings.group_display", lang)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    // Theme Segmented Selector
                                    .child(
                                        div()
                                            .flex()
                                            .when(is_desktop, |el| {
                                                el.flex_row().items_center().justify_between()
                                            })
                                            .when(!is_desktop, |el| {
                                                el.flex_col().items_start().gap_3()
                                            })
                                            .p_4()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_semibold()
                                                    .text_color(colors.foreground)
                                                    .child(t("settings.theme", lang)),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .p_1()
                                                    .gap_1()
                                                    .rounded_xl()
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .bg(colors.background)
                                                    .child(Self::render_theme_tab(
                                                        "theme_light",
                                                        t("settings.theme_light", lang),
                                                        s.theme == ThemeMode::Light,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetTheme(ThemeMode::Light), window, cx);
                                                        },
                                                    ))
                                                    .child(Self::render_theme_tab(
                                                        "theme_dark",
                                                        t("settings.theme_dark", lang),
                                                        s.theme == ThemeMode::Dark,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetTheme(ThemeMode::Dark), window, cx);
                                                        },
                                                    ))
                                                    .child(Self::render_theme_tab(
                                                        "theme_system",
                                                        t("settings.theme_system", lang),
                                                        s.theme == ThemeMode::System,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetTheme(ThemeMode::System), window, cx);
                                                        },
                                                    )),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Font Size Slider & Live Preview
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .p_4()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .flex()
                                                    .when(is_desktop, |el| {
                                                        el.flex_row().items_center().justify_between()
                                                    })
                                                    .when(!is_desktop, |el| {
                                                        el.flex_col().items_start().gap_3()
                                                    })
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_semibold()
                                                            .text_color(colors.foreground)
                                                            .child(t("settings.font_size", lang)),
                                                    )
                                                    .child(Self::render_slider(
                                                        "slider_font",
                                                        (s.font_size_scale * 100.0) as u32,
                                                        80,
                                                        130,
                                                        "%",
                                                        &colors,
                                                        window,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::DecFontScale, window, cx);
                                                        },
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::IncFontScale, window, cx);
                                                        },
                                                    )),
                                            )
                                            .child(
                                                div()
                                                    .p_3()
                                                    .rounded_xl()
                                                    .border_1()
                                                    .border_color(colors.border)
                                                    .bg(colors.background)
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(colors.foreground)
                                                            .child(t("settings.font_preview", lang)),
                                                    ),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Easy timer switch
                                    .child(Self::render_switch_row(
                                        t("settings.easy_timer", lang),
                                        t("settings.easy_timer_desc", lang),
                                        "sw_easy_timer",
                                        s.easy_show_timer,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleEasyTimer, window, cx);
                                        },
                                    ))
                                    // Keyboard shortcuts switch (desktop only)
                                    .when(is_desktop && !crate::is_native_mobile(), |el| {
                                        let sub = if cfg!(target_os = "macos") {
                                            t("settings.desktop_shortcuts_desc_macos", lang)
                                        } else {
                                            t("settings.desktop_shortcuts_desc", lang)
                                        };
                                        el.child(Self::render_divider(&colors)).child(
                                            Self::render_switch_row(
                                                t("settings.desktop_shortcuts", lang),
                                                sub,
                                                "sw_shortcuts",
                                                s.desktop_shortcuts_enabled,
                                                cx,
                                                move |this, window, cx| {
                                                    on_action(
                                                        this,
                                                        SettingsAction::ToggleDesktopShortcuts,
                                                        window,
                                                        cx,
                                                    );
                                                },
                                            ),
                                        )
                                    })
                                    .child(Self::render_divider(&colors))
                                    // Hard weight image questions switch
                                    .child(Self::render_switch_row(
                                        t("settings.hard_weight", lang),
                                        t("settings.hard_weight_desc", lang),
                                        "sw_hard_images",
                                        s.hard_weight_images,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleHardWeightImages, window, cx);
                                        },
                                    ))
                                    .child(Self::render_divider(&colors))
                                    // Shuffle answer options switch
                                    .child(Self::render_switch_row(
                                        t("settings.shuffle_options", lang),
                                        t("settings.shuffle_options_desc", lang),
                                        "sw_shuffle_options",
                                        s.shuffle_options,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleShuffleOptions, window, cx);
                                        },
                                    )),
                            ),
                    )
                    .when(!cfg!(target_arch = "wasm32") && cfg!(feature = "self-update"), |el| {
                        el.child(Self::render_updates_group(
                            s,
                            lang,
                            is_desktop,
                            update_status,
                            update_message,
                            cx,
                            on_action,
                        ))
                    })
                    // SECTION 4: Data & Storage (Clear History)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.muted_foreground)
                                    .child(t("settings.group_data", lang)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    .p_4()
                                    .child(
                                        div()
                                            .flex()
                                            .when(is_desktop, |el| {
                                                el.flex_row().items_center().justify_between()
                                            })
                                            .when(!is_desktop, |el| {
                                                el.flex_col().items_start().gap_3()
                                            })
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_0p5()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_semibold()
                                                            .text_color(colors.foreground)
                                                            .child(t("settings.clear_history", lang)),
                                                    )
                                                    .child(
                                                         div()
                                                             .text_xs()
                                                             .text_color(colors.muted_foreground)
                                                             .child(if confirm_clear {
                                                                 t("settings.clear_confirm", lang)
                                                             } else {
                                                                 t("settings.clear_history_desc", lang)
                                                             }),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_2()
                                                    .when(!confirm_clear, |el| {
                                                        el.child(
                                                            Button::new("btn_clear_history")
                                                                 .outline()
                                                                 .icon(IconName::Delete)
                                                                 .label(t("settings.clear_action", lang))
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    on_action(this, SettingsAction::RequestClearHistory(true), window, cx);
                                                                })),
                                                        )
                                                    })
                                                    .when(confirm_clear, |el| {
                                                        el.child(
                                                                 Button::new("btn_confirm_clear_yes")
                                                                     .primary()
                                                                 .label(t("settings.clear_yes", lang))
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    on_action(this, SettingsAction::ConfirmClearHistory, window, cx);
                                                                })),
                                                        )
                                                        .child(
                                                                 Button::new("btn_confirm_clear_no")
                                                                     .outline()
                                                                 .label(t("settings.cancel", lang))
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    on_action(this, SettingsAction::RequestClearHistory(false), window, cx);
                                                                })),
                                                        )
                                                    }),
                                            ),
                                    ),
                            ),
                    )
                    // SECTION 5: About
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.muted_foreground)
                                    .child(t("about.title", lang)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    // Version row (static, no link)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .px_4()
                                            .min_h(px(44.0))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_semibold()
                                                    .text_color(colors.foreground)
                                                    .child(tf(
                                                        "about.version",
                                                        lang,
                                                        &[("v", env!("CARGO_PKG_VERSION"))],
                                                    )),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Website row
                                    .child(
                                        Link::new("about_website")
                                            .href(amategeko_core::links::SITE_URL)
                                            .open_with(|href, _, _, app_cx| {
                                            #[cfg(target_arch = "wasm32")]
                                            amategeko_core::platform::open_url(href);
                                            #[cfg(not(target_arch = "wasm32"))]
                                            app_cx.open_url(href);
                                        })
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .px_4()
                                            .min_h(px(44.0))
                                            .hover(|el| el.bg(colors.background))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_semibold()
                                                    .text_color(colors.foreground)
                                                    .child(t("about.website", lang)),
                                            )
                                            .child(
                                                Icon::new(IconName::ExternalLink)
                                                    .size(px(14.0))
                                                    .text_color(colors.muted_foreground),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Guide / Docs row
                                    .child(
                                        Link::new("about_docs")
                                            .href(amategeko_core::links::DOCS_URL)
                                            .open_with(|href, _, _, app_cx| {
                                            #[cfg(target_arch = "wasm32")]
                                            amategeko_core::platform::open_url(href);
                                            #[cfg(not(target_arch = "wasm32"))]
                                            app_cx.open_url(href);
                                        })
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .px_4()
                                            .min_h(px(44.0))
                                            .hover(|el| el.bg(colors.background))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_semibold()
                                                    .text_color(colors.foreground)
                                                    .child(t("about.guide", lang)),
                                            )
                                            .child(
                                                Icon::new(IconName::ExternalLink)
                                                    .size(px(14.0))
                                                    .text_color(colors.muted_foreground),
                                            ),
                                    )
                                    .child(Self::render_divider(&colors))
                                    // Source code row
                                    .child(
                                        Link::new("about_source")
                                            .href(amategeko_core::links::REPO_URL)
                                            .open_with(|href, _, _, app_cx| {
                                            #[cfg(target_arch = "wasm32")]
                                            amategeko_core::platform::open_url(href);
                                            #[cfg(not(target_arch = "wasm32"))]
                                            app_cx.open_url(href);
                                        })
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .px_4()
                                            .min_h(px(44.0))
                                            .hover(|el| el.bg(colors.background))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_semibold()
                                                    .text_color(colors.foreground)
                                                    .child(t("about.source", lang)),
                                            )
                                            .child(
                                                Icon::new(IconName::ExternalLink)
                                                    .size(px(14.0))
                                                    .text_color(colors.muted_foreground),
                                            ),
                                    ),
                            ),
                    )
                    // Bottom Actions: Reset Defaults & Save
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .pt_2()
                            .pb_6()
                            .child(
                                Button::new("btn_reset_defaults")
                                    .outline()
                                    .label(t("settings.reset_defaults", lang))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(this, SettingsAction::ResetDefaults, window, cx);
                                    })),
                            )
                            .child({
                                let mut btn = Button::new("btn_save_settings")
                                    .primary()
                                    .label(t("settings.save_btn", lang));
                                if crate::shortcuts_ui_active(is_desktop, s.desktop_shortcuts_enabled) {
                                    let shortcut_str = if cfg!(target_os = "macos") { "Cmd+S" } else { "Ctrl+S" };
                                    btn = btn.tooltip(format!("{} ({shortcut_str})", t("settings.save_btn", lang)));
                                }
                                btn.on_click(cx.listener(move |this, _, window, cx| {
                                    on_action(this, SettingsAction::SaveSettings, window, cx);
                                }))
                            }),
                    ),
            )
            .when(is_desktop, |settings| {
                settings.child(AppFooter::render(true, page_max_width(), lang, cx))
            })
    }

    fn render_divider(colors: &ThemeColor) -> impl IntoElement {
        div().h(px(1.0)).bg(colors.border).mx_4()
    }

    fn render_updates_group<V: 'static>(
        settings: &amategeko_core::Settings,
        lang: Language,
        is_desktop: bool,
        status: crate::updater::UpdateStatus,
        message: Option<String>,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, SettingsAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let colors = cx.theme().colors;
        let checked = settings
            .last_update_check
            .map(Self::format_update_date)
            .unwrap_or_else(|| "-".into());
        div()
            .flex()
            .flex_col()
            .gap_1p5()
            .child(
                div()
                    .text_sm()
                    .font_semibold()
                    .text_color(colors.muted_foreground)
                    .child(t("settings.group_updates", lang)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .rounded_2xl()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.secondary)
                    .p_4()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_3()
                            .child(
                                div()
                                    .min_w_0()
                                    .text_sm()
                                    .font_semibold()
                                    .child(t("update.setting", lang)),
                            )
                            .child(
                                Switch::new("auto_check_updates")
                                    .checked(settings.auto_check_updates)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(
                                            this,
                                            SettingsAction::ToggleAutoUpdates,
                                            window,
                                            cx,
                                        );
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .when(is_desktop, |el| {
                                el.flex_row().items_center().justify_between()
                            })
                            .when(!is_desktop, |el| el.flex_col().items_start().gap_3())
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(div().text_sm().font_semibold().child(tf(
                                        "update.version",
                                        lang,
                                        &[("v", env!("CARGO_PKG_VERSION"))],
                                    )))
                                    .child(
                                        div().text_xs().text_color(colors.muted_foreground).child(
                                            tf("update.checked", lang, &[("date", &checked)]),
                                        ),
                                    )
                                    .when_some(message, |el, message| {
                                        el.child(
                                            div()
                                                .text_xs()
                                                .text_color(colors.muted_foreground)
                                                .child(message),
                                        )
                                    }),
                            )
                            .child(
                                Button::new("check_updates_now")
                                    .outline()
                                    .label(t("update.check", lang))
                                    .disabled(matches!(
                                        status,
                                        crate::updater::UpdateStatus::Checking
                                            | crate::updater::UpdateStatus::Downloading { .. }
                                    ))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(this, SettingsAction::CheckUpdates, window, cx);
                                    })),
                            ),
                    ),
            )
    }

    fn format_update_date(timestamp: u64) -> String {
        let days = (timestamp / 86_400).min(i64::MAX as u64) as i64;
        let shifted = days + 719_468;
        let era = if shifted >= 0 {
            shifted
        } else {
            shifted - 146_096
        } / 146_097;
        let day_of_era = shifted - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let mut year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_part = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_part + 2) / 5 + 1;
        let month = month_part + if month_part < 10 { 3 } else { -9 };
        year += i64::from(month <= 2);
        format!("{year:04}-{month:02}-{day:02}")
    }

    fn render_slider_row(
        title: &'static str,
        subtitle: &'static str,
        slider_control: impl IntoElement,
        is_desktop: bool,
    ) -> impl IntoElement {
        div()
            .flex()
            .when(is_desktop, |el| {
                el.flex_row().items_center().justify_between()
            })
            .when(!is_desktop, |el| el.flex_col().items_start().gap_3())
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(div().text_sm().font_semibold().child(title))
                    .child(div().text_xs().child(subtitle)),
            )
            .child(slider_control)
    }

    fn render_switch_row<V: 'static>(
        title: &'static str,
        subtitle: &'static str,
        switch_id: &'static str,
        is_checked: bool,
        cx: &mut Context<V>,
        on_toggle: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .p_4()
            .gap_3()
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(div().text_sm().font_semibold().child(title))
                    .child(div().text_xs().child(subtitle)),
            )
            .child(
                Switch::new(switch_id)
                    .checked(is_checked)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_toggle(this, window, cx);
                    })),
            )
    }

    fn render_theme_tab<V: 'static>(
        id: &'static str,
        label: &'static str,
        is_active: bool,
        colors: &ThemeColor,
        cx: &mut Context<V>,
        on_select: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        div()
            .id(id)
            .px_3()
            .py_1p5()
            .rounded_lg()
            .text_xs()
            .font_medium()
            .cursor_pointer()
            .bg(if is_active {
                colors.secondary
            } else {
                gpui::transparent_black()
            })
            .text_color(if is_active {
                colors.foreground
            } else {
                colors.muted_foreground
            })
            .hover(|el| el.bg(colors.secondary))
            .on_click(cx.listener(move |this, _, window, cx| {
                on_select(this, window, cx);
            }))
            .child(label)
    }

    fn render_slider<V: 'static>(
        id_prefix: &'static str,
        value: u32,
        min: u32,
        max: u32,
        unit: impl Into<gpui::SharedString>,
        colors: &ThemeColor,
        window: &mut Window,
        cx: &mut Context<V>,
        on_dec: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_inc: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let unit = unit.into();
        let control_state = window.use_keyed_state(id_prefix, cx, |_, cx| {
            let slider = cx.new(|_| {
                SliderState::new()
                    .min(min as f32)
                    .max(max as f32)
                    .step(1.0)
                    .default_value(value as f32)
            });
            SliderControlState {
                slider,
                current_value: std::rc::Rc::new(std::cell::Cell::new(value)),
                subscription: None,
            }
        });
        let slider_state = control_state.read(cx).slider.clone();
        let current_value = control_state.read(cx).current_value.clone();

        if control_state.read(cx).subscription.is_none() {
            let observed_value = current_value.clone();
            let subscription = cx.subscribe_in(
                &slider_state,
                window,
                move |this, _, event: &SliderEvent, window, cx| {
                    let slider_value = match event {
                        SliderEvent::Change(value) | SliderEvent::Release(value) => value,
                    };
                    let next = slider_value.start().round().clamp(min as f32, max as f32) as u32;
                    let previous = observed_value.replace(next);

                    if next < previous {
                        for _ in next..previous {
                            on_dec(this, window, cx);
                        }
                    } else {
                        for _ in previous..next {
                            on_inc(this, window, cx);
                        }
                    }
                },
            );
            control_state.update(cx, |state, _| state.subscription = Some(subscription));
        }

        if current_value.get() != value {
            current_value.set(value);
            slider_state.update(cx, |state, cx| {
                state.set_value(value as f32, window, cx);
            });
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(54.0))
                    .text_sm()
                    .font_bold()
                    .text_right()
                    .text_color(colors.foreground)
                    .child(format!("{value} {unit}")),
            )
            .child(Slider::new(&slider_state).w(px(160.0)))
    }
}
