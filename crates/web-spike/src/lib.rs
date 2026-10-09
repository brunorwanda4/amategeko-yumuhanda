use std::borrow::Cow;
use std::cell::RefCell;

use gpui_kit::assets::Assets;
use gpui_kit::component::button::Button;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::component::{h_flex, v_flex, ActiveTheme, Root, StyledExt};
use gpui_kit::{prelude::*, *};
use wasm_bindgen::prelude::*;

thread_local! {
    static APPLICATION: RefCell<Option<ApplicationHandle>> = const { RefCell::new(None) };
}

struct WebSpikeView {
    input_state: Entity<InputState>,
    switch_checked: bool,
    click_count: usize,
}

impl WebSpikeView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state =
            cx.new(|cx| InputState::new(window, cx).placeholder("Type here: e.g. it's or ’..."));
        Self {
            input_state,
            switch_checked: false,
            click_count: 0,
        }
    }
}

impl Render for WebSpikeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_mode_str = match cx.theme().mode {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
        };
        let theme_text = format!("Current Theme: {}", theme_mode_str);
        let button_label = format!("Clicked {} times", self.click_count);
        let switch_val = self.switch_checked;

        let rows: Vec<_> = (1..=200usize)
            .map(|i| {
                div()
                    .id(("row", i))
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .hover(|style| style.bg(cx.theme().muted))
                    .child(format!(
                        "Row {}: GPUI Kit web virtual scrolling spike item",
                        i
                    ))
            })
            .collect();

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                v_flex()
                    .p_4()
                    .gap_3()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_lg()
                                    .font_bold()
                                    .child("GPUI Kit WebAssembly Spike"),
                            )
                            .child(
                                div()
                                    .id("theme-indicator")
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(theme_text),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_4()
                            .items_center()
                            .child(Button::new("spike-btn").label(button_label).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.click_count += 1;
                                    cx.notify();
                                }),
                            ))
                            .child(Switch::new("spike-switch").checked(switch_val).on_change(
                                cx.listener(|this, checked, _, cx| {
                                    this.switch_checked = *checked;
                                    cx.notify();
                                }),
                            ))
                            .child(div().text_sm().child(if switch_val {
                                "Switch: ON"
                            } else {
                                "Switch: OFF"
                            })),
                    )
                    .child(
                        div()
                            .w_full()
                            .max_w(px(400.))
                            .child(Input::new(&self.input_state)),
                    ),
            )
            .child(
                v_flex()
                    .id("scroll-list")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}

fn apply_theme(mode: ThemeMode, cx: &mut App) {
    Theme::change(mode, None, cx);
    let theme = cx.global_mut::<Theme>();
    theme.font_family = "Inter Variable".into();
    theme.mono_font_family = "JetBrains Mono".into();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn set_theme(dark: bool) {
    let mode = if dark {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };
    APPLICATION.with(|application| {
        if let Some(handle) = application.borrow().as_ref() {
            handle.update(|cx| {
                apply_theme(mode, cx);
                cx.refresh_windows();
            });
        }
    });
}

#[wasm_bindgen]
pub fn run(dark: Option<bool>) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");

    #[cfg(target_family = "wasm")]
    gpui_kit::platform::web_init();
    #[cfg(not(target_family = "wasm"))]
    let app = gpui_kit::application();
    #[cfg(target_family = "wasm")]
    let app = gpui_kit::platform::single_threaded_web();

    let app = app.with_assets(Assets::new("https://gpui-kit.com/gallery/"));
    let launch = move |cx: &mut App| {
        gpui_kit::init(cx);

        let ui_font = Cow::Borrowed(include_bytes!("../fonts/Inter-Regular.ttf").as_slice());
        let cjk_font =
            Cow::Borrowed(include_bytes!("../fonts/NotoSansSC-Regular-subset.ttf").as_slice());
        let emoji_font = Cow::Borrowed(include_bytes!("../fonts/NotoEmoji-Regular.ttf").as_slice());
        let jetbrains_mono =
            Cow::Borrowed(include_bytes!("../fonts/JetBrainsMono-Regular.ttf").as_slice());
        let system_font =
            Cow::Borrowed(include_bytes!("../fonts/IBMPlexSans-Regular.ttf").as_slice());
        cx.text_system()
            .add_fonts(vec![
                ui_font,
                cjk_font,
                emoji_font,
                jetbrains_mono,
                system_font,
            ])
            .expect("Failed to load fonts");

        apply_theme(
            match dark {
                Some(true) => ThemeMode::Dark,
                _ => ThemeMode::Light,
            },
            cx,
        );

        cx.open_window(WindowOptions::default(), move |window, cx| {
            let view = cx.new(|cx| WebSpikeView::new(window, cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
        cx.activate(true);
    };

    #[cfg(target_family = "wasm")]
    APPLICATION.with(|application| {
        *application.borrow_mut() = Some(app.run_embedded(launch));
    });
    #[cfg(not(target_family = "wasm"))]
    app.run(launch);

    Ok(())
}
