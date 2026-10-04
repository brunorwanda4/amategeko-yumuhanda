use crate::state::{AppState, Screen};
use crate::ui::home::HomeView;
use crate::ui::questions::{QuestionsFilter, QuestionsView};
use crate::ui::quiz::QuizView;
use crate::ui::results::{ResultFilter, ResultsView};
use crate::ui::settings::{SettingsAction, SettingsView};
use crate::ui::stats::StatsView;
use amategeko_core::{QuizEngine, QuizMode, Strings};
use gpui::FocusHandle;
use gpui::InteractiveElement as _;
use gpui_kit::base::StyledExt;
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::{HashMap, HashSet};

pub struct ShellView {
    pub state: AppState,
    pub results_filter: ResultFilter,
    pub results_expanded: HashSet<usize>,
    pub questions_filter: QuestionsFilter,
    pub questions_search: String,
    pub questions_revealed: HashMap<u32, String>,
    pub settings_confirm_clear: bool,
    pub focus_mode: bool,
    pub focus_handle: FocusHandle,
}

impl ShellView {
    pub fn new(state: AppState, cx: &mut Context<Self>) -> Self {
        Self {
            state,
            results_filter: ResultFilter::All,
            results_expanded: HashSet::new(),
            questions_filter: QuestionsFilter::All,
            questions_search: String::new(),
            questions_revealed: HashMap::new(),
            settings_confirm_clear: false,
            focus_mode: false,
            focus_handle: cx.focus_handle(),
        }
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
                window_width,
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
                self.focus_mode,
                cx,
                |this, opt, _, cx| {
                    this.state.record_current_answer(opt);
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        let _ = QuizEngine::next_question(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        let _ = QuizEngine::previous_question(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        let _ = QuizEngine::skip_question(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        let _ = QuizEngine::confirm_and_advance_hard(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, idx, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        let _ = QuizEngine::jump_to_question(att, idx);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    if let Some(att) = &mut this.state.current_attempt {
                        let _ = QuizEngine::toggle_current_flag(att);
                        let _ = this.state.storage.save_in_progress(att);
                    }
                    cx.notify();
                },
                |this, qid, _, cx| {
                    this.state.progress.toggle_starred(qid);
                    let _ = this.state.storage.save_progress(&this.state.progress);
                    cx.notify();
                },
                |this, window, cx| {
                    this.focus_mode = !this.focus_mode;
                    if this.focus_mode {
                        if !window.is_fullscreen() {
                            window.toggle_fullscreen();
                        }
                    } else if window.is_fullscreen() {
                        window.toggle_fullscreen();
                    }
                    cx.notify();
                },
                |this, window, cx| {
                    if window.is_fullscreen() {
                        window.toggle_fullscreen();
                    }
                    this.focus_mode = false;
                    this.state.finish_current_quiz();
                    cx.notify();
                },
                |this, window, cx| {
                    if window.is_fullscreen() {
                        window.toggle_fullscreen();
                    }
                    this.focus_mode = false;
                    this.state.discard_in_progress();
                    this.state.navigate(Screen::Home);
                    cx.notify();
                },
            )
            .into_any_element(),
            Screen::Results => ResultsView::render(
                &self.state,
                is_desktop,
                self.results_filter,
                &self.results_expanded,
                cx,
                |this, filter, _, cx| {
                    this.results_filter = filter;
                    cx.notify();
                },
                |this, idx, _, cx| {
                    if this.results_expanded.contains(&idx) {
                        this.results_expanded.remove(&idx);
                    } else {
                        this.results_expanded.insert(idx);
                    }
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.start_retry_wrong();
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.start_quiz(QuizMode::Byoroshye);
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.navigate(Screen::Home);
                    cx.notify();
                },
            )
            .into_any_element(),
            Screen::Questions => QuestionsView::render(
                &self.state,
                is_desktop,
                self.questions_filter,
                &self.questions_search,
                self.state.settings.study_hide_answers,
                &self.questions_revealed,
                cx,
                |this, filter, _, cx| {
                    this.questions_filter = filter;
                    cx.notify();
                },
                |this, search, _, cx| {
                    this.questions_search = search;
                    cx.notify();
                },
                |this, hide, _, cx| {
                    this.state.settings.study_hide_answers = hide;
                    let s = this.state.settings.clone();
                    this.state.save_settings(s);
                    this.questions_revealed.clear();
                    cx.notify();
                },
                |this, qid, _, cx| {
                    this.state.progress.toggle_starred(qid);
                    let _ = this.state.storage.save_progress(&this.state.progress);
                    cx.notify();
                },
                |this, qid, opt, _, cx| {
                    this.questions_revealed.insert(qid, opt);
                    cx.notify();
                },
            )
            .into_any_element(),
            Screen::Stats => StatsView::render(
                &self.state,
                is_desktop,
                cx,
                |this, _, cx| {
                    this.state.start_quiz(QuizMode::WeakPractice);
                    cx.notify();
                },
                |this, _, cx| {
                    this.state.start_quiz(QuizMode::Byoroshye);
                    cx.notify();
                },
            )
            .into_any_element(),
            Screen::Settings => SettingsView::render(
                &self.state,
                is_desktop,
                self.settings_confirm_clear,
                cx,
                |this, action, window, cx| {
                    match action {
                        SettingsAction::DecPassMark => {
                            this.state.settings.pass_mark =
                                this.state.settings.pass_mark.saturating_sub(1).max(10);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::IncPassMark => {
                            this.state.settings.pass_mark =
                                (this.state.settings.pass_mark + 1).min(20);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::DecMediumTime => {
                            this.state.settings.medium_duration_mins = this
                                .state
                                .settings
                                .medium_duration_mins
                                .saturating_sub(1)
                                .max(10);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::IncMediumTime => {
                            this.state.settings.medium_duration_mins =
                                (this.state.settings.medium_duration_mins + 1).min(40);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::DecHardTime => {
                            this.state.settings.hard_duration_mins = this
                                .state
                                .settings
                                .hard_duration_mins
                                .saturating_sub(1)
                                .max(5);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::IncHardTime => {
                            this.state.settings.hard_duration_mins =
                                (this.state.settings.hard_duration_mins + 1).min(20);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::ToggleEasyTimer => {
                            this.state.settings.easy_show_timer =
                                !this.state.settings.easy_show_timer;
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::ToggleHardWeightImages => {
                            this.state.settings.hard_weight_images =
                                !this.state.settings.hard_weight_images;
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::ToggleDesktopShortcuts => {
                            this.state.settings.desktop_shortcuts_enabled =
                                !this.state.settings.desktop_shortcuts_enabled;
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::ToggleShowAllAnswersAtEnd => {
                            this.state.settings.study_hide_answers =
                                !this.state.settings.study_hide_answers;
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::SetTheme(mode) => {
                            this.state.settings.theme = mode;
                            this.state.save_settings(this.state.settings.clone());
                            crate::apply_theme(mode, Some(window), cx);
                        }
                        SettingsAction::DecFontScale => {
                            this.state.settings.font_size_scale =
                                (this.state.settings.font_size_scale - 0.05).max(0.80);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::IncFontScale => {
                            this.state.settings.font_size_scale =
                                (this.state.settings.font_size_scale + 0.05).min(1.33);
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::ResetDefaults => {
                            this.state.settings = amategeko_core::Settings::default();
                            this.state.save_settings(this.state.settings.clone());
                            crate::apply_theme(this.state.settings.theme, Some(window), cx);
                        }
                        SettingsAction::SaveSettings => {
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::RequestClearHistory(req) => {
                            this.settings_confirm_clear = req;
                        }
                        SettingsAction::ConfirmClearHistory => {
                            this.state.clear_history();
                            this.settings_confirm_clear = false;
                        }
                    }
                    cx.notify();
                },
            )
            .into_any_element(),
        };

        let root = if is_desktop {
            // Desktop Layout: Left Sidebar + Content
            // Focus Mode in Quiz hides the sidebar for a pure, distraction-free reading experience
            let show_sidebar = !is_in_quiz || !self.focus_mode;

            div()
                .flex()
                .flex_row()
                .size_full()
                .bg(colors.background)
                .when(show_sidebar, |el| el.child(self.render_desktop_sidebar(cx)))
                .child(div().flex_1().size_full().overflow_hidden().child(content))
        } else {
            // Mobile Layout: Top App Bar + Content + Bottom Bar (hidden in quiz)
            div()
                .flex()
                .flex_col()
                .size_full()
                .bg(colors.background)
                .child(self.render_mobile_top_bar(cx))
                .child(div().flex_1().size_full().overflow_hidden().child(content))
                .when(!is_in_quiz, |el| {
                    el.child(self.render_mobile_bottom_bar(cx))
                })
        };

        let shortcuts_enabled = self.state.settings.desktop_shortcuts_enabled;

        let root = root
            .track_focus(&self.focus_handle)
            .key_context("Shell")
            .on_key_down(
                cx.listener(move |this, event: &gpui::KeyDownEvent, window, cx| {
                    if this.state.active_screen != Screen::Quiz || !shortcuts_enabled {
                        return;
                    }

                    let key = event.keystroke.key.to_lowercase();
                    match key.as_str() {
                        "a" | "1" => {
                            let locked = this
                                .state
                                .current_attempt
                                .as_ref()
                                .map(|a| a.is_current_locked())
                                .unwrap_or(false);
                            if !locked {
                                this.state.record_current_answer("a");
                                cx.notify();
                            }
                        }
                        "b" | "2" => {
                            let locked = this
                                .state
                                .current_attempt
                                .as_ref()
                                .map(|a| a.is_current_locked())
                                .unwrap_or(false);
                            if !locked {
                                this.state.record_current_answer("b");
                                cx.notify();
                            }
                        }
                        "c" | "3" => {
                            let locked = this
                                .state
                                .current_attempt
                                .as_ref()
                                .map(|a| a.is_current_locked())
                                .unwrap_or(false);
                            if !locked {
                                this.state.record_current_answer("c");
                                cx.notify();
                            }
                        }
                        "d" | "4" => {
                            let locked = this
                                .state
                                .current_attempt
                                .as_ref()
                                .map(|a| a.is_current_locked())
                                .unwrap_or(false);
                            if !locked {
                                this.state.record_current_answer("d");
                                cx.notify();
                            }
                        }
                        "enter" => {
                            if let Some(att) = &mut this.state.current_attempt {
                                if att.mode == QuizMode::Bikomeye {
                                    let _ = QuizEngine::confirm_and_advance_hard(att);
                                    let _ = this.state.storage.save_in_progress(att);
                                } else {
                                    let is_last = att.current_index + 1 == att.total_questions();
                                    if is_last {
                                        if window.is_fullscreen() {
                                            window.toggle_fullscreen();
                                        }
                                        this.focus_mode = false;
                                        this.state.finish_current_quiz();
                                    } else {
                                        let _ = QuizEngine::next_question(att);
                                        let _ = this.state.storage.save_in_progress(att);
                                    }
                                }
                                cx.notify();
                            }
                        }
                        "arrowleft" | "left" => {
                            if let Some(att) = &mut this.state.current_attempt {
                                let _ = QuizEngine::previous_question(att);
                                let _ = this.state.storage.save_in_progress(att);
                                cx.notify();
                            }
                        }
                        "arrowright" | "right" => {
                            if let Some(att) = &mut this.state.current_attempt {
                                let is_last = att.current_index + 1 == att.total_questions();
                                if is_last {
                                    if window.is_fullscreen() {
                                        window.toggle_fullscreen();
                                    }
                                    this.focus_mode = false;
                                    this.state.finish_current_quiz();
                                } else {
                                    let _ = QuizEngine::next_question(att);
                                    let _ = this.state.storage.save_in_progress(att);
                                }
                                cx.notify();
                            }
                        }
                        "s" => {
                            if let Some(att) = &this.state.current_attempt {
                                if let Some(q) = att.current_question() {
                                    this.state.progress.toggle_starred(q.id);
                                    let _ = this.state.storage.save_progress(&this.state.progress);
                                    cx.notify();
                                }
                            }
                        }
                        "f" => {
                            this.focus_mode = !this.focus_mode;
                            if this.focus_mode {
                                if !window.is_fullscreen() {
                                    window.toggle_fullscreen();
                                }
                            } else if window.is_fullscreen() {
                                window.toggle_fullscreen();
                            }
                            cx.notify();
                        }
                        "escape" if this.focus_mode => {
                            this.focus_mode = false;
                            if window.is_fullscreen() {
                                window.toggle_fullscreen();
                            }
                            cx.notify();
                        }
                        _ => {}
                    }
                }),
            );

        if is_in_quiz {
            window.focus(&self.focus_handle, cx);
        }

        root
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
        target_screen: Screen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let bg_color = if is_active {
            colors.sidebar_accent
        } else {
            gpui::transparent_black()
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
            .gap_2p5()
            .px_3()
            .py_2()
            .rounded_lg()
            .bg(bg_color)
            .cursor_pointer()
            .hover(|el| el.bg(colors.sidebar_accent))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.navigate(target_screen.clone());
                cx.notify();
            }))
            .child(Icon::new(icon).size(px(16.0)).text_color(text_color))
            .child(
                div()
                    .text_xs()
                    .font_semibold()
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
            .px_4()
            .py_3()
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
            .px_2()
            .py_2()
            .border_t_1()
            .border_color(colors.border)
            .bg(colors.secondary)
            .child(self.render_mobile_nav_item(
                "m_home",
                Strings::NAV_HOME,
                IconName::LayoutDashboard,
                matches!(active, Screen::Home),
                Screen::Home,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_quiz",
                Strings::NAV_QUIZ,
                IconName::Play,
                matches!(active, Screen::Quiz),
                Screen::Quiz,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_questions",
                Strings::NAV_QUESTIONS,
                IconName::BookOpen,
                matches!(active, Screen::Questions),
                Screen::Questions,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_stats",
                Strings::NAV_STATS,
                IconName::ChartPie,
                matches!(active, Screen::Stats | Screen::Results),
                Screen::Stats,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_settings",
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
        target_screen: Screen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors;

        let color = if is_active {
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
            .gap_1()
            .px_2()
            .py_1()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.navigate(target_screen.clone());
                cx.notify();
            }))
            .child(Icon::new(icon).size(px(20.0)).text_color(color))
            .child(div().text_xs().font_medium().text_color(color).child(label))
    }
}
