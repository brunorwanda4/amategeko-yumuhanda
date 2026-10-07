#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use amategeko_app::{AppAssets, AppState, ShellView};
use amategeko_core::platform::{register_keep_awake, set_keep_awake_window_active, KeepAwake};
use amategeko_core::{Storage, SystemClock};
use directories::ProjectDirs;
use gpui_kit::component::Root;
use gpui_kit::*;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const LEGACY_MIGRATION_MARKER: &str = ".legacy-data-migrated-v1";
const PERSISTED_FILES: &[&str] = &[
    "settings.json",
    "progress.json",
    "in_progress.json",
    "window.json",
    "window.json.bak",
];

struct DesktopStorage {
    storage_dir: PathBuf,
}

impl Storage for DesktopStorage {
    fn storage_dir(&self) -> PathBuf {
        self.storage_dir.clone()
    }
}

struct DesktopKeepAwake {
    ui_thread: std::thread::ThreadId,
}

#[cfg(target_os = "windows")]
impl KeepAwake for DesktopKeepAwake {
    fn set(&self, on: bool) {
        if std::thread::current().id() != self.ui_thread {
            log::warn!("keep-awake change was requested outside the UI thread");
            return;
        }

        const ES_CONTINUOUS: u32 = 0x8000_0000;
        const ES_SYSTEM_REQUIRED: u32 = 0x0000_0001;
        const ES_DISPLAY_REQUIRED: u32 = 0x0000_0002;

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn SetThreadExecutionState(flags: u32) -> u32;
        }

        let flags = if on {
            ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED
        } else {
            ES_CONTINUOUS
        };
        // SAFETY: SetThreadExecutionState accepts this documented bitmask and has no
        // pointer arguments. Both transitions are restricted to the registering UI thread.
        if unsafe { SetThreadExecutionState(flags) } == 0 {
            log::warn!("SetThreadExecutionState failed");
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl KeepAwake for DesktopKeepAwake {
    fn set(&self, on: bool) {
        log::debug!("keep-awake request ignored on this desktop platform: {on}");
    }
}

fn prepare_storage_dir() -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("rw", "amategeko", "AmategekoYumuhanda")
        .ok_or_else(|| io::Error::other("Windows app-data directory is unavailable"))?;
    let storage_dir = project_dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&storage_dir)?;
    migrate_legacy_data(&storage_dir)?;
    Ok(storage_dir)
}

fn migrate_legacy_data(storage_dir: &Path) -> io::Result<()> {
    let marker = storage_dir.join(LEGACY_MIGRATION_MARKER);
    if marker.exists() {
        return Ok(());
    }

    let mut legacy_dirs = Vec::new();
    if let Ok(current_dir) = std::env::current_dir() {
        legacy_dirs.push(current_dir.join("data"));
    }
    if let Ok(executable) = std::env::current_exe() {
        if let Some(parent) = executable.parent() {
            legacy_dirs.push(parent.join("data"));
        }
    }

    for legacy_dir in legacy_dirs {
        for filename in PERSISTED_FILES {
            let source = legacy_dir.join(filename);
            let destination = storage_dir.join(filename);
            if source.is_file() && !destination.exists() {
                std::fs::copy(source, destination)?;
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        let filename = amategeko_core::dev::DEV_STATE_FILE;
        for legacy_dir in [
            std::env::current_dir().ok().map(|path| path.join("data")),
            std::env::current_exe()
                .ok()
                .and_then(|path| path.parent().map(|parent| parent.join("data"))),
        ]
        .into_iter()
        .flatten()
        {
            let source = legacy_dir.join(filename);
            let destination = storage_dir.join(filename);
            if source.is_file() && !destination.exists() {
                std::fs::copy(source, destination)?;
            }
        }
    }

    std::fs::write(marker, b"migrated\n")
}

fn main() {
    let keep_awake = Arc::new(DesktopKeepAwake {
        ui_thread: std::thread::current().id(),
    });
    if !register_keep_awake(keep_awake) {
        log::warn!("keep-awake service was already registered");
    }

    let storage_dir = match prepare_storage_dir() {
        Ok(storage_dir) => storage_dir,
        Err(error) => {
            eprintln!("Unable to prepare the application data folder: {error}");
            return;
        }
    };

    let storage = Arc::new(DesktopStorage { storage_dir });
    let clock = Arc::new(SystemClock);

    gpui_kit::application()
        .with_assets(AppAssets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);

            #[allow(unused_mut)]
            let mut app_state = AppState::new(storage.clone(), clock.clone());

            #[cfg(debug_assertions)]
            {
                let args: Vec<String> = std::env::args().collect();
                let mut i = 1;
                while i < args.len() {
                    if args[i] == "--timer" && i + 1 < args.len() {
                        app_state.debug_timer_override =
                            amategeko_core::dev::parse_timer_arg(&args[i + 1]);
                        i += 2;
                    } else if let Some(val) = args[i].strip_prefix("--timer=") {
                        app_state.debug_timer_override = amategeko_core::dev::parse_timer_arg(val);
                        i += 1;
                    } else {
                        i += 1;
                    }
                }
            }

            let initial_theme = app_state.settings.theme;
            amategeko_app::init_fonts(app_state.settings.font_size_scale, cx);
            amategeko_app::apply_theme(initial_theme, None, cx);

            let primary_id = cx.primary_display().map(|d| d.id());
            let displays: Vec<amategeko_core::window::DisplayRect> = cx
                .displays()
                .into_iter()
                .map(|d| {
                    let b = d.bounds();
                    let is_primary = Some(d.id()) == primary_id;
                    amategeko_core::window::DisplayRect {
                        bounds: amategeko_core::window::Rect {
                            x: b.origin.x.as_f32(),
                            y: b.origin.y.as_f32(),
                            width: b.size.width.as_f32(),
                            height: b.size.height.as_f32(),
                        },
                        primary: is_primary,
                    }
                })
                .collect();

            let saved_window = storage.load_window_state();
            let fitted = amategeko_core::window::fit_window(
                saved_window,
                &displays,
                amategeko_core::window::DEFAULT_WINDOW_SIZE,
                amategeko_core::window::MIN_WINDOW_SIZE,
            );
            let min_display = amategeko_core::window::display_index_for(&fitted, &displays)
                .and_then(|index| displays.get(index))
                .or_else(|| displays.iter().find(|display| display.primary))
                .or_else(|| displays.first());
            let (min_window_w, min_window_h) = min_display
                .map(|display| {
                    amategeko_core::window::min_size(display.bounds.width, display.bounds.height)
                })
                .unwrap_or((
                    amategeko_core::window::MIN_WINDOW_W,
                    amategeko_core::window::MIN_WINDOW_H,
                ));

            let bounds = Bounds {
                origin: point(px(fitted.x), px(fitted.y)),
                size: size(px(fitted.width), px(fitted.height)),
            };

            let window_bounds = if fitted.maximized {
                Some(WindowBounds::Maximized(bounds))
            } else {
                Some(WindowBounds::Windowed(bounds))
            };

            let options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Amategeko y'Umuhanda".into()),
                    ..Default::default()
                }),
                window_bounds,
                window_min_size: Some(size(px(min_window_w), px(min_window_h))),
                ..Default::default()
            };

            let storage_for_window = storage.clone();
            let initial_font_scale = app_state.settings.font_size_scale;
            cx.open_window(options, move |window, cx| {
                amategeko_app::apply_theme(initial_theme, Some(window), cx);
                window.set_rem_size(px(amategeko_core::rem_px(initial_font_scale)));

                let storage_for_close = storage_for_window.clone();
                window.on_window_should_close(cx, move |window, _cx| {
                    set_keep_awake_window_active(false);
                    let b = window.bounds();
                    let is_max = window.is_maximized();
                    let state = amategeko_core::window::WindowState {
                        x: b.origin.x.as_f32(),
                        y: b.origin.y.as_f32(),
                        width: b.size.width.as_f32(),
                        height: b.size.height.as_f32(),
                        maximized: is_max,
                    };
                    let _ = storage_for_close.save_window_state(&state);
                    true
                });

                let shell = cx.new(|cx| {
                    cx.observe_window_activation(window, |_this, window, _cx| {
                        set_keep_awake_window_active(window.is_window_active());
                    })
                    .detach();
                    ShellView::new(app_state, cx)
                });
                cx.new(|cx| Root::new(shell, window, cx))
            })
            .expect("Failed to open desktop window");
        });
}
