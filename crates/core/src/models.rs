use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Rw,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Rw => "rw",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Rw => "Ikinyarwanda",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub text: String,
    pub options: BTreeMap<String, String>,
    pub correct: String,
    pub image: Option<String>,
    pub has_image: bool,
    #[serde(default)]
    pub text_en: Option<String>,
    #[serde(default)]
    pub options_en: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub text_rw: Option<String>,
    #[serde(default)]
    pub options_rw: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub status_en: Option<String>,
}

impl Question {
    /// Returns the question text in the specified language, falling back to Kinyarwanda if English is missing.
    pub fn text_for(&self, lang: Language) -> &str {
        match lang {
            Language::En => {
                if let Some(en) = &self.text_en {
                    en.as_str()
                } else if let Some(rw) = &self.text_rw {
                    rw.as_str()
                } else {
                    &self.text
                }
            }
            Language::Rw => {
                if let Some(rw) = &self.text_rw {
                    rw.as_str()
                } else {
                    &self.text
                }
            }
        }
    }

    /// Returns secondary question text when "Show both languages" is enabled.
    pub fn secondary_text_for(&self, primary_lang: Language) -> Option<&str> {
        match primary_lang {
            Language::En => self.text_rw.as_deref().or(Some(&self.text)),
            Language::Rw => self.text_en.as_deref(),
        }
    }

    /// Returns options in the specified language, falling back to Kinyarwanda if English is missing.
    pub fn options_for(&self, lang: Language) -> &BTreeMap<String, String> {
        match lang {
            Language::En => {
                if let Some(opts) = &self.options_en {
                    opts
                } else if let Some(opts) = &self.options_rw {
                    opts
                } else {
                    &self.options
                }
            }
            Language::Rw => {
                if let Some(opts) = &self.options_rw {
                    opts
                } else {
                    &self.options
                }
            }
        }
    }

    /// Returns secondary options when "Show both languages" is enabled.
    pub fn secondary_options_for(
        &self,
        primary_lang: Language,
    ) -> Option<&BTreeMap<String, String>> {
        match primary_lang {
            Language::En => self.options_rw.as_ref().or(Some(&self.options)),
            Language::Rw => self.options_en.as_ref(),
        }
    }

    /// Returns secondary option text for a specific letter (a, b, c, d).
    pub fn secondary_option_for(&self, primary_lang: Language, letter: &str) -> Option<&str> {
        self.secondary_options_for(primary_lang)
            .and_then(|opts| opts.get(letter))
            .map(|s| s.as_str())
    }

    /// Whether the English question has draft status.
    pub fn is_draft_translation(&self) -> bool {
        self.status_en.as_deref() == Some("draft")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum QuestionSet {
    #[default]
    All,
    Signs,
    Rules,
    NotSeen,
    Starred,
    Mistakes,
}

impl QuestionSet {
    /// Every set, in the order the quiz page shows them.
    pub const ALL_SETS: [QuestionSet; 6] = [
        QuestionSet::All,
        QuestionSet::Signs,
        QuestionSet::Rules,
        QuestionSet::NotSeen,
        QuestionSet::Starred,
        QuestionSet::Mistakes,
    ];

    /// Whether a single question belongs to this set.
    pub fn contains(
        &self,
        q: &Question,
        starred: &std::collections::HashSet<u32>,
        stats: &std::collections::HashMap<u32, QuestionStat>,
    ) -> bool {
        match self {
            QuestionSet::All => true,
            QuestionSet::Signs => q.has_image,
            QuestionSet::Rules => !q.has_image,
            QuestionSet::NotSeen => stats.get(&q.id).map_or(0, |s| s.seen_count) == 0,
            QuestionSet::Starred => starred.contains(&q.id),
            QuestionSet::Mistakes => stats.get(&q.id).map_or(0, |s| s.wrong_count) > 0,
        }
    }

    /// Number of questions in this set, without cloning them.
    pub fn count(
        &self,
        questions: &[Question],
        starred: &std::collections::HashSet<u32>,
        stats: &std::collections::HashMap<u32, QuestionStat>,
    ) -> usize {
        questions
            .iter()
            .filter(|q| self.contains(q, starred, stats))
            .count()
    }

    pub fn filter_questions(
        &self,
        questions: &[Question],
        starred: &std::collections::HashSet<u32>,
        stats: &std::collections::HashMap<u32, QuestionStat>,
    ) -> Vec<Question> {
        questions
            .iter()
            .filter(|q| self.contains(q, starred, stats))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuizMode {
    /// Learning mode: instant feedback, no countdown, answer locked after first tap.
    Byoroshye,
    /// Exam simulation: 20m countdown, no instant feedback, question grid, flags.
    Hagati,
    /// Strict mode: 12m countdown, no going back, confirm required, weighted harder questions.
    Bikomeye,
    /// Practice mode focusing on most-missed questions (Easy rules).
    WeakPractice,
    /// Practice mode retrying wrong questions from the previous attempt (Easy rules).
    RetryWrong,
}

impl QuizMode {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::En => match self {
                QuizMode::Byoroshye => "Easy",
                QuizMode::Hagati => "Medium",
                QuizMode::Bikomeye => "Hard",
                QuizMode::WeakPractice => "Review Mistakes",
                QuizMode::RetryWrong => "Retry Mistakes",
            },
            Language::Rw => self.title_kinyarwanda(),
        }
    }

    pub fn title_kinyarwanda(&self) -> &'static str {
        match self {
            QuizMode::Byoroshye => "Byoroshye",
            QuizMode::Hagati => "Hagati",
            QuizMode::Bikomeye => "Bikomeye",
            QuizMode::WeakPractice => "Ibibazo nakosheje",
            QuizMode::RetryWrong => "Subiramo ibyo wakosheje",
        }
    }

    pub fn has_countdown(&self) -> bool {
        /*  */
        matches!(self, QuizMode::Hagati | QuizMode::Bikomeye)
    }

    pub fn allows_navigation_back(&self) -> bool {
        !matches!(self, QuizMode::Bikomeye)
    }

    pub fn has_question_grid(&self) -> bool {
        matches!(self, QuizMode::Hagati)
    }

    pub fn provides_instant_feedback(&self) -> bool {
        matches!(
            self,
            QuizMode::Byoroshye | QuizMode::WeakPractice | QuizMode::RetryWrong
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub easy_show_timer: bool,
    pub medium_duration_mins: u32,
    pub hard_duration_mins: u32,
    pub pass_mark: u32,
    pub theme: ThemeMode,
    pub font_size_scale: f32,
    pub desktop_shortcuts_enabled: bool,
    pub hard_weight_images: bool,
    pub study_hide_answers: bool,
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub question_language: Language,
    #[serde(default)]
    pub show_both_languages: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            easy_show_timer: false,
            medium_duration_mins: 20,
            hard_duration_mins: 12,
            pass_mark: 12,
            theme: ThemeMode::System,
            font_size_scale: 1.0,
            desktop_shortcuts_enabled: true,
            hard_weight_images: true,
            study_hide_answers: false,
            language: Language::En,
            question_language: Language::En,
            show_both_languages: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt {
    pub id: String,
    pub mode: QuizMode,
    pub questions: Vec<Question>,
    pub answers: HashMap<usize, String>,
    pub locked: HashMap<usize, bool>,
    pub flags: HashSet<usize>,
    pub current_index: usize,
    pub start_time_secs: u64,
    pub deadline_secs: Option<u64>,
    pub completed: bool,
}

impl Attempt {
    pub fn new(
        id: String,
        mode: QuizMode,
        questions: Vec<Question>,
        start_time_secs: u64,
        allowed_duration_secs: Option<u64>,
    ) -> Self {
        let deadline_secs = allowed_duration_secs.map(|dur| start_time_secs.saturating_add(dur));
        Self {
            id,
            mode,
            questions,
            answers: HashMap::new(),
            locked: HashMap::new(),
            flags: HashSet::new(),
            current_index: 0,
            start_time_secs,
            deadline_secs,
            completed: false,
        }
    }

    pub fn total_questions(&self) -> usize {
        self.questions.len()
    }

    pub fn answered_count(&self) -> usize {
        self.answers.len()
    }

    pub fn current_question(&self) -> Option<&Question> {
        self.questions.get(self.current_index)
    }

    pub fn current_answer(&self) -> Option<&String> {
        self.answers.get(&self.current_index)
    }

    pub fn is_current_locked(&self) -> bool {
        self.locked
            .get(&self.current_index)
            .copied()
            .unwrap_or(false)
    }

    pub fn is_current_flagged(&self) -> bool {
        self.flags.contains(&self.current_index)
    }

    pub fn unanswered_indices(&self) -> Vec<usize> {
        (0..self.questions.len())
            .filter(|i| !self.answers.contains_key(i))
            .collect()
    }

    /// Returns the total configured duration of this attempt in seconds, if a deadline was set.
    pub fn total_duration_secs(&self) -> Option<u32> {
        self.deadline_secs
            .map(|deadline| deadline.saturating_sub(self.start_time_secs) as u32)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionResult {
    pub question: Question,
    pub user_answer: Option<String>,
    pub correct_answer: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptResult {
    pub id: String,
    pub mode: QuizMode,
    pub score: u32,
    pub total: u32,
    pub pass_mark: u32,
    pub passed: bool,
    pub timestamp_secs: u64,
    pub duration_seconds: u32,
    pub question_results: Vec<QuestionResult>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionStat {
    pub question_id: u32,
    pub seen_count: u32,
    pub correct_count: u32,
    pub wrong_count: u32,
}

impl QuestionStat {
    pub fn new(question_id: u32) -> Self {
        Self {
            question_id,
            seen_count: 0,
            correct_count: 0,
            wrong_count: 0,
        }
    }

    pub fn record_answer(&mut self, is_correct: bool) {
        self.seen_count = self.seen_count.saturating_add(1);
        if is_correct {
            self.correct_count = self.correct_count.saturating_add(1);
        } else {
            self.wrong_count = self.wrong_count.saturating_add(1);
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    pub attempts: Vec<AttemptResult>,
    pub question_stats: HashMap<u32, QuestionStat>,
    pub starred_questions: HashSet<u32>,
    pub active_attempt: Option<Attempt>,
}

impl Progress {
    pub fn record_attempt_result(&mut self, result: AttemptResult) {
        for q_res in &result.question_results {
            let stat = self
                .question_stats
                .entry(q_res.question.id)
                .or_insert_with(|| QuestionStat::new(q_res.question.id));
            stat.record_answer(q_res.is_correct);
        }
        self.attempts.push(result);
        self.active_attempt = None;
    }

    pub fn toggle_starred(&mut self, question_id: u32) -> bool {
        if self.starred_questions.contains(&question_id) {
            self.starred_questions.remove(&question_id);
            false
        } else {
            self.starred_questions.insert(question_id);
            true
        }
    }

    pub fn is_starred(&self, question_id: u32) -> bool {
        self.starred_questions.contains(&question_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_set_filter() {
        let q1 = Question {
            id: 1,
            text: "Q1".into(),
            options: std::collections::BTreeMap::new(),
            correct: "a".into(),
            image: Some("sign.png".into()),
            has_image: true,
            text_en: None,
            options_en: None,
            text_rw: None,
            options_rw: None,
            status_en: None,
        };
        let q2 = Question {
            id: 2,
            text: "Q2".into(),
            options: std::collections::BTreeMap::new(),
            correct: "b".into(),
            image: None,
            has_image: false,
            text_en: None,
            options_en: None,
            text_rw: None,
            options_rw: None,
            status_en: None,
        };
        let questions = vec![q1, q2];
        let mut starred = std::collections::HashSet::new();
        starred.insert(1);
        let mut stats = std::collections::HashMap::new();
        stats.insert(
            2,
            QuestionStat {
                question_id: 2,
                seen_count: 1,
                correct_count: 0,
                wrong_count: 1,
            },
        );

        assert_eq!(
            QuestionSet::All
                .filter_questions(&questions, &starred, &stats)
                .len(),
            2
        );
        assert_eq!(
            QuestionSet::Signs
                .filter_questions(&questions, &starred, &stats)
                .len(),
            1
        );
        assert_eq!(
            QuestionSet::Rules
                .filter_questions(&questions, &starred, &stats)
                .len(),
            1
        );
        assert_eq!(
            QuestionSet::NotSeen
                .filter_questions(&questions, &starred, &stats)
                .len(),
            1
        );
        assert_eq!(
            QuestionSet::Starred
                .filter_questions(&questions, &starred, &stats)
                .len(),
            1
        );
        assert_eq!(
            QuestionSet::Mistakes
                .filter_questions(&questions, &starred, &stats)
                .len(),
            1
        );
    }
}
