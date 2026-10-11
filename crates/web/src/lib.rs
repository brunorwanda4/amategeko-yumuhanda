#![cfg(target_family = "wasm")]

use amategeko_app::{AppState, ShellView};
use amategeko_core::models::ThemeMode;
use amategeko_core::platform::{
    register_keep_awake, register_open_url, web_storage_key, KeepAwake, OpenUrl, Storage,
};
use amategeko_core::{Clock, QuestionBank};
use gpui_kit::component::Root;
use gpui_kit::*;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

thread_local! {
    static APPLICATION: RefCell<Option<ApplicationHandle>> = const { RefCell::new(None) };
}

pub struct WebStorage {
    memory: RwLock<HashMap<String, String>>,
}

impl WebStorage {
    pub fn new() -> Self {
        Self {
            memory: RwLock::new(HashMap::new()),
        }
    }
}

impl Storage for WebStorage {
    fn storage_dir(&self) -> PathBuf {
        PathBuf::from("localStorage")
    }

    fn read_file(&self, filename: &str) -> std::io::Result<String> {
        let key = web_storage_key(filename);
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            match storage.get_item(&key) {
                Ok(Some(val)) => return Ok(val),
                Ok(None) => {}
                Err(e) => {
                    log::warn!("localStorage.getItem failed for {key}: {e:?}");
                }
            }
        }
        if let Ok(mem) = self.memory.read() {
            if let Some(val) = mem.get(&key) {
                return Ok(val.clone());
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{filename} not found in storage"),
        ))
    }

    fn write_file(&self, filename: &str, content: &str) -> std::io::Result<()> {
        let key = web_storage_key(filename);
        if let Ok(mut mem) = self.memory.write() {
            mem.insert(key.clone(), content.to_string());
        }
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Err(e) = storage.set_item(&key, content) {
                log::warn!("Your progress is saved in memory because browser storage is full or restricted: {e:?}");
            }
        }
        Ok(())
    }

    fn delete_file(&self, filename: &str) -> std::io::Result<()> {
        let key = web_storage_key(filename);
        if let Ok(mut mem) = self.memory.write() {
            mem.remove(&key);
        }
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.remove_item(&key);
        }
        Ok(())
    }
}

pub struct WebClock;

impl Clock for WebClock {
    fn now_millis(&self) -> u64 {
        js_sys::Date::now() as u64
    }

    fn now_seconds(&self) -> u64 {
        self.now_millis() / 1000
    }
}

pub struct WebOpenUrl;

impl OpenUrl for WebOpenUrl {
    fn open_url(&self, url: &str) -> bool {
        if let Some(win) = web_sys::window() {
            win.open_with_url_and_target_and_features(url, "_blank", "noopener,noreferrer")
                .is_ok()
        } else {
            false
        }
    }
}

pub struct WebKeepAwake {
    requested: Arc<AtomicBool>,
    sentinel: Arc<Mutex<Option<JsValue>>>,
}

impl WebKeepAwake {
    pub fn new() -> Arc<Self> {
        let requested = Arc::new(AtomicBool::new(false));
        let sentinel: Arc<Mutex<Option<JsValue>>> = Arc::new(Mutex::new(None));

        let req_clone = requested.clone();
        let sent_clone = sentinel.clone();

        let visibility_closure = Closure::wrap(Box::new(move || {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if doc.visibility_state() == web_sys::VisibilityState::Visible {
                    if req_clone.load(Ordering::SeqCst) {
                        Self::request_lock(sent_clone.clone());
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback(
                "visibilitychange",
                visibility_closure.as_ref().unchecked_ref(),
            );
        }
        visibility_closure.forget();

        Arc::new(Self {
            requested,
            sentinel,
        })
    }

    fn request_lock(sentinel_arc: Arc<Mutex<Option<JsValue>>>) {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            if let Ok(wake_lock) = js_sys::Reflect::get(&nav, &JsValue::from_str("wakeLock")) {
                if !wake_lock.is_undefined() {
                    if let Ok(request_fn) =
                        js_sys::Reflect::get(&wake_lock, &JsValue::from_str("request"))
                    {
                        if let Ok(request_fn) = request_fn.dyn_into::<js_sys::Function>() {
                            if let Ok(promise_val) =
                                request_fn.call1(&wake_lock, &JsValue::from_str("screen"))
                            {
                                if let Ok(promise) = promise_val.dyn_into::<js_sys::Promise>() {
                                    wasm_bindgen_futures::spawn_local(async move {
                                        if let Ok(sentinel) =
                                            wasm_bindgen_futures::JsFuture::from(promise).await
                                        {
                                            if let Ok(mut lock) = sentinel_arc.lock() {
                                                *lock = Some(sentinel);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn release_lock(&self) {
        if let Ok(mut lock) = self.sentinel.lock() {
            if let Some(sentinel) = lock.take() {
                if let Ok(release_fn) =
                    js_sys::Reflect::get(&sentinel, &JsValue::from_str("release"))
                {
                    if let Ok(release_fn) = release_fn.dyn_into::<js_sys::Function>() {
                        let _ = release_fn.call0(&sentinel);
                    }
                }
            }
        }
    }
}

unsafe impl Send for WebKeepAwake {}
unsafe impl Sync for WebKeepAwake {}

impl KeepAwake for WebKeepAwake {
    fn set(&self, on: bool) {
        self.requested.store(on, Ordering::SeqCst);
        if on {
            Self::request_lock(self.sentinel.clone());
        } else {
            self.release_lock();
        }
    }
}

#[wasm_bindgen]
pub fn run(dark: Option<bool>) -> Result<(), JsValue> {
    let already_running = APPLICATION.with(|app| app.borrow().is_some());
    if already_running {
        return Ok(());
    }

    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);

    let _ = register_open_url(Arc::new(WebOpenUrl));
    let _ = register_keep_awake(WebKeepAwake::new());

    let storage = Arc::new(WebStorage::new());
    let clock = Arc::new(WebClock);

    let raw_questions = include_str!(concat!(env!("OUT_DIR"), "/web_questions.json"));
    let bank = match QuestionBank::from_rw_json_strings(raw_questions, "{}") {
        Ok(b) => Arc::new(b),
        Err(e) => {
            log::error!("Failed to parse bundled questions: {e}");
            Arc::new(QuestionBank::from_rw_json_strings("[]", "{}").unwrap())
        }
    };

    gpui_kit::platform::web_init();
    let app = gpui_kit::platform::single_threaded_web();
    let app = app.with_assets(amategeko_app::AppAssets);

    let launch = move |cx: &mut gpui::App| {
        gpui_kit::init(cx);

        let mut app_state = AppState::with_bank(bank, storage, clock);
        if let Some(is_dark) = dark {
            app_state.settings.theme = if is_dark {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
        }

        let initial_theme = app_state.settings.theme;
        amategeko_app::init_fonts(app_state.settings.font_size_scale, cx);

        // Load the fallback web fonts for cosmic-text and monospaced UI.
        // The web platform resolves GPUI's `.SystemUIFont` alias to IBM Plex
        // Sans and ships no system fonts in WASM. Text measured or rendered
        // before/with the system fallback needs IBM Plex Sans or cosmic-text panics.
        let system_font =
            Cow::Borrowed(include_bytes!("../fonts/IBMPlexSans-Regular.ttf").as_slice());
        let mono_font =
            Cow::Borrowed(include_bytes!("../fonts/JetBrainsMono-Regular.ttf").as_slice());
        let emoji_font = Cow::Borrowed(include_bytes!("../fonts/NotoEmoji-Regular.ttf").as_slice());
        let cjk_font =
            Cow::Borrowed(include_bytes!("../fonts/NotoSansSC-Regular-subset.ttf").as_slice());
        let _ = cx
            .text_system()
            .add_fonts(vec![system_font, mono_font, emoji_font, cjk_font]);

        amategeko_app::apply_theme(initial_theme, None, cx);

        cx.open_window(gpui::WindowOptions::default(), move |window, cx| {
            let view = cx.new(|cx| ShellView::new(app_state, cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
        cx.activate(true);
    };

    APPLICATION.with(|application| {
        *application.borrow_mut() = Some(app.run_embedded(launch));
    });

    Ok(())
}
