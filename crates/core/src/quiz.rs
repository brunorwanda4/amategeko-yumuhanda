use crate::data::QuestionBank;
use crate::error::{AppError, Result};
use crate::models::{
    Attempt, AttemptResult, Question, QuestionResult, QuestionStat, QuizMode, Settings,
};
use std::collections::HashMap;

pub struct QuizEngine;

pub const SWIPE_NEXT_ON_LEFT: bool = true;
pub const SWIPE_AXIS_LOCK_DISTANCE: f32 = 12.0;
pub const SWIPE_MIN_DISTANCE: f32 = 72.0;
pub const SWIPE_MAX_DURATION_MS: u64 = 600;
pub const SWIPE_EDGE_GUARD: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub fn swipe_decision(
    start: (f32, f32),
    end: (f32, f32),
    elapsed_ms: u64,
    screen_w: f32,
    edge: f32,
) -> Option<Direction> {
    if start.0 < edge || start.0 > screen_w - edge || elapsed_ms >= SWIPE_MAX_DURATION_MS {
        return None;
    }

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    if dx.abs() < SWIPE_MIN_DISTANCE || dx.abs() < dy.abs() * 2.0 {
        return None;
    }

    Some(if dx < 0.0 {
        Direction::Left
    } else {
        Direction::Right
    })
}

pub fn quiz_swipe_target(
    mode: QuizMode,
    index: usize,
    total: usize,
    answered: bool,
    direction: Direction,
) -> Option<usize> {
    quiz_swipe_target_with_direction(mode, index, total, answered, direction, SWIPE_NEXT_ON_LEFT)
}

fn quiz_swipe_target_with_direction(
    mode: QuizMode,
    index: usize,
    total: usize,
    answered: bool,
    direction: Direction,
    next_on_left: bool,
) -> Option<usize> {
    if mode == QuizMode::Bikomeye || total == 0 || index >= total {
        return None;
    }

    let moves_next = (direction == Direction::Left) == next_on_left;
    if moves_next {
        if index + 1 >= total {
            return None;
        }
        if mode.provides_instant_feedback() && !answered {
            return None;
        }
        Some(index + 1)
    } else {
        index.checked_sub(1)
    }
}

pub fn effective_length(mode: QuizMode, requested: usize, available: usize) -> usize {
    let requested = if mode == QuizMode::Bikomeye {
        20
    } else {
        requested.clamp(5, 100)
    };
    requested.min(available)
}

pub fn time_for(minutes_for_20: u32, n: usize) -> u64 {
    let numerator = u64::from(minutes_for_20)
        .saturating_mul(60)
        .saturating_mul(n as u64);
    numerator.div_ceil(20).max(60)
}

pub fn pass_threshold(pass_mark_of_20: u32, n: usize) -> usize {
    (usize::try_from(pass_mark_of_20)
        .unwrap_or(usize::MAX)
        .saturating_mul(n)
        .div_ceil(20))
    .max(1)
}

impl QuizEngine {
    /// Starts a new quiz attempt for the given mo  de.
    pub fn start_quiz(
        bank: &QuestionBank,
        mode: QuizMode,
        settings: &Settings,
        stats: &HashMap<u32, QuestionStat>,
        now_secs: u64,
    ) -> Result<Attempt> {
        let count = effective_length(mode, settings.quiz_length, bank.len());
        let questions =
            bank.generate_quiz_questions(count, mode, settings.hard_weight_images, stats);

        if questions.is_empty() {
            return Err(AppError::InvalidData(
                "Nta bibazo bibonetse byo gutangiza ikizamini".to_string(),
            ));
        }

        let allowed_duration_secs = match mode {
            QuizMode::Hagati => Some(time_for(settings.medium_duration_mins, questions.len())),
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
        mut questions: Vec<Question>,
        settings: &Settings,
        now_secs: u64,
    ) -> Result<Attempt> {
        if questions.is_empty() {
            return Err(AppError::InvalidData(
                "Nta bibazo bibonetse byo gutangiza ikizamini".to_string(),
            ));
        }

        if matches!(
            mode,
            QuizMode::Byoroshye | QuizMode::Hagati | QuizMode::Bikomeye
        ) {
            questions.truncate(effective_length(
                mode,
                settings.quiz_length,
                questions.len(),
            ));
        }

        let allowed_duration_secs = match mode {
            QuizMode::Hagati => Some(time_for(settings.medium_duration_mins, questions.len())),
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

        let total = attempt.total_questions() as u32;
        let scaled_pass_mark = pass_threshold(pass_mark, attempt.total_questions()) as u32;
        let passed = score >= scaled_pass_mark;

        AttemptResult {
            id: attempt.id.clone(),
            mode: attempt.mode,
            score,
            total,
            pass_mark: scaled_pass_mark,
            passed,
            timestamp_secs: finish_time_secs,
            duration_seconds,
            question_results,
        }
    }
}

#[cfg(test)]
mod length_tests {
    use super::{effective_length, pass_threshold, time_for};
    use crate::models::QuizMode;

    #[test]
    fn length_time_and_pass_helpers_scale_attempts() {
        assert_eq!(effective_length(QuizMode::Bikomeye, 50, 100), 20);
        assert_eq!(effective_length(QuizMode::Bikomeye, 50, 12), 12);
        assert_eq!(effective_length(QuizMode::Byoroshye, 3, 100), 5);
        assert_eq!(effective_length(QuizMode::Hagati, 120, 80), 80);
        assert_eq!(time_for(20, 40), 2_400);
        assert_eq!(time_for(1, 5), 60);
        assert_eq!(pass_threshold(12, 20), 12);
        assert_eq!(pass_threshold(12, 10), 6);
        assert_eq!(pass_threshold(12, 30), 18);
        assert_eq!(pass_threshold(12, 50), 30);
    }
}

#[cfg(test)]
mod swipe_tests {
    use super::{quiz_swipe_target, quiz_swipe_target_with_direction, Direction};
    use crate::models::QuizMode;

    #[test]
    fn swipe_rules_follow_mode_answers_boundaries_and_direction() {
        assert_eq!(
            quiz_swipe_target(QuizMode::Bikomeye, 1, 3, true, Direction::Left),
            None
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Byoroshye, 1, 3, false, Direction::Right),
            Some(0)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Byoroshye, 1, 3, false, Direction::Left),
            None
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Byoroshye, 1, 3, true, Direction::Left),
            Some(2)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 1, 3, false, Direction::Right),
            Some(0)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 1, 3, false, Direction::Left),
            Some(2)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Study, 1, 3, false, Direction::Right),
            Some(0)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Study, 1, 3, true, Direction::Left),
            Some(2)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 0, 3, false, Direction::Right),
            None
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 2, 3, false, Direction::Left),
            None
        );
        assert_eq!(
            quiz_swipe_target_with_direction(QuizMode::Hagati, 1, 3, false, Direction::Left, false,),
            Some(0)
        );
    }
}
