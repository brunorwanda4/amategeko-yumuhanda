use crate::state::{AppState, Screen};
use crate::ui::home::HomeView;
use crate::ui::questions::{QuestionsFilter, QuestionsView};
use crate::ui::quiz::QuizView;
use crate::ui::results::{ResultFilter, ResultsView};
use crate::ui::settings::{SettingsAction, SettingsView};
use crate::ui::stats::StatsView;
use amategeko_core::{
    t, QuizEngine, QuizMode, QuizTimer, Strings, TimerLevel, TimerState, TimerTracker,
};
use gpui::FocusHandle;
use gpui::InteractiveElement as _;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::alert::Alert;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

#[derive(Clone, Copy)]
struct TimerBanner {
    level: TimerLevel,
    remaining_seconds: u32,
}

impl TimerBanner {
    fn message(self, language: amategeko_core::Language) -> String {
        let time = QuizTimer::format_duration(self.remaining_seconds);
        match self.level {
            TimerLevel::Warning => Strings::timer_warning_for(&time, language),
            TimerLevel::Error => Strings::timer_error_for(&time, language),
            TimerLevel::Done => Strings::timer_done_for(language).to_string(),
            TimerLevel::Normal => String::new(),
        }
    }
}

pub struct ShellView {
    pub state: AppState,
    pub results_filter: ResultFilter,
    pub results_expanded: HashSet<usize>,
    pub questions_filter: QuestionsFilter,
    pub questions_search: String,
    pub questions_expanded: HashSet<u32>,
    pub questions_revealed: HashMap<u32, String>,
    pub settings_confirm_clear: bool,
    pub focus_mode: bool,
    pub focus_handle: FocusHandle,
    timer_tracker: TimerTracker,
    timer_attempt_id: Option<String>,
    timer_banner: Option<TimerBanner>,
    timer_banner_hide_at: Option<u64>,
    _timer_task: Task<()>,
}

impl ShellView {
    pub fn new(state: AppState, cx: &mut Context<Self>) -> Self {
        let timer_task = cx.spawn(async move |this, cx| loop {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            if this
                .update(cx, |this, cx| {
                    this.update_quiz_timer();
                    cx.notify();
                })
                .is_err()
            {
                break;
            }
        });

        Self {
            state,
            results_filter: ResultFilter::All,
            results_expanded: HashSet::new(),
            questions_filter: QuestionsFilter::All,
            questions_search: String::new(),
            questions_expanded: HashSet::new(),
            questions_revealed: HashMap::new(),
            settings_confirm_clear: false,
            focus_mode: false,
            focus_handle: cx.focus_handle(),
            timer_tracker: TimerTracker::new(),
            timer_attempt_id: None,
            timer_banner: None,
            timer_banner_hide_at: None,
            _timer_task: timer_task,
        }
    }

    fn update_quiz_timer(&mut self) {
        let now = self.state.clock.now_seconds();
        if self
            .timer_banner_hide_at
            .is_some_and(|hide_at| now >= hide_at)
        {
            self.timer_banner = None;
            self.timer_banner_hide_at = None;
        }

        let Some((attempt_id, timer_state)) = self.state.current_attempt.as_ref().map(|attempt| {
            (
                attempt.id.clone(),
                QuizTimer::state(attempt, now, self.state.settings.easy_show_timer),
            )
        }) else {
            return;
        };

        if self.timer_attempt_id.as_deref() != Some(attempt_id.as_str()) {
            self.timer_attempt_id = Some(attempt_id);
            self.timer_tracker.reset();
            self.timer_banner = None;
            self.timer_banner_hide_at = None;
        }

        let transition = match timer_state {
            TimerState::Countdown {
                remaining_seconds,
                level,
                ..
            } => self
                .timer_tracker
                .update_level(level)
                .map(|level| (level, remaining_seconds)),
            TimerState::Expired => self
                .timer_tracker
                .update_level(TimerLevel::Done)
                .map(|level| (level, 0)),
            TimerState::None | TimerState::Elapsed(_) => None,
        };

        if let Some((level, remaining_seconds)) = transition {
            self.timer_banner = Some(TimerBanner {
                level,
                remaining_seconds,
            });
            self.timer_banner_hide_at = (level != TimerLevel::Done).then_some(now + 5);

            if level == TimerLevel::Done {
                self.focus_mode = false;
                self.state.finish_current_quiz();
            }
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
                &self.questions_expanded,
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
                |this, qid, _, cx| {
                    if this.questions_expanded.contains(&qid) {
                        this.questions_expanded.remove(&qid);
                    } else {
                        this.questions_expanded.insert(qid);
                    }
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
                        SettingsAction::SetInterfaceLanguage(lang) => {
                            this.state.settings.language = lang;
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::SetQuestionLanguage(lang) => {
                            this.state.settings.question_language = lang;
                            this.state.save_settings(this.state.settings.clone());
                        }
                        SettingsAction::ToggleShowBothLanguages => {
                            this.state.settings.show_both_languages =
                                !this.state.settings.show_both_languages;
                            this.state.save_settings(this.state.settings.clone());
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
            let show_sidebar = !(is_in_quiz && self.focus_mode);

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

        let banner = self.timer_banner;
        let language = self.state.settings.language;
        let banner_top = if is_in_quiz {
            if is_desktop {
                64.0
            } else {
                112.0
            }
        } else if is_desktop {
            12.0
        } else {
            60.0
        };
        let banner_left = if is_desktop && !(is_in_quiz && self.focus_mode) {
            176.0
        } else {
            12.0
        };
        let root = root
            .relative()
            .when_some(banner, |root, banner| {
                let message = banner.message(language);
                let alert = match banner.level {
                    TimerLevel::Warning => Alert::warning("timer-warning-banner", message),
                    TimerLevel::Error => {
                        Alert::error("timer-error-banner", message).icon(IconName::CircleAlert)
                    }
                    TimerLevel::Done => {
                        Alert::error("timer-done-banner", message).icon(IconName::CircleAlert)
                    }
                    TimerLevel::Normal => Alert::new("timer-normal-banner", message),
                }
                .banner()
                .on_close(cx.listener(|this, _, _, cx| {
                    this.timer_banner = None;
                    this.timer_banner_hide_at = None;
                    cx.notify();
                }));

                root.child(
                    div()
                        .absolute()
                        .top(px(banner_top))
                        .left(px(banner_left))
                        .right(px(12.0))
                        .child(alert),
                )
            })
            .track_focus(&self.focus_handle)
            .key_context("Shell")
            .on_key_down(
                cx.listener(move |this, event: &gpui::KeyDownEvent, window, cx| {
                    if this.state.active_screen == Screen::Questions {
                        let key = event.keystroke.key.as_str();
                        if key.eq_ignore_ascii_case("backspace") {
                            this.questions_search.pop();
                            cx.notify();
                            return;
                        } else if key.eq_ignore_ascii_case("escape") {
                            this.questions_search.clear();
                            cx.notify();
                            return;
                        } else if key.chars().count() == 1 {
                            let ch = key.chars().next().unwrap();
                            if !ch.is_control() {
                                this.questions_search.push(ch);
                                cx.notify();
                                return;
                            }
                        }
                    }

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

        if is_in_quiz || active_screen == Screen::Questions {
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
        let lang = self.state.settings.language;

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
                        t("nav.home", lang),
                        IconName::House,
                        matches!(active, Screen::Home),
                        Screen::Home,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_quiz",
                        t("nav.quiz", lang),
                        IconName::Play,
                        matches!(active, Screen::Quiz),
                        Screen::Quiz,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_questions",
                        t("nav.questions", lang),
                        IconName::BookOpen,
                        matches!(active, Screen::Questions),
                        Screen::Questions,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_stats",
                        t("nav.stats", lang),
                        IconName::ChartPie,
                        matches!(active, Screen::Stats | Screen::Results),
                        Screen::Stats,
                        cx,
                    ))
                    .child(self.render_desktop_nav_item(
                        "nav_settings",
                        t("nav.settings", lang),
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
        let lang = self.state.settings.language;

        let title = match self.state.active_screen {
            Screen::Home => t("app.title", lang),
            Screen::Quiz => t("nav.quiz", lang),
            Screen::Results => t("results.title", lang),
            Screen::Questions => t("nav.questions", lang),
            Screen::Stats => t("nav.stats", lang),
            Screen::Settings => t("nav.settings", lang),
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
        let lang = self.state.settings.language;

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
                t("nav.home", lang),
                IconName::House,
                matches!(active, Screen::Home),
                Screen::Home,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_quiz",
                t("nav.quiz", lang),
                IconName::Play,
                matches!(active, Screen::Quiz),
                Screen::Quiz,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_questions",
                t("nav.questions", lang),
                IconName::BookOpen,
                matches!(active, Screen::Questions),
                Screen::Questions,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_stats",
                t("nav.stats", lang),
                IconName::ChartPie,
                matches!(active, Screen::Stats | Screen::Results),
                Screen::Stats,
                cx,
            ))
            .child(self.render_mobile_nav_item(
                "m_settings",
                t("nav.settings", lang),
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
