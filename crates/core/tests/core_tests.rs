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
            is_urgent,
        } => {
            assert_eq!(remaining_seconds, 1200);
            assert!(!is_urgent);
        }
        _ => panic!("Expected Countdown"),
    }

    // At 2100s, 100s remaining -> urgent!
    let state_urgent = QuizTimer::state(&attempt, 2100, true);
    match state_urgent {
        TimerState::Countdown {
            remaining_seconds,
            is_urgent,
        } => {
            assert_eq!(remaining_seconds, 100);
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
