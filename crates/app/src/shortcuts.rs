use crate::ui::scroll::vertical_scrollbar;
use amategeko_core::{t, Attempt, Language, QuizMode};
use gpui::InteractiveElement as _;
use gpui::*;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::*;

/// Representation of a key combination.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    /// Normalized key string, lowercase (e.g. "1", "a", "enter", "escape", "arrowleft", "?", "f1").
    pub key: String,
    /// Primary control modifier: Ctrl on Windows/Linux, Cmd on macOS.
    pub ctrl: bool,
    /// Alt / Option modifier.
    pub alt: bool,
    /// Shift modifier.
    pub shift: bool,
}

impl KeyCombo {
    pub fn plain(key: impl Into<String>) -> Self {
        Self {
            key: key.into().to_lowercase(),
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    pub fn ctrl(key: impl Into<String>) -> Self {
        Self {
            key: key.into().to_lowercase(),
            ctrl: true,
            alt: false,
            shift: false,
        }
    }

    pub fn ctrl_enter() -> Self {
        Self::ctrl("enter")
    }

    pub fn f1() -> Self {
        Self::plain("f1")
    }

    pub fn question_mark() -> Self {
        Self {
            key: "?".to_string(),
            ctrl: false,
            alt: false,
            shift: true,
        }
    }

    pub fn slash() -> Self {
        Self::plain("/")
    }

    pub fn is_single_key(&self) -> bool {
        !self.ctrl && !self.alt && self.key != "escape" && self.key != "f1"
    }

    /// Primary modifier name: "Cmd" on macOS, "Ctrl" elsewhere.
    pub fn primary_modifier_name() -> &'static str {
        if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            "Ctrl"
        }
    }

    /// Formats the key combination for human-readable display.
    pub fn display_string(&self) -> String {
        let key_name = match self.key.as_str() {
            "arrowleft" | "left" => "←".to_string(),
            "arrowright" | "right" => "→".to_string(),
            "arrowup" | "up" => "↑".to_string(),
            "arrowdown" | "down" => "↓".to_string(),
            "escape" => "Esc".to_string(),
            "enter" => "Enter".to_string(),
            other => other.to_uppercase(),
        };

        if self.ctrl {
            format!("{}+{}", Self::primary_modifier_name(), key_name)
        } else {
            key_name
        }
    }

    /// Checks if a GPUI KeyDownEvent matches this combo.
    pub fn matches(&self, event: &gpui::KeyDownEvent) -> bool {
        let ev_key = event.keystroke.key.to_lowercase();
        let ev_modifiers = event.keystroke.modifiers;

        let ctrl_matches = if cfg!(target_os = "macos") {
            self.ctrl == ev_modifiers.platform && !ev_modifiers.control
        } else {
            self.ctrl == ev_modifiers.control && !ev_modifiers.platform
        };

        let key_matches = match self.key.as_str() {
            "arrowleft" | "left" => ev_key == "arrowleft" || ev_key == "left",
            "arrowright" | "right" => ev_key == "arrowright" || ev_key == "right",
            "arrowup" | "up" => ev_key == "arrowup" || ev_key == "up",
            "arrowdown" | "down" => ev_key == "arrowdown" || ev_key == "down",
            "escape" | "esc" => ev_key == "escape" || ev_key == "esc",
            "enter" => ev_key == "enter",
            "?" => ev_key == "?" || (ev_key == "/" && ev_modifiers.shift),
            "/" => ev_key == "/" && !ev_modifiers.shift,
            other => ev_key == other,
        };

        key_matches
            && ctrl_matches
            && self.alt == ev_modifiers.alt
            && self.shift == ev_modifiers.shift
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyCombo, QuizMode, ShortcutAction, ShortcutRegistry};
    use std::collections::HashMap;

    #[test]
    fn shortcut_registry_has_no_context_conflicts() {
        let registry = ShortcutRegistry::new();
        let contexts = [
            (crate::Screen::Home, None),
            (crate::Screen::Results, None),
            (crate::Screen::Questions, None),
            (crate::Screen::Stats, None),
            (crate::Screen::Settings, None),
            (crate::Screen::Quiz, Some(QuizMode::Byoroshye)),
            (crate::Screen::Quiz, Some(QuizMode::Hagati)),
            (crate::Screen::Quiz, Some(QuizMode::Bikomeye)),
            (crate::Screen::Quiz, Some(QuizMode::WeakPractice)),
            (crate::Screen::Quiz, Some(QuizMode::RetryWrong)),
        ];

        for (screen, mode) in contexts {
            let shortcuts = registry.shortcuts_for_context(screen.clone(), mode);
            let mut seen = HashMap::new();
            for def in &shortcuts {
                if let Some(previous) = seen.insert(def.key.clone(), def.action) {
                    panic!(
                        "shortcut conflict on {screen:?}/{mode:?}: {:?} maps to {previous:?} and {:?}",
                        def.key, def.action
                    );
                }

                if screen == crate::Screen::Quiz
                    && !def.key.ctrl
                    && !def.key.alt
                    && !def.key.shift
                    && matches!(
                        def.key.key.as_str(),
                        "a" | "b" | "c" | "d" | "1" | "2" | "3" | "4"
                    )
                {
                    assert!(
                        matches!(def.action, ShortcutAction::ChooseOption(_)),
                        "quiz option key {:?} maps to {:?}",
                        def.key,
                        def.action
                    );
                }
            }
        }

        assert!(registry.all().iter().all(|def| {
            !(def.key == KeyCombo::plain("s") && def.label_key == "shortcuts.skip_question")
        }));

        let hard = registry.shortcuts_for_context(crate::Screen::Quiz, Some(QuizMode::Bikomeye));
        assert!(hard.iter().all(|def| {
            !def.key.ctrl
                && !def.key.alt
                && !def.key.shift
                && matches!(
                    def.key.key.as_str(),
                    "a" | "b" | "c" | "d" | "1" | "2" | "3" | "4" | "enter"
                )
        }));
    }
}

/// Logical actions triggered by keyboard shortcuts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutAction {
    // Global Navigation
    NavHome,
    NavQuiz,
    NavQuestions,
    NavStats,
    NavSettings,
    CloseOrBack,
    ShowHelp,
    ToggleTheme,

    // Home (mode picker)
    SelectQuizMode(u8), // 1 = Easy, 2 = Medium, 3 = Hard
    StartSelectedQuiz,

    // Quiz
    ChooseOption(char), // 'a' | 'b' | 'c' | 'd'
    NextOrConfirm,

    // Easy
    PrevQuestion,
    NextQuestion,
    StarQuestion,

    // Medium
    FlagQuestion,
    FinishExam,

    // Results
    RetryQuiz,
    RetryMistakes,
    FilterResultsAll,
    FilterResultsCorrect,
    FilterResultsWrong,

    // Questions
    FocusSearch,
    MoveUp,
    MoveDown,
    ToggleExpand,
    ToggleStarSelected,
    ToggleHideAnswers,

    // Stats
    FilterStats(u8),

    // Settings
    SaveSettings,
}

impl ShortcutAction {
    /// Returns true if this is a navigation action (disabled in Hard mode).
    pub fn is_navigation(&self) -> bool {
        matches!(
            self,
            ShortcutAction::NavHome
                | ShortcutAction::NavQuiz
                | ShortcutAction::NavQuestions
                | ShortcutAction::NavStats
                | ShortcutAction::NavSettings
                | ShortcutAction::CloseOrBack
                | ShortcutAction::PrevQuestion
                | ShortcutAction::NextQuestion
        )
    }

    /// Returns true if this is allowed in Hard mode quiz.
    pub fn is_allowed_in_hard(&self) -> bool {
        matches!(
            self,
            ShortcutAction::ChooseOption(_) | ShortcutAction::NextOrConfirm
        )
    }

    /// Returns true if this is a destructive action (must never be a single key).
    pub fn is_destructive(&self) -> bool {
        matches!(self, ShortcutAction::FinishExam)
    }
}

/// Scope/screen contexts where a shortcut can be active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutScope {
    Global,
    Home,
    QuizAll,
    QuizEasy,
    QuizMedium,
    QuizHard,
    Results,
    Questions,
    Stats,
    Settings,
}

/// A registered shortcut definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutDef {
    pub key: KeyCombo,
    pub action: ShortcutAction,
    pub scopes: Vec<ShortcutScope>,
    pub label_key: &'static str,
    pub custom_hint: Option<&'static str>,
}

impl ShortcutDef {
    pub fn is_single_key(&self) -> bool {
        self.key.is_single_key()
    }

    pub fn applies_to(&self, screen: crate::Screen, mode: Option<QuizMode>) -> bool {
        // In Hard mode, navigation shortcuts and Esc-back are strictly disabled
        if screen == crate::Screen::Quiz && mode == Some(QuizMode::Bikomeye) {
            if self.action.is_navigation() {
                return false;
            }
            if !self.action.is_allowed_in_hard() {
                return false;
            }
        }

        for scope in &self.scopes {
            match scope {
                ShortcutScope::Global => return true,
                ShortcutScope::Home if screen == crate::Screen::Home => return true,

                ShortcutScope::QuizAll if screen == crate::Screen::Quiz => return true,
                ShortcutScope::QuizEasy
                    if screen == crate::Screen::Quiz
                        && matches!(
                            mode,
                            Some(QuizMode::Byoroshye) | Some(QuizMode::WeakPractice)
                        ) =>
                {
                    return true;
                }
                ShortcutScope::QuizMedium
                    if screen == crate::Screen::Quiz && mode == Some(QuizMode::Hagati) =>
                {
                    return true;
                }
                ShortcutScope::QuizHard
                    if screen == crate::Screen::Quiz && mode == Some(QuizMode::Bikomeye) =>
                {
                    return true;
                }
                ShortcutScope::Results if screen == crate::Screen::Results => return true,
                ShortcutScope::Questions if screen == crate::Screen::Questions => return true,
                ShortcutScope::Stats if screen == crate::Screen::Stats => return true,
                ShortcutScope::Settings if screen == crate::Screen::Settings => return true,
                _ => {}
            }
        }

        false
    }
}

/// The single application-wide shortcut registry.
#[derive(Debug, Clone)]
pub struct ShortcutRegistry {
    shortcuts: Vec<ShortcutDef>,
}

impl Default for ShortcutRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutRegistry {
    /// Creates and populates the single shortcut registry.
    pub fn new() -> Self {
        let mut reg = Self {
            shortcuts: Vec::new(),
        };
        reg.register_all();
        reg
    }

    fn register_all(&mut self) {
        let mod_name = KeyCombo::primary_modifier_name();

        // 1. Global shortcuts
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("1"),
            action: ShortcutAction::NavHome,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.nav_home",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+1" } else { "Ctrl+1" }),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("2"),
            action: ShortcutAction::NavQuiz,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.nav_quiz",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+2" } else { "Ctrl+2" }),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("3"),
            action: ShortcutAction::NavQuestions,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.nav_questions",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+3" } else { "Ctrl+3" }),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("4"),
            action: ShortcutAction::NavStats,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.nav_stats",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+4" } else { "Ctrl+4" }),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("5"),
            action: ShortcutAction::NavSettings,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.nav_settings",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+5" } else { "Ctrl+5" }),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("escape"),
            action: ShortcutAction::CloseOrBack,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.close_or_back",
            custom_hint: Some("Esc"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::question_mark(),
            action: ShortcutAction::ShowHelp,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.help",
            custom_hint: Some("?"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::f1(),
            action: ShortcutAction::ShowHelp,
            scopes: vec![ShortcutScope::Global],
            label_key: "shortcuts.help",
            custom_hint: Some("F1"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("i"),
            action: ShortcutAction::ToggleTheme,
            scopes: vec![ShortcutScope::Global],
            label_key: "settings.theme",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+I" } else { "Ctrl+I" }),
        });

        // 2. Home screen shortcuts (mode picker): 1/2/3 select, Enter starts the selected mode
        for (key, n) in [("1", 1u8), ("2", 2), ("3", 3)] {
            self.shortcuts.push(ShortcutDef {
                key: KeyCombo::plain(key),
                action: ShortcutAction::SelectQuizMode(n),
                scopes: vec![ShortcutScope::Home],
                label_key: "shortcuts.quiz_pick_mode",
                custom_hint: Some(key),
            });
        }
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("enter"),
            action: ShortcutAction::StartSelectedQuiz,
            scopes: vec![ShortcutScope::Home],
            label_key: "shortcuts.quiz_start_mode",
            custom_hint: Some("Enter"),
        });

        // 3. Quiz screen shortcuts
        // Choices 1..4 in all quiz modes
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("1"),
            action: ShortcutAction::ChooseOption('a'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("1"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("2"),
            action: ShortcutAction::ChooseOption('b'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("2"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("3"),
            action: ShortcutAction::ChooseOption('c'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("3"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("4"),
            action: ShortcutAction::ChooseOption('d'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("4"),
        });

        // Letter choices A-D in all quiz modes.
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("a"),
            action: ShortcutAction::ChooseOption('a'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("A"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("b"),
            action: ShortcutAction::ChooseOption('b'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("B"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("c"),
            action: ShortcutAction::ChooseOption('c'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("C"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("d"),
            action: ShortcutAction::ChooseOption('d'),
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.choose_option",
            custom_hint: Some("D"),
        });

        // Enter: Next (Easy/Medium) or Confirm (Hard)
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("enter"),
            action: ShortcutAction::NextOrConfirm,
            scopes: vec![ShortcutScope::QuizAll],
            label_key: "shortcuts.next_or_confirm",
            custom_hint: Some("Enter"),
        });

        // 4. Easy mode navigation & features
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("arrowleft"),
            action: ShortcutAction::PrevQuestion,
            scopes: vec![ShortcutScope::QuizEasy],
            label_key: "shortcuts.prev_question",
            custom_hint: Some("←"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("arrowright"),
            action: ShortcutAction::NextQuestion,
            scopes: vec![ShortcutScope::QuizEasy],
            label_key: "shortcuts.next_question",
            custom_hint: Some("→"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("k"),
            action: ShortcutAction::StarQuestion,
            scopes: vec![ShortcutScope::QuizEasy, ShortcutScope::QuizMedium],
            label_key: "shortcuts.star_question",
            custom_hint: Some("K"),
        });

        // 5. Medium mode navigation & features
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("arrowleft"),
            action: ShortcutAction::PrevQuestion,
            scopes: vec![ShortcutScope::QuizMedium],
            label_key: "shortcuts.prev_question",
            custom_hint: Some("←"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("arrowright"),
            action: ShortcutAction::NextQuestion,
            scopes: vec![ShortcutScope::QuizMedium],
            label_key: "shortcuts.next_question",
            custom_hint: Some("→"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("m"),
            action: ShortcutAction::FlagQuestion,
            scopes: vec![ShortcutScope::QuizMedium],
            label_key: "shortcuts.flag_question",
            custom_hint: Some("M"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl_enter(),
            action: ShortcutAction::FinishExam,
            scopes: vec![ShortcutScope::QuizMedium],
            label_key: "shortcuts.finish_quiz",
            custom_hint: Some(if mod_name == "Cmd" {
                "Cmd+Enter"
            } else {
                "Ctrl+Enter"
            }),
        });

        // 6. Results screen shortcuts
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("r"),
            action: ShortcutAction::RetryQuiz,
            scopes: vec![ShortcutScope::Results],
            label_key: "shortcuts.retry_quiz",
            custom_hint: Some("R"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("w"),
            action: ShortcutAction::RetryMistakes,
            scopes: vec![ShortcutScope::Results],
            label_key: "shortcuts.retry_mistakes",
            custom_hint: Some("W"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("1"),
            action: ShortcutAction::FilterResultsAll,
            scopes: vec![ShortcutScope::Results],
            label_key: "shortcuts.filter_all",
            custom_hint: Some("1"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("2"),
            action: ShortcutAction::FilterResultsCorrect,
            scopes: vec![ShortcutScope::Results],
            label_key: "shortcuts.filter_correct",
            custom_hint: Some("2"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("3"),
            action: ShortcutAction::FilterResultsWrong,
            scopes: vec![ShortcutScope::Results],
            label_key: "shortcuts.filter_mistakes",
            custom_hint: Some("3"),
        });

        // 7. Questions screen shortcuts
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::slash(),
            action: ShortcutAction::FocusSearch,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.focus_search",
            custom_hint: Some("/"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("f"),
            action: ShortcutAction::FocusSearch,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.focus_search",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+F" } else { "Ctrl+F" }),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("arrowup"),
            action: ShortcutAction::MoveUp,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.move_up",
            custom_hint: Some("↑"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("arrowdown"),
            action: ShortcutAction::MoveDown,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.move_down",
            custom_hint: Some("↓"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("enter"),
            action: ShortcutAction::ToggleExpand,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.toggle_expand",
            custom_hint: Some("Enter"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("k"),
            action: ShortcutAction::ToggleStarSelected,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.star_question",
            custom_hint: Some("K"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("h"),
            action: ShortcutAction::ToggleHideAnswers,
            scopes: vec![ShortcutScope::Questions],
            label_key: "shortcuts.toggle_hide",
            custom_hint: Some("H"),
        });

        // 8. Stats screen shortcuts (1-4 filters)
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("1"),
            action: ShortcutAction::FilterStats(1),
            scopes: vec![ShortcutScope::Stats],
            label_key: "shortcuts.filter_stats",
            custom_hint: Some("1"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("2"),
            action: ShortcutAction::FilterStats(2),
            scopes: vec![ShortcutScope::Stats],
            label_key: "shortcuts.filter_stats",
            custom_hint: Some("2"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("3"),
            action: ShortcutAction::FilterStats(3),
            scopes: vec![ShortcutScope::Stats],
            label_key: "shortcuts.filter_stats",
            custom_hint: Some("3"),
        });
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::plain("4"),
            action: ShortcutAction::FilterStats(4),
            scopes: vec![ShortcutScope::Stats],
            label_key: "shortcuts.filter_stats",
            custom_hint: Some("4"),
        });

        // 9. Settings screen shortcuts
        self.shortcuts.push(ShortcutDef {
            key: KeyCombo::ctrl("s"),
            action: ShortcutAction::SaveSettings,
            scopes: vec![ShortcutScope::Settings],
            label_key: "shortcuts.save_settings",
            custom_hint: Some(if mod_name == "Cmd" { "Cmd+S" } else { "Ctrl+S" }),
        });
    }

    /// All registered shortcut definitions.
    pub fn all(&self) -> &[ShortcutDef] {
        &self.shortcuts
    }

    /// Returns all shortcuts active for a specific screen and optional quiz mode.
    pub fn shortcuts_for_context(
        &self,
        screen: crate::Screen,
        mode: Option<QuizMode>,
    ) -> Vec<&ShortcutDef> {
        self.shortcuts
            .iter()
            .filter(|def| def.applies_to(screen.clone(), mode))
            .collect()
    }

    /// Resolves an incoming key event to a shortcut action, obeying all mode rules, typing rules, and settings.
    pub fn resolve_event(
        &self,
        event: &gpui::KeyDownEvent,
        screen: crate::Screen,
        mode: Option<QuizMode>,
        shortcuts_enabled: bool,
        is_text_input_focused: bool,
    ) -> Option<ShortcutAction> {
        if !shortcuts_enabled {
            return None;
        }

        let ev_key = event.keystroke.key.to_lowercase();
        let ev_mod = event.keystroke.modifiers;
        let is_ctrl = if cfg!(target_os = "macos") {
            ev_mod.platform
        } else {
            ev_mod.control
        };

        // Rule: single-key shortcuts are off while a text input is focused (only Esc and Ctrl combos work)
        let is_single_key = !is_ctrl && !ev_mod.alt && ev_key != "escape" && ev_key != "f1";
        if is_text_input_focused && is_single_key {
            return None;
        }

        let active_shortcuts = self.shortcuts_for_context(screen, mode);
        for def in active_shortcuts {
            if def.key.matches(event) {
                if event.is_held
                    && matches!(
                        def.action,
                        ShortcutAction::ChooseOption(_)
                            | ShortcutAction::StarQuestion
                            | ShortcutAction::ToggleStarSelected
                    )
                {
                    return None;
                }
                return Some(def.action);
            }
        }

        None
    }

    /// Checks if advancing Next in Easy mode is allowed (blocked before answering).
    pub fn can_advance_easy_next(attempt: &Attempt) -> bool {
        if attempt.mode.provides_instant_feedback() {
            attempt.answers.contains_key(&attempt.current_index)
        } else {
            true
        }
    }

    /// Generates tooltip string for a given action from the registry.
    pub fn tooltip_for_action(
        &self,
        action: ShortcutAction,
        lang: Language,
        is_desktop: bool,
        shortcuts_enabled: bool,
    ) -> Option<String> {
        if !is_desktop || !shortcuts_enabled {
            return None;
        }

        let def = self.shortcuts.iter().find(|d| d.action == action)?;
        let key_str = def
            .custom_hint
            .map(|s| s.to_string())
            .unwrap_or_else(|| def.key.display_string());
        let label = t(def.label_key, lang);
        Some(format!("{label} ({key_str})"))
    }

    /// Generates short shortcut hint badge string (e.g. "Enter", "Ctrl+S") for a button.
    pub fn shortcut_hint(&self, action: ShortcutAction) -> Option<String> {
        let def = self.shortcuts.iter().find(|d| d.action == action)?;
        Some(
            def.custom_hint
                .map(|s| s.to_string())
                .unwrap_or_else(|| def.key.display_string()),
        )
    }

    /// Renders the Help Dialog generated from the registry.
    pub fn render_help_dialog<V: 'static>(
        &self,
        lang: Language,
        scroll_handle: &ScrollHandle,
        reveal_scrollbar: bool,
        cx: &mut Context<V>,
        on_close: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static + Copy,
    ) -> AnyElement {
        let theme = cx.theme();
        let colors = theme.colors;
        let mod_name = KeyCombo::primary_modifier_name();

        let groups: Vec<(&'static str, Vec<(String, String)>)> = vec![
            (
                "shortcuts.global",
                vec![
                    (
                        format!("{mod_name}+1..5"),
                        t("shortcuts.nav_home", lang).to_string(),
                    ),
                    (
                        "Esc".to_string(),
                        t("shortcuts.close_or_back", lang).to_string(),
                    ),
                    (
                        format!("{mod_name}+I"),
                        t("settings.theme", lang).to_string(),
                    ),
                    ("? / F1".to_string(), t("shortcuts.help", lang).to_string()),
                ],
            ),
            (
                "shortcuts.home_mode",
                vec![
                    (
                        "1-3".to_string(),
                        t("shortcuts.quiz_pick_mode", lang).to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        t("shortcuts.quiz_start_mode", lang).to_string(),
                    ),
                ],
            ),
            (
                "nav.quiz",
                vec![
                    (
                        "A-D / 1-4".to_string(),
                        t("shortcuts.choose_option", lang).to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        t("shortcuts.next_or_confirm", lang).to_string(),
                    ),
                ],
            ),
            (
                "shortcuts.easy_mode",
                vec![
                    (
                        "← / →".to_string(),
                        t("shortcuts.next_question", lang).to_string(),
                    ),
                    (
                        "K".to_string(),
                        t("shortcuts.star_question", lang).to_string(),
                    ),
                ],
            ),
            (
                "shortcuts.medium_mode",
                vec![
                    (
                        "← / →".to_string(),
                        t("shortcuts.prev_question", lang).to_string(),
                    ),
                    (
                        "M".to_string(),
                        t("shortcuts.flag_question", lang).to_string(),
                    ),
                    (
                        "K".to_string(),
                        t("shortcuts.star_question", lang).to_string(),
                    ),
                    (
                        format!("{mod_name}+Enter"),
                        t("shortcuts.finish_quiz", lang).to_string(),
                    ),
                ],
            ),
            (
                "shortcuts.hard_mode",
                vec![
                    (
                        "A-D / 1-4".to_string(),
                        t("shortcuts.choose_option", lang).to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        t("quiz.confirm_answer", lang).to_string(),
                    ),
                ],
            ),
            (
                "shortcuts.results_mode",
                vec![
                    ("R".to_string(), t("shortcuts.retry_quiz", lang).to_string()),
                    (
                        "W".to_string(),
                        t("shortcuts.retry_mistakes", lang).to_string(),
                    ),
                    (
                        "1 / 2 / 3".to_string(),
                        format!(
                            "{} / {} / {}",
                            t("shortcuts.filter_all", lang),
                            t("shortcuts.filter_correct", lang),
                            t("shortcuts.filter_mistakes", lang)
                        ),
                    ),
                    (
                        "Esc".to_string(),
                        t("shortcuts.results_home", lang).to_string(),
                    ),
                ],
            ),
            (
                "shortcuts.questions_mode",
                vec![
                    (
                        format!("/ cyangwa {mod_name}+F"),
                        t("shortcuts.focus_search", lang).to_string(),
                    ),
                    (
                        "↑ / ↓".to_string(),
                        t("shortcuts.move_up", lang).to_string(),
                    ),
                    (
                        "Enter".to_string(),
                        t("shortcuts.toggle_expand", lang).to_string(),
                    ),
                    (
                        "K".to_string(),
                        t("shortcuts.star_question", lang).to_string(),
                    ),
                    (
                        "H".to_string(),
                        t("shortcuts.toggle_hide", lang).to_string(),
                    ),
                ],
            ),
            (
                "shortcuts.stats_mode",
                vec![(
                    "1-4".to_string(),
                    t("shortcuts.filter_stats", lang).to_string(),
                )],
            ),
            (
                "shortcuts.settings_mode",
                vec![(
                    format!("{mod_name}+S"),
                    t("shortcuts.save_settings", lang).to_string(),
                )],
            ),
        ];

        let mut group_els: Vec<AnyElement> = Vec::new();
        for (group_title_key, items) in groups {
            let mut item_els: Vec<AnyElement> = Vec::new();
            for (key, desc) in items {
                item_els.push(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .py_1()
                        .child(
                            div()
                                .text_xs()
                                .font_medium()
                                .text_color(colors.foreground)
                                .child(desc),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_md()
                                .bg(colors.background)
                                .border_1()
                                .border_color(colors.border)
                                .text_xs()
                                .font_bold()
                                .text_color(colors.foreground)
                                .child(key),
                        )
                        .into_any_element(),
                );
            }

            group_els.push(
                div()
                    .flex()
                    .flex_col()
                    .gap_1p5()
                    .p_3()
                    .rounded_xl()
                    .bg(colors.secondary)
                    .border_1()
                    .border_color(colors.border)
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(colors.primary)
                            .child(t(group_title_key, lang).to_string()),
                    )
                    .children(item_els)
                    .into_any_element(),
            );
        }

        // Modal backdrop overlay containing centered dialog container
        div()
            .id("shortcuts_dialog_backdrop")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.65,
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                on_close(this, window, cx);
            }))
            .child(
                div()
                    .id("shortcuts_dialog_container")
                    .flex()
                    .flex_col()
                    .w(px(580.0))
                    .max_w(px(640.0))
                    .p_5()
                    .rounded_2xl()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.background)
                    .gap_4()
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, _| {})
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Icon::new(IconName::BookOpen)
                                            .size(px(20.0))
                                            .text_color(colors.primary),
                                    )
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_bold()
                                            .text_color(colors.foreground)
                                            .child(t("shortcuts.title", lang).to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .id("close_shortcuts_icon_btn")
                                    .cursor_pointer()
                                    .p_1()
                                    .rounded_md()
                                    .hover(|el| el.bg(colors.secondary))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_close(this, window, cx);
                                    }))
                                    .child(
                                        Icon::new(IconName::CircleX)
                                            .size(px(18.0))
                                            .text_color(colors.muted_foreground),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .id("shortcuts_dialog_scroll_list")
                            .track_scroll(scroll_handle)
                            .flex()
                            .flex_col()
                            .gap_3()
                            .max_h(px(400.0))
                            .overflow_y_scroll()
                            .pr_3()
                            .child(vertical_scrollbar(
                                "shortcuts_dialog_scrollbar",
                                scroll_handle,
                                true,
                                reveal_scrollbar,
                            ))
                            .children(group_els),
                    )
                    .child(
                        div().flex().flex_row().justify_end().pt_2().child(
                            Button::new("close_shortcuts_btn")
                                .primary()
                                .label(t("shortcuts.help_close", lang))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_close(this, window, cx);
                                })),
                        ),
                    ),
            )
            .into_any_element()
    }
}
