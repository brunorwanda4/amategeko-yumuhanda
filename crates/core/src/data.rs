fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

use crate::error::{AppError, Result};
use crate::models::{Language, Question, QuestionStat, QuizMode};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};

pub static BUNDLED_QUESTIONS_JSON: &str = include_str!("../../../assets/questions.json");
pub static BUNDLED_QUESTIONS_EN_JSON: &str = include_str!("../../../assets/questions_en.json");
pub static BUNDLED_OVERRIDES_JSON: &str = include_str!("../../../assets/overrides.json");

#[derive(Debug, Clone, Deserialize)]
pub struct TranslatedQuestion {
    pub id: u32,
    pub text: String,
    pub options: BTreeMap<String, String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub correct: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub has_image: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct QuestionBank {
    questions: Vec<Question>,
    by_id: HashMap<u32, usize>,
}

impl QuestionBank {
    pub fn load_bundled() -> Result<Self> {
        Self::from_json_strings(
            BUNDLED_QUESTIONS_JSON,
            BUNDLED_QUESTIONS_EN_JSON,
            BUNDLED_OVERRIDES_JSON,
        )
    }

    pub fn from_rw_json_strings(questions_rw_json: &str, overrides_json: &str) -> Result<Self> {
        Self::from_json_strings(questions_rw_json, "[]", overrides_json)
    }

    pub fn from_json_strings(
        questions_rw_json: &str,
        questions_en_json: &str,
        overrides_json: &str,
    ) -> Result<Self> {
        let mut questions: Vec<Question> = serde_json::from_str(questions_rw_json)?;
        let mut overridden_ids = HashSet::new();
        for q in &mut questions {
            q.text = normalize_whitespace(&q.text);
        }

        // Apply manual overrides if any
        if !overrides_json.trim().is_empty() && overrides_json.trim() != "{}" {
            let overrides: HashMap<String, Question> = serde_json::from_str(overrides_json)
                .map_err(|e| AppError::InvalidData(format!("Overrides parsing error: {e}")))?;

            for q in &mut questions {
                let id_str = q.id.to_string();
                if let Some(override_q) = overrides.get(&id_str) {
                    *q = override_q.clone();
                    overridden_ids.insert(q.id);
                }
            }
        }

        // Validate basic Kinyarwanda questions
        for q in &questions {
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
        }

        // Parse English translations if provided
        let mut en_by_id: HashMap<u32, TranslatedQuestion> = HashMap::new();
        if !questions_en_json.trim().is_empty() && questions_en_json.trim() != "[]" {
            let mut en_questions: Vec<TranslatedQuestion> = serde_json::from_str(questions_en_json)
                .map_err(|e| {
                    AppError::InvalidData(format!("English questions parsing error: {e}"))
                })?;
            for eq in &mut en_questions {
                eq.text = normalize_whitespace(&eq.text);
            }
            for eq in en_questions {
                en_by_id.insert(eq.id, eq);
            }
        }

        // Merge by ID: default question language = English; fallback = Kinyarwanda
        let mut by_id = HashMap::with_capacity(questions.len());
        for (idx, q) in questions.iter_mut().enumerate() {
            // Save original Kinyarwanda text & options
            let rw_text = q.text.clone();
            let rw_options = q.options.clone();
            q.text_rw = Some(rw_text.clone());
            q.options_rw = Some(rw_options.clone());

            // An override can change option positions, so its previous English
            // translation is unsafe until the owner reviews that mapping again.
            if !overridden_ids.contains(&q.id) {
                if let Some(en_q) = en_by_id.remove(&q.id) {
                    if !en_q.text.trim().is_empty() {
                        q.text_en = Some(en_q.text.clone());
                        q.options_en = Some(en_q.options.clone());
                        q.status_en = en_q.status;

                        // Set default question language to English
                        q.text = en_q.text;
                        q.options = en_q.options;
                    } else {
                        q.text_en = None;
                        q.options_en = None;
                    }
                }
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

    /// Search and filter question bank across text and language variations
    pub fn search(
        &self,
        query: &str,
        has_image_only: bool,
        starred_only: bool,
        mistakes_only: bool,
        starred_ids: &HashSet<u32>,
        question_stats: &HashMap<u32, QuestionStat>,
    ) -> Vec<&Question> {
        self.search_for_lang(
            query,
            has_image_only,
            starred_only,
            mistakes_only,
            starred_ids,
            question_stats,
            Language::En,
        )
    }

    /// Search and filter question bank for a specific question language
    #[allow(clippy::too_many_arguments)]
    pub fn search_for_lang(
        &self,
        query: &str,
        has_image_only: bool,
        starred_only: bool,
        mistakes_only: bool,
        starred_ids: &HashSet<u32>,
        question_stats: &HashMap<u32, QuestionStat>,
        lang: Language,
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

                // Search in active question language
                let text = q.text_for(lang).to_lowercase();
                if text.contains(&q_clean) {
                    return true;
                }

                let opts = q.options_for(lang);
                for opt_text in opts.values() {
                    if opt_text.to_lowercase().contains(&q_clean) {
                        return true;
                    }
                }

                // Fallback search across both languages
                if let Some(sec_text) = q.secondary_text_for(lang) {
                    if sec_text.to_lowercase().contains(&q_clean) {
                        return true;
                    }
                }

                if let Some(sec_opts) = q.secondary_options_for(lang) {
                    for opt_text in sec_opts.values() {
                        if opt_text.to_lowercase().contains(&q_clean) {
                            return true;
                        }
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
