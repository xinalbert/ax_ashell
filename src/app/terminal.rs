use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gpui::{Modifiers, Pixels, Point, Size, point, px, size};
use gpui_component::scroll::ScrollbarHandle;

use crate::terminal;

pub(crate) struct TerminalScrollbarState {
    line_height: Pixels,
    total_lines: usize,
    viewport_lines: usize,
    display_offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TerminalFontMetrics {
    pub(crate) cell_width: f32,
    pub(crate) line_height: f32,
}

impl TerminalFontMetrics {
    pub(crate) fn fallback(font_size: f32) -> Self {
        Self {
            cell_width: (font_size * 0.646).max(6.0),
            line_height: (font_size * 1.385).max(font_size + 2.0),
        }
    }
}

/// Short-lived local input kept outside the confirmed terminal buffer.
///
/// This is intentionally limited to printable ASCII on one terminal row. The
/// caller sends it to the remote backend only when the user submits or leaves
/// the supported input path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalInputBuffer {
    pub(crate) tab_id: String,
    pub(crate) text: String,
    pub(crate) cursor: usize,
    pub(crate) anchor_row: usize,
    pub(crate) anchor_col: usize,
    max_columns: usize,
    pub(crate) awaiting_remote_output: bool,
}

impl LocalInputBuffer {
    pub(crate) fn new(
        tab_id: String,
        anchor_row: usize,
        anchor_col: usize,
        max_columns: usize,
    ) -> Self {
        Self {
            tab_id,
            text: String::new(),
            cursor: 0,
            anchor_row,
            anchor_col,
            max_columns,
            awaiting_remote_output: false,
        }
    }

    pub(crate) fn insert_ascii(&mut self, text: &str) -> bool {
        if self.awaiting_remote_output
            || text.is_empty()
            || !text.bytes().all(|byte| (b' '..=b'~').contains(&byte))
            || self.text.len().saturating_add(text.len()) > self.max_columns
        {
            return false;
        }

        self.text.insert_str(self.cursor, text);
        self.cursor += text.len();
        true
    }

    pub(crate) fn backspace(&mut self) {
        if !self.awaiting_remote_output && self.cursor > 0 {
            self.cursor -= 1;
            self.text.remove(self.cursor);
        }
    }

    pub(crate) fn move_left(&mut self) {
        if !self.awaiting_remote_output {
            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    pub(crate) fn move_right(&mut self) {
        if !self.awaiting_remote_output {
            self.cursor = (self.cursor + 1).min(self.text.len());
        }
    }

    pub(crate) fn submit(&mut self) -> Option<Vec<u8>> {
        if self.awaiting_remote_output || self.text.is_empty() {
            return None;
        }

        self.awaiting_remote_output = true;
        let mut bytes = self.text.as_bytes().to_vec();
        bytes.push(b'\r');
        Some(bytes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TerminalLinkActivationPlatform {
    MacOs,
    Other,
}

impl TerminalLinkActivationPlatform {
    fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOs
        } else {
            Self::Other
        }
    }
}

pub(crate) fn terminal_link_activation_modifier_pressed(modifiers: &Modifiers) -> bool {
    terminal_link_activation_modifier_pressed_for_platform(
        modifiers,
        TerminalLinkActivationPlatform::current(),
    )
}

pub(crate) fn terminal_link_visual_active(
    has_hovered_target: bool,
    activation_modifier_pressed: bool,
) -> bool {
    has_hovered_target && activation_modifier_pressed
}

fn terminal_link_activation_modifier_pressed_for_platform(
    modifiers: &Modifiers,
    platform: TerminalLinkActivationPlatform,
) -> bool {
    match platform {
        TerminalLinkActivationPlatform::MacOs => modifiers.platform,
        TerminalLinkActivationPlatform::Other => modifiers.control,
    }
}

#[derive(Clone, Default)]
pub(crate) struct TerminalScrollbarHandle {
    state: Rc<RefCell<Option<TerminalScrollbarState>>>,
    pub(crate) future_display_offset: Rc<Cell<Option<usize>>>,
}

impl TerminalScrollbarHandle {
    pub(crate) fn update(&self, snapshot: &terminal::RenderSnapshot, line_height: Pixels) {
        self.state.replace(Some(TerminalScrollbarState {
            line_height,
            total_lines: snapshot.history_size + snapshot.rows,
            viewport_lines: snapshot.rows,
            display_offset: snapshot.display_offset,
        }));
    }
}

impl ScrollbarHandle for TerminalScrollbarHandle {
    fn offset(&self) -> Point<Pixels> {
        let state_ref = self.state.borrow();
        let Some(state) = state_ref.as_ref() else {
            return point(px(0.), px(0.));
        };
        let scroll_offset = state
            .total_lines
            .saturating_sub(state.viewport_lines)
            .saturating_sub(state.display_offset);
        point(px(0.), -(scroll_offset as f32 * state.line_height))
    }

    fn set_offset(&self, offset: Point<Pixels>) {
        let state_ref = self.state.borrow();
        let Some(state) = state_ref.as_ref() else {
            return;
        };
        let offset_delta = (offset.y / state.line_height).round() as i32;
        let max_offset = state.total_lines.saturating_sub(state.viewport_lines);
        let display_offset = (max_offset as i32 + offset_delta).clamp(0, max_offset as i32);
        self.future_display_offset
            .set(Some(display_offset as usize));
    }

    fn content_size(&self) -> Size<Pixels> {
        let state_ref = self.state.borrow();
        let Some(state) = state_ref.as_ref() else {
            return size(px(0.), px(0.));
        };
        size(
            px(0.),
            state.total_lines.max(state.viewport_lines) as f32 * state.line_height,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum HoverTargetKind {
    Url(String),
    Path(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HoveredUrl {
    pub(crate) target: HoverTargetKind,
    pub(crate) tab_id: String,
    pub(crate) cells: Vec<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use gpui::Modifiers;

    use super::{
        LocalInputBuffer, TerminalLinkActivationPlatform,
        terminal_link_activation_modifier_pressed_for_platform, terminal_link_visual_active,
    };

    #[test]
    fn terminal_link_activation_uses_command_on_macos() {
        assert!(terminal_link_activation_modifier_pressed_for_platform(
            &Modifiers {
                platform: true,
                ..Modifiers::default()
            },
            TerminalLinkActivationPlatform::MacOs,
        ));
        assert!(!terminal_link_activation_modifier_pressed_for_platform(
            &Modifiers {
                control: true,
                ..Modifiers::default()
            },
            TerminalLinkActivationPlatform::MacOs,
        ));
    }

    #[test]
    fn terminal_link_activation_uses_control_off_macos() {
        assert!(terminal_link_activation_modifier_pressed_for_platform(
            &Modifiers {
                control: true,
                ..Modifiers::default()
            },
            TerminalLinkActivationPlatform::Other,
        ));
        assert!(!terminal_link_activation_modifier_pressed_for_platform(
            &Modifiers {
                platform: true,
                ..Modifiers::default()
            },
            TerminalLinkActivationPlatform::Other,
        ));
    }

    #[test]
    fn terminal_link_visual_requires_hover_and_activation_modifier() {
        assert!(!terminal_link_visual_active(false, false));
        assert!(!terminal_link_visual_active(true, false));
        assert!(!terminal_link_visual_active(false, true));
        assert!(terminal_link_visual_active(true, true));
    }

    #[test]
    fn local_input_buffer_edits_ascii_at_the_local_caret() {
        let mut buffer = LocalInputBuffer::new("ssh".into(), 2, 8, 12);

        assert!(buffer.insert_ascii("ab"));
        buffer.move_left();
        assert!(buffer.insert_ascii("X"));
        buffer.backspace();
        buffer.move_right();

        assert_eq!(buffer.text, "ab");
        assert_eq!(buffer.cursor, 2);
        assert_eq!(buffer.submit(), Some(b"ab\r".to_vec()));
        assert!(buffer.awaiting_remote_output);
    }

    #[test]
    fn local_input_buffer_rejects_non_ascii_and_row_overflow() {
        let mut buffer = LocalInputBuffer::new("ssh".into(), 0, 0, 3);

        assert!(!buffer.insert_ascii("中"));
        assert!(buffer.insert_ascii("abc"));
        assert!(!buffer.insert_ascii("d"));
        assert_eq!(buffer.text, "abc");
    }
}
