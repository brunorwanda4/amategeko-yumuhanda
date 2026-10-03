use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub text: String,
    pub options: BTreeMap<String, String>,
    pub correct: String,
    pub image: Option<String>,
    pub has_image: bool,
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
        matches!(self, QuizMode::Hagati | QuizMode::Bikomeye)
    }

    pub fn allows_navigation_back(&self) -> bool {
        !matches!(self, QuizMode::Bikomeye)
    }

    pub fn has_question_grid(&self) -> bool {
        matches!(self, QuizMode::Hagati)
    }

    pub fn provides_instant_feedback(&self) -> bool {
        matches!(self, QuizMode::Byoroshye | QuizMode::WeakPractice | QuizMode::RetryWrong)
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            easy_show_timer: true,
            medium_duration_mins: 20,
            hard_duration_mins: 12,
            pass_mark: 12,
            theme: ThemeMode::System,
            font_size_scale: 1.0,
            desktop_shortcuts_enabled: true,
            hard_weight_images: true,
            study_hide_answers: false,
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
        self.locked.get(&self.current_index).copied().unwrap_or(false)
    }

    pub fn is_current_flagged(&self) -> bool {
        self.flags.contains(&self.current_index)
    }

    pub fn unanswered_indices(&self) -> Vec<usize> {
        (0..self.questions.len())
            .filter(|i| !self.answers.contains_key(i))
            .collect()
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
