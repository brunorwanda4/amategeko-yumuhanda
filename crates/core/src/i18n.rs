use crate::models::Language;
use std::collections::HashMap;
use std::sync::OnceLock;

pub static BUNDLED_I18N_EN_JSON: &str = include_str!("../../../assets/i18n/en.json");
pub static BUNDLED_I18N_RW_JSON: &str = include_str!("../../../assets/i18n/rw.json");

static EN_STRINGS: OnceLock<HashMap<String, String>> = OnceLock::new();
static RW_STRINGS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn en_strings() -> &'static HashMap<String, String> {
    EN_STRINGS.get_or_init(|| {
        serde_json::from_str(BUNDLED_I18N_EN_JSON).expect("en.json must be valid JSON")
    })
}

pub fn rw_strings() -> &'static HashMap<String, String> {
    RW_STRINGS.get_or_init(|| {
        serde_json::from_str(BUNDLED_I18N_RW_JSON).expect("rw.json must be valid JSON")
    })
}

pub struct I18n;

impl I18n {
    /// Looks up a translation for `key` in `lang`.
    /// If missing in Kinyarwanda, falls back to English.
    /// If missing in English, returns the `key`.
    pub fn t(key: &str, lang: Language) -> &str {
        match lang {
            Language::Rw => {
                if let Some(val) = rw_strings().get(key) {
                    return val.as_str();
                }
                if let Some(val) = en_strings().get(key) {
                    return val.as_str();
                }
            }
            Language::En => {
                if let Some(val) = en_strings().get(key) {
                    return val.as_str();
                }
            }
        }
        key
    }

    /// Looks up a translation for `key` in `lang` and replaces `{param}` placeholders.
    pub fn tf(key: &str, lang: Language, args: &[(&str, &str)]) -> String {
        let mut text = Self::t(key, lang).to_string();
        for (k, v) in args {
            text = text.replace(&format!("{{{k}}}"), v);
        }
        text
    }
}

/// Shorthand translation lookup.
pub fn t(key: &str, lang: Language) -> &str {
    I18n::t(key, lang)
}

/// Shorthand formatted translation lookup.
pub fn tf(key: &str, lang: Language, args: &[(&str, &str)]) -> String {
    I18n::tf(key, lang, args)
}

pub struct Strings;

impl Strings {
    // App & Nav
    pub const APP_TITLE: &'static str = "Amategeko y'Umuhanda";
    pub const NAV_HOME: &'static str = "Home";
    pub const NAV_QUIZ: &'static str = "Quiz";
    pub const NAV_QUESTIONS: &'static str = "Questions";
    pub const NAV_STATS: &'static str = "Statistics";
    pub const NAV_SETTINGS: &'static str = "Settings";

    // Home Screen
    pub const HOME_WELCOME: &'static str = "Prepare for Rwanda's official driving theory exam";
    pub const HOME_CHOOSE_MODE: &'static str = "Choose practice mode:";
    pub const HOME_RESUME_TITLE: &'static str = "You have an unfinished quiz";
    pub const HOME_RESUME_BTN: &'static str = "Resume Quiz";
    pub const HOME_DISCARD_BTN: &'static str = "Discard";
    pub const HOME_WEAK_TITLE: &'static str = "Frequently Missed Questions";
    pub const HOME_WEAK_DESC: &'static str =
        "Practice the questions you miss most often to master them";
    pub const HOME_WEAK_BTN: &'static str = "Practice Missed Questions";
    pub const HOME_START: &'static str = "Start";

    // Modes
    pub const MODE_EASY_TITLE: &'static str = "Easy";
    pub const MODE_EASY_DESC: &'static str =
        "Learning mode: instant feedback, untimed, locked answer";
    pub const MODE_MEDIUM_TITLE: &'static str = "Medium";
    pub const MODE_MEDIUM_DESC: &'static str =
        "Exam practice: 20 minutes, changeable answers, can skip";
    pub const MODE_HARD_TITLE: &'static str = "Hard";
    pub const MODE_HARD_DESC: &'static str = "Strict exam: 12 minutes, no going back or skipping";

    // Quiz Common
    pub const QUIZ_QUESTION_PROGRESS: &'static str = "Question {current} of {total}";
    pub const QUIZ_PREVIOUS: &'static str = "Previous";
    pub const QUIZ_NEXT: &'static str = "Next";
    pub const QUIZ_SKIP: &'static str = "Skip";
    pub const QUIZ_CONFIRM_ANSWER: &'static str = "Confirm Answer";
    pub const QUIZ_FINISH: &'static str = "Finish Quiz";
    pub const QUIZ_FLAG: &'static str = "Flag Question";
    pub const QUIZ_UNFLAG: &'static str = "Remove Flag";
    pub const QUIZ_STAR: &'static str = "Bookmark Question";
    pub const QUIZ_UNSTAR: &'static str = "Remove Bookmark";
    pub const QUIZ_NO_IMAGE: &'static str = "Road sign image appears here";
    pub const QUIZ_SHORTCUT_HINT: &'static str = "A-D or 1-4 to select; Enter to continue";

    // Quiz Feedback (Easy mode)
    pub const QUIZ_EASY_HINT: &'static str = "Tap an option to check if it is correct";
    pub const QUIZ_EASY_CORRECT: &'static str = "Correct! The right answer is {option}.";
    pub const QUIZ_EASY_WRONG: &'static str = "Incorrect. The right answer is {option}.";

    // Hard mode errors
    pub const QUIZ_HARD_NO_SELECTION: &'static str = "Select an answer before confirming";

    // Timer Warning States (Medium & Hard)
    pub const TIMER_WARNING_RW: &'static str = "Igihe kiregereje. Hasigaye {time}.";
    pub const TIMER_WARNING_EN: &'static str = "Time is running low. {time} left.";
    pub const TIMER_ERROR_RW: &'static str = "Igihe kigiye kurangira! Hasigaye {time}.";
    pub const TIMER_ERROR_EN: &'static str = "Time is almost up! {time} left.";
    pub const TIMER_DONE_RW: &'static str = "Igihe kirarangiye. Ikizamini cyoherejwe.";
    pub const TIMER_DONE_EN: &'static str = "Time is up. Exam submitted.";

    pub const TIMER_WARNING: &'static str = Self::TIMER_WARNING_EN;
    pub const TIMER_ERROR: &'static str = Self::TIMER_ERROR_EN;
    pub const TIMER_DONE: &'static str = Self::TIMER_DONE_EN;

    pub fn timer_warning(time: &str) -> String {
        Self::timer_warning_for(time, Language::En)
    }

    pub fn timer_warning_for(time: &str, lang: Language) -> String {
        I18n::tf("timer.warning", lang, &[("time", time)])
    }

    pub fn timer_warning_rw(time: &str) -> String {
        Self::timer_warning_for(time, Language::Rw)
    }

    pub fn timer_error(time: &str) -> String {
        Self::timer_error_for(time, Language::En)
    }

    pub fn timer_error_for(time: &str, lang: Language) -> String {
        I18n::tf("timer.error", lang, &[("time", time)])
    }

    pub fn timer_error_rw(time: &str) -> String {
        Self::timer_error_for(time, Language::Rw)
    }

    pub fn timer_done() -> &'static str {
        Self::timer_done_for(Language::En)
    }

    pub fn timer_done_for(lang: Language) -> &'static str {
        I18n::t("timer.done", lang)
    }

    pub fn timer_done_rw() -> &'static str {
        Self::timer_done_for(Language::Rw)
    }

    // Finish / Abandon Dialogs
    pub const DIALOG_FINISH_TITLE: &'static str = "Finish Quiz?";
    pub const DIALOG_UNANSWERED_WARNING: &'static str =
        "There are still {count} unanswered questions: {list}";
    pub const DIALOG_ALL_ANSWERED: &'static str = "All {count} questions have been answered.";
    pub const DIALOG_CONFIRM_FINISH: &'static str = "Confirm & Finish";
    pub const DIALOG_CANCEL: &'static str = "Continue Quiz";

    pub const DIALOG_ABANDON_TITLE: &'static str = "Leave Quiz?";
    pub const DIALOG_ABANDON_DESC_SAVABLE: &'static str =
        "Are you sure you want to leave? Your progress will be saved so you can resume later.";
    pub const DIALOG_ABANDON_DESC_STRICT: &'static str =
        "In Strict mode, leaving will end and submit your exam permanently.";
    pub const DIALOG_ABANDON_CONFIRM: &'static str = "Leave";

    // Results Screen
    pub const RESULTS_TITLE: &'static str = "Quiz Results";
    pub const RESULTS_PASSED: &'static str = "You Passed!";
    pub const RESULTS_FAILED: &'static str = "You Did Not Pass, Keep Practicing!";
    pub const RESULTS_SCORE: &'static str = "Your Score: {score} out of {total}";
    pub const RESULTS_PASS_MARK: &'static str = "Pass mark required: {pass_mark}/{total}";
    pub const RESULTS_DURATION: &'static str = "Time Taken: {duration}";
    pub const RESULTS_RETRY_WRONG: &'static str = "Retry Missed Questions";
    pub const RESULTS_NEW_QUIZ: &'static str = "Start New Quiz";
    pub const RESULTS_FILTER_ALL: &'static str = "All ({count})";
    pub const RESULTS_FILTER_CORRECT: &'static str = "Correct ({count})";
    pub const RESULTS_FILTER_WRONG: &'static str = "Mistakes ({count})";
    pub const RESULTS_YOUR_ANSWER: &'static str = "Your answer: {answer}";
    pub const RESULTS_CORRECT_ANSWER: &'static str = "Correct answer: {answer}";
    pub const RESULTS_UNANSWERED: &'static str = "Not answered";

    // Questions Screen (Question Bank)
    pub const QUESTIONS_SEARCH_PLACEHOLDER: &'static str = "Search questions or type a number...";
    pub const QUESTIONS_FILTER_ALL: &'static str = "All";
    pub const QUESTIONS_FILTER_IMAGE: &'static str = "With Images";
    pub const QUESTIONS_FILTER_MISTAKES: &'static str = "Mistakes";
    pub const QUESTIONS_FILTER_STARRED: &'static str = "Starred";
    pub const QUESTIONS_HIDE_ANSWERS: &'static str = "Hide Answers";
    pub const QUESTIONS_COUNT_INFO: &'static str = "Showing {visible} of {total} questions";

    // Statistics Screen
    pub const STATS_TITLE: &'static str = "Statistics & Progress";
    pub const STATS_TOTAL_ATTEMPTS: &'static str = "Total Quizzes";
    pub const STATS_AVERAGE_SCORE: &'static str = "Average Score";
    pub const STATS_HIGH_SCORE: &'static str = "High Score";
    pub const STATS_PASS_RATE: &'static str = "Pass Rate";
    pub const STATS_RECENT_CHART_TITLE: &'static str = "Last 10 Quizzes";
    pub const STATS_THRESHOLD_LABEL: &'static str = "Pass Mark";
    pub const STATS_TOP_MISSED_TITLE: &'static str = "Most Frequently Missed Questions";
    pub const STATS_MISSED_COUNT: &'static str = "Missed {wrong} times out of {seen}";
    pub const STATS_QUESTIONS_SEEN: &'static str = "Questions Seen: {seen} of {total}";
    pub const STATS_EMPTY: &'static str =
        "No quizzes completed yet. Complete a quiz to see your statistics!";

    // Settings Screen
    pub const SETTINGS_TITLE: &'static str = "Settings";
    pub const SETTINGS_PASS_MARK: &'static str = "Pass mark (out of 20)";
    pub const SETTINGS_MEDIUM_TIME: &'static str = "Medium mode duration (minutes)";
    pub const SETTINGS_HARD_TIME: &'static str = "Hard mode duration (minutes)";
    pub const SETTINGS_EASY_TIMER: &'static str = "Show timer in Easy mode";
    pub const SETTINGS_HARD_WEIGHT: &'static str = "Prioritize road signs in Hard mode";
    pub const SETTINGS_DESKTOP_SHORTCUTS: &'static str =
        "Enable keyboard shortcuts (A-D, Enter, F)";
    pub const SETTINGS_THEME: &'static str = "Theme";
    pub const SETTINGS_THEME_SYSTEM: &'static str = "System";
    pub const SETTINGS_THEME_LIGHT: &'static str = "Light";
    pub const SETTINGS_THEME_DARK: &'static str = "Dark";
    pub const SETTINGS_FONT_SIZE: &'static str = "Font size";
    pub const SETTINGS_FONT_PREVIEW: &'static str = "Sample text: Road Rules of Rwanda";
    pub const SETTINGS_CLEAR_HISTORY: &'static str = "Clear exam history and statistics";
    pub const SETTINGS_CLEAR_CONFIRM: &'static str =
        "Are you sure you want to delete all exam history and statistics?";
    pub const SETTINGS_SAVE_BTN: &'static str = "Save Changes";
}
