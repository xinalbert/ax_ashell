use gpui::{Hsla, InteractiveElement, Styled as _};
use gpui_component::ActiveTheme as _;

#[derive(Clone, Copy)]
pub(crate) struct FastHoverTokens {
    pub(crate) hover_bg: Hsla,
    pub(crate) hover_fg: Option<Hsla>,
    pub(crate) active_bg: Hsla,
    pub(crate) active_fg: Hsla,
}

#[derive(Clone, Copy)]
pub(crate) struct FastHoverOptions {
    hover_bg: Option<Hsla>,
    hover_fg: Option<Hsla>,
    active_bg: Option<Hsla>,
    active_fg: Option<Hsla>,
    apply_hover_fg: bool,
}

impl Default for FastHoverOptions {
    fn default() -> Self {
        Self {
            hover_bg: None,
            hover_fg: None,
            active_bg: None,
            active_fg: None,
            apply_hover_fg: true,
        }
    }
}

impl FastHoverOptions {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn hover_bg(mut self, color: Hsla) -> Self {
        self.hover_bg = Some(color);
        self
    }

    pub(crate) fn hover_fg(mut self, color: Hsla) -> Self {
        self.hover_fg = Some(color);
        self.apply_hover_fg = true;
        self
    }

    pub(crate) fn active_bg(mut self, color: Hsla) -> Self {
        self.active_bg = Some(color);
        self
    }

    pub(crate) fn active_fg(mut self, color: Hsla) -> Self {
        self.active_fg = Some(color);
        self
    }

    pub(crate) fn keep_text_color(mut self) -> Self {
        self.apply_hover_fg = false;
        self
    }

    pub(crate) fn resolve(self, cx: &gpui::App) -> FastHoverTokens {
        let theme = cx.theme();
        let default_fg = theme.sidebar_accent_foreground;

        FastHoverTokens {
            hover_bg: self
                .hover_bg
                .unwrap_or_else(|| theme.sidebar_accent.opacity(0.8)),
            hover_fg: self
                .apply_hover_fg
                .then_some(self.hover_fg.unwrap_or(default_fg)),
            active_bg: self.active_bg.unwrap_or(theme.sidebar_accent),
            active_fg: self.active_fg.unwrap_or(default_fg),
        }
    }
}

pub(crate) fn fast_hover_tokens(cx: &gpui::App) -> FastHoverTokens {
    FastHoverOptions::default().resolve(cx)
}

pub(crate) fn fast_hover_tokens_with_options(
    cx: &gpui::App,
    options: FastHoverOptions,
) -> FastHoverTokens {
    options.resolve(cx)
}

pub(crate) fn list_fast_hover_options(cx: &gpui::App) -> FastHoverOptions {
    let theme = cx.theme();

    FastHoverOptions::new()
        .hover_bg(theme.list_hover)
        .hover_fg(theme.foreground)
        .active_bg(theme.secondary)
        .active_fg(theme.foreground)
        .keep_text_color()
}

pub(crate) trait FastHoverExt: InteractiveElement + Sized {
    fn fast_hover(self, cx: &gpui::App) -> Self {
        self.fast_hover_options(cx, FastHoverOptions::default())
    }

    fn fast_hover_options(self, cx: &gpui::App, options: FastHoverOptions) -> Self {
        self.fast_hover_with_tokens(fast_hover_tokens_with_options(cx, options))
    }

    fn fast_hover_with_tokens(self, tokens: FastHoverTokens) -> Self {
        self.hover(move |style| {
            let style = style.bg(tokens.hover_bg);
            if let Some(hover_fg) = tokens.hover_fg {
                style.text_color(hover_fg)
            } else {
                style
            }
        })
    }
}

impl<T> FastHoverExt for T where T: InteractiveElement + Sized {}
