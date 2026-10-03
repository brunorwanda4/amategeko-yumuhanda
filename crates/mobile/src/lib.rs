//! Mobile entry points for Android and iOS.

extern crate gpui_mobile;

#[cfg(any(target_os = "ios", target_os = "android"))]
use gpui::{prelude::*, App, WindowOptions};

#[cfg(target_os = "android")]
use gpui::Application;

#[cfg(target_os = "android")]
use gpui_mobile::android::jni;

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

    let _platform = jni::init_platform(&app);
    let shared = match jni::shared_platform() {
        Some(s) => s,
        None => {
            log::error!("android_main: shared_platform() returned None");
            return;
        }
    };

    Application::with_platform(shared.into_rc()).run(|cx: &mut App| {
        gpui_kit::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| {
            let content = cx.new(|cx| amategeko_app::SpikeView::new(window, cx));
            cx.new(|cx| gpui_kit::component::Root::new(content, window, cx))
        })
        .expect("open mobile window");
    });
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn gpui_ios_register_app() {
    gpui_mobile::ios::ffi::set_app_callback(Box::new(|cx: &mut App| {
        gpui_kit::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| {
            let content = cx.new(|cx| amategeko_app::SpikeView::new(window, cx));
            cx.new(|cx| gpui_kit::component::Root::new(content, window, cx))
        })
        .expect("open mobile window");
    }));
}
