use crate::data::QuestionBank;
use crate::error::{AppError, Result};
use crate::models::{
    Attempt, AttemptResult, Question, QuestionResult, QuestionStat, QuizMode, Settings,
};
use std::collections::HashMap;

pub struct QuizEngine;

impl QuizEngine {
    /// Starts a new quiz attempt for the given mo  de.
    pub fn start_quiz(
        bank: &QuestionBank,
        mode: QuizMode,
        settings: &Settings,
        stats: &HashMap<u32, QuestionStat>,
        now_secs: u64,
    ) -> Result<Attempt> {
        let count = 20;
        let questions =
            bank.generate_quiz_questions(count, mode, settings.hard_weight_images, stats);

        if questions.is_empty() {
            return Err(AppError::InvalidData(
                "Nta bibazo bibonetse byo gutangiza ikizamini".to_string(),
            ));
        }

        let allowed_duration_secs = match mode {
            QuizMode::Hagati => Some((settings.medium_duration_mins as u64) * 60),
            QuizMode::Bikomeye => Some((settings.hard_duration_mins as u64) * 60),
            QuizMode::Byoroshye
            | QuizMode::WeakPractice
            | QuizMode::RetryWrong
            | QuizMode::Study => None,
        };

        let id = format!("attempt_{}_{}", mode.title_kinyarwanda(), now_secs);

        Ok(Attempt::new(
            id,
            mode,
            questions,
            now_secs,
            allowed_duration_secs,
        ))
    }

    /// Starts a quiz with an explicit set of pre-filtered questions.
    pub fn start_quiz_with_questions(
        mode: QuizMode,
        questions: Vec<Question>,
        settings: &Settings,
        now_secs: u64,
    ) -> Result<Attempt> {
        if questions.is_empty() {
            return Err(AppError::InvalidData(
                "Nta bibazo bibonetse byo gutangiza ikizamini".to_string(),
            ));
        }

        let allowed_duration_secs = match mode {
            QuizMode::Hagati => Some((settings.medium_duration_mins as u64) * 60),
            QuizMode::Bikomeye => Some((settings.hard_duration_mins as u64) * 60),
            QuizMode::Byoroshye
            | QuizMode::WeakPractice
            | QuizMode::RetryWrong
            | QuizMode::Study => None,
        };

        let id = format!("attempt_{}_{}", mode.title_kinyarwanda(), now_secs);

        Ok(Attempt::new(
            id,
            mode,
            questions,
            now_secs,
            allowed_duration_secs,
        ))
    }

    /// Starts a "Retry Wrong" quiz from the failed or unanswered questions of a prior result.
    pub fn start_retry_wrong(previous_result: &AttemptResult, now_secs: u64) -> Result<Attempt> {
        let wrong_questions: Vec<Question> = previous_result
            .question_results
            .iter()
            .filter(|qr| !qr.is_correct)
            .map(|qr| qr.question.clone())
            .collect();

        if wrong_questions.is_empty() {
            return Err(AppError::InvalidData(
                "Nta bibazo wakosheje bibonetse byo gusubiramo".to_string(),
            ));
        }

        let id = format!("retry_{}", now_secs);

        Ok(Attempt::new(
            id,
            QuizMode::RetryWrong,
            wrong_questions,
            now_secs,
            None, // Retry wrong uses Easy rules without countdown
        ))
    }

    /// Selects an option for the current question.
    /// In Easy mode, the answer is locked immediately upon first tap.
    /// In Medium mode, the answer can be freely changed.
    /// In Hard mode, the answer is selected but not confirmed/locked until confirm_and_advance.
    pub fn select_option(attempt: &mut Attempt, option_letter: &str) -> Result<()> {
        let idx = attempt.current_index;
        if idx >= attempt.questions.len() {
            return Err(AppError::InvalidData(
                "Ikibazo kirenze umubare w'ibibazo".into(),
            ));
        }

        if attempt.mode.provides_instant_feedback() {
            // Easy mode: lock on first answer
            if !attempt.locked.get(&idx).copied().unwrap_or(false) {
                attempt.answers.insert(idx, option_letter.to_lowercase());
                attempt.locked.insert(idx, true);
            }
        } else {
            // Medium or Hard mode: selection can be updated
            attempt.answers.insert(idx, option_letter.to_lowercase());
        }

        Ok(())
    }

    /// Advances to the next question in Hard mode. Requires an option to be selected.
    pub fn confirm_and_advance_hard(attempt: &mut Attempt) -> Result<bool> {
        let idx = attempt.current_index;
        if !attempt.answers.contains_key(&idx) {
            return Err(AppError::NoSelectionMade);
        }

        attempt.locked.insert(idx, true);

        if idx + 1 < attempt.questions.len() {
            attempt.current_index += 1;
            Ok(false) // Not finished
        } else {
            Ok(true) // Reached the end
        }
    }

    /// Navigates to the next question (Easy or Medium).
    pub fn next_question(attempt: &mut Attempt) -> bool {
        if attempt.current_index + 1 < attempt.questions.len() {
            attempt.current_index += 1;
            true
        } else {
            false
        }
    }

    /// Navigates to the previous question (Easy or Medium).
    pub fn previous_question(attempt: &mut Attempt) -> bool {
        if !attempt.mode.allows_navigation_back() {
            return false;
        }

        if attempt.current_index > 0 {
            attempt.current_index -= 1;
            true
        } else {
            false
        }
    }

    /// Skips the current question (Easy or Medium).
    pub fn skip_question(attempt: &mut Attempt) -> bool {
        Self::next_question(attempt)
    }

    /// Jumps to a specific question index (Medium mode question grid).
    pub fn jump_to_question(attempt: &mut Attempt, target_index: usize) -> bool {
        if !attempt.mode.has_question_grid() {
            return false;
        }

        if target_index < attempt.questions.len() {
            attempt.current_index = target_index;
            true
        } else {
            false
        }
    }

    /// Toggles the flag on the current question (Medium mode).
    pub fn toggle_current_flag(attempt: &mut Attempt) -> bool {
        let idx = attempt.current_index;
        if attempt.flags.contains(&idx) {
            attempt.flags.remove(&idx);
            false
        } else {
            attempt.flags.insert(idx);
            true
        }
    }

    /// Completes the attempt and computes the final `AttemptResult`.
    pub fn finish_attempt(
        attempt: &mut Attempt,
        pass_mark: u32,
        finish_time_secs: u64,
    ) -> AttemptResult {
        attempt.completed = true;

        let duration_seconds = (finish_time_secs.saturating_sub(attempt.start_time_secs)) as u32;
        let mut score: u32 = 0;
        let mut question_results = Vec::with_capacity(attempt.questions.len());

        for (idx, q) in attempt.questions.iter().enumerate() {
            let user_ans = attempt.answers.get(&idx).cloned();
            let is_correct = match &user_ans {
                Some(ans) => ans.eq_ignore_ascii_case(&q.correct),
                None => false,
            };

            if is_correct {
                score = score.saturating_add(1);
            }

            question_results.push(QuestionResult {
                question: q.clone(),
                user_answer: user_ans,
                correct_answer: q.correct.clone(),
                is_correct,
            });
        }

        let total = attempt.questions.len() as u32;
        let passed = score >= pass_mark;

        AttemptResult {
            id: attempt.id.clone(),
            mode: attempt.mode,
            score,
            total,
            pass_mark,
            passed,
            timestamp_secs: finish_time_secs,
            duration_seconds,
            question_results,
        }
    }
}
