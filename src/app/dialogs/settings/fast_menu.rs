use super::*;

use std::{cell::RefCell, rc::Rc};

use crate::app::hover::{FastHoverTokens, fast_hover_tokens};

use gpui::{IntoElement, Pixels, uniform_list};
use gpui_component::popover::PopoverState;

const FAST_SETTINGS_MENU_VIRTUAL_THRESHOLD: usize = 40;
const FAST_SETTINGS_MENU_ROW_HEIGHT: f32 = 26.0;
const FAST_SETTINGS_MENU_DEFAULT_MAX_HEIGHT: f32 = 320.0;

#[derive(Clone)]
pub(in crate::app::dialogs) struct FastMenuItem {
    label: String,
    checked: bool,
    action: Rc<dyn Fn(&mut AxShell, &mut Window, &mut Context<AxShell>)>,
}

impl FastMenuItem {
    pub(in crate::app::dialogs) fn new(
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

#[derive(Clone, Copy)]
struct FastSettingsMenuRowStyle {
    min_width: Pixels,
    row_height: Pixels,
    radius: Pixels,
    text_fg: gpui::Hsla,
    muted_fg: gpui::Hsla,
    hover_tokens: FastHoverTokens,
}

fn fast_settings_menu_row(
    item_id: String,
    item: FastMenuItem,
    row_style: FastSettingsMenuRowStyle,
    popover: gpui::Entity<PopoverState>,
    view: gpui::Entity<AxShell>,
) -> impl IntoElement {
    let FastMenuItem {
        label,
        checked,
        action,
    } = item;
    let hover_tokens = row_style.hover_tokens;

    h_flex()
        .id(item_id)
        .h(row_style.row_height)
        .w_full()
        .min_w(row_style.min_width)
        .items_center()
        .gap_2()
        .rounded(row_style.radius)
        .px_2()
        .cursor_pointer()
        .text_color(row_style.text_fg)
        .fast_hover_with_tokens(hover_tokens)
        .when(checked, move |this| {
            this.bg(hover_tokens.active_bg)
                .text_color(hover_tokens.active_fg)
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
        .child(div().w(px(14.)).flex_none().child(if checked {
            Icon::new(IconName::Check)
                .xsmall()
                .text_color(hover_tokens.active_fg)
                .into_any_element()
        } else {
            div().text_color(row_style.muted_fg).into_any_element()
        }))
        .child(
            div()
                .flex_1()
                .min_w(px(0.))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(label),
        )
}

pub(in crate::app::dialogs) fn fast_settings_menu(
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

pub(in crate::app::dialogs) fn fast_settings_menu_lazy(
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

pub(in crate::app::dialogs) fn fast_settings_menu_disabled(
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

pub(in crate::app::dialogs) fn fast_settings_menu_lazy_disabled(
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
    let cached_items: Rc<RefCell<Option<Rc<Vec<FastMenuItem>>>>> = Rc::new(RefCell::new(None));
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
            let items = if let Some(items) = cached_items.borrow().as_ref() {
                items.clone()
            } else {
                let items = Rc::new(build_items(window, cx));
                *cached_items.borrow_mut() = Some(items.clone());
                items
            };
            let theme = cx.theme();
            let hover_tokens = fast_hover_tokens(cx);
            let row_style = FastSettingsMenuRowStyle {
                min_width,
                row_height: px(FAST_SETTINGS_MENU_ROW_HEIGHT),
                radius: theme.radius,
                text_fg: theme.popover_foreground,
                muted_fg: theme.muted_foreground,
                hover_tokens,
            };
            let menu = v_flex()
                .id(menu_id.clone())
                .w(min_width)
                .min_w(min_width)
                .gap_0p5()
                .text_color(row_style.text_fg);

            if items.len() > FAST_SETTINGS_MENU_VIRTUAL_THRESHOLD {
                let list_id = format!("{menu_id}-virtual-list");
                let row_menu_id = menu_id.clone();
                let item_count = items.len();
                let menu_height = max_height.unwrap_or(px(FAST_SETTINGS_MENU_DEFAULT_MAX_HEIGHT));
                let view = view.clone();

                return menu.child(
                    uniform_list(list_id, item_count, move |range, _window, _cx| {
                        range
                            .into_iter()
                            .filter_map(|ix| {
                                let item = items.get(ix)?.clone();
                                Some(fast_settings_menu_row(
                                    format!("{row_menu_id}-item-{ix}"),
                                    item,
                                    row_style,
                                    popover.clone(),
                                    view.clone(),
                                ))
                            })
                            .collect::<Vec<_>>()
                    })
                    .w_full()
                    .min_w(min_width)
                    .h(menu_height),
                );
            }

            let menu = if let Some(max_height) = max_height {
                menu.max_h(max_height).overflow_y_scroll()
            } else {
                menu
            };

            menu.children(items.iter().cloned().enumerate().map(|(ix, item)| {
                fast_settings_menu_row(
                    format!("{menu_id}-item-{ix}"),
                    item,
                    row_style,
                    popover.clone(),
                    view.clone(),
                )
            }))
        })
}
