use crate::state::AppState;
use amategeko_core::Strings;
use gpui::InteractiveElement as _;
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
    SetTheme(amategeko_core::ThemeMode),
    DecFontScale,
    IncFontScale,
    RequestClearHistory(bool),
    ConfirmClearHistory,
    ResetDefaults,
    SaveSettings,
}

pub struct SettingsView;

impl SettingsView {
    pub fn render<V: 'static>(
        state: &AppState,
        is_desktop: bool,
        confirm_clear: bool,
        cx: &mut Context<V>,
        on_action: impl Fn(&mut V, SettingsAction, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let s = &state.settings;

        div()
            .id("settings_scroll_view")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(colors.background)
            .p_4()
            .when(is_desktop, |el| el.p_6())
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
                            .child(Strings::SETTINGS_TITLE),
                    )
                    // SECTION 1: Ikizamini
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
                                    .child("Ikizamini"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    // Row 1: Igihe cya Hagati
                                    .child(Self::render_slider_row(
                                        "Igihe cya Hagati",
                                        "Iminota y'ikizamini cy'ibibazo 20",
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
                                    // Row 2: Igihe cya Bikomeye
                                    .child(Self::render_slider_row(
                                        "Igihe cya Bikomeye",
                                        "Gito kurusha Hagati",
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
                                    // Row 3: Amanota yo gutsinda
                                    .child(Self::render_slider_row(
                                        "Amanota yo gutsinda",
                                        "Ibibazo bigomba kuba byujuje kuri 20",
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
                    // SECTION 2: Isura
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
                                    .child("Isura"),
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
                                                    .child("Insanganyamatsiko"),
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
                                                        "Umucyo",
                                                        s.theme == amategeko_core::ThemeMode::Light,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetTheme(amategeko_core::ThemeMode::Light), window, cx);
                                                        },
                                                    ))
                                                    .child(Self::render_theme_tab(
                                                        "theme_dark",
                                                        "Umwijima",
                                                        s.theme == amategeko_core::ThemeMode::Dark,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetTheme(amategeko_core::ThemeMode::Dark), window, cx);
                                                        },
                                                    ))
                                                    .child(Self::render_theme_tab(
                                                        "theme_system",
                                                        "Sisitemu",
                                                        s.theme == amategeko_core::ThemeMode::System,
                                                        &colors,
                                                        cx,
                                                        move |this, window, cx| {
                                                            on_action(this, SettingsAction::SetTheme(amategeko_core::ThemeMode::System), window, cx);
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
                                                            .child("Ubunini bw'inyandiko"),
                                                    )
                                                    .child(Self::render_slider(
                                                        "slider_font",
                                                        (s.font_size_scale * 15.0).round() as u32,
                                                        12,
                                                        20,
                                                        "px",
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
                                                            .child(Strings::SETTINGS_FONT_PREVIEW),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    // SECTION 3: Imikorere
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
                                    .child("Imikorere"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .rounded_2xl()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.secondary)
                                    // Easy timer switch
                                    .child(Self::render_switch_row(
                                        "Erekana igihe cyakoreshejwe muri Byoroshye",
                                        "Nta mwanya ugenwe, ariko igihe kiragaragara",
                                        "sw_easy_timer",
                                        s.easy_show_timer,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleEasyTimer, window, cx);
                                        },
                                    ))
                                    .child(Self::render_divider(&colors))
                                    // Keyboard shortcuts switch
                                    .child(Self::render_switch_row(
                                        "Koresha inyuguti za clavier",
                                        "A–D, 1–4, Enter, F",
                                        "sw_shortcuts",
                                        s.desktop_shortcuts_enabled,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleDesktopShortcuts, window, cx);
                                        },
                                    ))
                                    .child(Self::render_divider(&colors))
                                    // Hard weight image questions switch
                                    .child(Self::render_switch_row(
                                        "Shyiramo ibibazo by'ibyapa kenshi muri Bikomeye",
                                        "Ibyapa n'ibimenyetso by'umuhanda",
                                        "sw_hard_images",
                                        s.hard_weight_images,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleHardWeightImages, window, cx);
                                        },
                                    ))
                                    .child(Self::render_divider(&colors))
                                    // Show all answers at end switch
                                    .child(Self::render_switch_row(
                                        "Erekana ibisubizo byose ku iherezo",
                                        "Ibyo wakosheje n'ibyo wabonye neza",
                                        "sw_show_all",
                                        !s.study_hide_answers,
                                        cx,
                                        move |this, window, cx| {
                                            on_action(this, SettingsAction::ToggleShowAllAnswersAtEnd, window, cx);
                                        },
                                    )),
                            ),
                    )
                    // SECTION 4: Amakuru (Clear History)
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
                                    .child("Amakuru"),
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
                                                            .child("Siba amateka y'ibizamini"),
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
                    // Bottom Actions: Subiza ku by'ibanze & Bika
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
                                    .label("Subiza ku by'ibanze")
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(this, SettingsAction::ResetDefaults, window, cx);
                                    })),
                            )
                            .child(
                                Button::new("btn_save_settings")
                                    .primary()
                                    .label("Bika")
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(this, SettingsAction::SaveSettings, window, cx);
                                    })),
                            ),
                    ),
            )
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
            .py_1()
            .rounded_lg()
            .cursor_pointer()
            .text_xs()
            .font_semibold()
            .bg(if is_active {
                colors.foreground
            } else {
                gpui::hsla(0.0, 0.0, 0.0, 0.0)
            })
            .text_color(if is_active {
                colors.background
            } else {
                colors.muted_foreground
            })
            .child(label)
            .on_click(cx.listener(move |this, _, window, cx| {
                on_select(this, window, cx);
            }))
    }

    #[allow(clippy::too_many_arguments)]
    fn render_slider<V: 'static>(
        id_prefix: &'static str,
        val: u32,
        min: u32,
        max: u32,
        suffix: &'static str,
        colors: &ThemeColor,
        cx: &mut Context<V>,
        on_dec: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
        on_inc: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> impl IntoElement {
        let pct = if max > min {
            ((val.saturating_sub(min)) as f32 / (max - min) as f32).clamp(0.0, 1.0)
        } else {
            0.5
        };

        let track_w = 140.0;
        let thumb_pos = track_w * pct;

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            // Interactive track
            .child(
                div()
                    .id(format!("{id_prefix}_track"))
                    .relative()
                    .w(px(track_w))
                    .h(px(24.0))
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    // Left half clicks decrement
                    .child(
                        div()
                            .id(format!("{id_prefix}_dec_zone"))
                            .absolute()
                            .left_0()
                            .top_0()
                            .w(px(track_w / 2.0))
                            .h_full()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_dec(this, window, cx);
                            })),
                    )
                    // Right half clicks increment
                    .child(
                        div()
                            .id(format!("{id_prefix}_inc_zone"))
                            .absolute()
                            .right_0()
                            .top_0()
                            .w(px(track_w / 2.0))
                            .h_full()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                on_inc(this, window, cx);
                            })),
                    )
                    // The background track bar
                    .child(div().w_full().h(px(3.0)).rounded_full().bg(colors.border))
                    // Filled track portion
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .w(px(thumb_pos))
                            .h(px(3.0))
                            .rounded_full()
                            .bg(colors.primary),
                    )
                    // The thumb knob (circle)
                    .child(
                        div()
                            .absolute()
                            .left(px((thumb_pos - 7.0).max(0.0)))
                            .size(px(14.0))
                            .rounded_full()
                            .bg(colors.foreground)
                            .border_2()
                            .border_color(colors.background),
                    ),
            )
            // Value display label
            .child(
                div()
                    .w(px(55.0))
                    .text_sm()
                    .font_bold()
                    .text_right()
                    .text_color(colors.foreground)
                    .child(format!("{val} {suffix}")),
            )
    }
}
