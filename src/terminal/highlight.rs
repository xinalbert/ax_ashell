#[cfg(test)]
use crate::terminal::RenderCell;
use crate::terminal::RenderRow;
use gpui::Hsla;
use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

trait HslaExt {
    fn into_rgba_like(self, r: u8, g: u8, b: u8) -> Self;
}

impl HslaExt for Hsla {
    fn into_rgba_like(self, r: u8, g: u8, b: u8) -> Self {
        let rf = r as f32 / 255.0;
        let gf = g as f32 / 255.0;
        let bf = b as f32 / 255.0;
        let max = rf.max(gf).max(bf);
        let min = rf.min(gf).min(bf);
        let l = (max + min) / 2.0;
        if max == min {
            return Hsla {
                h: 0.0,
                s: 0.0,
                l,
                a: 1.0,
            };
        }
        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };
        let h = if max == rf {
            ((gf - bf) / d + if gf < bf { 6.0 } else { 0.0 }) / 6.0
        } else if max == gf {
            ((bf - rf) / d + 2.0) / 6.0
        } else {
            ((rf - gf) / d + 4.0) / 6.0
        };
        Hsla { h, s, l, a: 1.0 }
    }
}

#[derive(Debug, Clone)]
struct HighlightColors {
    // Log levels
    error: Hsla,    // ERROR, ERR
    critical: Hsla, // PANIC, FATAL, EMERGENCY, CRITICAL
    warning: Hsla,  // WARNING, WARN
    info: Hsla,     // INFO, NOTICE
    debug: Hsla,    // DEBUG, TRACE, DBG
    alert: Hsla,    // ALERT

    // Status indicators
    success: Hsla, // SUCCESS, OK, PASS, DONE, COMPLETED
    failure: Hsla, // FAILED, FAIL, FAILURE
    pending: Hsla, // PENDING, WAITING, PROCESSING
    running: Hsla, // RUNNING, ACTIVE, EXECUTING
    stopped: Hsla, // STOPPED, INACTIVE, HALTED, IDLE
    skipped: Hsla, // SKIPPED, SKIP

    // Network
    network_up: Hsla,   // UP, ONLINE, CONNECTED
    network_down: Hsla, // DOWN, OFFLINE, UNREACHABLE
    timeout: Hsla,      // TIMEOUT, TIMED OUT
    refused: Hsla,      // REFUSED, REJECTED, DENIED

    // Security & Auth
    security: Hsla, // SSH, SSL, TLS, CERTIFICATE
    auth: Hsla,     // AUTHENTICATED, AUTHORIZED, LOGIN
    danger: Hsla,   // ROOT, SUDO, PASSWORD, SECRET

    // Operations
    started: Hsla,    // START, BOOT, STARTING
    stopped_op: Hsla, // STOP, SHUTDOWN, STOPPING
    restart: Hsla,    // RESTART, RESTARTING
    deploy: Hsla,     // DEPLOY, DEPLOYED, DEPLOYMENT
    crashed: Hsla,    // CRASH, CRASHED, SIGSEGV

    // Resources
    memory: Hsla, // MEMORY, RAM, SWAP, HEAP
    cpu: Hsla,    // CPU, PROCESSOR, CORE
    disk: Hsla,   // DISK, STORAGE, PARTITION, MOUNT

    // HTTP codes
    http_2xx: Hsla, // 200-299 Success
    http_3xx: Hsla, // 300-399 Redirect
    http_4xx: Hsla, // 400-499 Client Error
    http_5xx: Hsla, // 500-599 Server Error

    // Dev / Exceptions
    exception: Hsla,  // Exception, Traceback, Error type
    deprecated: Hsla, // DEPRECATED, TODO, FIXME

    // Existing
    network: Hsla, // IP addresses
    url: Hsla,     // http://, https://
    port: Hsla,    // :22, :443, etc.
}

fn hsla(r: u8, g: u8, b: u8) -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    }
    .into_rgba_like(r, g, b)
}

fn highlight_colors() -> HighlightColors {
    HighlightColors {
        // Log levels
        error: hsla(224, 96, 96),     // #E06060 red
        critical: hsla(255, 50, 50),  // #FF3232 bright red
        warning: hsla(232, 201, 122), // #E8C97A yellow
        info: hsla(108, 180, 238),    // #6CB4EE blue
        debug: hsla(130, 140, 155),   // #828C9B gray
        alert: hsla(213, 126, 234),   // #D57EEA bright magenta

        // Status
        success: hsla(126, 198, 153), // #7EC699 green
        failure: hsla(232, 168, 124), // #E8A87C orange
        pending: hsla(232, 201, 122), // #E8C97A yellow
        running: hsla(86, 206, 234),  // #56CEEA cyan
        stopped: hsla(160, 165, 175), // #A0A5AF gray
        skipped: hsla(199, 146, 234), // #C792EA purple

        // Network
        network_up: hsla(126, 198, 153), // #7EC699 green
        network_down: hsla(224, 96, 96), // #E06060 red
        timeout: hsla(213, 126, 234),    // #D57EEA magenta
        refused: hsla(245, 160, 80),     // #F5A050 orange

        // Security
        security: hsla(86, 206, 234), // #56CEEA cyan
        auth: hsla(126, 198, 153),    // #7EC699 green
        danger: hsla(180, 50, 50),    // #B43232 dark red

        // Operations
        started: hsla(126, 198, 153),  // #7EC699 green
        stopped_op: hsla(224, 96, 96), // #E06060 red
        restart: hsla(232, 201, 122),  // #E8C97A yellow
        deploy: hsla(100, 210, 140),   // #64D28C bright green
        crashed: hsla(255, 50, 50),    // #FF3232 bright red

        // Resources
        memory: hsla(199, 146, 234), // #C792EA purple
        cpu: hsla(86, 206, 234),     // #56CEEA cyan
        disk: hsla(108, 180, 238),   // #6CB4EE blue

        // HTTP codes
        http_2xx: hsla(126, 198, 153), // #7EC699 green
        http_3xx: hsla(86, 206, 234),  // #56CEEA cyan
        http_4xx: hsla(232, 201, 122), // #E8C97A yellow
        http_5xx: hsla(224, 96, 96),   // #E06060 red

        // Dev
        exception: hsla(224, 96, 96),   // #E06060 red
        deprecated: hsla(245, 160, 80), // #F5A050 orange

        // Existing
        network: hsla(199, 146, 234), // #C792EA purple
        url: hsla(86, 212, 199),      // #56D4C7 teal
        port: hsla(130, 170, 200),    // #82AAC8 muted teal
    }
}

fn is_keyword_token_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn has_keyword_boundaries(text_bytes: &[u8], start: usize, len: usize) -> bool {
    let before_ok = start == 0 || !is_keyword_token_char(text_bytes[start - 1]);
    let end = start + len;
    let after_ok = end >= text_bytes.len() || !is_keyword_token_char(text_bytes[end]);
    before_ok && after_ok
}

/// Highlight all occurrences of keyword list in `text`, writing to `map`.
/// Case-insensitive and boundary-aware, so keywords must match complete tokens
/// or phrases rather than arbitrary substrings inside longer identifiers/words.
/// Each keyword only matches once per position (no overlapping highlights).
fn highlight_keywords(
    map: &mut HashMap<(i32, i32), Hsla>,
    text: &str,
    byte_to_col: &[i32],
    row_i32: i32,
    keywords: &[&str],
    color: Hsla,
) {
    let text_bytes = text.as_bytes();
    let text_lower: Vec<u8> = text_bytes.iter().map(|b| b.to_ascii_lowercase()).collect();

    for &kw in keywords {
        let kw_lower: Vec<u8> = kw.bytes().map(|b| b.to_ascii_lowercase()).collect();
        let mut start = 0;
        while start + kw_lower.len() <= text_lower.len() {
            if text_lower[start..].starts_with(&kw_lower)
                && has_keyword_boundaries(text_bytes, start, kw_lower.len())
            {
                let abs = start;
                let start_col = byte_to_col[abs];
                let end_col = byte_to_col[(abs + kw_lower.len() - 1).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(color);
                }
                start = abs + kw_lower.len();
            } else {
                start += 1;
            }
        }
    }
}

/// Highlight HTTP status codes (200, 301, 404, 500, etc.)
/// Only matches specific common HTTP codes, not all 3-digit numbers.
fn highlight_http_codes(
    map: &mut HashMap<(i32, i32), Hsla>,
    text: &str,
    byte_to_col: &[i32],
    row_i32: i32,
    colors: &HighlightColors,
) {
    let bytes = text.as_bytes();
    let len = bytes.len();

    // Specific HTTP codes to highlight
    const HTTP_CODES: &[(u16, bool)] = &[
        // 2xx
        (200, true),
        (201, true),
        (202, true),
        (204, true),
        (206, true),
        // 3xx
        (301, true),
        (302, true),
        (304, true),
        (307, true),
        (308, true),
        // 4xx
        (400, true),
        (401, true),
        (403, true),
        (404, true),
        (405, true),
        (408, true),
        (409, true),
        (410, true),
        (422, true),
        (429, true),
        // 5xx
        (500, true),
        (502, true),
        (503, true),
        (504, true),
    ];

    for i in 0..len.saturating_sub(2) {
        if !bytes[i].is_ascii_digit()
            || !bytes[i + 1].is_ascii_digit()
            || !bytes[i + 2].is_ascii_digit()
        {
            continue;
        }

        let code: u16 = ((bytes[i] - b'0') as u16) * 100
            + ((bytes[i + 1] - b'0') as u16) * 10
            + ((bytes[i + 2] - b'0') as u16);

        // Only match specific codes
        if !HTTP_CODES.iter().any(|&(c, _)| c == code) {
            continue;
        }

        // Must be at a boundary (not part of a longer number)
        let before_ok = i == 0 || !bytes[i - 1].is_ascii_digit();
        let after_ok = i + 3 >= len || !bytes[i + 3].is_ascii_digit();
        if !before_ok || !after_ok {
            continue;
        }

        let color = match code {
            200..=299 => colors.http_2xx,
            300..=399 => colors.http_3xx,
            400..=499 => colors.http_4xx,
            500..=599 => colors.http_5xx,
            _ => continue,
        };

        let start_col = byte_to_col[i];
        let end_col = byte_to_col[(i + 2).min(byte_to_col.len() - 1)];
        for c in start_col..=end_col {
            map.entry((row_i32, c)).or_insert(color);
        }
    }
}

#[cfg(test)]
fn highlight_cells(cells: &[Vec<RenderCell>]) -> HashMap<(i32, i32), Hsla> {
    let visible_rows = rows_from_cells(cells);
    highlight_rows_incremental(
        &visible_rows,
        &(0..cells.len()).collect(),
        true,
        &mut HighlightCache::default(),
    )
}

#[derive(Default)]
pub struct HighlightCache {
    keyword_rows: Vec<CachedHighlightRow>,
    url_rows: Vec<BTreeSet<i32>>,
    url_wraps: Vec<bool>,
}

struct CachedHighlightRow {
    source: Rc<RenderRow>,
    highlights: Vec<(i32, Hsla)>,
}

pub fn highlight_rows_incremental(
    rows: &[Rc<RenderRow>],
    dirty_rows: &BTreeSet<usize>,
    full_damage: bool,
    cache: &mut HighlightCache,
) -> HashMap<(i32, i32), Hsla> {
    let colors = highlight_colors();
    let cache_needs_reset =
        full_damage || cache.keyword_rows.len() != rows.len() || cache.url_rows.len() != rows.len();
    if cache_needs_reset {
        cache.keyword_rows = rows
            .iter()
            .map(|row| CachedHighlightRow {
                source: row.clone(),
                highlights: highlight_row(row, &colors),
            })
            .collect();
        cache.url_rows = vec![BTreeSet::new(); rows.len()];
        cache.url_wraps = vec![false; rows.len()];
    } else {
        for &row_idx in dirty_rows {
            let Some(row) = rows.get(row_idx) else {
                continue;
            };
            cache.keyword_rows[row_idx] = CachedHighlightRow {
                source: row.clone(),
                highlights: highlight_row(row, &colors),
            };
        }
    }

    // A disabled highlighter does not receive output damage. Recheck row identity
    // when it is enabled again so stale cached colors cannot be reused.
    let mut effective_dirty_rows = dirty_rows.clone();
    for (row_idx, row) in rows.iter().enumerate() {
        if !Rc::ptr_eq(&cache.keyword_rows[row_idx].source, row) {
            cache.keyword_rows[row_idx] = CachedHighlightRow {
                source: row.clone(),
                highlights: highlight_row(row, &colors),
            };
            effective_dirty_rows.insert(row_idx);
        }
    }

    let current_wraps: Vec<bool> = rows.iter().map(|row| row_wraps_to_next(row)).collect();
    let url_dirty_rows = url_dirty_rows(
        &current_wraps,
        &cache.url_wraps,
        &effective_dirty_rows,
        cache_needs_reset,
    );
    for &row_idx in &url_dirty_rows {
        cache.url_rows[row_idx].clear();
    }
    apply_url_highlights(rows, &url_dirty_rows, &mut cache.url_rows);
    cache.url_wraps = current_wraps;

    let mut map = HashMap::new();
    for (row_idx, cached) in cache.keyword_rows.iter().enumerate() {
        for &(col, color) in &cached.highlights {
            map.insert((row_idx as i32, col), color);
        }
        for &col in &cache.url_rows[row_idx] {
            map.entry((row_idx as i32, col)).or_insert(colors.url);
        }
    }
    map
}

fn highlight_row(row: &RenderRow, colors: &HighlightColors) -> Vec<(i32, Hsla)> {
    if row.cells.is_empty() {
        return Vec::new();
    }

    let mut map = HashMap::new();
    let mut chars_buf = String::with_capacity(128);
    let mut byte_to_col: Vec<i32> = Vec::with_capacity(128);
    for cell in &row.cells {
        chars_buf.push(cell.cell.c);
        while byte_to_col.len() < chars_buf.len() {
            byte_to_col.push(cell.col);
        }
    }
    let text = chars_buf.as_str();
    // Row blocks can move when scrollback advances, so colors are stored as
    // column-only data and assigned to the current viewport index by the caller.
    let row_i32 = 0;

    // ── 1. Critical errors (highest priority) ──────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "PANIC",
            "EMERGENCY",
            "FATAL",
            "SEGFAULT",
            "CRITICAL",
            "OOM",
            "OUT OF MEMORY",
            "KERNEL PANIC",
            "CORE DUMPED",
            "BUS ERROR",
        ],
        colors.critical,
    );

    // ── 2. Error keywords ──────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["ERROR", "ERR"],
        colors.error,
    );

    // ── 3. Alert ───────────────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["ALERT"],
        colors.alert,
    );

    // ── 4. Warning keywords ────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["WARNING", "WARN"],
        colors.warning,
    );

    // ── 5. Info keywords ───────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["INFO", "INFORMATION", "NOTICE"],
        colors.info,
    );

    // ── 6. Debug keywords ──────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["DEBUG", "DBG", "TRACE"],
        colors.debug,
    );

    // ── 7. Success status ──────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "SUCCESS",
            "SUCCEEDED",
            "SUCCESSFUL",
            "PASSED",
            "PASS",
            "OK",
            "DONE",
            "COMPLETED",
            "FINISHED",
            "COMPLETE",
        ],
        colors.success,
    );

    // ── 8. Failure status ──────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["FAILED", "FAILURE", "FAIL", "NOT OK"],
        colors.failure,
    );

    // ── 9. Pending / Waiting ───────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["PENDING", "WAITING", "PROCESSING", "IN PROGRESS", "QUEUED"],
        colors.pending,
    );

    // ── 10. Running / Active ───────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["RUNNING", "ACTIVE", "EXECUTING", "IN_PROGRESS", "LIVE"],
        colors.running,
    );

    // ── 11. Stopped / Inactive ─────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["STOPPED", "INACTIVE", "HALTED", "IDLE", "PAUSED"],
        colors.stopped,
    );

    // ── 12. Skipped ────────────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["SKIPPED", "SKIP", "SKIPPING"],
        colors.skipped,
    );

    // ── 13. Network UP ─────────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "UP",
            "ONLINE",
            "CONNECTED",
            "REACHABLE",
            "LISTENING",
            "ESTABLISHED",
            "LINK UP",
        ],
        colors.network_up,
    );

    // ── 14. Network DOWN ───────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "DOWN",
            "OFFLINE",
            "UNREACHABLE",
            "DISCONNECTED",
            "NOT LISTENING",
            "LINK DOWN",
            "NO CARRIER",
        ],
        colors.network_down,
    );

    // ── 15. Timeout ────────────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "TIMEOUT",
            "TIMED OUT",
            "TIMEOUTS",
            "ETIMEDOUT",
            "SLOW",
            "LATENCY",
        ],
        colors.timeout,
    );

    // ── 16. Refused / Denied ───────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "REFUSED",
            "REJECTED",
            "DENIED",
            "PERMISSION DENIED",
            "ACCESS DENIED",
            "FORBIDDEN",
            "BLOCKED",
            "DROP",
        ],
        colors.refused,
    );

    // ── 17. Security / Protocol ────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "SSH",
            "SSHD",
            "SSL",
            "TLS",
            "HTTPS",
            "CERTIFICATE",
            "CERT",
            "FIREWALL",
            "IPTABLES",
            "ACL",
            "WAF",
        ],
        colors.security,
    );

    // ── 18. Authentication ─────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "AUTHENTICATED",
            "ACCEPTED",
            "AUTHORIZED",
            "LOGIN",
            "LOGOUT",
            "LOGGED IN",
            "LOGGED OUT",
            "SESSION",
        ],
        colors.auth,
    );

    // ── 19. Danger (root/sudo/secrets) ─────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "ROOT",
            "SUDO",
            "UID=0",
            "PASSWORD",
            "SECRET",
            "TOKEN",
            "API_KEY",
            "APIKEY",
            "PRIVATE KEY",
            "CREDENTIALS",
        ],
        colors.danger,
    );

    // ── 20. Operations: Start ──────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "STARTED", "START", "STARTING", "BOOT", "BOOTING", "LAUNCHED", "LAUNCH",
        ],
        colors.started,
    );

    // ── 21. Operations: Stop ───────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "STOPPED",
            "STOP",
            "STOPPING",
            "SHUTDOWN",
            "SHUTTING DOWN",
            "TERMINATED",
        ],
        colors.stopped_op,
    );

    // ── 22. Operations: Restart ────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "RESTARTED",
            "RESTART",
            "RESTARTING",
            "RELOAD",
            "RELOADED",
            "RELOADING",
        ],
        colors.restart,
    );

    // ── 23. Operations: Deploy ─────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "DEPLOYED",
            "DEPLOYMENT",
            "DEPLOYING",
            "DEPLOY",
            "ROLLBACK",
            "ROLLED BACK",
            "ROLLING BACK",
            "UPGRADE",
            "UPGRADED",
        ],
        colors.deploy,
    );

    // ── 24. Operations: Crash ──────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "CRASH",
            "CRASHED",
            "CRASHING",
            "SIGSEGV",
            "SIGABRT",
            "SIGKILL",
            "DIED",
            "EXITED",
            "EXIT CODE",
            "CORE DUMP",
        ],
        colors.crashed,
    );

    // ── 25. Resources: Memory ──────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &["MEMORY", "RAM", "HEAP", "STACK", "SWAP", "MEM"],
        colors.memory,
    );

    // ── 26. Resources: CPU ─────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "CPU",
            "PROCESSOR",
            "CORE",
            "CORES",
            "THREAD",
            "THREADS",
            "LOAD",
        ],
        colors.cpu,
    );

    // ── 27. Resources: Disk ────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "DISK",
            "STORAGE",
            "PARTITION",
            "MOUNT",
            "FILESYSTEM",
            "INODE",
            "IOPS",
            "READ",
            "WRITE",
        ],
        colors.disk,
    );

    // ── 28. Dev: Exceptions ────────────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "EXCEPTION",
            "TRACEBACK",
            "THROW",
            "THROWN",
            "STACKTRACE",
            "TYPEERROR",
            "VALUEERROR",
            "KEYERROR",
            "ATTRIBUTEERROR",
            "INDEXERROR",
            "RUNTIMEERROR",
            "IOERROR",
            "OSERROR",
            "NULLPOINTER",
            "NPE",
            "SEGFAULT",
        ],
        colors.exception,
    );

    // ── 29. Dev: Deprecated / TODO ─────────────────────────
    highlight_keywords(
        &mut map,
        text,
        &byte_to_col,
        row_i32,
        &[
            "DEPRECATED",
            "TODO",
            "FIXME",
            "HACK",
            "XXX",
            "WORKAROUND",
            "TEMPORARY",
        ],
        colors.deprecated,
    );

    // ── 30. HTTP status codes ──────────────────────────────
    highlight_http_codes(&mut map, text, &byte_to_col, row_i32, &colors);

    // ── 31. IP addresses ───────────────────────────────────
    for m in find_ip_addresses(text) {
        let ip_len = find_ip_len(&text[m..]);
        let start_col = byte_to_col[m];
        let end_col = byte_to_col[(m + ip_len - 1).min(byte_to_col.len() - 1)];
        for c in start_col..=end_col {
            map.entry((row_i32, c)).or_insert(colors.network);
        }
    }

    // ── 33. Port numbers ───────────────────────────────────
    for m in find_ports(text) {
        let port_len = find_port_len(&text[m..]);
        let start_col = byte_to_col[m];
        let end_col = byte_to_col[(m + port_len - 1).min(byte_to_col.len() - 1)];
        for c in start_col..=end_col {
            map.entry((row_i32, c)).or_insert(colors.port);
        }
    }

    let mut highlights = map
        .into_iter()
        .filter_map(|((row, col), color)| (row == row_i32).then_some((col, color)))
        .collect::<Vec<_>>();
    highlights.sort_by_key(|(col, _)| *col);
    highlights
}

fn apply_url_highlights(
    rows: &[Rc<RenderRow>],
    dirty_rows: &BTreeSet<usize>,
    highlights: &mut [BTreeSet<i32>],
) {
    // URL detection must use logical lines because a terminal wrap can split a URL.
    let logical_lines = build_logical_lines(rows);
    for line in &logical_lines {
        if !line.rows.iter().any(|row| dirty_rows.contains(row)) {
            continue;
        }
        let text = line.text.as_str();
        for m in find_urls(text) {
            let url_len = find_url_len(&text[m..]);
            for i in 0..url_len {
                let idx = m + i;
                if idx < line.byte_to_cell.len() {
                    let (r, c) = line.byte_to_cell[idx];
                    highlights[r].insert(c as i32);
                }
            }
        }
    }
}

fn url_dirty_rows(
    current_wraps: &[bool],
    previous_wraps: &[bool],
    dirty_rows: &BTreeSet<usize>,
    full_damage: bool,
) -> BTreeSet<usize> {
    if full_damage {
        return (0..current_wraps.len()).collect();
    }

    let mut result = BTreeSet::new();
    for &row in dirty_rows {
        if row >= current_wraps.len() {
            continue;
        }

        let mut start = row;
        while start > 0
            && (current_wraps[start - 1] || previous_wraps.get(start - 1).copied().unwrap_or(false))
        {
            start -= 1;
        }
        let mut end = row;
        while end + 1 < current_wraps.len()
            && (current_wraps[end] || previous_wraps.get(end).copied().unwrap_or(false))
        {
            end += 1;
        }
        result.extend(start..=end);
    }
    result
}

fn row_wraps_to_next(row: &RenderRow) -> bool {
    row.cells.last().is_some_and(|cell| {
        cell.cell
            .flags
            .contains(alacritty_terminal::term::cell::Flags::WRAPLINE)
    })
}

#[cfg(test)]
fn rows_from_cells(cells: &[Vec<RenderCell>]) -> Vec<Rc<RenderRow>> {
    cells
        .iter()
        .cloned()
        .into_iter()
        .map(|mut cells| {
            cells.sort_by_key(|cell| cell.col);
            Rc::new(RenderRow { cells })
        })
        .collect()
}

fn find_ip_len(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut dots = 0u8;
    let mut digits = 0u8;
    let mut len = 0usize;

    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                digits += 1;
                if digits > 3 {
                    return 0;
                }
            }
            b'.' => {
                if digits == 0 {
                    return 0;
                }
                dots += 1;
                if dots > 3 {
                    return 0;
                }
                digits = 0;
            }
            _ => break, // Stop at first non-digit/non-dot (including '/')
        }
        len += 1;
    }

    if dots == 3 && digits > 0 { len } else { 0 }
}

fn find_ip_addresses(text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let bytes = text.as_bytes();
    let len = bytes.len();

    for i in 0..len {
        if bytes[i].is_ascii_digit() && (i == 0 || is_boundary(bytes[i - 1] as char)) {
            let remaining = &text[i..];
            let ip_len = find_ip_len(remaining);
            if ip_len > 0 {
                let ip_str = &remaining[..ip_len];
                let valid = ip_str.split('.').all(|octet| octet.parse::<u8>().is_ok());
                if valid {
                    positions.push(i);
                }
            }
        }
    }
    positions
}

fn is_boundary(c: char) -> bool {
    !c.is_ascii_alphanumeric() && c != '_'
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalTarget {
    Url(String),
    Path(String),
}

fn find_urls(text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;
    while let Some(pos) = text[start..].find("http") {
        let abs = start + pos;
        let remaining = &text[abs..];
        if remaining.starts_with("https://") || remaining.starts_with("http://") {
            if abs == 0 || is_boundary(text.as_bytes()[abs - 1] as char) {
                positions.push(abs);
            }
        }
        start = abs + 4;
    }
    positions
}

fn trim_wrapped_terminal_token_len(text: &str) -> usize {
    let mut len = text.len();

    while len > 0 {
        let candidate = &text[..len];
        let Some(last_char) = candidate.chars().next_back() else {
            break;
        };

        let trimmed_len = match last_char {
            '"' | '\'' | '`' | '<' | '>' | '“' | '”' | '‘' | '’' | '«' | '»' | '「' | '」'
            | '『' | '』' | '《' | '》' | '〈' | '〉' => len - last_char.len_utf8(),
            ')' if candidate.matches('(').count() < candidate.matches(')').count() => {
                len - last_char.len_utf8()
            }
            ']' if candidate.matches('[').count() < candidate.matches(']').count() => {
                len - last_char.len_utf8()
            }
            '}' if candidate.matches('{').count() < candidate.matches('}').count() => {
                len - last_char.len_utf8()
            }
            c if is_cjk_sentence_punctuation(c) => len - last_char.len_utf8(),
            '.' | ',' | ';' | ':' | '!' | '?' => len - last_char.len_utf8(),
            _ => break,
        };

        len = trimmed_len;
    }

    len
}

fn is_cjk_sentence_punctuation(c: char) -> bool {
    matches!(c, '，' | '。' | '、' | '；' | '：' | '！' | '？' | '…')
}

fn is_url_token_boundary(c: char) -> bool {
    c.is_ascii_whitespace() || is_cjk_sentence_punctuation(c)
}

fn find_url_len(text: &str) -> usize {
    let len = text.find(is_url_token_boundary).unwrap_or(text.len());
    trim_wrapped_terminal_token_len(&text[..len])
}

fn trim_path_leading_wrappers_len(text: &str) -> usize {
    let mut trimmed = 0;
    for ch in text.chars() {
        match ch {
            '"' | '\'' | '`' | '<' | '(' | '[' | '{' | '“' | '‘' | '«' | '「' | '『' | '《'
            | '〈' => trimmed += ch.len_utf8(),
            _ => break,
        }
    }
    trimmed
}

fn trim_trailing_location_suffix_len(text: &str) -> usize {
    let mut len = text.len();
    let mut trimmed_any = false;

    loop {
        let candidate = &text[..len];
        let Some((head, tail)) = candidate.rsplit_once(':') else {
            break;
        };
        if tail.is_empty() || !tail.as_bytes().iter().all(|b| b.is_ascii_digit()) {
            break;
        }
        len = head.len();
        trimmed_any = true;
    }

    if trimmed_any { len } else { text.len() }
}

fn find_path_len(text: &str) -> usize {
    let len = text
        .find(|c: char| c.is_ascii_whitespace())
        .unwrap_or(text.len());
    let mut len = trim_wrapped_terminal_token_len(&text[..len]);
    let location_trimmed = trim_trailing_location_suffix_len(&text[..len]);
    if location_trimmed != len {
        len = trim_wrapped_terminal_token_len(&text[..location_trimmed]);
    }
    len
}

fn is_path_candidate(text: &str) -> bool {
    if text.is_empty() || text.contains("://") {
        return false;
    }

    if matches!(text, "." | ".." | "~") {
        return true;
    }

    if text.starts_with('/')
        || text.starts_with("./")
        || text.starts_with("../")
        || text.starts_with("~/")
    {
        return true;
    }

    text.contains('/')
}

fn find_path_spans(text: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let bytes = text.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }

        let token_start = index;
        while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        let token_end = index;
        let token = &text[token_start..token_end];
        let leading_trim = trim_path_leading_wrappers_len(token);
        if leading_trim >= token.len() {
            continue;
        }
        let candidate_start = token_start + leading_trim;
        let candidate = &token[leading_trim..];
        let candidate_len = find_path_len(candidate);
        if candidate_len == 0 {
            continue;
        }
        let candidate = &candidate[..candidate_len];
        if is_path_candidate(candidate) {
            spans.push((candidate_start, candidate_len));
        }
    }

    spans
}

fn find_ports(text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let bytes = text.as_bytes();
    let len = bytes.len();

    for i in 0..len {
        if bytes[i] == b':'
            && i + 1 < len
            && bytes[i + 1].is_ascii_digit()
            && (i == 0 || is_boundary(bytes[i - 1] as char) || bytes[i - 1] == b' ')
        {
            let mut j = i + 1;
            while j < len && bytes[j].is_ascii_digit() {
                j += 1;
            }
            let port_str = &text[i + 1..j];
            if let Ok(port) = port_str.parse::<u16>() {
                if port > 0 {
                    let after_ok = j >= len || is_boundary(bytes[j] as char);
                    if after_ok {
                        positions.push(i);
                    }
                }
            }
        }
    }
    positions
}

fn find_port_len(text: &str) -> usize {
    if !text.starts_with(':') {
        return 0;
    }
    let mut len = 1;
    for b in text.as_bytes()[1..].iter() {
        if b.is_ascii_digit() {
            len += 1;
        } else {
            break;
        }
    }
    len
}

#[derive(Clone)]
pub struct LogicalLine {
    pub text: String,
    pub byte_to_cell: Vec<(usize, usize)>,
    pub rows: Vec<usize>,
}

pub fn build_logical_lines(rows: &[Rc<RenderRow>]) -> Vec<LogicalLine> {
    let mut logical_lines = Vec::new();
    let mut current_line: Option<LogicalLine> = None;

    for (row_idx, row) in rows.iter().enumerate() {
        if row.cells.is_empty() {
            if let Some(line) = current_line.take() {
                logical_lines.push(line);
            }
            continue;
        }

        let wraps_from_prev = row_idx > 0 && {
            current_line
                .as_ref()
                .and_then(|line| line.rows.last())
                .is_some_and(|&previous_row| row_wraps_to_next(&rows[previous_row]))
        };

        if !wraps_from_prev {
            if let Some(line) = current_line.take() {
                logical_lines.push(line);
            }
        }

        let mut line = current_line.take().unwrap_or_else(|| LogicalLine {
            text: String::with_capacity(128),
            byte_to_cell: Vec::with_capacity(128),
            rows: Vec::new(),
        });

        for cell in &row.cells {
            line.text.push(cell.cell.c);
            let end_len = line.text.len();
            while line.byte_to_cell.len() < end_len {
                line.byte_to_cell.push((row_idx, cell.col as usize));
            }
        }
        line.rows.push(row_idx);

        current_line = Some(line);
    }

    if let Some(line) = current_line.take() {
        logical_lines.push(line);
    }

    logical_lines
}

pub fn find_url_at_cell(
    rows: &[Rc<RenderRow>],
    row: usize,
    col: usize,
) -> Option<(String, Vec<(usize, usize)>)> {
    let logical_lines = build_logical_lines(rows);
    for line in logical_lines {
        let text = line.text.as_str();
        for m in find_urls(text) {
            let url_len = find_url_len(&text[m..]);
            let mut url_cells = Vec::with_capacity(url_len);
            let mut matched = false;
            for i in 0..url_len {
                let idx = m + i;
                if idx < line.byte_to_cell.len() {
                    let (r, c) = line.byte_to_cell[idx];
                    if r == row && c == col {
                        matched = true;
                    }
                    url_cells.push((r, c));
                }
            }
            if matched {
                let url_str = text[m..m + url_len].to_string();
                return Some((url_str, url_cells));
            }
        }
    }
    None
}

pub fn find_terminal_target_at_cell(
    rows: &[Rc<RenderRow>],
    row: usize,
    col: usize,
) -> Option<(TerminalTarget, Vec<(usize, usize)>)> {
    if let Some((url, cells)) = find_url_at_cell(rows, row, col) {
        return Some((TerminalTarget::Url(url), cells));
    }

    let logical_lines = build_logical_lines(rows);
    for line in logical_lines {
        let text = line.text.as_str();
        for (start, path_len) in find_path_spans(text) {
            let mut path_cells = Vec::with_capacity(path_len);
            let mut matched = false;
            for i in 0..path_len {
                let idx = start + i;
                if idx < line.byte_to_cell.len() {
                    let (r, c) = line.byte_to_cell[idx];
                    if r == row && c == col {
                        matched = true;
                    }
                    path_cells.push((r, c));
                }
            }
            if matched {
                let path = text[start..start + path_len].to_string();
                return Some((TerminalTarget::Path(path), path_cells));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{
        HighlightCache, TerminalTarget, find_path_len, find_path_spans,
        find_terminal_target_at_cell, find_url_len, highlight_cells, highlight_keywords,
        highlight_rows_incremental, hsla, rows_from_cells,
    };
    use crate::terminal::RenderCell;
    use std::collections::{BTreeSet, HashMap};

    fn highlighted_columns(text: &str, keywords: &[&str]) -> Vec<i32> {
        let mut map = HashMap::new();
        let byte_to_col: Vec<i32> = (0..text.len()).map(|idx| idx as i32).collect();
        highlight_keywords(&mut map, text, &byte_to_col, 0, keywords, hsla(255, 0, 0));

        let mut cols: Vec<i32> = map
            .keys()
            .filter_map(|(row, col)| if *row == 0 { Some(*col) } else { None })
            .collect();
        cols.sort_unstable();
        cols
    }

    #[test]
    fn keyword_highlight_matches_standalone_token() {
        assert_eq!(
            highlighted_columns("ERROR: failed", &["ERROR"]),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn keyword_highlight_skips_identifier_substrings() {
        assert!(highlighted_columns("my_ERROR ERRNO setup", &["ERROR", "ERR", "UP"]).is_empty());
    }

    #[test]
    fn keyword_highlight_matches_short_token_only_at_boundaries() {
        assert_eq!(
            highlighted_columns("status=OK, upstream", &["OK", "UP"]),
            vec![7, 8]
        );
    }

    #[test]
    fn keyword_highlight_matches_multi_word_phrase_with_outer_boundaries() {
        let expected: Vec<i32> = (7..20).collect();
        assert_eq!(
            highlighted_columns("panic: OUT OF MEMORY!", &["OUT OF MEMORY"]),
            expected
        );
    }

    #[test]
    fn find_url_len_trims_trailing_quotes_and_punctuation() {
        let text = "https://example.com/path?q=1\",";
        assert_eq!(find_url_len(text), "https://example.com/path?q=1".len());
    }

    #[test]
    fn find_url_len_trims_unmatched_closing_parenthesis_only() {
        let wrapped = "https://example.com/path(foo))";
        let balanced = "https://example.com/path(foo)";

        assert_eq!(find_url_len(wrapped), balanced.len());
        assert_eq!(find_url_len(balanced), balanced.len());
    }

    #[test]
    fn find_url_len_trims_angle_brackets_and_cjk_quotes() {
        let markdown = "https://example.com/docs>)";
        let chinese = "https://example.com/path?q=1》”";

        assert_eq!(find_url_len(markdown), "https://example.com/docs".len());
        assert_eq!(find_url_len(chinese), "https://example.com/path?q=1".len());
    }

    #[test]
    fn find_url_len_stops_at_cjk_sentence_punctuation() {
        let url = "https://github.com/abbodi1406/vcredist";

        for text in [
            "https://github.com/abbodi1406/vcredist，可以用这个工具",
            "https://github.com/abbodi1406/vcredist。下一句",
            "https://github.com/abbodi1406/vcredist；继续",
            "https://github.com/abbodi1406/vcredist：说明",
            "https://github.com/abbodi1406/vcredist！",
            "https://github.com/abbodi1406/vcredist？",
        ] {
            assert_eq!(find_url_len(text), url.len());
        }
    }

    #[test]
    fn find_path_len_trims_line_column_suffix() {
        let path = "./src/main.rs:12:3),";
        assert_eq!(find_path_len(path), "./src/main.rs".len());
    }

    #[test]
    fn find_path_spans_detects_absolute_and_relative_paths() {
        let text = "open /srv/app and ../logs/build.log:14";
        let spans = find_path_spans(text);
        let extracted: Vec<&str> = spans
            .iter()
            .map(|(start, len)| &text[*start..*start + *len])
            .collect();
        assert_eq!(extracted, vec!["/srv/app", "../logs/build.log"]);
    }

    #[test]
    fn find_path_spans_ignores_plain_words() {
        assert!(find_path_spans("warning error output").is_empty());
    }

    #[test]
    fn find_terminal_target_at_cell_returns_path_when_hovering_path() {
        let text = "../logs/app.log";
        let cells = text
            .chars()
            .enumerate()
            .map(|(col, ch)| RenderCell {
                col: col as i32,
                cell: alacritty_terminal::term::cell::Cell {
                    c: ch,
                    ..Default::default()
                },
            })
            .collect::<Vec<_>>();

        let rows = rows_from_cells(&[cells]);
        let target = find_terminal_target_at_cell(&rows, 0, 3);
        assert_eq!(
            target.map(|(target, _)| target),
            Some(TerminalTarget::Path("../logs/app.log".to_string()))
        );
    }

    #[test]
    fn incremental_highlights_match_full_highlights_after_rows_move() {
        let previous = render_cells(&["INFO ready", "ERROR failed"]);
        let current = render_cells(&["ERROR failed", "WARN retry"]);
        let mut cache = HighlightCache::default();
        let previous_rows = rows_from_cells(&previous);
        let current_rows = rows_from_cells(&current);

        let _ = highlight_rows_incremental(&previous_rows, &(0..2).collect(), true, &mut cache);
        let incremental =
            highlight_rows_incremental(&current_rows, &(0..2).collect(), true, &mut cache);

        assert_eq!(incremental, highlight_cells(&current));
    }

    #[test]
    fn incremental_highlights_preserve_urls_across_wrapped_rows() {
        let mut cells = render_cells(&["https://exa", "mple.com/log"]);
        cells
            .get_mut(0)
            .expect("first row")
            .iter_mut()
            .find(|cell| cell.col == 10)
            .expect("last cell of first row")
            .cell
            .flags
            .insert(alacritty_terminal::term::cell::Flags::WRAPLINE);

        let rows = rows_from_cells(&cells);
        let highlights = highlight_rows_incremental(
            &rows,
            &(0..2).collect(),
            true,
            &mut HighlightCache::default(),
        );

        assert!(highlights.contains_key(&(0, 0)));
        assert!(highlights.contains_key(&(1, 11)));
    }

    #[test]
    fn reenabled_highlighter_refreshes_rows_not_seen_while_disabled() {
        let previous = rows_from_cells(&render_cells(&["INFO ready"]));
        let current = rows_from_cells(&render_cells(&["ERROR failed"]));
        let mut cache = HighlightCache::default();

        let _ = highlight_rows_incremental(&previous, &(0..1).collect(), true, &mut cache);
        let reenabled = highlight_rows_incremental(&current, &BTreeSet::new(), false, &mut cache);

        assert!(reenabled.contains_key(&(0, 0)));
        assert_eq!(reenabled, highlight_cells(&render_cells(&["ERROR failed"])));
    }

    #[test]
    fn reenabled_highlighter_refreshes_url_rows_not_seen_while_disabled() {
        let mut previous_cells = render_cells(&["https://exa", "mple.com/log"]);
        previous_cells
            .get_mut(0)
            .expect("first row")
            .iter_mut()
            .find(|cell| cell.col == 10)
            .expect("last cell of wrapped row")
            .cell
            .flags
            .insert(alacritty_terminal::term::cell::Flags::WRAPLINE);
        let previous = rows_from_cells(&previous_cells);
        let current_cells = render_cells(&["no link here", "still no link"]);
        let current = rows_from_cells(&current_cells);
        let mut cache = HighlightCache::default();

        let _ = highlight_rows_incremental(&previous, &(0..2).collect(), true, &mut cache);
        let reenabled = highlight_rows_incremental(&current, &BTreeSet::new(), false, &mut cache);

        assert_eq!(reenabled, highlight_cells(&current_cells));
        assert!(!reenabled.contains_key(&(0, 0)));
        assert!(!reenabled.contains_key(&(1, 0)));
    }

    #[test]
    fn url_damage_expands_across_all_wrapped_rows() {
        let mut cells = render_cells(&["https://exa", "mple.com/lo", "gs/file"]);
        for row in 0..2 {
            cells
                .get_mut(row)
                .expect("wrapped row")
                .iter_mut()
                .find(|cell| cell.col == 10)
                .expect("last cell of wrapped row")
                .cell
                .flags
                .insert(alacritty_terminal::term::cell::Flags::WRAPLINE);
        }
        let rows = rows_from_cells(&cells);
        let mut cache = HighlightCache::default();
        let _ = highlight_rows_incremental(&rows, &(0..3).collect(), true, &mut cache);

        let mut changed_cells = cells.clone();
        changed_cells
            .get_mut(2)
            .expect("last wrapped row")
            .iter_mut()
            .find(|cell| cell.col == 0)
            .expect("last wrapped row first cell")
            .cell
            .c = ' ';
        let changed_rows = rows_from_cells(&changed_cells);
        let updated = highlight_rows_incremental(
            &changed_rows,
            &std::iter::once(2).collect(),
            false,
            &mut cache,
        );

        assert_eq!(updated, highlight_cells(&changed_cells));
    }

    #[test]
    fn url_damage_uses_previous_wrap_boundary_after_wrap_is_removed() {
        let mut cells = render_cells(&["https://exa", "mple.com/log"]);
        cells
            .get_mut(0)
            .expect("first row")
            .iter_mut()
            .find(|cell| cell.col == 10)
            .expect("last cell of wrapped row")
            .cell
            .flags
            .insert(alacritty_terminal::term::cell::Flags::WRAPLINE);
        let rows = rows_from_cells(&cells);
        let mut cache = HighlightCache::default();
        let _ = highlight_rows_incremental(&rows, &(0..2).collect(), true, &mut cache);

        let mut changed_cells = cells.clone();
        changed_cells
            .get_mut(0)
            .expect("first row")
            .iter_mut()
            .find(|cell| cell.col == 10)
            .expect("last cell of wrapped row")
            .cell
            .flags
            .remove(alacritty_terminal::term::cell::Flags::WRAPLINE);
        let changed_rows = rows_from_cells(&changed_cells);
        let updated = highlight_rows_incremental(
            &changed_rows,
            &std::iter::once(0).collect(),
            false,
            &mut cache,
        );

        assert_eq!(updated, highlight_cells(&changed_cells));
    }

    fn render_cells(lines: &[&str]) -> Vec<Vec<RenderCell>> {
        lines
            .iter()
            .map(|text| {
                text.chars()
                    .enumerate()
                    .map(|(col, ch)| RenderCell {
                        col: col as i32,
                        cell: alacritty_terminal::term::cell::Cell {
                            c: ch,
                            ..Default::default()
                        },
                    })
                    .collect()
            })
            .collect()
    }
}
