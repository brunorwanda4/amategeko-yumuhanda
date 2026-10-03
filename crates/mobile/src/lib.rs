//! Mobile entry points for Android and iOS.

extern crate gpui_mobile;

#[cfg(any(target_os = "ios", target_os = "android"))]
use std::path::PathBuf;
#[cfg(any(target_os = "ios", target_os = "android"))]
use std::sync::Arc;

#[cfg(any(target_os = "ios", target_os = "android"))]
use amategeko_app::{AppState, ShellView};
#[cfg(any(target_os = "ios", target_os = "android"))]
use amategeko_core::{Storage, SystemClock};

#[cfg(any(target_os = "ios", target_os = "android"))]
use gpui::{prelude::*, App, WindowOptions};

#[cfg(target_os = "android")]
use gpui::Application;

#[cfg(target_os = "android")]
use gpui_mobile::android::jni;

#[cfg(any(target_os = "ios", target_os = "android"))]
struct MobileStorage {
    data_dir: PathBuf,
}

#[cfg(any(target_os = "ios", target_os = "android"))]
impl Storage for MobileStorage {
    fn storage_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("amategeko"),
    );

    jni::install_panic_hook();
    log::info!("android_main: entered");

    let data_dir = app
        .internal_data_path()
        .unwrap_or_else(|| PathBuf::from("/data/data/dev.gpui.mobile.example/files"));

    let storage = Arc::new(MobileStorage { data_dir });
    let clock = Arc::new(SystemClock);

    let _platform = jni::init_platform(&app);
    let shared = match jni::shared_platform() {
        Some(s) => s,
        None => {
            log::error!("android_main: shared_platform() returned None");
            return;
        }
    };

    Application::with_platform(shared.into_rc()).run(move |cx: &mut App| {
        gpui_kit::init(cx);

        let app_state = AppState::new(storage.clone(), clock.clone());

        cx.open_window(WindowOptions::default(), |window, cx| {
            let shell = cx.new(|_| ShellView::new(app_state));
            cx.new(|cx| gpui_kit::component::Root::new(shell, window, cx))
        })
        .expect("open mobile window");
    });
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn gpui_ios_register_app() {
    let data_dir =
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into())).join("Documents");
    let storage = Arc::new(MobileStorage { data_dir });
    let clock = Arc::new(SystemClock);

    gpui_mobile::ios::ffi::set_app_callback(Box::new(move |cx: &mut App| {
        gpui_kit::init(cx);

        let app_state = AppState::new(storage.clone(), clock.clone());

        cx.open_window(WindowOptions::default(), |window, cx| {
            let shell = cx.new(|_| ShellView::new(app_state));
            cx.new(|cx| gpui_kit::component::Root::new(shell, window, cx))
        })
        .expect("open mobile window");
    }));
}
