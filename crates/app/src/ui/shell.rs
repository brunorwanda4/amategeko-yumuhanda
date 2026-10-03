use crate::state::{AppState, Screen};
use crate::ui::home::HomeView;
use crate::ui::quiz::QuizView;
use amategeko_core::{QuizEngine, Strings};
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct ShellView {
    pub state: AppState,
}

impl ShellView {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Render for ShellView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let window_width = window.bounds().size.width;
        let is_desktop = window_width >= px(700.0);

        let active_screen = self.state.active_screen.clone();
        let is_in_quiz = active_screen == Screen::Quiz;

        let content = match active_screen {
            Screen::Home => HomeView::render(
                &self.state,
                is_desktop,
                cx,
                |this, mode, _, cx| {
                    this.state.start_quiz(mode);
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.resume_attempt();
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.discard_in_progress();
                    cx.notify();
                },
            )
            .into_any_element(),
            Screen::Quiz => QuizView::render(
                &self.state,
                is_desktop,
                cx,
                |this, letter, _, cx| {
                    this.state.record_current_answer(letter);
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        QuizEngine::next_question(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        QuizEngine::previous_question(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        QuizEngine::skip_question(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        match QuizEngine::confirm_and_advance_hard(att) {
                            Ok(true) => {
                                this.state.finish_current_quiz();
                            }
                            Ok(false) => {
                                let _ = this.state.storage.save_in_progress(att);
                            }
                            Err(e) => {
                                log::warn!("Hard confirm error: {e}");
                            }
                        }
                    }
                    cx.notify();
                },
                |this, target_idx, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        QuizEngine::jump_to_question(att, target_idx);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        QuizEngine::toggle_current_flag(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, q_id, _, cx| {
                    this.state.progress.toggle_starred(q_id);
                    let _ = this.state.storage.save_progress(&this.state.progress);
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.finish_current_quiz();
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.navigate(Screen::Home);
                    cx.notify();
                },
            )
            .into_any_element(),
            Screen::Results => div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .p_4()
                .child(
                    div()
                        .text_xl()
                        .font_bold()
                        .text_color(colors.foreground)
                        .child(Strings::RESULTS_TITLE),
                )
                .into_any_element(),
            Screen::Questions => div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .p_4()
                .child(
                    div()
                        .text_xl()
                        .font_bold()
                        .text_color(colors.foreground)
                        .child("Ibibazo 390 by'Amategeko y'Umuhanda"),
                )
                .into_any_element(),
            Screen::Stats => div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .p_4()
                .child(
                    div()
                        .text_xl()
                        .font_bold()
                        .text_color(colors.foreground)
                        .child(Strings::STATS_TITLE),
                )
                .into_any_element(),
            Screen::Settings => div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .p_4()
                .child(
                    div()
                        .text_xl()
                        .font_bold()
                        .text_color(colors.foreground)
                        .child(Strings::SETTINGS_TITLE),
                )
                .into_any_element(),
        };

        if is_desktop {
            // Desktop Layout: Left Sidebar + Content
            div()
                .flex()
                .flex_row()
                .size_full()
                .bg(colors.background)
                .child(self.render_desktop_sidebar(cx))
                .child(
                    div()
                        .flex_1()
                        .size_full()
                        .overflow_hidden()
                        .child(content),
                )
        } else {
            // Mobile Layout: Top App Bar + Content + Bottom Bar (hidden in quiz)
            div()
                .flex()
                .flex_col()
                .size_full()
                .bg(colors.background)
                .child(self.render_mobile_top_bar(cx))
                .child(
                    div()
                        .flex_1()
                        .size_full()
                        .overflow_hidden()
                        .child(content),
                )
                .when(!is_in_quiz, |el| el.child(self.render_mobile_bottom_bar(cx)))
        }
    }
}

impl ShellView {
    fn render_desktop_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let active = &self.state.active_screen;

        div()
            .flex()
            .flex_col()
            .w(px(160.0))
            .h_full()
            .border_r_1()
            .border_color(colors.border)
            .bg(colors.sidebar)
            .p_3()
            .gap_4()
            // App Title in Sidebar
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .py_2()
                    .px_1()
                    .child(
                        div()
                            .text_sm()
                            .font_bold()
                            .text_color(colors.foreground)
                            .child("Amategeko"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors.muted_foreground)
                            .child("y'Umuhanda"),
                    ),
            )
            // Navigation Items
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(self.render_desktop_nav_item(
                        "nav_home",
                        Strings::NAV_HOME,
                        IconName::LayoutDashboard,
                        matches!(active, Screen::Home),
                        Screen::Home,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_quiz",
                        Strings::NAV_QUIZ,
                        IconName::Play,
                        matches!(active, Screen::Quiz),
                        Screen::Quiz,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_questions",
                        Strings::NAV_QUESTIONS,
                        IconName::BookOpen,
                        matches!(active, Screen::Questions),
                        Screen::Questions,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_stats",
                        Strings::NAV_STATS,
                        IconName::ChartPie,
                        matches!(active, Screen::Stats | Screen::Results),
                        Screen::Stats,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_settings",
                        Strings::NAV_SETTINGS,
                        IconName::Settings,
                        matches!(active, Screen::Settings),
                        Screen::Settings,
                        cx,
                    )),
            )
    }

    fn render_desktop_nav_item(
        &self,
        id: &'static str,
        label: &'static str,
        icon: IconName,
        is_active: bool,
        target: Screen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let bg_color = if is_active {
            colors.sidebar_accent
        } else {
            gpui::hsla(0.0, 0.0, 0.0, 0.0)
        };

        let text_color = if is_active {
            colors.primary
        } else {
            colors.foreground
        };

        div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .px_3()
            .py_2()
            .rounded_lg()
            .bg(bg_color)
            .hover(|el| el.bg(colors.sidebar_accent))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.navigate(target.clone());
                cx.notify();
            }))
            .child(
                Icon::new(icon)
                    .size(px(18.0))
                    .text_color(text_color),
            )
            .child(
                div()
                    .text_sm()
                    .when(is_active, |el| el.font_semibold())
                    .text_color(text_color)
                    .child(label),
            )
    }

    fn render_mobile_top_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let title = match self.state.active_screen {
            Screen::Home => Strings::APP_TITLE,
            Screen::Quiz => Strings::NAV_QUIZ,
            Screen::Results => Strings::RESULTS_TITLE,
            Screen::Questions => Strings::NAV_QUESTIONS,
            Screen::Stats => Strings::NAV_STATS,
            Screen::Settings => Strings::NAV_SETTINGS,
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(px(52.0))
            .px_4()
            .border_b_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .child(
                div()
                    .text_base()
                    .font_bold()
                    .text_color(colors.foreground)
                    .child(title),
            )
    }

    fn render_mobile_bottom_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let active = &self.state.active_screen;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_around()
            .h(px(60.0))
            .border_t_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .child(self.render_mobile_nav_item(
                "m_nav_home",
                Strings::NAV_HOME,
                IconName::LayoutDashboard,
                matches!(active, Screen::Home),
                Screen::Home,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_nav_quiz",
                Strings::NAV_QUIZ,
                IconName::Play,
                matches!(active, Screen::Quiz),
                Screen::Quiz,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_nav_questions",
                Strings::NAV_QUESTIONS,
                IconName::BookOpen,
                matches!(active, Screen::Questions),
                Screen::Questions,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_nav_stats",
                Strings::NAV_STATS,
                IconName::ChartPie,
                matches!(active, Screen::Stats | Screen::Results),
                Screen::Stats,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_nav_settings",
                Strings::NAV_SETTINGS,
                IconName::Settings,
                matches!(active, Screen::Settings),
                Screen::Settings,
                cx,
            ))
    }

    fn render_mobile_nav_item(
        &self,
        id: &'static str,
        label: &'static str,
        icon: IconName,
        is_active: bool,
        target: Screen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let icon_color = if is_active {
            colors.primary
        } else {
            colors.muted_foreground
        };

        div()
            .id(id)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .flex_1()
            .h_full()
            .gap_1()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.navigate(target.clone());
                cx.notify();
            }))
            .child(
                Icon::new(icon)
                    .size(px(20.0))
                    .text_color(icon_color),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(icon_color)
                    .when(is_active, |el| el.font_semibold())
                    .child(label),
            )
    }
}
