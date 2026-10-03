use amategeko_app::SpikeView;
use gpui_kit::component::Root;
use gpui_kit::*;

fn main() {
    gpui_kit::application()
        .with_assets(assets::Assets)
        .run(|cx: &mut App| {
            gpui_kit::init(cx);
            cx.open_window(WindowOptions::default(), |window, cx| {
                let content = cx.new(|cx| SpikeView::new(window, cx));
                cx.new(|cx| Root::new(content, window, cx))
            })
            .expect("Failed to open desktop window");
        });
}
