use crate::models::{AttemptResult, Progress, QuestionStat, QuizMode};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StatsFilter {
    #[default]
    All,
    Easy,
    Medium,
    Hard,
}

impl FromStr for StatsFilter {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_str(s))
    }
}

impl StatsFilter {
    pub fn matches(&self, mode: QuizMode) -> bool {
        match self {
            StatsFilter::All => true,
            StatsFilter::Easy => mode == QuizMode::Byoroshye,
            StatsFilter::Medium => mode == QuizMode::Hagati,
            StatsFilter::Hard => mode == QuizMode::Bikomeye,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StatsFilter::All => "all",
            StatsFilter::Easy => "easy",
            StatsFilter::Medium => "medium",
            StatsFilter::Hard => "hard",
        }
    }

    pub fn parse_str(s: &str) -> Self {
        match s {
            "easy" => StatsFilter::Easy,
            "med" | "medium" => StatsFilter::Medium,
            "hard" => StatsFilter::Hard,
            _ => StatsFilter::All,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentAttemptStat {
    pub score: u32,
    pub passed: bool,
    pub date_str: String,
}

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
    pub last_10_attempts: Vec<RecentAttemptStat>,
    pub most_missed: Vec<MostMissedQuestion>,
    pub mode_summaries: Vec<ModeStatSummary>,
    pub questions_seen_count: u32,
}

/// Converts a Unix timestamp in seconds to (day, month) in civil UTC calendar.
pub fn timestamp_to_day_month(timestamp_secs: u64) -> (u32, u32) {
    if timestamp_secs == 0 {
        return (1, 1);
    }
    let days = (timestamp_secs / 86400) as i64;
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (d, m)
}

/// Formats a Unix timestamp in seconds as "D/M" (e.g. "2/10", "12/9").
pub fn timestamp_to_day_month_str(timestamp_secs: u64) -> String {
    let (d, m) = timestamp_to_day_month(timestamp_secs);
    format!("{d}/{m}")
}

pub struct StatsCalculator;

impl StatsCalculator {
    pub fn compute_summary(progress: &Progress) -> StatsSummary {
        Self::compute_summary_filtered(progress, StatsFilter::All)
    }

    pub fn compute_summary_filtered(progress: &Progress, filter: StatsFilter) -> StatsSummary {
        let filtered_attempts: Vec<&AttemptResult> = progress
            .attempts
            .iter()
            .filter(|a| filter.matches(a.mode))
            .collect();

        let total_attempts = filtered_attempts.len() as u32;

        let (average_score, high_score, pass_rate_percentage) = if total_attempts == 0 {
            (0.0, 0, 0.0)
        } else {
            let mut sum_percentage = 0.0;
            let mut high_score: u32 = 0;
            let mut passed_count: u32 = 0;

            for a in &filtered_attempts {
                let percentage = score_percentage(a);
                sum_percentage += percentage;
                high_score = high_score.max(percentage.round() as u32);
                if a.score >= a.pass_mark {
                    passed_count = passed_count.saturating_add(1);
                }
            }

            let avg = sum_percentage / (total_attempts as f32);
            let pass_rate = ((passed_count as f32) / (total_attempts as f32)) * 100.0;
            (avg, high_score, pass_rate)
        };

        // Last 10 attempts (chronological: oldest to newest of the last 10)
        let last_10_filtered: Vec<&AttemptResult> = filtered_attempts
            .iter()
            .rev()
            .take(10)
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        let last_10_scores: Vec<(u32, u32)> = last_10_filtered
            .iter()
            .map(|a| (a.score, a.pass_mark))
            .collect();

        let last_10_attempts: Vec<RecentAttemptStat> = last_10_filtered
            .iter()
            .map(|a| RecentAttemptStat {
                score: score_percentage(a).round() as u32,
                passed: a.score >= a.pass_mark,
                date_str: timestamp_to_day_month_str(a.timestamp_secs),
            })
            .collect();

        // Top 4 most missed questions overall
        let mut missed_list: Vec<&QuestionStat> = progress
            .question_stats
            .values()
            .filter(|s| s.wrong_count > 0)
            .collect();

        missed_list.sort_by(|a, b| {
            b.wrong_count
                .cmp(&a.wrong_count)
                .then_with(|| b.seen_count.cmp(&a.seen_count))
                .then_with(|| a.question_id.cmp(&b.question_id))
        });

        let most_missed = missed_list
            .into_iter()
            .take(4)
            .map(|s| MostMissedQuestion {
                question_id: s.question_id,
                wrong_count: s.wrong_count,
                seen_count: s.seen_count,
            })
            .collect();

        // Mode breakdown for the 3 exam modes: Easy, Medium, Hard
        let main_modes = [QuizMode::Byoroshye, QuizMode::Hagati, QuizMode::Bikomeye];

        let mode_summaries = main_modes
            .iter()
            .map(|&m| {
                let attempts_for_mode: Vec<&AttemptResult> =
                    progress.attempts.iter().filter(|a| a.mode == m).collect();

                let count = attempts_for_mode.len() as u32;
                if count == 0 {
                    ModeStatSummary {
                        mode: m,
                        attempts_count: 0,
                        passed_count: 0,
                        average_score: 0.0,
                    }
                } else {
                    let pass = attempts_for_mode
                        .iter()
                        .filter(|a| a.score >= a.pass_mark)
                        .count() as u32;
                    let total_mode_percentage: f32 =
                        attempts_for_mode.iter().map(|a| score_percentage(a)).sum();
                    ModeStatSummary {
                        mode: m,
                        attempts_count: count,
                        passed_count: pass,
                        average_score: total_mode_percentage / (count as f32),
                    }
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
            last_10_attempts,
            most_missed,
            mode_summaries,
            questions_seen_count,
        }
    }
}

fn score_percentage(attempt: &AttemptResult) -> f32 {
    let total = attempt.total_questions();
    if total == 0 {
        0.0
    } else {
        (attempt.score as f32 / total as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Question, QuestionResult};

    fn make_test_question(id: u32) -> Question {
        Question {
            id,
            text: format!("Q{id}"),
            options: Default::default(),
            correct: "a".into(),
            image: None,
            has_image: false,
            text_en: None,
            options_en: None,
            text_rw: None,
            options_rw: None,
            status_en: None,
        }
    }

    fn make_attempt(
        id: &str,
        mode: QuizMode,
        score: u32,
        pass_mark: u32,
        timestamp_secs: u64,
    ) -> AttemptResult {
        AttemptResult {
            id: id.to_string(),
            mode,
            score,
            total: 20,
            pass_mark,
            passed: score >= pass_mark,
            timestamp_secs,
            duration_seconds: 300,
            question_results: Vec::new(),
        }
    }

    #[test]
    fn test_stats_empty_history() {
        let progress = Progress::default();
        let summary = StatsCalculator::compute_summary(&progress);

        assert_eq!(summary.total_attempts, 0);
        assert_eq!(summary.average_score, 0.0);
        assert_eq!(summary.high_score, 0);
        assert_eq!(summary.pass_rate_percentage, 0.0);
        assert!(summary.last_10_scores.is_empty());
        assert!(summary.last_10_attempts.is_empty());
        assert!(summary.most_missed.is_empty());
        assert_eq!(summary.questions_seen_count, 0);

        // 3 modes present with 0 attempts and 0.0 avg
        assert_eq!(summary.mode_summaries.len(), 3);
        for ms in &summary.mode_summaries {
            assert_eq!(ms.attempts_count, 0);
            assert_eq!(ms.average_score, 0.0);
        }
    }

    #[test]
    fn test_stats_average() {
        let mut progress = Progress::default();
        progress
            .attempts
            .push(make_attempt("1", QuizMode::Byoroshye, 10, 12, 1000));
        progress
            .attempts
            .push(make_attempt("2", QuizMode::Byoroshye, 15, 12, 2000));
        progress
            .attempts
            .push(make_attempt("3", QuizMode::Byoroshye, 20, 12, 3000));

        let summary = StatsCalculator::compute_summary(&progress);
        assert_eq!(summary.total_attempts, 3);
        assert!((summary.average_score - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stats_best_score() {
        let mut progress = Progress::default();
        progress
            .attempts
            .push(make_attempt("1", QuizMode::Byoroshye, 8, 12, 1000));
        progress
            .attempts
            .push(make_attempt("2", QuizMode::Hagati, 19, 12, 2000));
        progress
            .attempts
            .push(make_attempt("3", QuizMode::Bikomeye, 14, 12, 3000));

        let summary = StatsCalculator::compute_summary(&progress);
        assert_eq!(summary.high_score, 19);
    }

    #[test]
    fn test_stats_pass_rate() {
        let mut progress = Progress::default();
        // 2 passed, 2 failed
        progress
            .attempts
            .push(make_attempt("1", QuizMode::Byoroshye, 14, 12, 1000)); // pass
        progress
            .attempts
            .push(make_attempt("2", QuizMode::Byoroshye, 10, 12, 2000)); // fail
        progress
            .attempts
            .push(make_attempt("3", QuizMode::Byoroshye, 16, 12, 3000)); // pass
        progress
            .attempts
            .push(make_attempt("4", QuizMode::Byoroshye, 11, 12, 4000)); // fail

        let summary = StatsCalculator::compute_summary(&progress);
        assert_eq!(summary.total_attempts, 4);
        assert!((summary.pass_rate_percentage - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stats_last_10() {
        let mut progress = Progress::default();
        // Insert 15 attempts (scores 1..=15)
        for i in 1..=15 {
            progress.attempts.push(make_attempt(
                &format!("att_{i}"),
                QuizMode::Byoroshye,
                i,
                12,
                1727827200, // 2024-10-02 -> "2/10"
            ));
        }

        let summary = StatsCalculator::compute_summary(&progress);
        assert_eq!(summary.total_attempts, 15);
        assert_eq!(summary.last_10_attempts.len(), 10);
        assert_eq!(summary.last_10_scores.len(), 10);

        // Chronological: scores 6..=15
        let expected_scores: Vec<u32> = (6..=15).collect();
        let actual_scores: Vec<u32> = summary.last_10_attempts.iter().map(|a| a.score).collect();
        assert_eq!(actual_scores, expected_scores);

        // Check date formatting
        assert_eq!(summary.last_10_attempts[0].date_str, "2/10");
    }

    #[test]
    fn test_stats_most_missed_ranking() {
        let mut progress = Progress::default();

        let mut record_q = |qid: u32, wrong: u32, seen: u32| {
            let mut res = make_attempt("res", QuizMode::Byoroshye, 10, 12, 1000);
            for _ in 0..wrong {
                res.question_results.push(QuestionResult {
                    question: make_test_question(qid),
                    user_answer: Some("b".into()),
                    correct_answer: "a".into(),
                    is_correct: false,
                });
            }
            for _ in 0..(seen - wrong) {
                res.question_results.push(QuestionResult {
                    question: make_test_question(qid),
                    user_answer: Some("a".into()),
                    correct_answer: "a".into(),
                    is_correct: true,
                });
            }
            progress.record_attempt_result(res);
        };

        record_q(1, 1, 2);
        record_q(2, 4, 5);
        record_q(3, 3, 3);
        record_q(4, 2, 2);
        record_q(5, 5, 6);
        record_q(6, 0, 1); // 0 wrong -> should not be included

        let summary = StatsCalculator::compute_summary(&progress);
        // Top 4 most missed
        assert_eq!(summary.most_missed.len(), 4);
        assert_eq!(summary.most_missed[0].question_id, 5); // 5 wrong
        assert_eq!(summary.most_missed[1].question_id, 2); // 4 wrong
        assert_eq!(summary.most_missed[2].question_id, 3); // 3 wrong
        assert_eq!(summary.most_missed[3].question_id, 4); // 2 wrong
        assert_eq!(summary.most_missed[0].wrong_count, 5);
        assert_eq!(summary.most_missed[0].seen_count, 6);
    }

    #[test]
    fn test_stats_mode_filter() {
        let mut progress = Progress::default();
        progress
            .attempts
            .push(make_attempt("1", QuizMode::Byoroshye, 10, 12, 1000));
        progress
            .attempts
            .push(make_attempt("2", QuizMode::Byoroshye, 12, 12, 2000));
        progress
            .attempts
            .push(make_attempt("3", QuizMode::Byoroshye, 14, 12, 3000));
        progress
            .attempts
            .push(make_attempt("4", QuizMode::Hagati, 15, 12, 4000));
        progress
            .attempts
            .push(make_attempt("5", QuizMode::Hagati, 17, 12, 5000));
        progress
            .attempts
            .push(make_attempt("6", QuizMode::Bikomeye, 9, 12, 6000));

        // Add 1 question stat to test seen count is untouched by filter
        let mut res = make_attempt("seen", QuizMode::Byoroshye, 10, 12, 1000);
        res.question_results.push(QuestionResult {
            question: make_test_question(42),
            user_answer: Some("a".into()),
            correct_answer: "a".into(),
            is_correct: true,
        });
        progress.record_attempt_result(res);

        // All filter
        let summary_all = StatsCalculator::compute_summary_filtered(&progress, StatsFilter::All);
        assert_eq!(summary_all.total_attempts, 7); // 6 + 1 with question_result
        assert_eq!(summary_all.questions_seen_count, 1);

        // Easy filter
        let summary_easy = StatsCalculator::compute_summary_filtered(&progress, StatsFilter::Easy);
        assert_eq!(summary_easy.total_attempts, 4);
        assert_eq!(summary_easy.questions_seen_count, 1); // Not affected

        // Medium filter
        let summary_med = StatsCalculator::compute_summary_filtered(&progress, StatsFilter::Medium);
        assert_eq!(summary_med.total_attempts, 2);
        assert_eq!(summary_med.high_score, 17);
        assert!((summary_med.average_score - 16.0).abs() < f32::EPSILON);
        assert_eq!(summary_med.questions_seen_count, 1); // Not affected

        // Hard filter
        let summary_hard = StatsCalculator::compute_summary_filtered(&progress, StatsFilter::Hard);
        assert_eq!(summary_hard.total_attempts, 1);
        assert_eq!(summary_hard.high_score, 9);
        assert_eq!(summary_hard.questions_seen_count, 1); // Not affected
    }

    #[test]
    fn test_timestamp_to_day_month() {
        assert_eq!(timestamp_to_day_month(0), (1, 1));
        assert_eq!(timestamp_to_day_month_str(0), "1/1");

        // 2024-10-02 00:00:00 UTC = 1727827200
        assert_eq!(timestamp_to_day_month(1727827200), (2, 10));
        assert_eq!(timestamp_to_day_month_str(1727827200), "2/10");

        // 2024-09-12 00:00:00 UTC = 1726099200
        assert_eq!(timestamp_to_day_month(1726099200), (12, 9));
        assert_eq!(timestamp_to_day_month_str(1726099200), "12/9");

        // 2024-09-30 00:00:00 UTC = 1727654400
        assert_eq!(timestamp_to_day_month(1727654400), (30, 9));
        assert_eq!(timestamp_to_day_month_str(1727654400), "30/9");
    }
}
