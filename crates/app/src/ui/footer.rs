use amategeko_core::{t, Language, APP_VERSION, PROJECT_CREDIT};
use gpui_kit::assets::IconName;
use gpui_kit::base::Link;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct AppFooter;

impl AppFooter {
    pub fn render<V: 'static>(
        is_desktop: bool,
        max_width: Pixels,
        language: Language,
        cx: &mut Context<V>,
    ) -> AnyElement {
        let colors = cx.theme().colors;
        let accessibility_label = format!(
            "{} {}, GitHub",
            t("about.built_by", Language::En),
            PROJECT_CREDIT.name
        );

        let credit = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap_1()
            .child(t("about.built_by", language))
            .child(
                Link::new("footer_credit_link")
                    .href(PROJECT_CREDIT.url)
                    .open_with(|href, _, _, app_cx| {
                        #[cfg(target_arch = "wasm32")]
                        amategeko_core::platform::open_url(href);
                        #[cfg(not(target_arch = "wasm32"))]
                        app_cx.open_url(href);
                    })
                    .accessibility_label(accessibility_label)
                    .debug_selector(|| "footer-credit-link".into())
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .px_1()
                    .rounded_sm()
                    .text_color(colors.muted_foreground)
                    .hover(|link| link.text_color(colors.foreground))
                    .focus_visible(|link| link.border_1().border_color(colors.ring))
                    .child(
                        Icon::new(IconName::Github)
                            .size(px(12.0))
                            .text_color(colors.muted_foreground),
                    )
                    .child(PROJECT_CREDIT.name),
            );

        div()
            .tab_group()
            .flex()
            .flex_col()
            .when(is_desktop, |footer| {
                footer.flex_row().items_center().justify_between()
            })
            .when(!is_desktop, |footer| {
                footer.items_center().justify_center().gap_1()
            })
            .w_full()
            .max_w(max_width)
            .mx_auto()
            .mt_auto()
            .pt_4()
            .border_t_1()
            .border_color(colors.border)
            .text_xs()
            .text_color(colors.muted_foreground)
            .child(credit)
            .child(format!("v{APP_VERSION}"))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Modifiers, Render, TestAppContext};

    use super::*;

    struct FooterHarness;

    impl Render for FooterHarness {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            AppFooter::render(true, px(640.0), Language::En, cx)
        }
    }

    #[::core::prelude::v1::test]
    fn credit_link_opens_configured_github_profile() {
        let mut cx = TestAppContext::single();
        cx.update(gpui_kit::init);
        let (_, cx) = cx.add_window_view(|_, _| FooterHarness);
        cx.update(|window, cx| window.draw(cx).clear(cx));

        let bounds = cx
            .debug_bounds("footer-credit-link")
            .expect("credit link must be rendered");
        assert_eq!(cx.opened_url(), None);

        cx.simulate_click(bounds.center(), Modifiers::default());

        assert_eq!(cx.opened_url().as_deref(), Some(PROJECT_CREDIT.url));
    }
}
