use amategeko_core::*;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    Quiz,
    Results,
    Questions,
    Stats,
    Settings,
}

pub struct AppState {
    pub bank: Arc<QuestionBank>,
    pub settings: Settings,
    pub progress: Progress,
    pub active_screen: Screen,
    pub current_attempt: Option<Attempt>,
    pub last_result: Option<AttemptResult>,
    pub storage: Arc<dyn Storage>,
    pub clock: Arc<dyn Clock>,
    pub pending_leave_dialog: bool,
}

impl AppState {
    pub fn new(storage: Arc<dyn Storage>, clock: Arc<dyn Clock>) -> Self {
        let bank = Arc::new(QuestionBank::load_bundled().unwrap_or_else(|e| {
            log::error!("Failed to load question bank: {e}");
            panic!("Question bank is required: {e}");
        }));

        let settings = storage.load_settings();
        let mut progress = storage.load_progress();

        // Check if there was an in-progress attempt saved on disk
        let mut current_attempt = storage.load_in_progress();
        let mut last_result = None;

        if let Some(mut att) = current_attempt.take() {
            let now = clock.now_seconds();
            if att.mode.has_countdown() && QuizTimer::is_expired(&att, now) {
                // Auto-submit expired attempt on startup
                let res = QuizEngine::finish_attempt(&mut att, settings.pass_mark, now);
                progress.record_attempt_result(res.clone());
                let _ = storage.save_progress(&progress);
                let _ = storage.clear_in_progress();
                last_result = Some(res);
            } else {
                current_attempt = Some(att);
            }
        }

        Self {
            bank,
            settings,
            progress,
            active_screen: Screen::Home,
            current_attempt,
            last_result,
            storage,
            clock,
            pending_leave_dialog: false,
        }
    }

    pub fn navigate(&mut self, target: Screen) {
        self.active_screen = target;
    }

    pub fn start_quiz(&mut self, mode: QuizMode) {
        let now = self.clock.now_seconds();
        let stats = &self.progress.question_stats;
        match QuizEngine::start_quiz(&self.bank, mode, &self.settings, stats, now) {
            Ok(attempt) => {
                let _ = self.storage.save_in_progress(&attempt);
                self.current_attempt = Some(attempt);
                self.active_screen = Screen::Quiz;
            }
            Err(e) => {
                log::error!("Failed to start quiz: {e}");
            }
        }
    }

    pub fn start_retry_wrong(&mut self) {
        if let Some(prev) = &self.last_result {
            let now = self.clock.now_seconds();
            match QuizEngine::start_retry_wrong(prev, now) {
                Ok(attempt) => {
                    let _ = self.storage.save_in_progress(&attempt);
                    self.current_attempt = Some(attempt);
                    self.active_screen = Screen::Quiz;
                }
                Err(e) => {
                    log::error!("Failed to start retry wrong: {e}");
                }
            }
        }
    }

    pub fn resume_attempt(&mut self) {
        if self.current_attempt.is_some() {
            self.active_screen = Screen::Quiz;
        }
    }

    pub fn discard_in_progress(&mut self) {
        self.current_attempt = None;
        let _ = self.storage.clear_in_progress();
    }

    pub fn record_current_answer(&mut self, option: &str) {
        if let Some(att) = &mut self.current_attempt {
            let _ = QuizEngine::select_option(att, option);
            let _ = self.storage.save_in_progress(att);
        }
    }

    pub fn finish_current_quiz(&mut self) {
        if let Some(mut att) = self.current_attempt.take() {
            let now = self.clock.now_seconds();
            let result = QuizEngine::finish_attempt(&mut att, self.settings.pass_mark, now);
            self.progress.record_attempt_result(result.clone());
            let _ = self.storage.save_progress(&self.progress);
            let _ = self.storage.clear_in_progress();
            self.last_result = Some(result);
            self.active_screen = Screen::Results;
        }
    }

    pub fn save_settings(&mut self, new_settings: Settings) {
        self.settings = new_settings;
        let _ = self.storage.save_settings(&self.settings);
    }

    pub fn clear_history(&mut self) {
        self.progress = Progress::default();
        let _ = self.storage.save_progress(&self.progress);
        let _ = self.storage.clear_in_progress();
        self.current_attempt = None;
        self.last_result = None;
    }
}
