use crate::error::{AppError, Result};
use crate::models::{Question, QuestionStat, QuizMode};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

pub static BUNDLED_QUESTIONS_JSON: &str = include_str!("../../../assets/questions.json");
pub static BUNDLED_OVERRIDES_JSON: &str = include_str!("../../../assets/overrides.json");

#[derive(Debug, Clone)]
pub struct QuestionBank {
    questions: Vec<Question>,
    by_id: HashMap<u32, usize>,
}

impl QuestionBank {
    pub fn load_bundled() -> Result<Self> {
        Self::from_json_strings(BUNDLED_QUESTIONS_JSON, BUNDLED_OVERRIDES_JSON)
    }

    pub fn from_json_strings(questions_json: &str, overrides_json: &str) -> Result<Self> {
        let mut questions: Vec<Question> = serde_json::from_str(questions_json)?;

        // Apply manual overrides if any
        if !overrides_json.trim().is_empty() && overrides_json.trim() != "{}" {
            let overrides: HashMap<String, Question> = serde_json::from_str(overrides_json)
                .map_err(|e| AppError::InvalidData(format!("Overrides parsing error: {e}")))?;

            for q in &mut questions {
                let id_str = q.id.to_string();
                if let Some(override_q) = overrides.get(&id_str) {
                    *q = override_q.clone();
                }
            }
        }

        // Validate questions
        let mut by_id = HashMap::with_capacity(questions.len());
        for (idx, q) in questions.iter().enumerate() {
            if q.text.trim().is_empty() {
                return Err(AppError::InvalidData(format!(
                    "Ikibazo #{} ntigifite inyandiko",
                    q.id
                )));
            }

            if q.options.len() < 3 || q.options.len() > 4 {
                return Err(AppError::InvalidData(format!(
                    "Ikibazo #{} gifite amahitamo agomba kuba 3 cyangwa 4 (gifite {})",
                    q.id,
                    q.options.len()
                )));
            }

            if !q.options.contains_key(&q.correct) {
                return Err(AppError::InvalidData(format!(
                    "Ikibazo #{} gifite igisubizo cy'ukuri '{}' kidahari mu mahitamo",
                    q.id, q.correct
                )));
            }

            by_id.insert(q.id, idx);
        }

        Ok(Self { questions, by_id })
    }

    pub fn all(&self) -> &[Question] {
        &self.questions
    }

    pub fn len(&self) -> usize {
        self.questions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }

    pub fn get(&self, id: u32) -> Option<&Question> {
        self.by_id.get(&id).and_then(|&idx| self.questions.get(idx))
    }

    /// Search and filter question bank
    pub fn search(
        &self,
        query: &str,
        has_image_only: bool,
        starred_only: bool,
        mistakes_only: bool,
        starred_ids: &HashSet<u32>,
        question_stats: &HashMap<u32, QuestionStat>,
    ) -> Vec<&Question> {
        let q_clean = query.trim().to_lowercase();
        let query_number: Option<u32> = q_clean.parse().ok();

        self.questions
            .iter()
            .filter(|q| {
                if has_image_only && !q.has_image {
                    return false;
                }

                if starred_only && !starred_ids.contains(&q.id) {
                    return false;
                }

                if mistakes_only {
                    let wrong = question_stats
                        .get(&q.id)
                        .map(|s| s.wrong_count)
                        .unwrap_or(0);
                    if wrong == 0 {
                        return false;
                    }
                }

                if q_clean.is_empty() {
                    return true;
                }

                // If query is an exact number or prefix of ID
                if let Some(num) = query_number {
                    if q.id == num {
                        return true;
                    }
                }

                // Match in ID string or text or options text
                if q.id.to_string().starts_with(&q_clean) {
                    return true;
                }

                if q.text.to_lowercase().contains(&q_clean) {
                    return true;
                }

                for opt_text in q.options.values() {
                    if opt_text.to_lowercase().contains(&q_clean) {
                        return true;
                    }
                }

                false
            })
            .collect()
    }

    /// Generate questions for a quiz attempt based on the specified mode.
    pub fn generate_quiz_questions(
        &self,
        count: usize,
        mode: QuizMode,
        weight_images_in_hard: bool,
        stats: &HashMap<u32, QuestionStat>,
    ) -> Vec<Question> {
        let mut rng = thread_rng();

        match mode {
            QuizMode::WeakPractice => {
                // Collect questions with mistakes, sorted by wrong_count descending
                let mut mistakes: Vec<(u32, &Question)> = self
                    .questions
                    .iter()
                    .filter_map(|q| {
                        stats.get(&q.id).and_then(|s| {
                            if s.wrong_count > 0 {
                                Some((s.wrong_count, q))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                mistakes.sort_by_key(|a| std::cmp::Reverse(a.0));

                let mut selected: Vec<Question> = mistakes
                    .iter()
                    .take(count)
                    .map(|(_, q)| (*q).clone())
                    .collect();

                // If fewer than count mistakes exist, fill remainder with random questions
                if selected.len() < count {
                    let existing_ids: HashSet<u32> = selected.iter().map(|q| q.id).collect();
                    let mut remaining: Vec<Question> = self
                        .questions
                        .iter()
                        .filter(|q| !existing_ids.contains(&q.id))
                        .cloned()
                        .collect();
                    remaining.shuffle(&mut rng);
                    for q in remaining.into_iter().take(count - selected.len()) {
                        selected.push(q);
                    }
                }

                selected.shuffle(&mut rng);
                selected
            }
            QuizMode::Bikomeye if weight_images_in_hard => {
                // Weighted selection for Hard mode:
                // Prioritize sign/marking image questions and tricky questions
                let mut image_questions: Vec<Question> = self
                    .questions
                    .iter()
                    .filter(|q| q.has_image)
                    .cloned()
                    .collect();
                image_questions.shuffle(&mut rng);

                let mut text_questions: Vec<Question> = self
                    .questions
                    .iter()
                    .filter(|q| !q.has_image)
                    .cloned()
                    .collect();
                text_questions.shuffle(&mut rng);

                // Target approximately 50% image questions (e.g. 10 of 20)
                let target_images = (count / 2).min(image_questions.len());
                let target_texts = count.saturating_sub(target_images);

                let mut selected: Vec<Question> = Vec::with_capacity(count);
                selected.extend(image_questions.into_iter().take(target_images));
                selected.extend(text_questions.into_iter().take(target_texts));

                // If still short, draw from anything remaining
                if selected.len() < count {
                    let selected_ids: HashSet<u32> = selected.iter().map(|q| q.id).collect();
                    let mut fallback: Vec<Question> = self
                        .questions
                        .iter()
                        .filter(|q| !selected_ids.contains(&q.id))
                        .cloned()
                        .collect();
                    fallback.shuffle(&mut rng);
                    selected.extend(fallback.into_iter().take(count - selected.len()));
                }

                selected.shuffle(&mut rng);
                selected
            }
            _ => {
                // Byoroshye, Hagati, or regular Hard: uniform random 20 questions
                let mut pool = self.questions.clone();
                pool.shuffle(&mut rng);
                pool.into_iter().take(count).collect()
            }
        }
    }
}
