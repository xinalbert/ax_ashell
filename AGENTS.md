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
