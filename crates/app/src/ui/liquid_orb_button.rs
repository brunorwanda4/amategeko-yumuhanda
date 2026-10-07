use gpui::prelude::*;
use gpui::{
    div, hsla, linear_color_stop, linear_gradient, point, px, BoxShadow, Context, IntoElement,
    SharedString, Window,
};
use gpui_kit::assets::IconName;
use gpui_kit::component::{ActiveTheme, Icon};

pub enum LiquidShape {
    /// Circle with an icon only
    Orb { diameter: f32 },
    /// Pill with an icon and a label, e.g. "+ Sign up"
    Pill { width: f32, height: f32 },
}

pub struct LiquidMetalButton;

impl LiquidMetalButton {
    pub fn new<V: 'static>(
        id: &'static str,
        shape: LiquidShape,
        icon: IconName,
        label: Option<SharedString>,
        disabled: bool,
        cx: &mut Context<V>,
        on_click: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static,
    ) -> impl IntoElement {
        let colors = cx.theme().colors;

        let (w, h) = match shape {
            LiquidShape::Orb { diameter } => (diameter.max(44.0), diameter.max(44.0)),
            LiquidShape::Pill { width, height } => (width.max(96.0), height.max(40.0)),
        };
        let rim = 1.5_f32;
        let iw = w - rim * 2.0;
        let ih = h - rim * 2.0;
        let group = SharedString::from(format!("liquid-{id}"));

        let silver = hsla(0.0, 0.0, 0.88, 0.95);
        let steel = hsla(0.60, 0.10, 0.22, 0.7);
        let dark = hsla(0.0, 0.0, 0.04, 1.0);
        let warm = hsla(0.08, 0.95, 0.68, 1.0);
        let cool = hsla(0.60, 0.85, 0.72, 1.0);

        let mask_idle_h = ih - 1.5;
        let mask_hover_h = ih - 7.0;

        // Metallic rim: dim when idle, bright with warm bottom on hover
        let rim_layer = div()
            .absolute()
            .inset_0()
            .rounded_full()
            .bg(linear_gradient(
                180.,
                linear_color_stop(silver.opacity(0.5), 0.0),
                linear_color_stop(steel.opacity(0.5), 1.0),
            ))
            .when(!disabled, |el| {
                el.group_hover(group.clone(), move |s| {
                    s.bg(linear_gradient(
                        180.,
                        linear_color_stop(silver, 0.0),
                        linear_color_stop(warm.opacity(0.9), 1.0),
                    ))
                })
            });

        // Dark base face
        let face_bg = div()
            .absolute()
            .top(px(rim))
            .left(px(rim))
            .w(px(iw))
            .h(px(ih))
            .rounded_full()
            .bg(dark);

        // Light layer (blue -> orange), faint when idle, full on hover
        let light = div()
            .absolute()
            .top(px(rim))
            .left(px(rim))
            .w(px(iw))
            .h(px(ih))
            .rounded_full()
            .bg(linear_gradient(
                90.,
                linear_color_stop(cool, 0.0),
                linear_color_stop(warm, 1.0),
            ))
            .opacity(0.25)
            .when(!disabled, |el| {
                el.group_hover(group.clone(), |s| s.opacity(1.0))
            });

        // Dark mask: shorter on hover, which reveals a crescent of light at the bottom
        let mask = div()
            .absolute()
            .top(px(rim))
            .left(px(rim))
            .w(px(iw))
            .h(px(mask_idle_h))
            .rounded_full()
            .bg(dark)
            .when(!disabled, |el| {
                el.group_hover(group.clone(), move |s| s.h(px(mask_hover_h)))
            });

        // Icon + label
        let content = div()
            .absolute()
            .top(px(rim))
            .left(px(rim))
            .w(px(iw))
            .h(px(ih))
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .text_color(if disabled {
                colors.muted_foreground
            } else {
                hsla(0., 0., 1., 1.)
            })
            .child(Icon::new(icon).size(px(if label.is_some() { 14.0 } else { 20.0 })))
            .when_some(label, |el, text| {
                el.child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child(text),
                )
            });

        // Root: no overflow_hidden so the shadow/halo isn't clipped
        div()
            .id(id)
            .group(group.clone())
            .relative()
            .flex_none()
            .w(px(w))
            .h(px(h))
            .rounded_full()
            .shadow(vec![BoxShadow {
                inset: false,
                color: hsla(0., 0., 0., 0.6),
                offset: point(px(0.), px(6.)),
                blur_radius: px(18.),
                spread_radius: px(0.),
            }])
            .child(rim_layer)
            .child(face_bg)
            .child(light)
            .child(mask)
            .child(content)
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .group_hover(group, move |s| {
                        s.shadow(vec![BoxShadow {
                            inset: false,
                            color: warm.opacity(0.4),
                            offset: point(px(3.), px(4.)),
                            blur_radius: px(28.),
                            spread_radius: px(1.),
                        }])
                    })
                    .active(|el| el.opacity(0.85))
                    .focus_visible(|el| el.border_2().border_color(colors.ring))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_click(this, window, cx);
                    }))
            })
            .when(disabled, |el| el.opacity(0.42))
    }
}

pub struct LiquidOrbButton;

impl LiquidOrbButton {
    pub fn new<V: 'static>(
        id: &'static str,
        icon: IconName,
        size: f32,
        disabled: bool,
        cx: &mut Context<V>,
        on_click: impl Fn(&mut V, &mut Window, &mut Context<V>) + 'static,
    ) -> impl IntoElement {
        LiquidMetalButton::new(
            id,
            LiquidShape::Orb { diameter: size },
            icon,
            None,
            disabled,
            cx,
            on_click,
        )
    }
}
