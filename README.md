[中文](README.md) | [English](README.en.md)

# ax_ashell

![Preview](preview.png)

`ax_ashell` 是一个基于 Rust 与 GPUI Component 的桌面终端工作区，面向本地 Shell、SSH 远程连接和内置 SFTP 文件管理场景。

Forked from https://github.com/rust-kotlin/ashell.git

仓库地址：<https://github.com/xinalbert/ax_ashell>

## 项目概览

- 本地终端与 SSH 远程会话
- 内置 SFTP 浏览、上传、下载与传输记录
- 多标签、多 Pane 工作区
- 系统监控侧栏、主题切换、内置字体
- 设置页、快捷键管理、配置同步等桌面能力

## 快速开始

运行开发版：

```bash
cargo run --release
```

开发期自动重编译并重启：

```bash
cargo dev-reload
```

默认监听 `src`、`assets`、`locales`、`Cargo.toml`、`Cargo.lock`、`build.rs` 和 `.cargo`。

`cargo dev-reload` 在 debug 模式下会把 dev-reload 自身事件、`cargo build` 输出和被拉起应用的 `stdout/stderr` 落盘到 `target/debug/dev-reload-logs/session-<timestamp>/`；`cargo dev-reload --release` 不生成这些日志文件。

若后续文件变更导致编译失败，`cargo dev-reload` 会保留当前正在运行的旧进程并继续监听，不会直接退出；只有初次启动就编译失败时才会返回错误结束。

## 打包

macOS `.app`：

```bash
./scripts/package-macos-app.sh
open target/release/ax_ashell.app
```

Debian / Ubuntu `.deb`：

```bash
sudo apt install pkg-config libfontconfig1-dev
cargo install cargo-deb
cargo build --release
cargo deb
```

安装示例：

```bash
sudo dpkg -i target/debian/ax_ashell_<version>-1_amd64.deb
```

## 版本规则

- 对外版本按日期表示：`YYYY.MM.DD`
- 同一天多次发布追加序号：`YYYY.MM.DD.1`、`YYYY.MM.DD.2`
- 由于 Cargo / 包管理需要兼容 semver，仓库内部构建版本使用兼容映射；当前起始版本为 `2026.7.6`

## 发布状态

- 当前仓库保留 GitHub Actions 构建与 artifact 上传
- 自动发布 GitHub Release、Homebrew cask 等依赖 token / 密钥的流程已暂时停用
- 当前不再维护 README 中的 Homebrew cask 安装说明

## 资源说明

- 运行时与打包图标统一使用 `assets/icons/terminal_icon_all_formats`

## 许可证

本项目采用 [GPL-3.0-or-later](LICENSE)。
