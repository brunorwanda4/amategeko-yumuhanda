#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use amategeko_app::{AppAssets, AppState, ShellView};
use amategeko_core::platform::{
    register_keep_awake, register_update_installer, set_keep_awake_window_active, KeepAwake,
    UpdateInstallOutcome, UpdateInstaller,
};
use amategeko_core::{Storage, SystemClock};
use directories::ProjectDirs;
use gpui_kit::component::Root;
use gpui_kit::*;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[cfg(target_os = "windows")]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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

struct DesktopUpdateInstaller;

#[cfg(target_os = "windows")]
const DETACHED_PROCESS: u32 = 0x0000_0008;
#[cfg(target_os = "windows")]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(target_os = "windows")]
const MB_ICONINFORMATION: u32 = 0x0000_0040;
#[cfg(target_os = "windows")]
const MB_SETFOREGROUND: u32 = 0x0001_0000;

#[cfg(target_os = "windows")]
#[link(name = "user32")]
unsafe extern "system" {
    fn MessageBoxW(
        window: *mut c_void,
        text: *const u16,
        caption: *const u16,
        message_type: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn show_update_ready_message() -> io::Result<()> {
    let text: Vec<u16> = "Ready. The app will close and update."
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let caption: Vec<u16> = "Amategeko y'Umuhanda"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // SAFETY: Both pointers reference live, NUL-terminated UTF-16 buffers for the call.
    let result = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            MB_ICONINFORMATION | MB_SETFOREGROUND,
        )
    };
    if result == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn launch_update_installer(package: &Path) -> io::Result<()> {
    std::process::Command::new(package)
        .args([
            "/VERYSILENT",
            "/SUPPRESSMSGBOXES",
            "/NORESTART",
            "/CLOSEAPPLICATIONS",
        ])
        .creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW)
        .spawn()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn show_update_ready_message() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "automatic update installation is only supported on Windows",
    ))
}

#[cfg(not(target_os = "windows"))]
fn launch_update_installer(_package: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "automatic update installation is only supported on Windows",
    ))
}

impl UpdateInstaller for DesktopUpdateInstaller {
    fn update_dir(&self) -> io::Result<PathBuf> {
        let dir = std::env::temp_dir().join("amategeko-yumuhanda-updates");
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    fn install(&self, package: &Path) -> io::Result<UpdateInstallOutcome> {
        let launch = (|| -> io::Result<()> {
            let update_dir = self.update_dir()?.canonicalize()?;
            let package = package.canonicalize()?;
            if !package.starts_with(&update_dir)
                || package.extension().and_then(|value| value.to_str()) != Some("exe")
            {
                return Err(io::Error::other("invalid update installer path"));
            }

            let install_dir = std::env::current_exe()?
                .parent()
                .map(Path::to_path_buf)
                .ok_or_else(|| io::Error::other("executable has no parent directory"))?;
            if !install_dir.join("unins000.exe").is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Inno Setup uninstaller not found",
                ));
            }

            show_update_ready_message()?;
            launch_update_installer(&package)
        })();

        if launch.is_err() {
            return Ok(UpdateInstallOutcome::ManualRequired);
        }
        std::process::exit(0);
    }

    fn open_download_page(&self, url: &str) -> io::Result<()> {
        std::process::Command::new("explorer.exe")
            .arg(url)
            .spawn()?;
        Ok(())
    }
}

fn cleanup_downloaded_installers() {
    std::thread::spawn(|| {
        let dir = std::env::temp_dir().join("amategeko-yumuhanda-updates");
        for _ in 0..30 {
            let mut pending = false;
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let matches =
                        path.file_name()
                            .and_then(|name| name.to_str())
                            .is_some_and(|name| {
                                name.starts_with("amategeko-yumuhanda-")
                                    && (name.ends_with("-windows-setup.exe")
                                        || name.ends_with("-windows-setup.exe.minisig"))
                            });
                    if matches && std::fs::remove_file(&path).is_err() {
                        pending = true;
                    }
                }
            }
            if !pending {
                let _ = std::fs::remove_dir(&dir);
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
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
    cleanup_downloaded_installers();
    let keep_awake = Arc::new(DesktopKeepAwake {
        ui_thread: std::thread::current().id(),
    });
    if !register_keep_awake(keep_awake) {
        log::warn!("keep-awake service was already registered");
    }
    if !register_update_installer(Arc::new(DesktopUpdateInstaller)) {
        log::warn!("update installer service was already registered");
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
