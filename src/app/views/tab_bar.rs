use super::*;

#[derive(Clone)]
struct WorkspaceTabDrag {
    group_id: String,
}

impl Render for WorkspaceTabDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_2()
            .py_1()
            .rounded_sm()
            .bg(gpui::black().opacity(0.75))
            .text_size(rems(0.833))
            .text_color(gpui::white())
            .child("Move workspace tab")
    }
}

impl AxShell {
    pub(super) fn render_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let workspace_tabs = self.workspace_tabs();
        let selected = self.active_workspace_tab_index(&workspace_tabs);
        let is_integrated = self.active_title_bar_style == crate::config::TitleBarStyle::Integrated;
        let is_linux_native_drag_bar = cfg!(target_os = "linux")
            && self.active_title_bar_style == crate::config::TitleBarStyle::Native;
        let color_inactive_tabs = self.config.color_inactive_tabs();

        h_flex()
            .flex_1()
            .min_w(px(0.))
            .h_full()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .h_full()
                    .overflow_x_hidden()
                    .child({
                        TabBar::new("ax_shell-tab-bar")
                            .track_scroll(&self.tabs_scroll_handle)
                            .selected_index(selected)
                            .children(workspace_tabs.iter().enumerate().map(|(ix, tab)| {
                                match tab.page {
                                    WorkspacePage::Settings => {
                                        let selected = self.workspace_tab_selected(tab);
                                        let indicator_color = if selected || color_inactive_tabs {
                                            cx.theme().primary
                                        } else {
                                            cx.theme().muted_foreground.opacity(0.5)
                                        };

                                        Tab::new()
                                            .min_w(px(120.))
                                            .max_w(px(WORKSPACE_TAB_MAX_WIDTH))
                                            .prefix(div().w(px(5.)).h(px(32.)).bg(indicator_color))
                                            .child(
                                                div()
                                                    .min_w(px(0.))
                                                    .overflow_hidden()
                                                    .text_ellipsis()
                                                    .whitespace_nowrap()
                                                    .when(selected, |this| {
                                                        this.font_weight(FontWeight::BOLD)
                                                            .text_color(cx.theme().primary)
                                                            .text_base()
                                                    })
                                                    .child(t!("settings").to_string()),
                                            )
                                            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                                window.prevent_default();
                                                cx.stop_propagation();
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                cx.stop_propagation();
                                                this.open_settings_page(cx);
                                            }))
                                            .suffix(
                                                Button::new("settings-tab-close")
                                                    .ghost()
                                                    .xsmall()
                                                    .icon(IconName::Close)
                                                    .mr(px(5.))
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        |_, window, cx| {
                                                            window.prevent_default();
                                                            cx.stop_propagation();
                                                        },
                                                    )
                                                    .on_click(cx.listener(
                                                        |this, _, window, cx| {
                                                            this.request_close_settings_page(
                                                                window, cx,
                                                            );
                                                        },
                                                    )),
                                            )
                                    }
                                    page => {
                                        let group_index = tab.group_index.unwrap_or(ix);
                                        let group_id = tab.group_id.clone().unwrap_or_default();
                                        let group = self
                                            .tab_groups
                                            .iter()
                                            .find(|group| group.id == group_id);
                                        let pane_ids = group
                                            .map(|group| {
                                                group
                                                    .pane_root
                                                    .tab_ids()
                                                    .iter()
                                                    .map(|id| id.to_string())
                                                    .collect::<Vec<_>>()
                                            })
                                            .unwrap_or_default();
                                        let close_id = if self.active_group.as_deref()
                                            == Some(group_id.as_str())
                                        {
                                            self.active_tab.clone().unwrap_or_else(|| {
                                                pane_ids.first().cloned().unwrap_or_default()
                                            })
                                        } else {
                                            pane_ids.first().cloned().unwrap_or_default()
                                        };
                                        let selected = self.workspace_tab_selected(tab);
                                        let status_color = pane_ids
                                            .first()
                                            .and_then(|id| {
                                                self.tabs.iter().find(|tab| tab.id == *id)
                                            })
                                            .map(|tab| {
                                                if tab.connected {
                                                    cx.theme().success
                                                } else {
                                                    cx.theme().danger
                                                }
                                            })
                                            .unwrap_or(cx.theme().success);
                                        let indicator_color = if selected || color_inactive_tabs {
                                            status_color
                                        } else {
                                            cx.theme().muted_foreground.opacity(0.5)
                                        };
                                        let label = match page {
                                            WorkspacePage::Terminal => {
                                                let title = group
                                                    .map(|group| group.title.clone())
                                                    .unwrap_or_else(|| "Unknown".to_string());
                                                let instance_number = group
                                                    .map(|group| group.instance_number)
                                                    .unwrap_or(1);
                                                workspace_group_tab_label(
                                                    group_index,
                                                    &title,
                                                    instance_number,
                                                    WorkspacePage::Terminal,
                                                    pane_ids.len(),
                                                )
                                            }
                                            WorkspacePage::Sftp => {
                                                let title = group
                                                    .map(|group| group.title.clone())
                                                    .unwrap_or_else(|| "Unknown".to_string());
                                                let instance_number = group
                                                    .map(|group| group.instance_number)
                                                    .unwrap_or(1);
                                                workspace_group_tab_label(
                                                    group_index,
                                                    &title,
                                                    instance_number,
                                                    WorkspacePage::Sftp,
                                                    0,
                                                )
                                            }
                                            WorkspacePage::Settings => unreachable!(),
                                        };
                                        let target_page = page;
                                        let target_group_id = group_id.clone();
                                        let drop_target_group_id = group_id.clone();
                                        let dragged_group_id = group_id.clone();
                                        let close_sftp_group_id = group_id.clone();
                                        let move_terminal_group_id = group_id.clone();
                                        let is_terminal_tab =
                                            target_page == WorkspacePage::Terminal;
                                        let is_sftp_tab = target_page == WorkspacePage::Sftp;

                                        Tab::new()
                                            .min_w(px(if is_terminal_tab { 96. } else { 88. }))
                                            .max_w(px(WORKSPACE_TAB_MAX_WIDTH))
                                            .prefix(div().w(px(5.)).h(px(32.)).bg(indicator_color))
                                            .child(
                                                div()
                                                    .min_w(px(0.))
                                                    .overflow_hidden()
                                                    .text_ellipsis()
                                                    .whitespace_nowrap()
                                                    .when(selected, |this| {
                                                        this.font_weight(FontWeight::BOLD)
                                                            .text_color(cx.theme().primary)
                                                            .text_base()
                                                    })
                                                    .child(label),
                                            )
                                            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                                window.prevent_default();
                                                cx.stop_propagation();
                                            })
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                cx.stop_propagation();
                                                this.activate_group_page(
                                                    target_group_id.clone(),
                                                    target_page,
                                                    window,
                                                    cx,
                                                );
                                            }))
                                            .cursor_move()
                                            .on_drag(
                                                WorkspaceTabDrag {
                                                    group_id: dragged_group_id,
                                                },
                                                |drag, _, _, cx| cx.new(|_| drag.clone()),
                                            )
                                            .drag_over::<WorkspaceTabDrag>({
                                                let drop_target_group_id =
                                                    drop_target_group_id.clone();
                                                move |this, drag, _, cx| {
                                                    if drag.group_id == drop_target_group_id {
                                                        this
                                                    } else {
                                                        this.border_l_2()
                                                            .border_color(cx.theme().drag_border)
                                                    }
                                                }
                                            })
                                            .on_drop(cx.listener(
                                                move |this, drag: &WorkspaceTabDrag, _, cx| {
                                                    this.move_workspace_group_before(
                                                        &drag.group_id,
                                                        Some(&drop_target_group_id),
                                                        cx,
                                                    );
                                                },
                                            ))
                                            .when(is_terminal_tab, |this| {
                                                this.suffix(
                                                    h_flex()
                                                        .items_center()
                                                        .child(
                                                            Button::new(("tab-move-window", ix))
                                                                .ghost()
                                                                .xsmall()
                                                                .icon(IconName::ExternalLink)
                                                                .tooltip(
                                                                    t!(
                                                                        "move_workspace_to_new_window"
                                                                    )
                                                                    .to_string(),
                                                                )
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    |_, window, cx| {
                                                                        window.prevent_default();
                                                                        cx.stop_propagation();
                                                                    },
                                                                )
                                                                .on_click(cx.listener(
                                                                    move |this, _, window, cx| {
                                                                        window.prevent_default();
                                                                        cx.stop_propagation();
                                                                        this.move_workspace_group_to_new_window(
                                                                            move_terminal_group_id
                                                                                .clone(),
                                                                            window,
                                                                            cx,
                                                                        );
                                                                    },
                                                                )),
                                                        )
                                                        .child(
                                                            Button::new(("tab-close", ix))
                                                                .ghost()
                                                                .xsmall()
                                                                .icon(IconName::Close)
                                                                .mr(px(5.))
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    |_, window, cx| {
                                                                        window.prevent_default();
                                                                        cx.stop_propagation();
                                                                    },
                                                                )
                                                                .on_click(cx.listener(
                                                                    move |this, _, window, cx| {
                                                                        window.prevent_default();
                                                                        cx.stop_propagation();
                                                                        if !close_id.is_empty() {
                                                                            this.close_tab(
                                                                                close_id.clone(),
                                                                                cx,
                                                                            );
                                                                        }
                                                                    },
                                                                )),
                                                        ),
                                                )
                                            })
                                            .when(is_sftp_tab, |this| {
                                                this.suffix(
                                                    h_flex()
                                                        .items_center()
                                                        .child(
                                                            Button::new(("sftp-tab-close", ix))
                                                                .ghost()
                                                                .xsmall()
                                                                .icon(IconName::Close)
                                                                .mr(px(5.))
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    |_, window, cx| {
                                                                        window.prevent_default();
                                                                        cx.stop_propagation();
                                                                    },
                                                                )
                                                                .on_click(cx.listener(
                                                                    move |this, _, window, cx| {
                                                                        window.prevent_default();
                                                                        cx.stop_propagation();
                                                                        this.close_sftp_page(
                                                                            close_sftp_group_id
                                                                                .clone(),
                                                                            window,
                                                                            cx,
                                                                        );
                                                                    },
                                                                )),
                                                        ),
                                                )
                                            })
                                    }
                                }
                            }))
                            .suffix(h_flex().h_full().flex_none().when(
                                is_integrated || is_linux_native_drag_bar,
                                |this| {
                                    this.child(
                                        div()
                                            .id("tab-bar-drag-spacer")
                                            .w(px(56.))
                                            .h_full()
                                            .flex_shrink_0()
                                            .when(is_integrated, |this| {
                                                this.window_control_area(
                                                    gpui::WindowControlArea::Drag,
                                                )
                                            })
                                            .when(is_linux_native_drag_bar, |this| {
                                                Self::bind_titlebar_drag(this, cx)
                                            }),
                                    )
                                },
                            ))
                            .last_empty_space(
                                div()
                                    .id("tab-bar-drop-at-end")
                                    .w(px(16.))
                                    .h_full()
                                    .flex_none()
                                    .drag_over::<WorkspaceTabDrag>(|this, _, _, cx| {
                                        this.bg(cx.theme().drop_target)
                                    })
                                    .on_drop(cx.listener(
                                        |this, drag: &WorkspaceTabDrag, _, cx| {
                                            this.move_workspace_group_before(
                                                &drag.group_id,
                                                None,
                                                cx,
                                            );
                                        },
                                    )),
                            )
                            .w_full()
                            .h_full()
                    }),
            )
            .child(
                h_flex()
                    .on_mouse_down(MouseButton::Left, |_, window, cx| {
                        window.prevent_default();
                        cx.stop_propagation();
                    })
                    .flex_none()
                    .items_center()
                    .gap_1()
                    .pr(px(6.))
                    .child(
                        Button::new("open-selector")
                            .secondary()
                            .small()
                            .rounded(px(999.))
                            .icon(IconName::Plus)
                            .tooltip(t!("settings_open_session").to_string())
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_selector_dialog(window, cx)
                            })),
                    )
                    .child(
                        Button::new("split-horizontal")
                            .secondary()
                            .small()
                            .rounded(px(999.))
                            .icon(IconName::PanelBottom)
                            .tooltip(t!("settings_split_pane_down").to_string())
                            .disabled(self.workspace_page != WorkspacePage::Terminal)
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.prevent_default();
                                cx.stop_propagation();
                                this.split_current_pane("down", cx);
                            })),
                    )
                    .child(
                        Button::new("split-vertical")
                            .secondary()
                            .small()
                            .rounded(px(999.))
                            .icon(IconName::PanelRight)
                            .tooltip(t!("settings_split_pane_right").to_string())
                            .disabled(self.workspace_page != WorkspacePage::Terminal)
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.prevent_default();
                                cx.stop_propagation();
                                this.split_current_pane("right", cx);
                            })),
                    )
                    .when(self.workspace_page == WorkspacePage::Terminal, |this| {
                        this.child(self.render_search_button(cx))
                    }),
            )
    }
}
