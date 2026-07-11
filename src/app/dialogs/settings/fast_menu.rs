use super::*;

use std::rc::Rc;

use gpui::{IntoElement, Pixels};

#[derive(Clone)]
pub(super) struct FastMenuItem {
    label: String,
    checked: bool,
    action: Rc<dyn Fn(&mut AxShell, &mut Window, &mut Context<AxShell>)>,
}

impl FastMenuItem {
    pub(super) fn new(
        label: impl Into<String>,
        checked: bool,
        action: impl Fn(&mut AxShell, &mut Window, &mut Context<AxShell>) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            checked,
            action: Rc::new(action),
        }
    }
}

pub(super) fn fast_settings_menu(
    id: impl Into<String>,
    label: impl Into<String>,
    icon: Option<IconName>,
    min_width: Pixels,
    max_height: Option<Pixels>,
    items: Vec<FastMenuItem>,
    view: gpui::Entity<AxShell>,
) -> impl IntoElement {
    fast_settings_menu_disabled(id, label, icon, min_width, max_height, false, items, view)
}

pub(super) fn fast_settings_menu_lazy(
    id: impl Into<String>,
    label: impl Into<String>,
    icon: Option<IconName>,
    min_width: Pixels,
    max_height: Option<Pixels>,
    build_items: impl Fn(&mut Window, &mut gpui::App) -> Vec<FastMenuItem> + 'static,
    view: gpui::Entity<AxShell>,
) -> impl IntoElement {
    fast_settings_menu_lazy_disabled(
        id,
        label,
        icon,
        min_width,
        max_height,
        false,
        build_items,
        view,
    )
}

pub(super) fn fast_settings_menu_disabled(
    id: impl Into<String>,
    label: impl Into<String>,
    icon: Option<IconName>,
    min_width: Pixels,
    max_height: Option<Pixels>,
    disabled: bool,
    items: Vec<FastMenuItem>,
    view: gpui::Entity<AxShell>,
) -> impl IntoElement {
    fast_settings_menu_lazy_disabled(
        id,
        label,
        icon,
        min_width,
        max_height,
        disabled,
        move |_, _| items.clone(),
        view,
    )
}

pub(super) fn fast_settings_menu_lazy_disabled(
    id: impl Into<String>,
    label: impl Into<String>,
    icon: Option<IconName>,
    min_width: Pixels,
    max_height: Option<Pixels>,
    disabled: bool,
    build_items: impl Fn(&mut Window, &mut gpui::App) -> Vec<FastMenuItem> + 'static,
    view: gpui::Entity<AxShell>,
) -> impl IntoElement {
    let id = id.into();
    let popover_id = format!("{id}-popover");
    let menu_id = format!("{id}-menu");
    let build_items = Rc::new(build_items);
    let trigger = Button::new(id.clone())
        .small()
        .label(label.into())
        .disabled(disabled);
    let trigger = if let Some(icon) = icon {
        trigger.icon(icon)
    } else {
        trigger
    };

    Popover::new(popover_id)
        .anchor(Anchor::BottomRight)
        .p_1()
        .text_sm()
        .trigger(trigger)
        .content(move |_state, window, cx| {
            let popover = cx.entity();
            let items = build_items(window, cx);
            let theme = cx.theme();
            let hover_bg = theme.accent;
            let hover_fg = theme.accent_foreground;
            let checked_bg = theme.accent;
            let checked_fg = theme.accent_foreground;
            let text_fg = theme.popover_foreground;
            let muted_fg = theme.muted_foreground;
            let mut menu = v_flex()
                .id(menu_id.clone())
                .w(min_width)
                .min_w(min_width)
                .gap_0p5()
                .text_color(text_fg);
            if let Some(max_height) = max_height {
                menu = menu.max_h(max_height).overflow_y_scroll();
            }

            menu.children(items.iter().enumerate().map(|(ix, item)| {
                let action = item.action.clone();
                let view = view.clone();
                let popover = popover.clone();
                let item_id = format!("{menu_id}-item-{ix}");
                h_flex()
                    .id(item_id)
                    .h(px(26.))
                    .w_full()
                    .min_w(min_width)
                    .items_center()
                    .gap_2()
                    .rounded(cx.theme().radius)
                    .px_2()
                    .cursor_pointer()
                    .text_color(text_fg)
                    .hover(move |this| this.bg(hover_bg).text_color(hover_fg))
                    .when(item.checked, move |this| {
                        this.bg(checked_bg).text_color(checked_fg)
                    })
                    .on_mouse_down(MouseButton::Left, |_, window, cx| {
                        window.prevent_default();
                        cx.stop_propagation();
                    })
                    .on_click(move |_, window, cx| {
                        popover.update(cx, |state, cx| state.dismiss(window, cx));
                        view.update(cx, |this, cx| {
                            (action)(this, window, cx);
                        });
                        window.prevent_default();
                        cx.stop_propagation();
                    })
                    .child(div().w(px(14.)).flex_none().child(if item.checked {
                        Icon::new(IconName::Check)
                            .xsmall()
                            .text_color(checked_fg)
                            .into_any_element()
                    } else {
                        div().text_color(muted_fg).into_any_element()
                    }))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.))
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(item.label.clone()),
                    )
            }))
        })
}
