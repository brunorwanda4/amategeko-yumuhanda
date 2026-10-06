use amategeko_core::*;
use std::collections::{HashMap, HashSet};

#[test]
fn test_question_bank_load_bundled() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    assert_eq!(bank.len(), 390);

    for q in bank.all() {
        assert!(q.id > 0);
        assert!(!q.text.trim().is_empty());
        assert!(q.options.len() >= 3 && q.options.len() <= 4);
        assert!(q.options.contains_key(&q.correct));
    }
}

#[test]
fn test_question_bank_search_and_filter() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let mut starred = HashSet::new();
    starred.insert(1);
    starred.insert(5);

    let mut stats = HashMap::new();
    let mut s1 = QuestionStat::new(1);
    s1.wrong_count = 3;
    stats.insert(1, s1);

    // Filter by number
    let res = bank.search("1", false, false, false, &starred, &stats);
    assert!(!res.is_empty());
    assert_eq!(res[0].id, 1);

    // Filter by image
    let res_img = bank.search("", true, false, false, &starred, &stats);
    assert_eq!(res_img.len(), 128);
    for q in res_img {
        assert!(q.has_image);
    }

    // Filter by starred
    let res_star = bank.search("", false, true, false, &starred, &stats);
    assert_eq!(res_star.len(), 2);

    // Filter by mistakes
    let res_mistakes = bank.search("", false, false, true, &starred, &stats);
    assert_eq!(res_mistakes.len(), 1);
    assert_eq!(res_mistakes[0].id, 1);
}

#[test]
fn test_quiz_generation_and_uniqueness() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();

    // Test Byoroshye (Easy)
    let attempt_easy = QuizEngine::start_quiz(&bank, QuizMode::Byoroshye, &settings, &stats, 1000)
        .expect("Easy attempt failed");
    assert_eq!(attempt_easy.total_questions(), 20);
    assert_eq!(attempt_easy.deadline_secs, None);

    // Verify 20 unique IDs
    let unique_ids: HashSet<u32> = attempt_easy.questions.iter().map(|q| q.id).collect();
    assert_eq!(unique_ids.len(), 20);

    // Test Hagati (Medium)
    let attempt_med = QuizEngine::start_quiz(&bank, QuizMode::Hagati, &settings, &stats, 1000)
        .expect("Medium attempt failed");
    assert_eq!(attempt_med.total_questions(), 20);
    assert_eq!(attempt_med.deadline_secs, Some(1000 + 20 * 60));

    // Test Bikomeye (Hard) with image weighting
    let attempt_hard = QuizEngine::start_quiz(&bank, QuizMode::Bikomeye, &settings, &stats, 1000)
        .expect("Hard attempt failed");
    assert_eq!(attempt_hard.total_questions(), 20);
    assert_eq!(attempt_hard.deadline_secs, Some(1000 + 12 * 60));
    let image_q_count = attempt_hard
        .questions
        .iter()
        .filter(|q| q.has_image)
        .count();
    assert!(
        image_q_count >= 5,
        "Hard mode should weight image questions"
    );
}

#[test]
fn test_easy_mode_instant_feedback_and_locking() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();

    let mut attempt = QuizEngine::start_quiz(&bank, QuizMode::Byoroshye, &settings, &stats, 1000)
        .expect("Quiz start failed");

    // First tap locks answer
    assert!(!attempt.is_current_locked());
    QuizEngine::select_option(&mut attempt, "a").expect("Select option failed");
    assert!(attempt.is_current_locked());
    assert_eq!(attempt.current_answer(), Some(&"a".to_string()));

    // Subsequent tap cannot change locked answer in Easy mode
    QuizEngine::select_option(&mut attempt, "b").expect("Select option failed");
    assert_eq!(attempt.current_answer(), Some(&"a".to_string()));

    // Can freely move forward and back
    assert!(QuizEngine::next_question(&mut attempt));
    assert_eq!(attempt.current_index, 1);
    assert!(QuizEngine::previous_question(&mut attempt));
    assert_eq!(attempt.current_index, 0);
}

#[test]
fn test_medium_mode_changeable_answers_flags_and_jumping() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();

    let mut attempt = QuizEngine::start_quiz(&bank, QuizMode::Hagati, &settings, &stats, 1000)
        .expect("Quiz start failed");

    // Answers are not locked; can be changed freely
    QuizEngine::select_option(&mut attempt, "a").expect("Select option failed");
    assert_eq!(attempt.current_answer(), Some(&"a".to_string()));
    assert!(!attempt.is_current_locked());

    QuizEngine::select_option(&mut attempt, "c").expect("Select option failed");
    assert_eq!(attempt.current_answer(), Some(&"c".to_string()));

    // Flags toggle
    assert!(!attempt.is_current_flagged());
    assert!(QuizEngine::toggle_current_flag(&mut attempt));
    assert!(attempt.is_current_flagged());
    assert!(!QuizEngine::toggle_current_flag(&mut attempt));
    assert!(!attempt.is_current_flagged());

    // Jump to question
    assert!(QuizEngine::jump_to_question(&mut attempt, 15));
    assert_eq!(attempt.current_index, 15);
}

#[test]
fn test_hard_mode_strict_navigation_and_validation() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();

    let mut attempt = QuizEngine::start_quiz(&bank, QuizMode::Bikomeye, &settings, &stats, 1000)
        .expect("Quiz start failed");

    // Going back is not allowed
    assert!(!QuizEngine::previous_question(&mut attempt));

    // Jumping is not allowed
    assert!(!QuizEngine::jump_to_question(&mut attempt, 5));

    // Confirm without selection fails with NoSelectionMade
    let err = QuizEngine::confirm_and_advance_hard(&mut attempt);
    assert!(err.is_err());
    match err.unwrap_err() {
        AppError::NoSelectionMade => {}
        other => panic!("Expected NoSelectionMade, got {other:?}"),
    }

    // Select and confirm succeeds
    QuizEngine::select_option(&mut attempt, "b").expect("Select failed");
    let finished = QuizEngine::confirm_and_advance_hard(&mut attempt).expect("Advance failed");
    assert!(!finished);
    assert_eq!(attempt.current_index, 1);
}

#[test]
fn test_retry_wrong_flow() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();

    let mut attempt = QuizEngine::start_quiz(&bank, QuizMode::Byoroshye, &settings, &stats, 1000)
        .expect("Quiz start failed");

    // Answer 5 correctly, rest unanswered or wrong
    for i in 0..5 {
        attempt.current_index = i;
        let correct = attempt.questions[i].correct.clone();
        QuizEngine::select_option(&mut attempt, &correct).unwrap();
    }

    let result = QuizEngine::finish_attempt(&mut attempt, 12, 1100);
    assert_eq!(result.score, 5);
    assert!(!result.passed);

    // Start retry wrong
    let retry_attempt =
        QuizEngine::start_retry_wrong(&result, 1200).expect("Retry wrong start failed");
    assert_eq!(retry_attempt.total_questions(), 15);
    assert_eq!(retry_attempt.mode, QuizMode::RetryWrong);
}

#[test]
fn test_deadline_timer() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();

    let attempt = QuizEngine::start_quiz(&bank, QuizMode::Hagati, &settings, &stats, 1000)
        .expect("Quiz start failed");

    // At 1000s, full 1200s (20 mins) remaining
    let state = QuizTimer::state(&attempt, 1000, true);
    match state {
        TimerState::Countdown {
            remaining_seconds,
            total_seconds,
            level,
            is_urgent,
        } => {
            assert_eq!(remaining_seconds, 1200);
            assert_eq!(total_seconds, 1200);
            assert_eq!(level, TimerLevel::Normal);
            assert!(!is_urgent);
        }
        _ => panic!("Expected Countdown"),
    }

    // At 2100s, 100s remaining -> urgent (Error level)!
    let state_urgent = QuizTimer::state(&attempt, 2100, true);
    match state_urgent {
        TimerState::Countdown {
            remaining_seconds,
            total_seconds,
            level,
            is_urgent,
        } => {
            assert_eq!(remaining_seconds, 100);
            assert_eq!(total_seconds, 1200);
            assert_eq!(level, TimerLevel::Error);
            assert!(is_urgent);
        }
        _ => panic!("Expected Countdown with urgency"),
    }

    // At 2200s -> expired!
    assert!(QuizTimer::is_expired(&attempt, 2200));
    let state_exp = QuizTimer::state(&attempt, 2201, true);
    assert_eq!(state_exp, TimerState::Expired);

    assert_eq!(QuizTimer::format_duration(125), "02:05");
}

#[test]
fn test_easy_elapsed_timer_starts_at_zero_and_resumes() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let settings = Settings::default();
    let stats = HashMap::new();
    let started_at = 10_000;
    let attempt = QuizEngine::start_quiz(&bank, QuizMode::Byoroshye, &settings, &stats, started_at)
        .expect("Quiz start failed");

    assert_eq!(
        QuizTimer::state(&attempt, started_at, false),
        TimerState::None
    );
    assert_eq!(
        QuizTimer::state(&attempt, started_at, true),
        TimerState::Elapsed(0)
    );

    let saved = serde_json::to_string(&attempt).expect("Attempt should serialize");
    let restored: Attempt = serde_json::from_str(&saved).expect("Attempt should deserialize");
    assert_eq!(restored.start_time_secs, started_at);
    assert_eq!(
        QuizTimer::state(&restored, started_at + 125, true),
        TimerState::Elapsed(125)
    );
}

#[test]
fn test_timer_levels_thresholds_20min_and_12min() {
    assert_eq!(WARNING_THRESHOLD_PERCENT, 25);
    assert_eq!(ERROR_THRESHOLD_PERCENT, 10);
    assert_eq!(TimerLevel::WARNING_THRESHOLD_PERCENT, 25);
    assert_eq!(TimerLevel::ERROR_THRESHOLD_PERCENT, 10);
    assert_eq!(QuizTimer::WARNING_THRESHOLD_PERCENT, 25);
    assert_eq!(QuizTimer::ERROR_THRESHOLD_PERCENT, 10);

    // Severity order
    assert!(TimerLevel::Warning > TimerLevel::Normal);
    assert!(TimerLevel::Error > TimerLevel::Warning);
    assert!(TimerLevel::Done > TimerLevel::Error);
    assert!(TimerLevel::Warning.is_worse_than(&TimerLevel::Normal));
    assert!(TimerLevel::Error.is_worse_than(&TimerLevel::Warning));

    // 20 min total = 1200s
    // 25% = 300s (05:00), 10% = 120s (02:00)
    let total_20m = 1200;
    assert_eq!(level(1200, total_20m), TimerLevel::Normal);
    assert_eq!(level(301, total_20m), TimerLevel::Normal);
    assert_eq!(level(300, total_20m), TimerLevel::Warning);
    assert_eq!(level(200, total_20m), TimerLevel::Warning);
    assert_eq!(level(121, total_20m), TimerLevel::Warning);
    assert_eq!(level(120, total_20m), TimerLevel::Error);
    assert_eq!(level(60, total_20m), TimerLevel::Error);
    assert_eq!(level(1, total_20m), TimerLevel::Error);
    assert_eq!(level(0, total_20m), TimerLevel::Done);

    // Also via TimerLevel::level and QuizTimer::level
    assert_eq!(TimerLevel::level(300, total_20m), TimerLevel::Warning);
    assert_eq!(QuizTimer::level(120, total_20m), TimerLevel::Error);

    // 12 min total = 720s
    // 25% = 180s (03:00), 10% = 72s (01:12)
    let total_12m = 720;
    assert_eq!(level(720, total_12m), TimerLevel::Normal);
    assert_eq!(level(181, total_12m), TimerLevel::Normal);
    assert_eq!(level(180, total_12m), TimerLevel::Warning);
    assert_eq!(level(100, total_12m), TimerLevel::Warning);
    assert_eq!(level(73, total_12m), TimerLevel::Warning);
    assert_eq!(level(72, total_12m), TimerLevel::Error);
    assert_eq!(level(30, total_12m), TimerLevel::Error);
    assert_eq!(level(1, total_12m), TimerLevel::Error);
    assert_eq!(level(0, total_12m), TimerLevel::Done);

    // Edge case: 0 total duration
    assert_eq!(level(10, 0), TimerLevel::Done);
}

#[test]
fn test_timer_level_never_goes_back_unless_restarted() {
    let mut tracker = TimerTracker::new();
    assert_eq!(tracker.current_level, TimerLevel::Normal);

    // Starting at 20 min (1200s) -> level is Normal, no transition reported
    assert_eq!(tracker.update(1200, 1200), None);
    assert_eq!(tracker.current_level, TimerLevel::Normal);

    // Remaining drops to 300s (Warning) -> transition reported
    assert_eq!(tracker.update(300, 1200), Some(TimerLevel::Warning));
    assert_eq!(tracker.current_level, TimerLevel::Warning);

    // Subsequent tick at 290s (still Warning) -> no transition reported
    assert_eq!(tracker.update(290, 1200), None);
    assert_eq!(tracker.current_level, TimerLevel::Warning);

    // Clock jitter or backwards tick: remaining increases to 600s
    // Level must NOT go back to Normal!
    assert_eq!(tracker.update(600, 1200), None);
    assert_eq!(tracker.current_level, TimerLevel::Warning);

    // Remaining drops to 120s (Error) -> transition reported
    assert_eq!(tracker.update(120, 1200), Some(TimerLevel::Error));
    assert_eq!(tracker.current_level, TimerLevel::Error);

    // Jitter: remaining increases to 200s (Warning) -> level remains Error!
    assert_eq!(tracker.update(200, 1200), None);
    assert_eq!(tracker.current_level, TimerLevel::Error);

    // Remaining drops to 0s (Done) -> transition reported
    assert_eq!(tracker.update(0, 1200), Some(TimerLevel::Done));
    assert_eq!(tracker.current_level, TimerLevel::Done);

    // Attempt restarts: reset() restores to Normal
    tracker.reset();
    assert_eq!(tracker.current_level, TimerLevel::Normal);
    assert_eq!(tracker.update(1200, 1200), None);
}

#[test]
fn test_timer_resume_past_threshold_reports_correct_level() {
    // 1. Resumes directly past Warning threshold (e.g. 250s remaining of 1200s)
    let mut tracker1 = TimerTracker::new();
    assert_eq!(tracker1.update(250, 1200), Some(TimerLevel::Warning));
    assert_eq!(tracker1.current_level, TimerLevel::Warning);
    // Next tick does not repeat banner
    assert_eq!(tracker1.update(240, 1200), None);

    // 2. Resumes directly past Error threshold (e.g. 80s remaining of 1200s)
    let mut tracker2 = TimerTracker::new();
    assert_eq!(tracker2.update(80, 1200), Some(TimerLevel::Error));
    assert_eq!(tracker2.current_level, TimerLevel::Error);
    assert_eq!(tracker2.update(70, 1200), None);

    // 3. Resumes already expired (0s remaining)
    let mut tracker3 = TimerTracker::new();
    assert_eq!(tracker3.update(0, 1200), Some(TimerLevel::Done));
    assert_eq!(tracker3.current_level, TimerLevel::Done);
}

#[test]
fn test_project_credit_and_version_metadata() {
    assert_eq!(PROJECT_CREDIT.name, "Rwanda Bruno");
    assert_eq!(PROJECT_CREDIT.url, "https://github.com/brunorwanda4");
    assert_eq!(APP_VERSION, env!("CARGO_PKG_VERSION"));
    assert_eq!(t("about.built_by", Language::En), "Built by");
    assert_eq!(t("about.built_by", Language::Rw), "Byakozwe na");
}

#[test]
fn test_i18n_parity_and_timer_strings() {
    let rw_content = include_str!("../../../assets/i18n/rw.json");
    let en_content = include_str!("../../../assets/i18n/en.json");

    let rw_json: serde_json::Value =
        serde_json::from_str(rw_content).expect("rw.json must be valid JSON");
    let en_json: serde_json::Value =
        serde_json::from_str(en_content).expect("en.json must be valid JSON");

    let rw_map = rw_json.as_object().expect("rw.json root must be object");
    let en_map = en_json.as_object().expect("en.json root must be object");

    // Parity check: same keys in both files
    assert_eq!(
        rw_map.keys().collect::<HashSet<_>>(),
        en_map.keys().collect::<HashSet<_>>()
    );

    // Values non-empty check
    for (k, v) in rw_map {
        let s = v.as_str().expect("rw value must be string");
        assert!(!s.trim().is_empty(), "rw key {k} is empty");
    }
    for (k, v) in en_map {
        let s = v.as_str().expect("en value must be string");
        assert!(!s.trim().is_empty(), "en key {k} is empty");
    }

    assert_eq!(
        rw_map.get("timer.warning").and_then(|v| v.as_str()),
        Some("Igihe kiregereje. Hasigaye {time}.")
    );
    assert_eq!(
        en_map.get("timer.warning").and_then(|v| v.as_str()),
        Some("Time is running low. {time} left.")
    );

    assert_eq!(
        rw_map.get("timer.error").and_then(|v| v.as_str()),
        Some("Igihe kigiye kurangira! Hasigaye {time}.")
    );
    assert_eq!(
        en_map.get("timer.error").and_then(|v| v.as_str()),
        Some("Time is almost up! {time} left.")
    );

    assert_eq!(
        rw_map.get("timer.done").and_then(|v| v.as_str()),
        Some("Igihe kirarangiye. Ikizamini cyoherejwe.")
    );
    assert_eq!(
        en_map.get("timer.done").and_then(|v| v.as_str()),
        Some("Time is up. Exam submitted.")
    );

    // In-app formatting helpers
    assert_eq!(
        Strings::timer_warning("05:00"),
        "Time is running low. 05:00 left."
    );
    assert_eq!(
        Strings::timer_warning_rw("05:00"),
        "Igihe kiregereje. Hasigaye 05:00."
    );
    assert_eq!(
        Strings::timer_error("02:00"),
        "Time is almost up! 02:00 left."
    );
    assert_eq!(
        Strings::timer_error_rw("02:00"),
        "Igihe kigiye kurangira! Hasigaye 02:00."
    );
    assert_eq!(Strings::timer_done(), "Time is up. Exam submitted.");
    assert_eq!(
        Strings::timer_done_rw(),
        "Igihe kirarangiye. Ikizamini cyoherejwe."
    );
}

#[test]
fn test_settings_defaults_and_language() {
    let settings = Settings::default();
    assert!(!settings.easy_show_timer);
    assert_eq!(settings.language, Language::En);
    assert_eq!(settings.question_language, Language::En);
    assert!(!settings.show_both_languages);
    assert!(!settings.shuffle_options);
    assert_eq!(settings.pass_mark, 12);
    assert_eq!(settings.medium_duration_mins, 20);
    assert_eq!(settings.hard_duration_mins, 12);
}

#[test]
fn test_old_settings_file_loads_without_shuffle_options() {
    let mut value = serde_json::to_value(Settings::default()).unwrap();
    value.as_object_mut().unwrap().remove("shuffle_options");
    let loaded: Settings = serde_json::from_value(value).unwrap();
    assert!(!loaded.shuffle_options);
}

#[test]
fn test_option_order_unshuffled_is_original() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let attempt = QuizEngine::start_quiz(
        &bank,
        QuizMode::Hagati,
        &Settings::default(),
        &HashMap::new(),
        1000,
    )
    .expect("Quiz start failed");

    for idx in 0..attempt.total_questions() {
        let expected: Vec<String> = attempt.questions[idx]
            .options_for(Language::En)
            .keys()
            .cloned()
            .collect();
        assert_eq!(attempt.option_order(idx, Language::En, false), expected);
    }
}

#[test]
fn test_option_order_shuffled_is_stable_and_keeps_all_keys() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let attempt = QuizEngine::start_quiz(
        &bank,
        QuizMode::Hagati,
        &Settings::default(),
        &HashMap::new(),
        1000,
    )
    .expect("Quiz start failed");

    let mut changed = 0;
    for idx in 0..attempt.total_questions() {
        let original = attempt.option_order(idx, Language::En, false);
        let first = attempt.option_order(idx, Language::En, true);
        let second = attempt.option_order(idx, Language::En, true);

        // Same attempt and question always gives the same order.
        assert_eq!(first, second);

        // Same set of keys, so saved answers and the correct key stay valid.
        let mut sorted = first.clone();
        sorted.sort();
        assert_eq!(sorted, original);
        assert!(first.contains(&attempt.questions[idx].correct));

        if first != original {
            changed += 1;
        }
    }
    assert!(changed > 0, "shuffle never changed the order");
}

#[test]
fn test_option_order_differs_between_attempts() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let start = |now| {
        QuizEngine::start_quiz_with_questions(
            QuizMode::Byoroshye,
            bank.all()[..30].to_vec(),
            &Settings::default(),
            now,
        )
        .expect("Quiz start failed")
    };
    let first = start(1000);
    let second = start(2000);

    let differs = (0..first.total_questions()).any(|i| {
        first.option_order(i, Language::En, true) != second.option_order(i, Language::En, true)
    });
    assert!(differs);
}

#[test]
fn test_option_key_at_maps_screen_position_to_key() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    let attempt = QuizEngine::start_quiz(
        &bank,
        QuizMode::Byoroshye,
        &Settings::default(),
        &HashMap::new(),
        1000,
    )
    .expect("Quiz start failed");

    let order = attempt.option_order(0, Language::En, true);
    for (pos, key) in order.iter().enumerate() {
        assert_eq!(
            attempt.option_key_at(Language::En, true, pos).as_ref(),
            Some(key)
        );
    }
    assert_eq!(attempt.option_key_at(Language::En, true, order.len()), None);
}

#[test]
fn test_bilingual_questions_and_fallback() {
    let bank = QuestionBank::load_bundled().expect("Failed to load bundled questions");
    assert_eq!(bank.len(), 390);

    for q in bank.all() {
        // Primary text in English should not be empty
        let text_en = q.text_for(Language::En);
        assert!(!text_en.trim().is_empty());

        // Primary text in Kinyarwanda should not be empty
        let text_rw = q.text_for(Language::Rw);
        assert!(!text_rw.trim().is_empty());

        // Secondary text for EN should give RW
        let sec_for_en = q.secondary_text_for(Language::En);
        assert!(sec_for_en.is_some());
        assert_eq!(sec_for_en.unwrap(), text_rw);

        // Secondary text for RW should give EN
        let sec_for_rw = q.secondary_text_for(Language::Rw);
        assert!(sec_for_rw.is_some());
        assert_eq!(sec_for_rw.unwrap(), text_en);

        // Options
        let opts_en = q.options_for(Language::En);
        let opts_rw = q.options_for(Language::Rw);
        assert_eq!(opts_en.len(), opts_rw.len());
        assert!(opts_en.contains_key(&q.correct));
        assert!(opts_rw.contains_key(&q.correct));
    }

    // Test fallback when EN is missing
    let mut custom_q = Question {
        id: 9999,
        text: "Kinyarwanda text".into(),
        options: [("a".into(), "Kinyarwanda opt".into())]
            .into_iter()
            .collect(),
        correct: "a".into(),
        image: None,
        has_image: false,
        text_en: None,
        options_en: None,
        text_rw: Some("Kinyarwanda text".into()),
        options_rw: Some(
            [("a".into(), "Kinyarwanda opt".into())]
                .into_iter()
                .collect(),
        ),
        status_en: None,
    };
    // Should fall back to Kinyarwanda
    assert_eq!(custom_q.text_for(Language::En), "Kinyarwanda text");
    assert_eq!(
        custom_q.options_for(Language::En).get("a").unwrap(),
        "Kinyarwanda opt"
    );

    custom_q.text_en = Some("English text".into());
    assert_eq!(custom_q.text_for(Language::En), "English text");
}

#[test]
fn test_stats_calculator() {
    let mut progress = Progress::default();

    // 1st attempt: 15/20 passed
    let res1 = AttemptResult {
        id: "att_1".into(),
        mode: QuizMode::Byoroshye,
        score: 15,
        total: 20,
        pass_mark: 12,
        passed: true,
        timestamp_secs: 1000,
        duration_seconds: 300,
        question_results: vec![
            QuestionResult {
                question: Question {
                    id: 10,
                    text: "Q10".into(),
                    options: Default::default(),
                    correct: "a".into(),
                    image: None,
                    has_image: false,
                    text_en: None,
                    options_en: None,
                    text_rw: None,
                    options_rw: None,
                    status_en: None,
                },
                user_answer: Some("a".into()),
                correct_answer: "a".into(),
                is_correct: true,
            },
            QuestionResult {
                question: Question {
                    id: 20,
                    text: "Q20".into(),
                    options: Default::default(),
                    correct: "b".into(),
                    image: None,
                    has_image: false,
                    text_en: None,
                    options_en: None,
                    text_rw: None,
                    options_rw: None,
                    status_en: None,
                },
                user_answer: Some("c".into()),
                correct_answer: "b".into(),
                is_correct: false,
            },
        ],
    };
    progress.record_attempt_result(res1);

    // 2nd attempt: 10/20 failed
    let res2 = AttemptResult {
        id: "att_2".into(),
        mode: QuizMode::Hagati,
        score: 10,
        total: 20,
        pass_mark: 12,
        passed: false,
        timestamp_secs: 2000,
        duration_seconds: 500,
        question_results: vec![QuestionResult {
            question: Question {
                id: 20,
                text: "Q20".into(),
                options: Default::default(),
                correct: "b".into(),
                image: None,
                has_image: false,
                text_en: None,
                options_en: None,
                text_rw: None,
                options_rw: None,
                status_en: None,
            },
            user_answer: Some("d".into()),
            correct_answer: "b".into(),
            is_correct: false,
        }],
    };
    progress.record_attempt_result(res2);

    let summary = StatsCalculator::compute_summary(&progress);
    assert_eq!(summary.total_attempts, 2);
    assert_eq!(summary.high_score, 15);
    assert_eq!(summary.average_score, 12.5);
    assert_eq!(summary.pass_rate_percentage, 50.0);
    assert_eq!(summary.last_10_scores, vec![(15, 12), (10, 12)]);

    // Q20 was missed twice
    assert_eq!(summary.most_missed.len(), 1);
    assert_eq!(summary.most_missed[0].question_id, 20);
    assert_eq!(summary.most_missed[0].wrong_count, 2);
}

#[test]
fn test_in_memory_persistence() {
    let storage = InMemoryStorage::new();

    // Settings
    let settings = Settings {
        pass_mark: 14,
        ..Default::default()
    };
    storage.save_settings(&settings).unwrap();

    let loaded_settings = storage.load_settings();
    assert_eq!(loaded_settings.pass_mark, 14);

    // Progress
    let mut progress = Progress::default();
    progress.toggle_starred(42);
    storage.save_progress(&progress).unwrap();

    let loaded_progress = storage.load_progress();
    assert!(loaded_progress.is_starred(42));
    assert!(!loaded_progress.is_starred(99));
}
