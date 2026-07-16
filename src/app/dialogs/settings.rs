use super::*;

mod about;
mod appearance;
mod custom;
pub(super) mod fast_menu;
mod font_page;
mod general;
mod help;
mod keybindings;
mod monitoring;
mod proxy;
mod shell;
mod sync;
mod terminal;
mod workspace;

impl AxShell {
    pub(crate) fn show_settings_dialog(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.open_settings_page(cx);
    }

    pub(crate) fn render_settings_page(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        use gpui_component::setting::Settings;

        let view = cx.entity();
        let general_page = general::settings_general_page(&view, self);
        let appearance_page = appearance::settings_appearance_page(&view, self);
        let custom_theme_page = custom::settings_custom_page(&view, self, cx);
        let terminal_page = terminal::settings_terminal_page(&view, self);
        let workspace_page = workspace::settings_workspace_page(&view, self);
        let monitoring_page = monitoring::settings_monitoring_page(&view, self);
        let connection_page = proxy::settings_connection_page(&view, self);
        let settings_id = format!("settings-{}", self.settings_page_generation);
        let initial_page = gpui_component::setting::SelectIndex {
            page_ix: self.settings_initial_page,
            group_ix: None,
        };
        shell::settings_page_shell(
            view.clone(),
            &self.focus_handle,
            Settings::new(settings_id)
                .sidebar_width(px(180.))
                .sidebar_style(div().bg(cx.theme().background).style())
                .default_selected_index(initial_page)
                .page(general_page)
                .page(appearance_page)
                .page(custom_theme_page)
                .page(terminal_page)
                .page(workspace_page)
                .page(monitoring_page)
                .page(connection_page)
                .page(sync::settings_sync_page(&view, self))
                .page(keybindings::settings_keybindings_page(
                    &view,
                    &self.config,
                    self.recording_action.as_deref(),
                    self.keybind_error.as_ref(),
                ))
                .page(help::settings_help_page())
                .page(about::settings_about_page()),
        )
    }
}
