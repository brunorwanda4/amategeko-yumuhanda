use amategeko_core::*;
use rand::seq::SliceRandom;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Home,
    /// Quiz page (Ikizamini): pick a mode and start an exam.
    QuizStart,
    Quiz,
    Results,
    Questions,
    Stats,
    Settings,
}

#[cfg(debug_assertions)]
impl From<amategeko_core::dev::DevScreen> for Screen {
    fn from(ds: amategeko_core::dev::DevScreen) -> Self {
        match ds {
            amategeko_core::dev::DevScreen::Home => Screen::Home,
            amategeko_core::dev::DevScreen::Quiz => Screen::Quiz,
            amategeko_core::dev::DevScreen::Results => Screen::Results,
            amategeko_core::dev::DevScreen::Browse => Screen::Questions,
            amategeko_core::dev::DevScreen::Stats => Screen::Stats,
            amategeko_core::dev::DevScreen::Settings => Screen::Settings,
        }
    }
}

#[cfg(debug_assertions)]
impl From<Screen> for amategeko_core::dev::DevScreen {
    fn from(s: Screen) -> Self {
        match s {
            Screen::Home => amategeko_core::dev::DevScreen::Home,
            Screen::Quiz | Screen::QuizStart => amategeko_core::dev::DevScreen::Quiz,
            Screen::Results => amategeko_core::dev::DevScreen::Results,
            Screen::Questions => amategeko_core::dev::DevScreen::Browse,
            Screen::Stats => amategeko_core::dev::DevScreen::Stats,
            Screen::Settings => amategeko_core::dev::DevScreen::Settings,
        }
    }
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
    #[cfg(debug_assertions)]
    pub debug_timer_override: Option<u64>,
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

        #[cfg(debug_assertions)]
        let mut active_screen = Screen::Home;

        #[cfg(debug_assertions)]
        if let Some(dev) = storage.load_dev_state() {
            let restored: Screen = dev.screen.into();
            if restored != Screen::Quiz || current_attempt.is_some() {
                active_screen = restored;
            }
        }

        #[cfg(not(debug_assertions))]
        let active_screen = Screen::Home;

        Self {
            bank,
            settings,
            progress,
            active_screen,
            current_attempt,
            last_result,
            storage,
            clock,
            pending_leave_dialog: false,
            #[cfg(debug_assertions)]
            debug_timer_override: None,
        }
    }

    pub fn navigate(&mut self, target: Screen) {
        self.active_screen = target;
    }

    pub fn start_quiz(&mut self, mode: QuizMode) {
        let now = self.clock.now_seconds();
        let stats = &self.progress.question_stats;
        match QuizEngine::start_quiz(&self.bank, mode, &self.settings, stats, now) {
            Ok(attempt) => self.begin_attempt(attempt, now),
            Err(e) => {
                log::error!("Failed to start quiz: {e}");
            }
        }
    }

    /// Starts a quiz from the quiz page. Easy draws up to 20 random questions from
    /// `set`; Medium and Hard always use the whole bank, so `set` is ignored for them.
    pub fn start_quiz_in_set(&mut self, mode: QuizMode, set: QuestionSet) {
        if mode != QuizMode::Byoroshye || set == QuestionSet::All {
            self.start_quiz(mode);
            return;
        }

        let now = self.clock.now_seconds();
        let mut pool = set.filter_questions(
            self.bank.all(),
            &self.progress.starred_questions,
            &self.progress.question_stats,
        );
        pool.shuffle(&mut rand::thread_rng());
        pool.truncate(20);

        match QuizEngine::start_quiz_with_questions(mode, pool, &self.settings, now) {
            Ok(attempt) => self.begin_attempt(attempt, now),
            Err(e) => {
                log::error!("Failed to start quiz from set: {e}");
            }
        }
    }

    /// Number of questions in `set`, computed from the bank and the saved progress.
    pub fn question_set_count(&self, set: QuestionSet) -> usize {
        set.count(
            self.bank.all(),
            &self.progress.starred_questions,
            &self.progress.question_stats,
        )
    }

    fn begin_attempt(&mut self, attempt: Attempt, now: u64) {
        #[allow(unused_mut)]
        let mut attempt = attempt;
        #[cfg(debug_assertions)]
        if let Some(secs) = self.debug_timer_override {
            amategeko_core::dev::apply_timer_override(&mut attempt, secs, now);
        }
        #[cfg(not(debug_assertions))]
        let _ = now;
        let _ = self.storage.save_in_progress(&attempt);
        self.current_attempt = Some(attempt);
        self.active_screen = Screen::Quiz;
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
