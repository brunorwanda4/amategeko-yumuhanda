use crate::state::AppState;
use crate::ui::footer::AppFooter;
use crate::ui::scroll::vertical_scrollbar;
use amategeko_core::{t, Language, ThemeMode};
use gpui::InteractiveElement as _;
use gpui_kit::base::Disableable as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::theme::ThemeColor;
use gpui_kit::component::{ActiveTheme, IconName};
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
}

pub struct SettingsView;

impl SettingsView {
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        scroll_handle: &ScrollHandle,
        reveal_scrollbar: bool,
        confirm_clear: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, SettingsAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let s = &state.settings;
        let lang = s.language;

        div()
            .id("settings_scroll_view")
            .track_scroll(scroll_handle)
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .p_4()
            .when(is_desktop, |el| el.p_6())
            .child(vertical_scrollbar(
                "settings_scrollbar",
                scroll_handle,
                is_desktop,
                reveal_scrollbar,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(720.0))
                    .mx_auto()
                    .gap_6()
                    // Page Title Header
                    .child(
                        div()
                            .text_2xl()
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
                                    )),
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
                                            cx,
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::DecMediumTime, window, cx);
                                            },
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::IncMediumTime, window, cx);
                                            },
                                        ),
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
                                            cx,
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::DecHardTime, window, cx);
                                            },
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::IncHardTime, window, cx);
                                            },
                                        ),
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
                                            "/ 20",
                                            &colors,
                                            cx,
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::DecPassMark, window, cx);
                                            },
                                            move |this, window, cx| {
                                                on_action(this, SettingsAction::IncPassMark, window, cx);
                                            },
                                        ),
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
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
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
                                                    .flex_row()
                                                    .items_center()
                                                    .justify_between()
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
                                        "Nta mwanya ugenwe, ariko igihe kiragaragara",
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
                                            "Cmd+1..5, A–D, Enter, ? / F1"
                                        } else {
                                            "Ctrl+1..5, A–D, Enter, ? / F1"
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
                                        "Ibyapa n'ibimenyetso by'umuhanda",
                                        "sw_hard_images",
                                        s.hard_weight_images,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleHardWeightImages, window, cx);
                                        },
                                    )),
                            ),
                    )
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
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
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
                                                            .child("Bikuraho amanota n'imibare yose"),
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
                                                                .label("Siba")
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    on_action(this, SettingsAction::RequestClearHistory(true), window, cx);
                                                                })),
                                                        )
                                                    })
                                                    .when(confirm_clear, |el| {
                                                        el.child(
                                                            Button::new("btn_confirm_clear_yes")
                                                                .primary()
                                                                .label("Yego, Siba")
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    on_action(this, SettingsAction::ConfirmClearHistory, window, cx);
                                                                })),
                                                        )
                                                        .child(
                                                            Button::new("btn_confirm_clear_no")
                                                                .outline()
                                                                .label("Reka")
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    on_action(this, SettingsAction::RequestClearHistory(false), window, cx);
                                                                })),
                                                        )
                                                    }),
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
                settings.child(AppFooter::render(true, px(720.0), lang, cx))
            })
    }

    fn render_divider(colors: &ThemeColor) -> impl IntoElement {
        div().h(px(1.0)).bg(colors.border).mx_4()
    }

    fn render_slider_row(
        title: &'static str,
        subtitle: &'static str,
        slider_control: impl IntoElement,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
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
            .child(
                div()
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

    #[allow(clippy::too_many_arguments)]
    fn render_slider<V: 'static>(
        id_prefix: &'static str,
        value: u32,
        min: u32,
        max: u32,
        unit: &'static str,
        colors: &ThemeColor,
        cx: &mut Context<V>,
        on_dec: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_inc: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let is_min = value <= min;
        let is_max = value >= max;

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
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .p_0p5()
                    .gap_1()
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.background)
                    .child(
                        Button::new(format!("{id_prefix}_dec"))
                            .ghost()
                            .label("-")
                            .disabled(is_min)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_dec(this, window, cx);
                            })),
                    )
                    .child(
                        Button::new(format!("{id_prefix}_inc"))
                            .ghost()
                            .label("+")
                            .disabled(is_max)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_inc(this, window, cx);
                            })),
                    ),
            )
    }
}
