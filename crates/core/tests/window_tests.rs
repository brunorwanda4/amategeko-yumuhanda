use amategeko_core::window::{
    display_index_for, fit_window, is_on_any_display, min_window_size, DisplayRect, Rect,
    WindowState, DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE, SCREEN_MARGIN, WINDOW_BACKUP_FILE,
    WINDOW_FILE,
};
use amategeko_core::{InMemoryStorage, Storage};

fn display(x: f32, y: f32, width: f32, height: f32, primary: bool) -> DisplayRect {
    DisplayRect {
        bounds: Rect {
            x,
            y,
            width,
            height,
        },
        primary,
    }
}

fn full_hd() -> DisplayRect {
    display(0.0, 0.0, 1920.0, 1040.0, true)
}

fn assert_inside(state: &WindowState, d: &DisplayRect) {
    let b = d.bounds;
    assert!(state.x >= b.x, "left edge off-screen: {state:?}");
    assert!(state.y >= b.y, "top edge off-screen: {state:?}");
    assert!(
        state.x + state.width <= b.x + b.width,
        "right edge: {state:?}"
    );
    assert!(
        state.y + state.height <= b.y + b.height,
        "bottom edge: {state:?}"
    );
}

fn assert_centered(state: &WindowState, d: &DisplayRect) {
    let b = d.bounds;
    let cx = state.x + state.width / 2.0;
    let cy = state.y + state.height / 2.0;
    assert!(
        (cx - (b.x + b.width / 2.0)).abs() < 0.5,
        "not centered x: {state:?}"
    );
    assert!(
        (cy - (b.y + b.height / 2.0)).abs() < 0.5,
        "not centered y: {state:?}"
    );
}

#[test]
fn no_saved_state_gives_default_size_centered_on_primary() {
    let secondary = display(-1280.0, 0.0, 1280.0, 1024.0, false);
    let primary = full_hd();
    let state = fit_window(
        None,
        &[secondary, primary],
        DEFAULT_WINDOW_SIZE,
        MIN_WINDOW_SIZE,
    );

    assert_eq!(state.width, DEFAULT_WINDOW_SIZE.width);
    assert_eq!(state.height, DEFAULT_WINDOW_SIZE.height);
    assert!(!state.maximized);
    assert_centered(&state, &primary);
}

#[test]
fn default_size_respects_screen_fraction_and_minimum() {
    let laptop = display(0.0, 0.0, 1280.0, 680.0, true);
    let state = fit_window(None, &[laptop], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);

    assert!((state.width - 1024.0).abs() < 0.01);
    assert!((state.height - MIN_WINDOW_SIZE.height).abs() < 0.01);
    assert_centered(&state, &laptop);
}

#[test]
fn saved_state_that_fits_is_kept_exactly() {
    let saved = WindowState {
        x: 200.0,
        y: 100.0,
        width: 1000.0,
        height: 700.0,
        maximized: false,
    };
    let state = fit_window(
        Some(saved),
        &[full_hd()],
        DEFAULT_WINDOW_SIZE,
        MIN_WINDOW_SIZE,
    );
    assert_eq!(state, saved);
}

#[test]
fn saved_size_bigger_than_display_is_clamped() {
    let d = display(0.0, 0.0, 1366.0, 728.0, true);
    let saved = WindowState {
        x: 10.0,
        y: 10.0,
        width: 2400.0,
        height: 1400.0,
        maximized: false,
    };
    let state = fit_window(Some(saved), &[d], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);

    assert_eq!(state.width, 1366.0 - 2.0 * SCREEN_MARGIN);
    assert_eq!(state.height, 728.0 - 2.0 * SCREEN_MARGIN);
    assert_inside(&state, &d);
}

#[test]
fn saved_state_on_second_monitor_stays_there() {
    let primary = full_hd();
    let second = display(1920.0, 0.0, 2560.0, 1400.0, false);
    let saved = WindowState {
        x: 2200.0,
        y: 150.0,
        width: 1200.0,
        height: 800.0,
        maximized: false,
    };
    let displays = [primary, second];
    let state = fit_window(Some(saved), &displays, DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);

    assert_eq!(state, saved);
    assert_eq!(display_index_for(&state, &displays), Some(1));
}

#[test]
fn off_screen_position_is_centered_on_primary() {
    // Saved on a second monitor that is now unplugged.
    let primary = full_hd();
    let saved = WindowState {
        x: 2200.0,
        y: 150.0,
        width: 1200.0,
        height: 800.0,
        maximized: false,
    };
    let state = fit_window(
        Some(saved),
        &[primary],
        DEFAULT_WINDOW_SIZE,
        MIN_WINDOW_SIZE,
    );

    assert_eq!(state.width, 1200.0);
    assert_eq!(state.height, 800.0);
    assert_centered(&state, &primary);
    assert_eq!(display_index_for(&state, &[primary]), Some(0));
}

#[test]
fn partly_off_screen_window_is_pulled_inside() {
    let d = full_hd();
    let saved = WindowState {
        x: 1500.0,
        y: -40.0,
        width: 800.0,
        height: 600.0,
        maximized: false,
    };
    let state = fit_window(Some(saved), &[d], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);

    assert_eq!(state.width, 800.0);
    assert_eq!(state.height, 600.0);
    assert_inside(&state, &d);
}

#[test]
fn tiny_display_at_150_percent_scale_still_fits() {
    // 1366x768 at 150% is about 910x512 logical pixels.
    let tiny = display(0.0, 0.0, 910.0, 512.0, true);

    let fresh = fit_window(None, &[tiny], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert_inside(&fresh, &tiny);
    assert_centered(&fresh, &tiny);

    let saved = WindowState {
        x: 0.0,
        y: 0.0,
        width: 1100.0,
        height: 750.0,
        maximized: false,
    };
    let restored = fit_window(Some(saved), &[tiny], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert_inside(&restored, &tiny);

    // The minimum size shrinks too, so the OS never forces the window past the screen.
    let min = min_window_size(&tiny.bounds, MIN_WINDOW_SIZE);
    assert_eq!(min.width, MIN_WINDOW_SIZE.width);
    assert_eq!(min.height, 512.0 - 2.0 * SCREEN_MARGIN);
    assert!(fresh.height >= min.height && fresh.width >= min.width);
}

#[test]
fn maximized_is_kept() {
    let d = full_hd();
    let saved = WindowState {
        x: 100.0,
        y: 80.0,
        width: 1000.0,
        height: 700.0,
        maximized: true,
    };
    let state = fit_window(Some(saved), &[d], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert!(state.maximized);

    let moved = WindowState { x: 5000.0, ..saved };
    let state = fit_window(Some(moved), &[d], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert!(state.maximized);
    assert_centered(&state, &d);
}

#[test]
fn invalid_saved_numbers_fall_back_to_default() {
    let d = full_hd();
    let saved = WindowState {
        x: f32::NAN,
        y: 0.0,
        width: -5.0,
        height: 700.0,
        maximized: false,
    };
    let state = fit_window(Some(saved), &[d], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert_eq!(state.width, DEFAULT_WINDOW_SIZE.width);
    assert_centered(&state, &d);
}

#[test]
fn no_displays_does_not_panic() {
    let state = fit_window(None, &[], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert_eq!(state.width, DEFAULT_WINDOW_SIZE.width);
    assert_eq!(state.height, DEFAULT_WINDOW_SIZE.height);
}

#[test]
fn minimized_window_position_is_not_on_any_display() {
    // Windows parks minimized windows at about (-32000, -32000).
    let parked = Rect {
        x: -32000.0,
        y: -32000.0,
        width: 160.0,
        height: 28.0,
    };
    assert!(!is_on_any_display(&parked, &[full_hd()]));

    let normal = Rect {
        x: 100.0,
        y: 100.0,
        width: 800.0,
        height: 600.0,
    };
    assert!(is_on_any_display(&normal, &[full_hd()]));
}

#[test]
fn window_state_round_trips_through_storage() {
    let storage = InMemoryStorage::new();
    assert_eq!(storage.load_window_state(), None);

    let state = WindowState {
        x: 12.5,
        y: 40.0,
        width: 900.0,
        height: 640.0,
        maximized: true,
    };
    storage.save_window_state(&state).unwrap();
    assert_eq!(storage.load_window_state(), Some(state));

    storage.clear_window_state().unwrap();
    assert_eq!(storage.load_window_state(), None);
}

#[test]
fn corrupt_window_file_falls_back_to_default_and_keeps_backup() {
    let storage = InMemoryStorage::new();
    storage.write_file(WINDOW_FILE, "{ not json").unwrap();

    let loaded = storage.load_window_state();
    assert_eq!(loaded, None);
    assert_eq!(storage.read_file(WINDOW_BACKUP_FILE).unwrap(), "{ not json");

    let d = full_hd();
    let state = fit_window(loaded, &[d], DEFAULT_WINDOW_SIZE, MIN_WINDOW_SIZE);
    assert_eq!(state.width, DEFAULT_WINDOW_SIZE.width);
    assert_eq!(state.height, DEFAULT_WINDOW_SIZE.height);
    assert_centered(&state, &d);
}
