use crate::state::AppState;
use amategeko_core::{Strings, ThemeMode};
use gpui::InteractiveElement as _;
use gpui_kit::base::{Disableable as _, StyledExt};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    SetTheme(ThemeMode),
    DecFontScale,
    IncFontScale,
    RequestClearHistory(bool),
    ConfirmClearHistory,
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
            .gap_4()
            // Title Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        Icon::new(IconName::Settings)
                            .size(px(24.0))
                            .text_color(colors.primary),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_extrabold()
                            .text_color(colors.foreground)
                            .child(Strings::SETTINGS_TITLE),
                    ),
            )
            // SECTION 1: QUIZ RULES & TIMERS
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
                            .text_base()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child("Igenamiterere ry'Ibizamini"),
                    )
                    // Pass mark stepper (10..20)
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
                                            .child(Strings::SETTINGS_PASS_MARK),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child("Amanota ukeneye kugira ngo utsinde ikizamini"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Button::new("pass_mark_dec")
                                            .outline()
                                            .label("-")
                                            .disabled(s.pass_mark <= 10)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::DecPassMark, window, cx);
                                            })),
                                    )
                                    .child(
                                        div()
                                            .w(px(55.0))
                                            .text_sm()
                                            .font_bold()
                                            .text_center()
                                            .text_color(colors.primary)
                                            .child(format!("{}/20", s.pass_mark)),
                                    )
                                    .child(
                                        Button::new("pass_mark_inc")
                                            .outline()
                                            .label("+")
                                            .disabled(s.pass_mark >= 20)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::IncPassMark, window, cx);
                                            })),
                                    ),
                            ),
                    )
                    // Medium mode duration (10..40 mins)
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
                                            .child(Strings::SETTINGS_MEDIUM_TIME),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child("Igihe cyo gukora ikizamini cyo mu rwego rwo Hagati"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Button::new("medium_time_dec")
                                            .outline()
                                            .label("-")
                                            .disabled(s.medium_duration_mins <= 10)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::DecMediumTime, window, cx);
                                            })),
                                    )
                                    .child(
                                        div()
                                            .w(px(55.0))
                                            .text_sm()
                                            .font_bold()
                                            .text_center()
                                            .text_color(colors.foreground)
                                            .child(format!("{} min", s.medium_duration_mins)),
                                    )
                                    .child(
                                        Button::new("medium_time_inc")
                                            .outline()
                                            .label("+")
                                            .disabled(s.medium_duration_mins >= 40)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::IncMediumTime, window, cx);
                                            })),
                                    ),
                            ),
                    )
                    // Hard mode duration (5..20 mins)
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
                                            .child(Strings::SETTINGS_HARD_TIME),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child("Igihe cyo gukora ikizamini Gikomeye cy'isaha ngufi"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Button::new("hard_time_dec")
                                            .outline()
                                            .label("-")
                                            .disabled(s.hard_duration_mins <= 5)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::DecHardTime, window, cx);
                                            })),
                                    )
                                    .child(
                                        div()
                                            .w(px(55.0))
                                            .text_sm()
                                            .font_bold()
                                            .text_center()
                                            .text_color(colors.foreground)
                                            .child(format!("{} min", s.hard_duration_mins)),
                                    )
                                    .child(
                                        Button::new("hard_time_inc")
                                            .outline()
                                            .label("+")
                                            .disabled(s.hard_duration_mins >= 20)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::IncHardTime, window, cx);
                                            })),
                                    ),
                            ),
                    )
                    // Easy mode elapsed time switch
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
                                            .child(Strings::SETTINGS_EASY_TIMER),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child("Kwerekana iminota umaze mu cyiciro Byoroshye"),
                                    ),
                            )
                            .child(
                                Switch::new("easy_timer_switch")
                                    .checked(s.easy_show_timer)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(this, SettingsAction::ToggleEasyTimer, window, cx);
                                    })),
                            ),
                    )
                    // Hard mode weighted sign/image questions switch
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
                                            .child("Kweza ibyapa mu Gikomeye"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.muted_foreground)
                                            .child("Ibibazo by'ibyapa n'ibimenyetso biza kenshi mu Gikomeye"),
                                    ),
                            )
                            .child(
                                Switch::new("hard_weight_switch")
                                    .checked(s.hard_weight_images)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action(this, SettingsAction::ToggleHardWeightImages, window, cx);
                                    })),
                            ),
                    )
                    // Desktop shortcuts switch (visible only on desktop)
                    .when(is_desktop, |el| {
                        el.child(
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
                                                .child(Strings::SETTINGS_DESKTOP_SHORTCUTS),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(colors.muted_foreground)
                                                .child("Kanda A-D cyangwa 1-4, Enter, F kugira ngo uhitemo"),
                                        ),
                                )
                                .child(
                                    Switch::new("shortcuts_switch")
                                        .checked(s.desktop_shortcuts_enabled)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            on_action(this, SettingsAction::ToggleDesktopShortcuts, window, cx);
                                        })),
                                ),
                        )
                    }),
            )
            // SECTION 2: THEME & DISPLAY
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
                            .text_base()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child("Insanganyamatsiko n'Urumuri"),
                    )
                    // Theme segmented buttons
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(colors.foreground)
                                    .child(Strings::SETTINGS_THEME),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap_2()
                                    // System Theme
                                    .child(
                                        div()
                                            .id("theme_system_btn")
                                            .flex_1()
                                            .py_2()
                                            .px_3()
                                            .rounded_xl()
                                            .cursor_pointer()
                                            .border_1()
                                            .border_color(if s.theme == ThemeMode::System {
                                                colors.primary
                                            } else {
                                                colors.border
                                            })
                                            .bg(if s.theme == ThemeMode::System {
                                                colors.primary
                                            } else {
                                                colors.background
                                            })
                                            .text_xs()
                                            .font_bold()
                                            .text_center()
                                            .text_color(if s.theme == ThemeMode::System {
                                                colors.primary_foreground
                                            } else {
                                                colors.foreground
                                            })
                                            .child(Strings::SETTINGS_THEME_SYSTEM)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::SetTheme(ThemeMode::System), window, cx);
                                            })),
                                    )
                                    // Light Theme
                                    .child(
                                        div()
                                            .id("theme_light_btn")
                                            .flex_1()
                                            .py_2()
                                            .px_3()
                                            .rounded_xl()
                                            .cursor_pointer()
                                            .border_1()
                                            .border_color(if s.theme == ThemeMode::Light {
                                                colors.primary
                                            } else {
                                                colors.border
                                            })
                                            .bg(if s.theme == ThemeMode::Light {
                                                colors.primary
                                            } else {
                                                colors.background
                                            })
                                            .text_xs()
                                            .font_bold()
                                            .text_center()
                                            .text_color(if s.theme == ThemeMode::Light {
                                                colors.primary_foreground
                                            } else {
                                                colors.foreground
                                            })
                                            .child(Strings::SETTINGS_THEME_LIGHT)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::SetTheme(ThemeMode::Light), window, cx);
                                            })),
                                    )
                                    // Dark Theme
                                    .child(
                                        div()
                                            .id("theme_dark_btn")
                                            .flex_1()
                                            .py_2()
                                            .px_3()
                                            .rounded_xl()
                                            .cursor_pointer()
                                            .border_1()
                                            .border_color(if s.theme == ThemeMode::Dark {
                                                colors.primary
                                            } else {
                                                colors.border
                                            })
                                            .bg(if s.theme == ThemeMode::Dark {
                                                colors.primary
                                            } else {
                                                colors.background
                                            })
                                            .text_xs()
                                            .font_bold()
                                            .text_center()
                                            .text_color(if s.theme == ThemeMode::Dark {
                                                colors.primary_foreground
                                            } else {
                                                colors.foreground
                                            })
                                            .child(Strings::SETTINGS_THEME_DARK)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                on_action(this, SettingsAction::SetTheme(ThemeMode::Dark), window, cx);
                                            })),
                                    ),
                            ),
                    )
                    // Font size scaling stepper (0.85x .. 1.25x)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
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
                                            .child(Strings::SETTINGS_FONT_SIZE),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                Button::new("font_dec")
                                                    .outline()
                                                    .label("-")
                                                    .disabled(s.font_size_scale <= 0.85)
                                                    .on_click(cx.listener(move |this, _, window, cx| {
                                                        on_action(this, SettingsAction::DecFontScale, window, cx);
                                                    })),
                                            )
                                            .child(
                                                div()
                                                    .w(px(55.0))
                                                    .text_sm()
                                                    .font_bold()
                                                    .text_center()
                                                    .text_color(colors.primary)
                                                    .child(format!("{:.0}%", s.font_size_scale * 100.0)),
                                            )
                                            .child(
                                                Button::new("font_inc")
                                                    .outline()
                                                    .label("+")
                                                    .disabled(s.font_size_scale >= 1.25)
                                                    .on_click(cx.listener(move |this, _, window, cx| {
                                                        on_action(this, SettingsAction::IncFontScale, window, cx);
                                                    })),
                                            ),
                                    ),
                            )
                            // Live Font Preview
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
            )
            // SECTION 3: DANGER ZONE (CLEAR HISTORY)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .rounded_2xl()
                    .border_1()
                    .border_color(colors.danger)
                    .bg(colors.secondary)
                    .gap_3()
                    .child(
                        div()
                            .text_base()
                            .font_bold()
                            .text_color(colors.danger)
                            .child("Akarere ko Kwitonda (Danger Zone)"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors.muted_foreground)
                            .child("Gusiba amakuru yose y'ibizamini byakozwe, amanota, n'imibare byose byasubira ku busa."),
                    )
                    // Clear history button or confirmation dialog
                    .when(!confirm_clear, |el| {
                        el.child(
                            Button::new("clear_history_btn")
                                .outline()
                                .label(Strings::SETTINGS_CLEAR_HISTORY)
                                .icon(IconName::CircleX)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_action(this, SettingsAction::RequestClearHistory(true), window, cx);
                                })),
                        )
                    })
                    .when(confirm_clear, |el| {
                        el.child(
                            div()
                                .flex()
                                .flex_col()
                                .p_3()
                                .rounded_xl()
                                .border_1()
                                .border_color(colors.danger)
                                .bg(colors.background)
                                .gap_3()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_bold()
                                        .text_color(colors.danger)
                                        .child(Strings::SETTINGS_CLEAR_CONFIRM),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap_3()
                                        .child(
                                            Button::new("confirm_clear_yes")
                                                .primary()
                                                .label("Yego, Siba Byose")
                                                .icon(IconName::CircleX)
                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                    on_action(this, SettingsAction::ConfirmClearHistory, window, cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("confirm_clear_no")
                                                .outline()
                                                .label("Reka")
                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                    on_action(this, SettingsAction::RequestClearHistory(false), window, cx);
                                                })),
                                        ),
                                ),
                        )
                    }),
            )
    }
}
