use crate::{
    config::LocalShellProfile,
    events::{BackendEvent, BackendEventSender},
    session::Session,
};
use alacritty_terminal::{
    grid::{Dimensions, Scroll},
    index::{Column, Line, Point, Side},
    selection::{Selection, SelectionRange, SelectionType},
    term::{Term, TermDamage, TermMode, cell::Cell, point_to_viewport, viewport_to_point},
    vte::ansi::{CursorShape, Processor},
};
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    ops::Range,
    rc::Rc,
    time::{Duration, Instant},
};

use super::{
    backend::{BackendCommand, BackendTx},
    cwd::extract_shell_working_directory,
    listener::{TerminalListener, TerminalSize, new_term},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabKind {
    Local,
    Ssh,
    Serial,
    Telnet,
}

pub struct TerminalTab {
    pub id: String,
    pub title: String,
    pub kind: TabKind,
    pub status: String,
    pub connected: bool,
    pub disconnected_reason: Option<String>,
    /// Set after a probable system resume until a current-context health
    /// check succeeds or a new backend connects.
    pub connection_may_be_stale: bool,
    /// Incremented each time the tab is reconnected. Used to ignore stale
    /// `BackendEvent::Closed` from the previous backend after a retry.
    pub backend_generation: u32,
    /// Set to `true` when the current backend sends its first `Output` or
    /// `Connected` event. Used to skip stale `Closed` events that arrive
    /// before the new backend has started producing output.
    pub backend_initialized: bool,
    pub session: Option<Session>,
    pub local_shell_profile: Option<LocalShellProfile>,
    processor: Processor,
    term: Term<TerminalListener>,
    pub cols: u16,
    pub rows: u16,
    pub backend: std::sync::Arc<std::sync::Mutex<BackendTx>>,
    pub shell_working_dir: Option<String>,
    cwd_osc_buffer: Vec<u8>,
    events: BackendEventSender,
    pub scroll_pixel_y: f32,
    highlight_cache: RefCell<super::highlight::HighlightCache>,
    highlight_refresh: RefCell<HighlightRefresh>,
    snapshot_cache: RefCell<Option<SnapshotCache>>,
    pending_damage: RefCell<DirtyRows>,
    // Tracks terminal or viewport mutations without scanning every visible cell.
    dirty_generation: u64,
}

struct SnapshotCache {
    dirty_generation: u64,
    keyword_highlight_enabled: bool,
    highlight_generation: u64,
    snapshot: Rc<RenderSnapshot>,
}

const HIGHLIGHT_REFRESH_INTERVAL: Duration = Duration::from_millis(125);
const MAX_SYNCHRONOUS_HIGHLIGHT_ROWS: usize = 4;

struct HighlightRefresh {
    pending_damage: DirtyRows,
    source_rows: Option<Rc<Vec<Rc<RenderRow>>>>,
    highlights: Rc<HashMap<(i32, i32), gpui::Hsla>>,
    last_refresh: Option<Instant>,
    generation: u64,
    enabled: bool,
    force: bool,
    deferred_large_refresh: bool,
}

impl Default for HighlightRefresh {
    fn default() -> Self {
        Self {
            pending_damage: DirtyRows::full(),
            source_rows: None,
            highlights: Rc::new(HashMap::new()),
            last_refresh: None,
            generation: 0,
            enabled: false,
            force: false,
            deferred_large_refresh: false,
        }
    }
}

impl HighlightRefresh {
    fn record_damage(&mut self, damage: DirtyRows) {
        self.pending_damage.extend(damage);
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.deferred_large_refresh = false;
    }

    fn should_refresh_at(&self, now: Instant) -> bool {
        self.force
            || ((self.pending_damage.full || !self.pending_damage.rows.is_empty())
                && self.last_refresh.is_none_or(|last_refresh| {
                    now.duration_since(last_refresh) >= HIGHLIGHT_REFRESH_INTERVAL
                }))
    }

    fn cached_highlights_for(
        &self,
        rows: &Rc<Vec<Rc<RenderRow>>>,
    ) -> Rc<HashMap<(i32, i32), gpui::Hsla>> {
        let Some(source_rows) = &self.source_rows else {
            return Rc::new(HashMap::new());
        };
        if source_rows.len() != rows.len() {
            return Rc::new(HashMap::new());
        }

        // A row can safely keep a deferred highlight only when the current
        // snapshot still holds the exact same verified row block. This also
        // handles scrolling without moving a stale bottom-row highlight onto
        // a row that was modified before the same output batch scrolled.
        let source_to_current = source_rows
            .iter()
            .enumerate()
            .filter_map(|(source_row, source)| {
                rows.iter()
                    .position(|current| Rc::ptr_eq(source, current))
                    .map(|current_row| (source_row, current_row))
            })
            .collect::<HashMap<_, _>>();
        if source_to_current.len() == rows.len()
            && source_to_current
                .iter()
                .all(|(source_row, current_row)| source_row == current_row)
        {
            return self.highlights.clone();
        }
        if source_to_current.is_empty() {
            return Rc::new(HashMap::new());
        }

        Rc::new(
            self.highlights
                .iter()
                .filter_map(|((row, col), color)| {
                    let source_row = usize::try_from(*row).ok()?;
                    let current_row = source_to_current.get(&source_row)?;
                    Some(((*current_row as i32, *col), *color))
                })
                .collect(),
        )
    }
}

#[derive(Clone, Default)]
struct DirtyRows {
    full: bool,
    rows: BTreeSet<usize>,
}

impl DirtyRows {
    fn full() -> Self {
        Self {
            full: true,
            rows: BTreeSet::new(),
        }
    }

    fn mark_full(&mut self) {
        self.full = true;
        self.rows.clear();
    }

    fn extend(&mut self, damage: Self) {
        if damage.full {
            self.mark_full();
        } else if !self.full {
            self.rows.extend(damage.rows);
        }
    }
}

#[derive(Clone, Copy)]
pub struct CursorState {
    pub row: usize,
    pub col: usize,
    pub shape: CursorShape,
}

#[derive(Clone, PartialEq)]
pub struct RenderCell {
    pub col: i32,
    pub cell: Cell,
}

pub struct RenderRow {
    pub cells: Vec<RenderCell>,
}

struct VisibleRowBuild {
    rows: Rc<Vec<Rc<RenderRow>>>,
    rebuilt_rows: BTreeSet<usize>,
}

#[derive(Clone)]
pub struct RenderSnapshot {
    pub visible_rows: Rc<Vec<Rc<RenderRow>>>,
    pub cursor: Option<CursorState>,
    pub selection: Option<ViewportSelection>,
    pub display_offset: usize,
    pub history_size: usize,
    pub rows: usize,
    pub cols: usize,
    pub highlights: Rc<HashMap<(i32, i32), gpui::Hsla>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalComposition {
    pub tab_id: String,
    pub text: String,
    pub selected_range_utf16: Option<Range<usize>>,
    pub anchor_row: usize,
    pub anchor_col: usize,
}

#[derive(Clone)]
pub struct TerminalFrozenSelection {
    pub tab_id: String,
    pub selection: ViewportSelection,
    pub text: String,
}

#[derive(Clone, Copy, Default)]
pub struct TerminalMouseTrackingMode {
    pub mouse_tracking: bool,
    pub alternate_scroll: bool,
    pub sgr_mouse: bool,
}

#[derive(Clone, Copy)]
pub struct ViewportSelection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub is_block: bool,
}

impl TerminalTab {
    pub fn new_local(
        id: String,
        title: String,
        profile: LocalShellProfile,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        let mut tab = Self::new(
            id,
            title,
            TabKind::Local,
            "local shell".into(),
            backend,
            events,
        );
        tab.local_shell_profile = Some(profile);
        tab
    }

    pub fn new_ssh(
        id: String,
        session: &Session,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        let mut tab = Self::new(
            id,
            session.name.clone(),
            TabKind::Ssh,
            format!(
                "connecting {}@{}:{}",
                session.user, session.host, session.port
            ),
            backend,
            events,
        );
        tab.session = Some(session.clone());
        tab.connected = false;
        tab
    }

    pub fn new_serial_or_telnet(
        id: String,
        session: &Session,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        let (kind, status) = match session.kind {
            crate::session::SessionKind::Serial => (
                TabKind::Serial,
                format!(
                    "opening serial {} @ {}",
                    session.serial_port, session.baud_rate
                ),
            ),
            crate::session::SessionKind::Telnet => (
                TabKind::Telnet,
                format!("connecting telnet {}:{}", session.host, session.port),
            ),
            crate::session::SessionKind::Ssh => unreachable!("SSH uses new_ssh"),
        };
        let mut tab = Self::new(id, session.name.clone(), kind, status, backend, events);
        tab.session = Some(session.clone());
        tab.connected = false;
        tab
    }

    fn new(
        id: String,
        title: String,
        kind: TabKind,
        status: String,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        let shared_backend = std::sync::Arc::new(std::sync::Mutex::new(backend));
        Self {
            id: id.clone(),
            title,
            kind,
            status,
            connected: matches!(kind, TabKind::Local),
            disconnected_reason: None,
            connection_may_be_stale: false,
            backend_generation: 0,
            backend_initialized: true,
            session: None,
            local_shell_profile: None,
            processor: Processor::new(),
            term: new_term(100, 30, shared_backend.clone(), id, events.clone()),
            cols: 100,
            rows: 30,
            backend: shared_backend,
            shell_working_dir: None,
            cwd_osc_buffer: Vec::new(),
            events: events.clone(),
            scroll_pixel_y: 0.0,
            highlight_cache: RefCell::new(super::highlight::HighlightCache::default()),
            highlight_refresh: RefCell::new(HighlightRefresh::default()),
            snapshot_cache: RefCell::new(None),
            pending_damage: RefCell::new(DirtyRows::full()),
            dirty_generation: 0,
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return false;
        }
        self.capture_working_directory(bytes);
        self.processor.advance(&mut self.term, bytes);
        self.collect_term_damage();
        self.mark_dirty();
        true
    }

    fn capture_working_directory(&mut self, bytes: &[u8]) {
        let mut buffered = Vec::with_capacity(self.cwd_osc_buffer.len() + bytes.len());
        buffered.extend_from_slice(&self.cwd_osc_buffer);
        buffered.extend_from_slice(bytes);
        let (path, pending) = extract_shell_working_directory(&buffered);
        self.cwd_osc_buffer = pending;

        let Some(path) = path else {
            return;
        };
        if path.is_empty() || self.shell_working_dir.as_deref() == Some(path.as_str()) {
            return;
        }
        self.shell_working_dir = Some(path.clone());
        // This runs while draining backend events on the UI thread, so it
        // must not wait for space in the same queue.
        let _ = self.events.try_send(BackendEvent::WorkingDirectoryChanged {
            tab_id: self.id.clone(),
            path,
        });
    }

    /// Send a command to the backend. Thread-safe via the shared Arc<Mutex>.
    pub fn send_backend(&self, command: BackendCommand) {
        if let Ok(backend) = self.backend.lock() {
            backend.send(command);
        }
    }

    /// Replace the backend with a new one. The `Term`'s internal listener
    /// shares the same `Arc`, so user input is automatically routed to the
    /// new backend. The previous backend is always asked to stop first.
    pub fn set_backend(&mut self, new_backend: BackendTx) {
        let old_backend = self
            .backend
            .lock()
            .ok()
            .map(|mut backend| std::mem::replace(&mut *backend, new_backend));
        if let Some(old_backend) = old_backend {
            old_backend.shutdown();
        }
    }

    pub fn shutdown_backend(&self) {
        if let Ok(backend) = self.backend.lock() {
            backend.shutdown();
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let new_cols = cols.max(1);
        let new_rows = rows.max(1);
        if self.cols != new_cols || self.rows != new_rows {
            self.cols = new_cols;
            self.rows = new_rows;
            tracing::info!(
                component = "terminal",
                operation = "resize",
                cols = self.cols,
                rows = self.rows,
                "Terminal resized"
            );
            self.term.resize(TerminalSize::new(self.cols, self.rows));
            self.collect_term_damage();
            self.mark_dirty();
            self.send_backend(BackendCommand::Resize { cols, rows });
        }
    }

    pub fn cursor_state(&self) -> Option<CursorState> {
        let content = self.term.renderable_content();
        if matches!(content.cursor.shape, CursorShape::Hidden) || content.display_offset > 0 {
            return None;
        }
        let row = content.cursor.point.line.0;
        if row < 0 {
            return None;
        }
        let row = row as usize;
        if row >= self.rows as usize {
            return None;
        }

        Some(CursorState {
            row,
            col: content.cursor.point.column.0,
            shape: content.cursor.shape,
        })
    }

    pub fn app_cursor_mode(&self) -> bool {
        self.term.mode().contains(TermMode::APP_CURSOR)
    }

    pub fn mouse_tracking_mode(&self) -> TerminalMouseTrackingMode {
        let mode = self.term.mode();
        TerminalMouseTrackingMode {
            mouse_tracking: mode.intersects(
                TermMode::MOUSE_REPORT_CLICK | TermMode::MOUSE_MOTION | TermMode::MOUSE_DRAG,
            ),
            alternate_scroll: mode.contains(TermMode::ALT_SCREEN | TermMode::ALTERNATE_SCROLL),
            sgr_mouse: mode.contains(TermMode::SGR_MOUSE),
        }
    }

    pub fn render_snapshot(&self, keyword_highlight_enabled: bool) -> Rc<RenderSnapshot> {
        self.render_snapshot_at(keyword_highlight_enabled, Instant::now())
    }

    fn render_snapshot_at(
        &self,
        keyword_highlight_enabled: bool,
        now: Instant,
    ) -> Rc<RenderSnapshot> {
        let highlight_generation = self.highlight_refresh.borrow().generation;
        let highlight_due = keyword_highlight_enabled && self.highlight_refresh_due(now);
        if let Some(snapshot) = self
            .snapshot_cache
            .borrow()
            .as_ref()
            .filter(|cache| {
                cache.dirty_generation == self.dirty_generation
                    && cache.keyword_highlight_enabled == keyword_highlight_enabled
                    && cache.highlight_generation == highlight_generation
                    && !highlight_due
            })
            .map(|cache| cache.snapshot.clone())
        {
            return snapshot;
        }

        let rows = self.rows;
        let cols = self.cols;
        let content = self.term.renderable_content();
        let dirty_rows = std::mem::take(&mut *self.pending_damage.borrow_mut());
        let snapshot_cache = self.snapshot_cache.borrow();
        let previous = snapshot_cache.as_ref().map(|cache| &cache.snapshot);
        let visible_rows = build_visible_rows(
            &self.term,
            content.display_offset,
            rows as usize,
            cols as usize,
            previous,
            &dirty_rows,
        );
        drop(snapshot_cache);

        let highlights = self.highlight_snapshot(&visible_rows, keyword_highlight_enabled, now);
        let highlight_generation = self.highlight_refresh.borrow().generation;

        let snapshot = Rc::new(RenderSnapshot {
            visible_rows: visible_rows.rows,
            cursor: self.cursor_state(),
            selection: viewport_selection_from_range(
                content.display_offset,
                self.rows as usize,
                self.cols as usize,
                &content.selection,
            ),
            display_offset: content.display_offset,
            history_size: self.term.grid().history_size(),
            rows: self.rows as usize,
            cols: self.cols as usize,
            highlights,
        });
        *self.snapshot_cache.borrow_mut() = Some(SnapshotCache {
            dirty_generation: self.dirty_generation,
            keyword_highlight_enabled,
            highlight_generation,
            snapshot: snapshot.clone(),
        });
        snapshot
    }

    /// Return `(grid_line_base, rows_data)` for the **entire** terminal buffer
    /// including scrollback history. `grid_line_base` is the grid line index of
    /// the first row (typically `-history_size`). Each entry in `rows_data` is
    /// a sorted `Vec<(col, char)>` for that row.
    pub fn full_grid_rows(&self) -> (i32, Vec<Vec<(i32, char)>>) {
        let grid = self.term.grid();
        let history = grid.history_size() as i32;
        let screen = grid.screen_lines() as i32;
        let total = history + screen;
        let cols = self.cols as i32;
        let start_line = -history;

        let mut rows_data: Vec<Vec<(i32, char)>> = Vec::with_capacity(total as usize);
        for line_idx in start_line..(start_line + total) {
            let line = Line(line_idx);
            let mut cells: Vec<(i32, char)> = Vec::new();
            for col_idx in 0..cols {
                let point = Point::new(line, Column(col_idx as usize));
                let c = grid[point].c;
                if c != ' ' && c != '\0' {
                    cells.push((col_idx, c));
                }
            }
            rows_data.push(cells);
        }
        (start_line, rows_data)
    }

    pub fn scroll_history(&mut self, delta: i32) {
        if delta != 0 {
            let display_offset = self.term.grid().display_offset();
            self.term.scroll_display(Scroll::Delta(delta));
            if self.term.grid().display_offset() != display_offset {
                self.collect_term_damage();
                self.force_highlight_refresh();
                self.mark_dirty();
            }
        }
    }

    pub fn scroll_up_by(&mut self, lines: usize) {
        if lines != 0 {
            let display_offset = self.term.grid().display_offset();
            self.term.scroll_display(Scroll::Delta(lines as i32));
            if self.term.grid().display_offset() != display_offset {
                self.collect_term_damage();
                self.force_highlight_refresh();
                self.mark_dirty();
            }
        }
    }

    pub fn scroll_down_by(&mut self, lines: usize) {
        if lines != 0 {
            let display_offset = self.term.grid().display_offset();
            self.term.scroll_display(Scroll::Delta(-(lines as i32)));
            if self.term.grid().display_offset() != display_offset {
                self.collect_term_damage();
                self.force_highlight_refresh();
                self.mark_dirty();
            }
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        let display_offset = self.term.grid().display_offset();
        self.term.scroll_display(Scroll::Bottom);
        if self.term.grid().display_offset() != display_offset {
            self.collect_term_damage();
            self.force_highlight_refresh();
            self.mark_dirty();
        }
    }

    pub fn display_offset(&self) -> usize {
        self.term.grid().display_offset()
    }

    /// Make delayed colors current before an operation that exposes terminal text.
    pub fn force_highlight_refresh(&self) {
        self.highlight_refresh.borrow_mut().force = true;
    }

    pub fn highlight_refresh_due(&self, now: Instant) -> bool {
        let refresh = self.highlight_refresh.borrow();
        refresh.enabled && refresh.should_refresh_at(now)
    }

    #[allow(dead_code)]
    pub fn has_selection(&self) -> bool {
        self.term
            .selection_to_string()
            .is_some_and(|text| !text.is_empty())
    }

    pub fn selection_active(&self) -> bool {
        self.term
            .selection
            .as_ref()
            .is_some_and(|selection| !selection.is_empty())
    }

    pub fn clear_selection(&mut self) {
        if self.term.selection.take().is_some() {
            self.mark_dirty();
        }
    }

    pub fn selection_text(&self) -> Option<String> {
        self.term
            .selection_to_string()
            .filter(|text| !text.is_empty())
    }

    pub fn begin_selection(
        &mut self,
        row: usize,
        col: usize,
        side: Side,
        selection_type: SelectionType,
    ) {
        let point = viewport_to_point(
            self.term.grid().display_offset(),
            Point::new(row, Column(col)),
        );
        self.term.selection = Some(Selection::new(selection_type, point, side));
        self.force_highlight_refresh();
        self.mark_dirty();
    }

    pub fn update_selection(&mut self, row: usize, col: usize, side: Side) {
        let point = viewport_to_point(
            self.term.grid().display_offset(),
            Point::new(row, Column(col)),
        );
        if let Some(selection) = self.term.selection.as_mut() {
            selection.update(point, side);
            self.force_highlight_refresh();
            self.mark_dirty();
        }
    }

    pub fn paste_text(&mut self, text: &str) {
        let bracketed = self.term.mode().contains(TermMode::BRACKETED_PASTE);
        let paste_text = text
            .replace('\x1b', "")
            .replace("\r\n", "\r")
            .replace('\n', "\r");

        let mut bytes = Vec::new();
        if bracketed {
            bytes.extend_from_slice(b"\x1b[200~");
        }
        bytes.extend_from_slice(paste_text.as_bytes());
        if bracketed {
            bytes.extend_from_slice(b"\x1b[201~");
        }

        self.send_backend(BackendCommand::Input(bytes));
    }

    fn collect_term_damage(&mut self) {
        let damage = match self.term.damage() {
            TermDamage::Full => DirtyRows::full(),
            TermDamage::Partial(lines) => DirtyRows {
                full: false,
                rows: lines
                    .filter_map(|line| (line.line < self.rows as usize).then_some(line.line))
                    .collect(),
            },
        };
        self.term.reset_damage();
        self.pending_damage.borrow_mut().extend(damage.clone());
        self.highlight_refresh.borrow_mut().record_damage(damage);
    }

    fn highlight_snapshot(
        &self,
        visible_rows: &VisibleRowBuild,
        keyword_highlight_enabled: bool,
        now: Instant,
    ) -> Rc<HashMap<(i32, i32), gpui::Hsla>> {
        if !keyword_highlight_enabled {
            self.highlight_refresh.borrow_mut().disable();
            return Rc::new(HashMap::new());
        }

        let mut refresh = self.highlight_refresh.borrow_mut();
        if !refresh.enabled {
            refresh.enabled = true;
            refresh.pending_damage.mark_full();
            refresh.force = true;
        }

        if refresh.force {
            let damage = std::mem::take(&mut refresh.pending_damage);
            refresh.highlights = Rc::new(super::highlight::highlight_rows_incremental(
                &visible_rows.rows,
                &damage.rows,
                damage.full,
                &mut self.highlight_cache.borrow_mut(),
            ));
            refresh.source_rows = Some(visible_rows.rows.clone());
            refresh.last_refresh = Some(now);
            refresh.generation = refresh.generation.wrapping_add(1);
            refresh.force = false;
            refresh.deferred_large_refresh = false;
            return refresh.highlights.clone();
        }

        if !visible_rows.rebuilt_rows.is_empty()
            && visible_rows.rebuilt_rows.len() <= MAX_SYNCHRONOUS_HIGHLIGHT_ROWS
            && !refresh.deferred_large_refresh
        {
            refresh.highlights = Rc::new(super::highlight::highlight_rows_incremental(
                &visible_rows.rows,
                &visible_rows.rebuilt_rows,
                false,
                &mut self.highlight_cache.borrow_mut(),
            ));
            refresh.source_rows = Some(visible_rows.rows.clone());
            // Keep a cheap delayed correction for boundary cases without
            // turning a scroll-related full damage into a full-screen rescan.
            refresh.pending_damage = DirtyRows {
                full: false,
                rows: visible_rows.rebuilt_rows.clone(),
            };
            refresh.last_refresh = Some(now);
            refresh.generation = refresh.generation.wrapping_add(1);
            return refresh.highlights.clone();
        }

        if refresh.should_refresh_at(now) {
            let damage = std::mem::take(&mut refresh.pending_damage);
            refresh.highlights = Rc::new(super::highlight::highlight_rows_incremental(
                &visible_rows.rows,
                &damage.rows,
                damage.full,
                &mut self.highlight_cache.borrow_mut(),
            ));
            refresh.source_rows = Some(visible_rows.rows.clone());
            refresh.last_refresh = Some(now);
            refresh.generation = refresh.generation.wrapping_add(1);
            refresh.deferred_large_refresh = false;
            return refresh.highlights.clone();
        }

        if visible_rows.rebuilt_rows.is_empty() && !refresh.deferred_large_refresh {
            refresh.pending_damage = DirtyRows::default();
            refresh.source_rows = Some(visible_rows.rows.clone());
            return refresh.highlights.clone();
        }

        if visible_rows.rebuilt_rows.len() > MAX_SYNCHRONOUS_HIGHLIGHT_ROWS {
            refresh.deferred_large_refresh = true;
        }

        refresh.cached_highlights_for(&visible_rows.rows)
    }

    fn mark_dirty(&mut self) {
        self.dirty_generation = self.dirty_generation.wrapping_add(1);
    }
}

fn build_visible_rows(
    term: &Term<TerminalListener>,
    display_offset: usize,
    rows: usize,
    cols: usize,
    previous: Option<&Rc<RenderSnapshot>>,
    damage: &DirtyRows,
) -> VisibleRowBuild {
    let mut visible_rows = previous
        .filter(|snapshot| snapshot.rows == rows && snapshot.cols == cols)
        .map_or_else(
            || {
                (0..rows)
                    .map(|_| Rc::new(RenderRow { cells: Vec::new() }))
                    .collect()
            },
            |snapshot| snapshot.visible_rows.as_ref().clone(),
        );

    let scroll_rows = previous
        .filter(|snapshot| snapshot.display_offset == 0 && display_offset == 0)
        .map(|snapshot| {
            term.grid()
                .history_size()
                .saturating_sub(snapshot.history_size)
        })
        .filter(|scroll_rows| *scroll_rows > 0 && *scroll_rows < rows);
    let mut reused_rows = BTreeSet::new();
    if damage.full
        && let Some(scroll_rows) = scroll_rows
    {
        let previous_rows = previous
            .expect("scroll row reuse requires a previous snapshot")
            .visible_rows
            .as_ref();
        for row in 0..rows - scroll_rows {
            let candidate = &previous_rows[row + scroll_rows];
            if render_row_matches_term(term, candidate, display_offset, row, cols) {
                visible_rows[row] = candidate.clone();
                reused_rows.insert(row);
            }
        }
    }

    if let Some(previous_snapshot) =
        previous.filter(|snapshot| snapshot.rows == rows && snapshot.cols == cols)
    {
        let previous_rows = previous_snapshot.visible_rows.as_ref();
        let candidate_rows = if damage.full {
            (0..rows).collect()
        } else {
            damage.rows.clone()
        };
        for row in candidate_rows {
            if reused_rows.contains(&row) {
                continue;
            }
            let Some(candidate) = previous_rows.get(row) else {
                continue;
            };
            if render_row_matches_term(term, candidate, display_offset, row, cols) {
                visible_rows[row] = candidate.clone();
                reused_rows.insert(row);
            }
        }
    }

    let rebuild_all = damage.full || previous.is_none();
    let changed_rows = rebuild_all
        .then(|| {
            (0..rows)
                .filter(|row| !reused_rows.contains(row))
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_else(|| {
            damage
                .rows
                .iter()
                .copied()
                .filter(|row| !reused_rows.contains(row))
                .collect()
        });

    if changed_rows.is_empty() {
        return VisibleRowBuild {
            rows: Rc::new(visible_rows),
            rebuilt_rows: changed_rows,
        };
    }

    let mut rebuilt_rows = changed_rows
        .iter()
        .copied()
        .map(|row| (row, Vec::with_capacity(cols)))
        .collect::<HashMap<_, _>>();
    let grid = term.grid();
    for &row in &changed_rows {
        let grid_row = viewport_to_point(display_offset, Point::new(row, Column(0))).line;
        let cells = rebuilt_rows
            .get_mut(&row)
            .expect("changed row is initialized");
        for col in 0..cols {
            let point = Point::new(grid_row, Column(col));
            cells.push(RenderCell {
                col: col as i32,
                cell: grid[point].clone(),
            });
        }
    }

    for &row in &changed_rows {
        let cells = rebuilt_rows.remove(&row).unwrap_or_default();
        visible_rows[row] = Rc::new(RenderRow { cells });
    }

    VisibleRowBuild {
        rows: Rc::new(visible_rows),
        rebuilt_rows: changed_rows,
    }
}

fn render_row_matches_term(
    term: &Term<TerminalListener>,
    row: &RenderRow,
    display_offset: usize,
    viewport_row: usize,
    cols: usize,
) -> bool {
    if row.cells.len() != cols {
        return false;
    }
    let grid_row = viewport_to_point(display_offset, Point::new(viewport_row, Column(0))).line;
    let grid = term.grid();
    row.cells.iter().enumerate().all(|(col, render_cell)| {
        render_cell.col == col as i32 && render_cell.cell == grid[Point::new(grid_row, Column(col))]
    })
}

fn viewport_selection_from_range(
    display_offset: usize,
    rows: usize,
    cols: usize,
    selection: &Option<SelectionRange>,
) -> Option<ViewportSelection> {
    let SelectionRange {
        start,
        end,
        is_block,
    } = selection.as_ref().copied()?;

    let top_point = viewport_to_point(display_offset, Point::new(0, Column(0)));
    let bottom_point = viewport_to_point(
        display_offset,
        Point::new(rows.saturating_sub(1), Column(0)),
    );

    let top_line = top_point.line;
    let bottom_line = bottom_point.line;

    let start_vp = if start.line < top_line {
        Point::new(0, Column(0))
    } else if start.line > bottom_line {
        Point::new(rows.saturating_sub(1), Column(cols.saturating_sub(1)))
    } else {
        point_to_viewport(display_offset, start).unwrap_or(Point::new(0, Column(0)))
    };

    let end_vp = if end.line < top_line {
        Point::new(0, Column(0))
    } else if end.line > bottom_line {
        Point::new(rows.saturating_sub(1), Column(cols.saturating_sub(1)))
    } else {
        point_to_viewport(display_offset, end).unwrap_or(Point::new(
            rows.saturating_sub(1),
            Column(cols.saturating_sub(1)),
        ))
    };

    Some(ViewportSelection {
        start_row: start_vp.line,
        start_col: start_vp.column.0,
        end_row: end_vp.line,
        end_col: end_vp.column.0,
        is_block,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, mpsc},
        time::{Duration, Instant},
    };

    use alacritty_terminal::term::TermMode;

    use crate::{
        events::backend_event_channel,
        terminal::{BackendShutdown, BackendTx},
    };

    use super::{HIGHLIGHT_REFRESH_INTERVAL, SelectionType, Side, TerminalTab};

    struct NoopShutdown;

    impl BackendShutdown for NoopShutdown {
        fn shutdown(&self) {}
    }

    #[test]
    fn csi_2k_erases_the_entire_current_line() {
        let mut tab = test_tab(8, 4);
        tab.feed(b"abcdef");

        tab.feed(b"\r\x1b[2K");

        assert_eq!(row_text(&tab, 0), "        ");
    }

    #[test]
    fn csi_2j_erases_the_visible_display() {
        let mut tab = test_tab(8, 4);
        tab.feed(b"\x1b[1;1Hfirst\x1b[2;1Hsecond");

        tab.feed(b"\x1b[2J");

        for row in 0..4 {
            assert_eq!(row_text(&tab, row), "        ");
        }
    }

    #[test]
    fn decstbm_scrolls_only_inside_the_configured_region() {
        let mut tab = test_tab(8, 5);
        tab.feed(b"\x1b[1;1H1111\x1b[2;1H2222\x1b[3;1H3333\x1b[4;1H4444\x1b[5;1H5555");

        tab.feed(b"\x1b[2;4r\x1b[4;1H\n");

        assert_eq!(row_text(&tab, 0), "1111    ");
        assert_eq!(row_text(&tab, 1), "3333    ");
        assert_eq!(row_text(&tab, 2), "4444    ");
        assert_eq!(row_text(&tab, 3), "        ");
        assert_eq!(row_text(&tab, 4), "5555    ");
    }

    #[test]
    fn alternate_screen_restores_the_primary_buffer() {
        let mut tab = test_tab(12, 4);
        tab.feed(b"primary");

        tab.feed(b"\x1b[?1049h\x1b[1;1H");
        assert!(tab.term.mode().contains(TermMode::ALT_SCREEN));
        tab.feed(b"alternate");
        assert_eq!(row_text(&tab, 0), "alternate   ");

        tab.feed(b"\x1b[?1049l");

        assert!(!tab.term.mode().contains(TermMode::ALT_SCREEN));
        assert_eq!(row_text(&tab, 0), "primary     ");
    }

    #[test]
    fn feed_advances_dirty_generation_once_per_nonempty_batch() {
        let mut tab = test_tab(8, 4);
        let initial_generation = tab.dirty_generation;

        assert!(!tab.feed(b""));
        assert_eq!(tab.dirty_generation, initial_generation);
        assert!(tab.feed(b"first"));
        assert_eq!(tab.dirty_generation, initial_generation.wrapping_add(1));
        assert!(tab.feed(b"second"));
        assert_eq!(tab.dirty_generation, initial_generation.wrapping_add(2));
    }

    #[test]
    fn combined_output_matches_sequential_output_across_escape_boundaries() {
        let mut sequential = test_tab(8, 4);
        let mut combined = test_tab(8, 4);

        sequential.feed(b"\x1b[31mred");
        sequential.feed(b"\x1b[0m plain\r\n");
        combined.feed(b"\x1b[31mred\x1b[0m plain\r\n");

        let sequential_snapshot = sequential.render_snapshot(false);
        let combined_snapshot = combined.render_snapshot(false);
        assert!(same_visible_rows(&sequential_snapshot, &combined_snapshot));
        assert_eq!(
            row_text(&sequential, 0),
            row_text(&combined, 0),
            "combined output preserves terminal text"
        );
    }

    #[test]
    fn render_snapshot_reuses_generation_and_separates_keyword_setting() {
        let mut tab = test_tab(16, 4);
        tab.feed(b"ERROR");

        let first = tab.render_snapshot(true);
        let repeated = tab.render_snapshot(true);
        assert!(std::rc::Rc::ptr_eq(&first, &repeated));
        assert!(!first.highlights.is_empty());

        let disabled = tab.render_snapshot(false);
        assert!(!std::rc::Rc::ptr_eq(&first, &disabled));
        assert!(disabled.highlights.is_empty());
    }

    #[test]
    fn small_output_highlights_changed_rows_without_waiting_for_interval() {
        let mut tab = test_tab(16, 4);
        let start = Instant::now();
        tab.feed(b"ERROR");
        let first = tab.render_snapshot_at(true, start);
        assert!(!first.highlights.is_empty());

        tab.feed(b"\rWARN ");
        let immediate = tab.render_snapshot_at(true, start + Duration::from_millis(1));
        assert!(!immediate.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 2);

        let refreshed = tab.render_snapshot_at(
            true,
            start + Duration::from_millis(1) + HIGHLIGHT_REFRESH_INTERVAL,
        );
        assert!(!refreshed.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 3);
    }

    #[test]
    fn large_visible_rebuild_keeps_the_highlight_interval() {
        let mut tab = test_tab(16, 4);
        let start = Instant::now();
        tab.feed(b"ERROR");
        let _ = tab.render_snapshot_at(true, start);

        tab.resize(16, 5);
        let deferred = tab.render_snapshot_at(true, start + Duration::from_millis(1));
        assert!(deferred.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 1);

        let refreshed = tab.render_snapshot_at(
            true,
            start + Duration::from_millis(1) + HIGHLIGHT_REFRESH_INTERVAL,
        );
        assert!(!refreshed.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 2);
    }

    #[test]
    fn large_rebuild_is_not_bypassed_by_a_following_small_update() {
        let mut tab = test_tab(16, 4);
        let start = Instant::now();
        tab.feed(b"ERROR");
        let _ = tab.render_snapshot_at(true, start);

        tab.resize(16, 5);
        let _ = tab.render_snapshot_at(true, start + Duration::from_millis(1));
        tab.feed(b"\rWARN ");
        let deferred = tab.render_snapshot_at(true, start + Duration::from_millis(2));
        assert!(deferred.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 1);

        let refreshed = tab.render_snapshot_at(
            true,
            start + Duration::from_millis(1) + HIGHLIGHT_REFRESH_INTERVAL,
        );
        assert!(!refreshed.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 2);
    }

    #[test]
    fn unchanged_rows_keep_deferred_highlights_across_full_damage() {
        let mut tab = test_tab(32, 4);
        let start = Instant::now();
        tab.feed(b"retry 0\r\nERROR stable https://x.test");
        let first = tab.render_snapshot_at(true, start);
        let stable_row = first.visible_rows[1].clone();
        assert!(first.highlights.contains_key(&(1, 0)));
        assert!(first.highlights.contains_key(&(1, 13)));

        tab.feed(b"\x1b[1;1Hretry 1");
        tab.pending_damage.borrow_mut().mark_full();
        tab.highlight_refresh
            .borrow_mut()
            .pending_damage
            .mark_full();
        let deferred = tab.render_snapshot_at(true, start + Duration::from_millis(1));

        assert!(std::rc::Rc::ptr_eq(&stable_row, &deferred.visible_rows[1]));
        assert!(deferred.highlights.contains_key(&(1, 0)));
        assert!(deferred.highlights.contains_key(&(1, 13)));
        assert_eq!(tab.highlight_refresh.borrow().generation, 2);
    }

    #[test]
    fn forced_highlight_refresh_bypasses_output_interval() {
        let mut tab = test_tab(16, 4);
        let start = Instant::now();
        tab.feed(b"ERROR");
        let _ = tab.render_snapshot_at(true, start);

        tab.feed(b"\rWARN ");
        tab.force_highlight_refresh();
        let refreshed = tab.render_snapshot_at(true, start + Duration::from_millis(1));

        assert!(!refreshed.highlights.is_empty());
        assert_eq!(tab.highlight_refresh.borrow().generation, 2);
    }

    #[test]
    fn output_and_selection_invalidate_render_snapshot() {
        let mut tab = test_tab(16, 4);
        let initial = tab.render_snapshot(false);

        tab.feed(b"output");
        let after_output = tab.render_snapshot(false);
        assert!(!std::rc::Rc::ptr_eq(&initial, &after_output));

        tab.begin_selection(0, 0, Side::Left, SelectionType::Simple);
        let after_begin_selection = tab.render_snapshot(false);
        assert!(!std::rc::Rc::ptr_eq(&after_output, &after_begin_selection));

        tab.update_selection(0, 1, Side::Right);
        let after_update_selection = tab.render_snapshot(false);
        assert!(!std::rc::Rc::ptr_eq(
            &after_begin_selection,
            &after_update_selection
        ));

        tab.clear_selection();
        let after_clear_selection = tab.render_snapshot(false);
        assert!(!std::rc::Rc::ptr_eq(
            &after_update_selection,
            &after_clear_selection
        ));
    }

    #[test]
    fn output_reuses_undamaged_visible_rows() {
        let mut tab = test_tab(12, 4);
        tab.feed(b"first\r\nsecond");
        let before = tab.render_snapshot(false);
        let first_row = before.visible_rows[0].clone();

        tab.feed(b"\r\nthird");
        let after = tab.render_snapshot(false);

        assert!(std::rc::Rc::ptr_eq(&first_row, &after.visible_rows[0]));
    }

    #[test]
    fn bottom_scroll_reuses_verified_shifted_rows() {
        let mut tab = test_tab(12, 4);
        tab.feed(b"line0\r\nline1\r\nline2\r\nline3");
        let before = tab.render_snapshot(false);
        let prior_second_row = before.visible_rows[1].clone();

        tab.feed(b"\r\nline4");
        let after = tab.render_snapshot(false);

        assert_eq!(row_text(&tab, 0), "line1       ");
        assert!(std::rc::Rc::ptr_eq(
            &prior_second_row,
            &after.visible_rows[0]
        ));
    }

    #[test]
    fn bottom_scroll_rebuilds_row_changed_before_same_batch_scroll() {
        let mut tab = test_tab(12, 4);
        tab.feed(b"line0\r\nline1\r\nline2\r\nline3");
        let before = tab.render_snapshot(false);
        let prior_bottom_row = before.visible_rows[3].clone();

        tab.feed(b"\rCHANGED\r\nline4");
        let after = tab.render_snapshot(false);

        assert_eq!(row_text(&tab, 2), "CHANGED     ");
        assert!(!std::rc::Rc::ptr_eq(
            &prior_bottom_row,
            &after.visible_rows[2]
        ));
    }

    #[test]
    fn deferred_highlights_follow_only_verified_scrolled_rows() {
        let mut tab = test_tab(12, 4);
        let start = Instant::now();
        tab.feed(b"INFO zero\r\nINFO one\r\nINFO two\r\nERROR three");
        let before = tab.render_snapshot_at(true, start);
        assert!(before.highlights.contains_key(&(1, 0)));
        assert!(before.highlights.contains_key(&(3, 0)));

        tab.feed(b"\rCHANGED\r\nINFO four");
        let after = tab.render_snapshot_at(true, start + Duration::from_millis(1));

        // The old second row was verified unchanged and moved into row zero.
        assert!(after.highlights.contains_key(&(0, 0)));
        // The prior bottom row was overwritten before scrolling, so its ERROR
        // color must not be reused for the new CHANGED row.
        assert!(!after.highlights.contains_key(&(2, 0)));
    }

    #[test]
    fn bottom_scroll_highlights_new_row_without_waiting_for_interval() {
        let mut tab = test_tab(16, 4);
        let start = Instant::now();
        tab.feed(b"INFO zero\r\nINFO one\r\nINFO two\r\nINFO three");
        let before = tab.render_snapshot_at(true, start);
        let prior_second_row = before.visible_rows[1].clone();

        tab.feed(b"\r\nERROR four");
        let after = tab.render_snapshot_at(true, start + Duration::from_millis(1));

        assert!(std::rc::Rc::ptr_eq(
            &prior_second_row,
            &after.visible_rows[0]
        ));
        assert!(after.highlights.contains_key(&(3, 0)));
        assert_eq!(tab.highlight_refresh.borrow().generation, 2);
    }

    fn test_tab(cols: u16, rows: u16) -> TerminalTab {
        let (commands, _commands_rx) = mpsc::channel();
        let backend = BackendTx::Local {
            commands,
            shutdown: Arc::new(NoopShutdown),
        };
        let (events, _events_rx) = backend_event_channel();
        let profile = crate::config::LocalShellProfile {
            id: "test-shell".into(),
            name: "Test shell".into(),
            program: "test-shell".into(),
            args: vec!["--interactive".into()],
        };
        let mut tab = TerminalTab::new_local(
            "test-tab".to_string(),
            "test terminal".to_string(),
            profile,
            backend,
            events,
        );
        tab.resize(cols, rows);
        tab
    }

    #[test]
    fn local_tab_retains_its_shell_profile() {
        let tab = test_tab(12, 4);

        let profile = tab
            .local_shell_profile
            .as_ref()
            .expect("local tabs retain a shell profile");
        assert_eq!(profile.id, "test-shell");
        assert_eq!(profile.args, vec!["--interactive"]);
    }

    fn row_text(tab: &TerminalTab, row: i32) -> String {
        let snapshot = tab.render_snapshot(false);
        (0..snapshot.cols as i32)
            .map(|col| {
                snapshot
                    .visible_rows
                    .get(row as usize)
                    .and_then(|render_row| render_row.cells.iter().find(|cell| cell.col == col))
                    .map(|cell| cell.cell.c)
                    .unwrap_or(' ')
            })
            .collect()
    }

    fn same_visible_rows(left: &super::RenderSnapshot, right: &super::RenderSnapshot) -> bool {
        left.visible_rows.len() == right.visible_rows.len()
            && left
                .visible_rows
                .iter()
                .zip(right.visible_rows.iter())
                .all(|(left, right)| {
                    left.cells.len() == right.cells.len()
                        && left
                            .cells
                            .iter()
                            .zip(&right.cells)
                            .all(|(left, right)| left.col == right.col && left.cell == right.cell)
                })
    }
}
