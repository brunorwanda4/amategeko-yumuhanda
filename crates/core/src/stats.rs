use crate::models::{AttemptResult, Progress, QuestionStat, QuizMode};

#[derive(Debug, Clone, PartialEq)]
pub struct MostMissedQuestion {
    pub question_id: u32,
    pub wrong_count: u32,
    pub seen_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModeStatSummary {
    pub mode: QuizMode,
    pub attempts_count: u32,
    pub passed_count: u32,
    pub average_score: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatsSummary {
    pub total_attempts: u32,
    pub average_score: f32,
    pub high_score: u32,
    pub pass_rate_percentage: f32,
    pub last_10_scores: Vec<(u32, u32)>, // (score, pass_mark)
    pub most_missed: Vec<MostMissedQuestion>,
    pub mode_summaries: Vec<ModeStatSummary>,
    pub questions_seen_count: u32,
}

pub struct StatsCalculator;

impl StatsCalculator {
    pub fn compute_summary(progress: &Progress) -> StatsSummary {
        let total_attempts = progress.attempts.len() as u32;

        if total_attempts == 0 {
            return StatsSummary {
                total_attempts: 0,
                average_score: 0.0,
                high_score: 0,
                pass_rate_percentage: 0.0,
                last_10_scores: Vec::new(),
                most_missed: Vec::new(),
                mode_summaries: Vec::new(),
                questions_seen_count: progress
                    .question_stats
                    .values()
                    .filter(|s| s.seen_count > 0)
                    .count() as u32,
            };
        }

        let mut sum_score: u32 = 0;
        let mut high_score: u32 = 0;
        let mut passed_count: u32 = 0;

        for a in &progress.attempts {
            sum_score = sum_score.saturating_add(a.score);
            if a.score > high_score {
                high_score = a.score;
            }
            if a.passed {
                passed_count = passed_count.saturating_add(1);
            }
        }

        let average_score = (sum_score as f32) / (total_attempts as f32);
        let pass_rate_percentage = ((passed_count as f32) / (total_attempts as f32)) * 100.0;

        // Last 10 scores
        let last_10_scores: Vec<(u32, u32)> = progress
            .attempts
            .iter()
            .rev()
            .take(10)
            .map(|a| (a.score, a.pass_mark))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // Top 5 most missed
        let mut missed_list: Vec<&QuestionStat> = progress
            .question_stats
            .values()
            .filter(|s| s.wrong_count > 0)
            .collect();

        missed_list.sort_by_key(|a| std::cmp::Reverse(a.wrong_count));

        let most_missed = missed_list
            .into_iter()
            .take(5)
            .map(|s| MostMissedQuestion {
                question_id: s.question_id,
                wrong_count: s.wrong_count,
                seen_count: s.seen_count,
            })
            .collect();

        // Mode breakdown
        let modes = [
            QuizMode::Byoroshye,
            QuizMode::Hagati,
            QuizMode::Bikomeye,
            QuizMode::WeakPractice,
            QuizMode::RetryWrong,
        ];

        let mode_summaries = modes
            .iter()
            .filter_map(|&m| {
                let attempts_for_mode: Vec<&AttemptResult> = progress
                    .attempts
                    .iter()
                    .filter(|a| a.mode == m)
                    .collect();

                if attempts_for_mode.is_empty() {
                    None
                } else {
                    let count = attempts_for_mode.len() as u32;
                    let pass = attempts_for_mode.iter().filter(|a| a.passed).count() as u32;
                    let total_mode_score: u32 = attempts_for_mode.iter().map(|a| a.score).sum();
                    Some(ModeStatSummary {
                        mode: m,
                        attempts_count: count,
                        passed_count: pass,
                        average_score: (total_mode_score as f32) / (count as f32),
                    })
                }
            })
            .collect();

        let questions_seen_count = progress
            .question_stats
            .values()
            .filter(|s| s.seen_count > 0)
            .count() as u32;

        StatsSummary {
            total_attempts,
            average_score,
            high_score,
            pass_rate_percentage,
            last_10_scores,
            most_missed,
            mode_summaries,
            questions_seen_count,
        }
    }
}
