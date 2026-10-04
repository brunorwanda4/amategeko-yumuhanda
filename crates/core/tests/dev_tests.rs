use amategeko_core::dev::{
    apply_timer_override, parse_timer_arg, BrowseFilter, DevScreen, DevState, DEV_STATE_FILE,
};
use amategeko_core::models::{Attempt, QuizMode};
use amategeko_core::platform::{InMemoryStorage, Storage};

#[test]
fn test_load_dev_state_missing_file() {
    let storage = InMemoryStorage::new();
    assert_eq!(storage.load_dev_state(), None);
}

#[test]
fn test_load_dev_state_corrupt_file() {
    let storage = InMemoryStorage::new();

    // Malformed JSON syntax
    let _ = storage.write_file(DEV_STATE_FILE, "{ corrupt: json syntax, ");
    assert_eq!(storage.load_dev_state(), None);

    // Truncated file
    let _ = storage.write_file(DEV_STATE_FILE, "{\"screen\": \"qu");
    assert_eq!(storage.load_dev_state(), None);

    // Invalid data types (e.g. string for scroll_position)
    let _ = storage.write_file(
        DEV_STATE_FILE,
        "{\"screen\": \"quiz\", \"scroll_position\": \"not_a_number\"}",
    );
    assert_eq!(storage.load_dev_state(), None);

    // Negative / invalid scroll position
    let _ = storage.write_file(
        DEV_STATE_FILE,
        "{\"screen\": \"quiz\", \"scroll_position\": -100.0}",
    );
    assert_eq!(storage.load_dev_state(), None);
}

#[test]
fn test_load_dev_state_old_or_partial_file() {
    let storage = InMemoryStorage::new();

    // Minimal empty object
    let _ = storage.write_file(DEV_STATE_FILE, "{}");
    let state = storage.load_dev_state().expect("Should parse empty object");
    assert_eq!(state.screen, DevScreen::Home);
    assert_eq!(state.browse_filter, None);
    assert_eq!(state.scroll_position, 0.0);

    // Old format with only screen
    let _ = storage.write_file(DEV_STATE_FILE, "{\"screen\": \"stats\"}");
    let state = storage.load_dev_state().expect("Should parse old state");
    assert_eq!(state.screen, DevScreen::Stats);
    assert_eq!(state.browse_filter, None);
    assert_eq!(state.scroll_position, 0.0);

    // Alias check for questions screen
    let _ = storage.write_file(DEV_STATE_FILE, "{\"screen\": \"questions\"}");
    let state = storage
        .load_dev_state()
        .expect("Should parse questions alias");
    assert_eq!(state.screen, DevScreen::Browse);
}

#[test]
fn test_load_dev_state_unknown_future_fields() {
    let storage = InMemoryStorage::new();

    let json = r#"{
        "screen": "browse",
        "browse_filter": "has_image",
        "future_field_xyz": 12345,
        "unknown_object": { "a": true }
    }"#;
    let _ = storage.write_file(DEV_STATE_FILE, json);
    let state = storage
        .load_dev_state()
        .expect("Should ignore unknown fields");
    assert_eq!(state.screen, DevScreen::Browse);
    assert_eq!(state.browse_filter, Some(BrowseFilter::HasImage));
}

#[test]
fn test_save_and_load_dev_state_roundtrip() {
    let storage = InMemoryStorage::new();

    let mut state = DevState::new(DevScreen::Browse);
    state.browse_filter = Some(BrowseFilter::Starred);
    state.stats_filter = Some("all".to_string());
    state.results_filter = Some("wrong".to_string());
    state.scroll_position = 280.5;

    storage.save_dev_state(&state).expect("Save should succeed");

    let loaded = storage.load_dev_state().expect("Load should succeed");
    assert_eq!(loaded.screen, DevScreen::Browse);
    assert_eq!(loaded.browse_filter, Some(BrowseFilter::Starred));
    assert_eq!(loaded.stats_filter.as_deref(), Some("all"));
    assert_eq!(loaded.results_filter.as_deref(), Some("wrong"));
    assert!((loaded.scroll_position - 280.5).abs() < f32::EPSILON);

    storage.clear_dev_state().expect("Clear should delete file");
    assert_eq!(storage.load_dev_state(), None);
}

#[test]
fn test_parse_timer_arg() {
    assert_eq!(parse_timer_arg("30s"), Some(30));
    assert_eq!(parse_timer_arg("30S"), Some(30));
    assert_eq!(parse_timer_arg("30"), Some(30));
    assert_eq!(parse_timer_arg(" 45s "), Some(45));
    assert_eq!(parse_timer_arg("0s"), None);
    assert_eq!(parse_timer_arg("invalid"), None);
    assert_eq!(parse_timer_arg(""), None);
}

#[test]
fn test_apply_timer_override() {
    let start_time = 1_000_000u64;
    let total_duration = 1200u64; // 20 mins
    let mut attempt = Attempt::new(
        "test_hagati".to_string(),
        QuizMode::Hagati,
        vec![],
        start_time,
        Some(total_duration),
    );

    let now = start_time + 100;
    apply_timer_override(&mut attempt, 30, now);

    assert_eq!(attempt.deadline_secs, Some(now + 30));
    assert_eq!(attempt.start_time_secs, now - (total_duration - 30));
    assert_eq!(
        attempt.total_duration_secs(),
        Some(total_duration as u32),
        "Total duration must be preserved"
    );
    assert_eq!(
        attempt.deadline_secs.unwrap() - now,
        30,
        "Remaining seconds must equal 30s"
    );
}
