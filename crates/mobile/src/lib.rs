//! Mobile entry points for Android and iOS.

extern crate gpui_mobile;

#[cfg(any(target_os = "ios", target_os = "android"))]
use std::path::PathBuf;
#[cfg(any(target_os = "ios", target_os = "android"))]
use std::sync::Arc;

#[cfg(any(target_os = "ios", target_os = "android"))]
use amategeko_app::{AppState, ShellView};
#[cfg(any(target_os = "ios", target_os = "android"))]
use amategeko_core::platform::{
    register_keep_awake, register_update_installer, KeepAwake, UpdateInstallOutcome,
    UpdateInstaller,
};
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
struct AndroidKeepAwake {
    app: android_activity::AndroidApp,
}

#[cfg(target_os = "android")]
struct AndroidUpdateInstaller {
    app: android_activity::AndroidApp,
}

#[cfg(target_os = "android")]
impl AndroidUpdateInstaller {
    fn with_env<T>(
        &self,
        operation: impl FnOnce(
            &mut ::jni::JNIEnv<'_>,
            ::jni::objects::JObject<'_>,
        ) -> ::jni::errors::Result<T>,
    ) -> std::io::Result<T> {
        use ::jni::JavaVM;

        // SAFETY: AndroidActivity owns the VM for the process and keeps it alive
        // for the lifetime of this registered service.
        let vm = unsafe { JavaVM::from_raw(self.app.vm_as_ptr().cast()) }
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        let mut env = vm
            .attach_current_thread()
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        // SAFETY: AndroidActivity supplies this live Activity reference, used only
        // for the duration of the JNI call.
        let activity =
            unsafe { ::jni::objects::JObject::from_raw(self.app.activity_as_ptr().cast()) };
        operation(&mut env, activity).map_err(|error| std::io::Error::other(error.to_string()))
    }
}

#[cfg(target_os = "android")]
impl UpdateInstaller for AndroidUpdateInstaller {
    fn update_dir(&self) -> std::io::Result<PathBuf> {
        self.with_env(|env, activity| {
            let value =
                env.call_method(activity, "updateCachePath", "()Ljava/lang/String;", &[])?;
            let object = value.l()?;
            let string = ::jni::objects::JString::from(object);
            let path = env.get_string(&string)?.to_string_lossy().into_owned();
            Ok(PathBuf::from(path))
        })
    }

    fn install(&self, package: &std::path::Path) -> std::io::Result<UpdateInstallOutcome> {
        let path = package.to_string_lossy().into_owned();
        self.with_env(|env, activity| {
            let path = env.new_string(path)?;
            let result = env
                .call_method(
                    activity,
                    "requestInstallUpdate",
                    "(Ljava/lang/String;)I",
                    &[::jni::objects::JValue::Object(&path)],
                )?
                .i()?;
            Ok(match result {
                0 => UpdateInstallOutcome::Launched,
                1 => UpdateInstallOutcome::PermissionRequired,
                _ => UpdateInstallOutcome::ManualRequired,
            })
        })
    }

    fn open_download_page(&self, url: &str) -> std::io::Result<()> {
        let url = url.to_owned();
        self.with_env(|env, activity| {
            let url = env.new_string(url)?;
            env.call_method(
                activity,
                "openExternalUrl",
                "(Ljava/lang/String;)V",
                &[::jni::objects::JValue::Object(&url)],
            )?;
            Ok(())
        })
    }
}

#[cfg(target_os = "android")]
impl KeepAwake for AndroidKeepAwake {
    fn set(&self, on: bool) {
        use ::jni::objects::{JObject, JValue};
        use ::jni::JavaVM;

        // SAFETY: AndroidActivity owns the VM for the process and keeps it alive for
        // the lifetime of this platform service.
        let vm = match unsafe { JavaVM::from_raw(self.app.vm_as_ptr().cast()) } {
            Ok(vm) => vm,
            Err(error) => {
                log::warn!("unable to access Android VM for keep-awake: {error}");
                return;
            }
        };
        let mut env = match vm.attach_current_thread() {
            Ok(env) => env,
            Err(error) => {
                log::warn!("unable to attach Android thread for keep-awake: {error}");
                return;
            }
        };
        // SAFETY: activity_as_ptr is the live Activity reference supplied by
        // android-activity; it is used only for this JNI method call.
        let activity = unsafe { JObject::from_raw(self.app.activity_as_ptr().cast()) };
        if let Err(error) = env.call_method(
            activity,
            "setKeepScreenOn",
            "(Z)V",
            &[JValue::Bool(if on { 1 } else { 0 })],
        ) {
            log::warn!("unable to change Android keep-awake flag: {error}");
        }
    }
}

#[cfg(target_os = "ios")]
struct IosKeepAwake;

#[cfg(target_os = "ios")]
impl KeepAwake for IosKeepAwake {
    fn set(&self, on: bool) {
        use objc2::runtime::AnyObject;
        use objc2::{class, msg_send};

        // SAFETY: this runs from GPUI's main thread and sends documented UIApplication
        // selectors with the expected argument and return types.
        unsafe {
            let application: *mut AnyObject = msg_send![class!(UIApplication), sharedApplication];
            let _: () = msg_send![application, setIdleTimerDisabled: on];
        }
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

    if !register_keep_awake(Arc::new(AndroidKeepAwake { app: app.clone() })) {
        log::warn!("keep-awake service was already registered");
    }
    if !register_update_installer(Arc::new(AndroidUpdateInstaller { app: app.clone() })) {
        log::warn!("update installer service was already registered");
    }

    let data_dir = app
        .internal_data_path()
        .unwrap_or_else(|| PathBuf::from("/data/data/dev.gpui.mobile.example/files"));

    let storage = Arc::new(MobileStorage { data_dir });
    let clock = Arc::new(SystemClock);

    let _platform = jni::init_platform(&app);
    let initial_system_dark = jni::query_night_mode_via_jni();
    let shared = match jni::shared_platform() {
        Some(s) => s,
        None => {
            log::error!("android_main: shared_platform() returned None");
            return;
        }
    };

    Application::with_platform(shared.into_rc())
        .with_assets(amategeko_app::AppAssets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);
            amategeko_app::set_system_theme_dark(initial_system_dark);

            let app_state = AppState::new(storage.clone(), clock.clone());
            let initial_theme = app_state.settings.theme;
            let initial_font_scale = app_state.settings.font_size_scale;
            amategeko_app::init_fonts(initial_font_scale, cx);
            amategeko_app::apply_theme(initial_theme, None, cx);

            cx.open_window(WindowOptions::default(), |window, cx| {
                amategeko_app::apply_theme(initial_theme, Some(window), cx);
                window.set_rem_size(gpui::px(amategeko_core::rem_px(initial_font_scale)));
                let shell = cx.new(|cx| {
                    cx.observe_window_appearance(window, |this: &mut ShellView, window, cx| {
                        let is_dark = matches!(
                            window.appearance(),
                            gpui::WindowAppearance::Dark | gpui::WindowAppearance::VibrantDark
                        );
                        amategeko_app::set_system_theme_dark(is_dark);
                        if this.state.settings.theme == amategeko_core::ThemeMode::System {
                            amategeko_app::apply_theme(
                                amategeko_core::ThemeMode::System,
                                Some(window),
                                cx,
                            );
                            cx.notify();
                        }
                    })
                    .detach();
                    ShellView::new(app_state, cx)
                });
                cx.new(|cx| gpui_kit::component::Root::new(shell, window, cx))
            })
            .expect("open mobile window");
        });
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn gpui_ios_register_app() {
    if !register_keep_awake(Arc::new(IosKeepAwake)) {
        log::warn!("keep-awake service was already registered");
    }

    let data_dir =
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into())).join("Documents");
    let storage = Arc::new(MobileStorage { data_dir });
    let clock = Arc::new(SystemClock);

    gpui_mobile::ios::ffi::set_app_callback(Box::new(move |cx: &mut App| {
        gpui_kit::init(cx);

        let app_state = AppState::new(storage.clone(), clock.clone());
        let initial_theme = app_state.settings.theme;
        let initial_font_scale = app_state.settings.font_size_scale;
        amategeko_app::init_fonts(initial_font_scale, cx);
        amategeko_app::apply_theme(initial_theme, None, cx);

        cx.open_window(WindowOptions::default(), |window, cx| {
            amategeko_app::apply_theme(initial_theme, Some(window), cx);
            window.set_rem_size(gpui::px(amategeko_core::rem_px(initial_font_scale)));
            let shell = cx.new(|cx| ShellView::new(app_state, cx));
            cx.new(|cx| gpui_kit::component::Root::new(shell, window, cx))
        })
        .expect("open mobile window");
    }));
}
