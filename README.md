[简体中文](README.zh.md)

# AxShell

[![CI](https://img.shields.io/github/actions/workflow/status/xinalbert/axshell/ci.yml?branch=main&label=CI)](https://github.com/xinalbert/axshell/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/xinalbert/axshell)](https://github.com/xinalbert/axshell/releases/latest)
[![License](https://img.shields.io/github/license/xinalbert/axshell)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.88.0%2B-dea584?logo=rust)](https://www.rust-lang.org/)
[![Platforms](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-1f6feb)](https://github.com/xinalbert/axshell/releases/latest)

![AxShell workspace preview](preview.png)

AxShell is a Rust and GPUI desktop terminal workspace for local shells, SSH, serial, and Telnet sessions, SFTP file management, and repeatable remote operations.

Forked from <https://github.com/rust-kotlin/ashell.git>. The current project is maintained at <https://github.com/xinalbert/axshell>.

## Highlights

- Local terminals plus saved SSH, serial, and Telnet sessions; SSH supports password or private-key authentication and host-key fingerprint confirmation
- Multi-tab and multi-pane workspaces with draggable session groups, detachable terminal windows, configurable keybindings, and search
- Built-in SFTP browsing, transfer control, remote editing, large-directory pagination, and batch download file details
- Themes, fonts, tab color behavior, monitoring, and workspace preferences
- Encrypted session sync over HTTPS WebDAV or S3-compatible storage
- Global and per-session proxy support plus SSH X11 forwarding

## Quick Start

AxShell requires Rust `1.88.0` or newer.

```bash
cargo run --release
```

For restart-based development reload:

```bash
cargo dev-reload
```

## Documentation

- [Documentation index](docs/README.md)
- [Getting started](docs/getting-started.md)
- [Feature guides](docs/README.md#feature-guides)
- [Serial and Telnet](docs/features/serial-telnet.md)
- [Workspace tabs and windows](docs/features/workspace.md)
- [Bundled fonts](docs/features/bundled-fonts.md)
- [Development and packaging](docs/development.md)
- [GitHub releases](https://github.com/xinalbert/axshell/releases)

## Project Notes

- Release tags use `vYYYY.M.D` and `vYYYY.M.D-N`.
- Release automation builds Windows x86_64, Linux x86_64/aarch64, and macOS architecture-specific and universal packages.
- Existing `ax_ashell` configuration is copied into the current `ax_shell` configuration directory when migration is needed; the old directory is left untouched.

## Contributing And Support

Use [GitHub Issues](https://github.com/xinalbert/axshell/issues) for bugs and feature requests. See [Development and Packaging](docs/development.md) before preparing code or release changes.

## License

Licensed under [GPL-3.0-or-later](LICENSE).
