# Agent Instructions

## Scope

- This file is repository-level guidance for Codex agents working in this project.
- Codex loads `AGENTS.md` by default. Do not rely on a file named `.agent` unless `project_doc_fallback_filenames` is explicitly configured for that client.
- Follow nested `AGENTS.override.md` / `AGENTS.md` files if they are added closer to the files being changed.

## Project Basics

- This is a Rust 2024 / GPUI desktop app.
- Minimum Rust version is declared in `Cargo.toml` as `rust-version = "1.88.0"`.
- Use `cargo` as the package manager and keep `Cargo.toml` / `Cargo.lock` unchanged unless the task requires dependency or release metadata changes.
- Prefer existing local helpers and module boundaries over introducing new abstractions.

## Rust Module Layout

- Use the modern Rust module layout:
  - Module `foo` lives in `foo.rs`.
  - Child module `bar` of `foo` lives in `foo/bar.rs`.
  - The parent module file, such as `foo.rs`, declares direct children with `mod bar;` or `pub mod bar;`.
- Do not add new `mod.rs` files. `mod.rs` is still legal Rust, but it is the old-style layout and should not be used for new modules in this repo.
- Never create both `foo.rs` and `foo/mod.rs`; they define the same module and will conflict.
- Module and file names should use `snake_case`.
- `mod` establishes the module tree. `use` only shortens paths. `pub use` is for deliberate public API re-exports.
- Keep child module declarations in the parent module, not all at crate root. Example: `src/database.rs` declares `pub mod mysql;`, and `src/database/mysql.rs` implements `crate::database::mysql`.
- Use Rust 2018+ paths: `crate::`, `self::`, `super::`, and external crate names. Do not add `extern crate` unless there is a concrete compatibility reason.

## Project-Specific Module Notes

- `src/app.rs`, `src/backend.rs`, `src/config.rs`, `src/session.rs`, `src/sftp.rs`, and `src/terminal.rs` are module entry files. Their same-name directories contain child modules.
- Several modules use `#[path]` compatibility exports from `src/app.rs`; preserve these unless the task explicitly asks for a larger module-path migration.
- `src/session/config.rs` is a compatibility re-export layer. Do not add new real configuration logic there; use `src/config/store.rs` or `src/session/model.rs` as appropriate.

## Settings Dropdown And Hover Performance

- Before changing AxShell hover/dropdown/list behavior, read the project-local skill at `.agents/skills/ax-ashell-fast-hover/SKILL.md` and follow its shared fast hover workflow.
- Fast hover styling is centralized in `src/app/hover.rs`. Use `.fast_hover(cx)` for the default hover, `.fast_hover_options(cx, FastHoverOptions::new()...)` when a list needs custom hover/active colors or preserved text color, and `.fast_hover_with_tokens(tokens)` only when tokens are intentionally precomputed for many rows.
- Do not hand-code repeated hover tokens such as `sidebar_accent.opacity(0.8)` / `sidebar_accent_foreground` in feature modules. Add a parameter to `FastHoverOptions` if a new hover requirement appears.
- Settings dropdowns and long hover lists should use the local `src/app/dialogs/settings/fast_menu.rs` helper plus the shared fast hover interface. Do not add a second Settings dropdown / hover-list implementation unless there is a documented reason.
- Long dropdowns and saved/transfer/file lists should follow the SFTP list model: fixed-height rows, `gpui::uniform_list` or equivalent virtual rendering, and only visible rows rendered during scroll / hover.
- When investigating slow hover in a dropdown or list, check both layers:
  - row rendering / hover styling, such as `fast_menu` or `uniform_list` row elements
  - candidate/data construction, such as font enumeration, monospace filtering, SFTP entries, or other expensive builders
- Do not assume hover is cheap just because the list is already visible. Verify that hover or popover re-rendering does not rebuild expensive candidates. Cache expensive candidate lists outside the hover path when the data is expected to be stable during the menu session.
- For Settings font dropdowns, keep UI Font and Terminal Font on the shared `fast_menu` path and cache system font names / terminal monospace filtering rather than creating private dropdown logic.

## Implementation Tracking

- Before real implementation work, read `docs/project-env-audit/current.md` and `docs/project-implementation-tracker/current.md`.
- For feature, fix, refactor, migration, or scoped delivery work, keep `docs/project-implementation-tracker/current.md` current and append meaningful progress to `docs/project-implementation-tracker/changes/YYYY/MM.md`.
- Refresh `docs/project-implementation-tracker/project-map.md` when adding, moving, or redefining important source, workflow, or project-guidance files.
- Run `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` before finishing work that touched tracking docs.

## Verification

- For Rust source changes, default verification is:
  - `rustfmt --edition 2024 <changed-rust-files>`
  - `cargo check`
  - focused tests for the touched behavior when available
  - `cargo test --quiet` for broader behavior changes
  - `git diff --check`
- For Markdown-only guidance or documentation changes, at minimum run `git diff --check`; run the tracking docs validator if `docs/project-implementation-tracker/` changed.
- GUI behavior still needs manual verification when the change affects real interaction, rendering, or platform-specific window behavior.

## Release And Tags

- Release tags are the canonical version source.
- Use `vYYYY.M.D` for the first release of a date, and `vYYYY.M.D-N` for later releases on the same date.
- Do not use zero-padded date segments in tags such as `v2026.07.09`.
- Validate tag mappings with `python3 scripts/release_version.py env --tag <tag>` before creating release tags.
- The release workflow syncs `Cargo.toml` / `Cargo.lock` inside the runner from the tag; do not manually edit manifest versions just to publish a tag.

## Git Hygiene

- Keep commits narrow and reviewable. Separate behavior changes from tracking/docs updates when they are different review units.
- Stage explicit files or hunks. Avoid `git add .` and `git commit -a` in dirty worktrees.
- Review `git diff --cached --stat` and `git diff --cached` before each commit.
- Do not create or move tags without confirming the tag format and current target commit.
