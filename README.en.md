[中文](README.md) | [English](README.en.md)

# ax_ashell

![Preview](preview.png)

`ax_ashell` is a Rust and GPUI Component based desktop terminal workspace for local shells, SSH sessions, and built-in SFTP file management.

Forked from https://github.com/rust-kotlin/ashell.git

Repository: <https://github.com/xinalbert/ax_ashell>

## Overview

- Local terminal tabs and SSH remote sessions
- Built-in SFTP browsing, upload, download, and transfer history
- Multi-tab, multi-pane workspace
- Sidebar telemetry, theme switching, and bundled fonts
- Desktop settings, keybinding management, and config sync

## Quick Start

Run the app locally:

```bash
cargo run --release
```

Use the restart-based development reloader:

```bash
cargo dev-reload
```

By default it watches `src`, `assets`, `locales`, `Cargo.toml`, `Cargo.lock`, `build.rs`, and `.cargo`.

## Packaging

macOS `.app`:

```bash
./scripts/package-macos-app.sh
open target/release/ax_ashell.app
```

Debian / Ubuntu `.deb`:

```bash
sudo apt install pkg-config libfontconfig1-dev
cargo install cargo-deb
cargo build --release
cargo deb
```

Install example:

```bash
sudo dpkg -i target/debian/ax_ashell_<version>-1_amd64.deb
```

## Versioning

- Public release labels use dates: `YYYY.MM.DD`
- Multiple releases on the same day append a sequence: `YYYY.MM.DD.1`, `YYYY.MM.DD.2`
- Cargo and package metadata still need semver-compatible values; the current internal build version starts at `2026.7.6`

## Release Status

- GitHub Actions currently keeps build and artifact upload steps enabled
- Token-backed publishing paths such as GitHub Release automation and Homebrew cask updates are disabled for now
- The README no longer documents Homebrew cask installation as an active distribution path

## Assets

- Runtime and packaging icons are sourced from `assets/icons/terminal_icon_all_formats`

## License

This project is licensed under [GPL-3.0-or-later](LICENSE).
