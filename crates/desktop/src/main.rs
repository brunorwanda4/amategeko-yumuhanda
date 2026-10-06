use amategeko_app::{AppAssets, AppState, ShellView};
use amategeko_core::{Storage, SystemClock};
use directories::ProjectDirs;
use gpui_kit::component::Root;
use gpui_kit::*;
use std::path::PathBuf;
use std::sync::Arc;

struct DesktopStorage {
    storage_dir: PathBuf,
}

impl Storage for DesktopStorage {
    fn storage_dir(&self) -> PathBuf {
        self.storage_dir.clone()
    }
}

fn main() {
    let storage_dir =
        if let Some(proj_dirs) = ProjectDirs::from("rw", "amategeko", "AmategekoYumuhanda") {
            proj_dirs.data_dir().to_path_buf()
        } else {
            PathBuf::from("data")
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
                ..Default::default()
            };

            let storage_for_window = storage.clone();
            let initial_font_scale = app_state.settings.font_size_scale;
            cx.open_window(options, move |window, cx| {
                amategeko_app::apply_theme(initial_theme, Some(window), cx);
                window.set_rem_size(px(amategeko_core::rem_px(initial_font_scale)));

                let storage_for_close = storage_for_window.clone();
                window.on_window_should_close(cx, move |window, _cx| {
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

                let shell = cx.new(|cx| ShellView::new(app_state, cx));
                cx.new(|cx| Root::new(shell, window, cx))
            })
            .expect("Failed to open desktop window");
        });
}
