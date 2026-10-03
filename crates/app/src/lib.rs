//! Shared UI for Amategeko y'Umuhanda (Desktop & Mobile).

use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::*;

pub struct SpikeView {
    count: usize,
    switch_val: bool,
}

impl SpikeView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            count: 0,
            switch_val: true,
        }
    }
}

impl Render for SpikeView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.count;
        let switch_val = self.switch_val;

        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .p_4()
            .child(div().text_xl().font_bold().child("Amategeko y'Umuhanda"))
            .child(div().text_sm().child(format!("Kanda inshuro: {count}")))
            .child(
                Button::new("spike_btn")
                    .primary()
                    .label("Kanda hano (Test Button)")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
                        cx.notify();
                    })),
            )
            .child(
                Switch::new("spike_switch")
                    .label("Urungano (Switch)")
                    .checked(switch_val)
                    .on_change(cx.listener(|this, val, _, cx| {
                        this.switch_val = *val;
                        cx.notify();
                    })),
            )
    }
}
