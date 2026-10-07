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
    /// Ordered, resumable study of the full question bank using Easy rules.
    Study,
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
                QuizMode::Study => "Study",
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
            QuizMode::Study => "Kwiga",
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
            QuizMode::Byoroshye | QuizMode::WeakPractice | QuizMode::RetryWrong | QuizMode::Study
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
    /// Show the answer options of each quiz question in a different order.
    /// Stored answers and the correct answer keep their original keys.
    #[serde(default)]
    pub shuffle_options: bool,
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
            shuffle_options: false,
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

    /// Option keys of question `idx` in the order they are shown on screen.
    ///
    /// Without `shuffle` this is the original order (a, b, c, d). With `shuffle`
    /// the order is mixed, but it only depends on the attempt id and the question,
    /// so it stays the same across re-renders and when an attempt is resumed.
    /// The keys themselves never change, so saved answers and `correct` stay valid.
    pub fn option_order(&self, idx: usize, lang: Language, shuffle: bool) -> Vec<String> {
        let Some(question) = self.questions.get(idx) else {
            return Vec::new();
        };
        let mut keys: Vec<String> = question.options_for(lang).keys().cloned().collect();
        if shuffle {
            let seed = stable_seed(&self.id, question.id, idx);
            shuffle_with_seed(&mut keys, seed);
        }
        keys
    }

    /// Option key shown at screen position `position` (0 = first row) of the current question.
    pub fn option_key_at(&self, lang: Language, shuffle: bool, position: usize) -> Option<String> {
        self.option_order(self.current_index, lang, shuffle)
            .into_iter()
            .nth(position)
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

/// FNV-1a hash. Written by hand so the result never changes between Rust versions
/// (an attempt saved before an update must show the same order after it).
fn stable_seed(attempt_id: &str, question_id: u32, idx: usize) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |byte: u8| {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    };
    attempt_id.bytes().for_each(&mut feed);
    question_id.to_le_bytes().into_iter().for_each(&mut feed);
    (idx as u64).to_le_bytes().into_iter().for_each(&mut feed);
    hash
}

/// Deterministic Fisher-Yates shuffle driven by a splitmix64 generator.
fn shuffle_with_seed<T>(items: &mut [T], seed: u64) {
    let mut state = seed;
    let mut next = || {
        state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    for i in (1..items.len()).rev() {
        let j = (next() % (i as u64 + 1)) as usize;
        items.swap(i, j);
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

/// Resumable state for one pass through the full ordered question bank.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StudyProgress {
    /// ID of the question that should be shown when study resumes.
    pub last_id: Option<u32>,
    /// Locked answer keyed by question ID for the current study round.
    pub answers: HashMap<u32, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    pub attempts: Vec<AttemptResult>,
    pub question_stats: HashMap<u32, QuestionStat>,
    pub starred_questions: HashSet<u32>,
    pub active_attempt: Option<Attempt>,
    #[serde(default)]
    pub study: StudyProgress,
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

    pub fn record_study_answer(&mut self, question_id: u32, answer: &str, is_correct: bool) {
        if self.study.answers.contains_key(&question_id) {
            return;
        }
        self.study
            .answers
            .insert(question_id, answer.to_lowercase());
        self.question_stats
            .entry(question_id)
            .or_insert_with(|| QuestionStat::new(question_id))
            .record_answer(is_correct);
    }
}

/// Finds the saved question in sorted IDs, falling forward when the ID disappeared.
pub fn study_index(sorted_ids: &[u32], last_id: Option<u32>) -> usize {
    if sorted_ids.is_empty() {
        return 0;
    }
    let Some(last_id) = last_id else {
        return 0;
    };
    sorted_ids.iter().position(|id| *id >= last_id).unwrap_or(0)
}

/// Returns the completed whole-number percentage, bounded to 0 through 100.
pub fn percent_done(answered: usize, total: usize) -> u32 {
    answered
        .min(total)
        .saturating_mul(100)
        .checked_div(total)
        .unwrap_or(0) as u32
}

/// Clamps a one-based question number and returns its zero-based index.
pub fn clamp_jump(question_number: usize, total: usize) -> usize {
    if total == 0 {
        0
    } else {
        question_number.clamp(1, total) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn study_helpers_cover_resume_percent_and_jump_bounds() {
        let ids = [1, 3, 5];
        assert_eq!(study_index(&ids, Some(3)), 1);
        assert_eq!(study_index(&ids, Some(2)), 1);
        assert_eq!(study_index(&ids, Some(9)), 0);
        assert_eq!(study_index(&[], Some(1)), 0);
        assert_eq!(percent_done(2, 4), 50);
        assert_eq!(clamp_jump(0, 3), 0);
        assert_eq!(clamp_jump(4, 3), 2);
    }

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
