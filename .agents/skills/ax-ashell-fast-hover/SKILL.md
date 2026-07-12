---
name: ax-ashell-fast-hover
description: Use when working in AxShell on GPUI hover performance, Settings dropdowns, font menus, hover_list/hover-list behavior, fast_menu, SFTP/sidebar/saved-session/selector lists, context menus, or Chinese requests such as "下拉 hover 卡", "统一 hover", "快速 hover". Enforces the shared src/app/hover.rs FastHoverOptions/FastHoverExt API, Settings fast_menu, and gpui::uniform_list for long lists.
---

# AxShell Fast Hover

## Workflow

1. Read `AGENTS.md`, `docs/project-env-audit/current.md`, `docs/project-implementation-tracker/current.md`, and `docs/project-implementation-tracker/project-map.md` before implementation.
2. Locate the relevant UI path from `project-map.md`, then inspect the actual Rust files before editing.
3. Use the shared hover API in `src/app/hover.rs` for row hover styling. Add parameters to `FastHoverOptions` if a real new hover requirement appears.
4. Use `src/app/dialogs/settings/fast_menu.rs` for Settings dropdowns and small Settings-style dropdowns outside Settings when the user wants the same fast hover behavior.
5. Use `gpui::uniform_list` for long dropdowns/lists so only visible rows render during scroll and hover.
6. Update implementation tracking docs for feature/fix/refactor work, then run the verification checklist below.

## Shared Hover API

Prefer these interfaces:

```rust
use crate::app::hover::{
    FastHoverExt, FastHoverOptions, fast_hover_tokens, list_fast_hover_options,
};
```

- Use `.fast_hover(cx)` for default menu/sidebar hover.
- Use `.fast_hover_options(cx, FastHoverOptions::new()...)` when a row needs custom hover background, foreground, active colors, or preserved text color.
- Use `.fast_hover_options(cx, list_fast_hover_options(cx))` for SFTP/file-list style `list_hover` visuals.
- Use `fast_hover_tokens(cx)` plus `.fast_hover_with_tokens(tokens)` when rendering many rows from the same closure so tokens are resolved once.
- For checked/selected rows, set active style explicitly from the same tokens: `tokens.active_bg` and `tokens.active_fg`.

Do not repeat raw theme tokens in feature modules, especially:

```rust
theme.sidebar_accent.opacity(0.8)
theme.sidebar_accent_foreground
```

If the current API cannot express a required hover variant, extend `FastHoverOptions` instead of creating a private helper.

## Dropdowns And Lists

- Settings dropdowns must use `fast_settings_menu`, `fast_settings_menu_disabled`, `fast_settings_menu_lazy`, or `fast_settings_menu_lazy_disabled`.
- Expensive Settings dropdown candidates, such as UI Font or Terminal Font, must be lazy-built for menu open and cached for the menu session. Do not enumerate system fonts or run monospace filtering on every render or hover.
- Long dropdowns/lists must use fixed-height rows, stable row ids, and `uniform_list`.
- Keep row render functions cheap: no filesystem scans, font enumeration, SFTP fetches, sorting of large candidate sets, or heavy filtering inside hover-driven row render paths.
- For small self-rendered context menus, render lightweight rows with shared fast hover tokens. Avoid nested Button hover plus custom row hover on the same item.

## Disallowed Paths

Do not introduce these for new AxShell dropdown/list/menu hover work unless the commit documents a concrete reason:

- `DropdownMenu` / `dropdown_menu`
- `PopupMenuItem`
- `ContextMenuExt`
- package-provided `context_menu(...)`
- hand-written `.hover(...)` color blocks that duplicate `FastHoverOptions`
- full rendering of long dropdown/list rows when `uniform_list` is viable

Existing ordinary hover in unrelated controls, such as terminal splitters, may remain if it is not a dropdown/list/menu row and is not part of the reported hover latency.

## Known AxShell Hot Paths

- `src/app/hover.rs`: shared fast hover options, tokens, and extension trait.
- `src/app/dialogs/settings/fast_menu.rs`: Settings fast dropdown helper; includes lazy item construction and virtualized long menus.
- `src/app/dialogs/settings/font_page.rs`: UI Font and Terminal Font dropdown candidate construction and caching.
- `src/app/dialogs/ssh.rs`: SSH group dropdown reuses Settings fast menu.
- `src/app/dialogs/selector.rs`: saved session selector list.
- `src/app/views/sidebar.rs`: saved sessions sidebar rows and self-rendered saved-session context menu.
- `src/app/views/sftp_panel.rs`: SFTP local/remote file lists with `list_fast_hover_options`.
- `src/app/views/sftp_panel/transfer_panel.rs`: transfer history list.
- `src/app/views/layout.rs`: self-rendered SFTP and saved-session context menu overlays.

## Verification

For hover/dropdown/list work, run at least:

```bash
rg -n "DropdownMenu|dropdown_menu|PopupMenuItem|ContextMenuExt|context_menu\\(|\\.hover\\(|fast_hover|uniform_list" src/app src/terminal
```

For Rust changes, also run:

```bash
rustfmt --edition 2024 <changed-rust-files>
cargo check
git diff --check
```

Run focused tests or `cargo test --quiet` when behavior changes beyond styling. If tracking docs changed, run:

```bash
python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .
```

GUI hover feel still requires manual verification in the real app. State that clearly if it was not performed.
