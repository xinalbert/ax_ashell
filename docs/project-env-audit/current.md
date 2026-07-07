# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮 AxShell 项目改名已完成；运行环境事实未变，允许后续平台安装包手工验证

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“AxShell 项目改名”的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`src/main.rs`，`src/app/startup.rs`，`src/session/config.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/app/startup.rs src/backend/local.rs src/session/config.rs examples/dev_reload.rs src/main.rs src/app/mod.rs src/app/ui.rs src/app/dialogs.rs src/app/keybinding_recorder.rs src/app/config_sync.rs src/app/search.rs src/app/theme.rs src/session/mod.rs src/sftp/mod.rs src/sftp/ops.rs src/sync/mod.rs src/terminal/element.rs src/terminal/input.rs`，`cargo check`，`cargo check --example dev_reload`，`cargo test`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不依赖联网、外部服务或远程 SSH 服务器；验证边界主要是本机 Rust 工具链、Cargo 包名/二进制名、配置目录迁移、macOS/Linux 打包元数据和文档引用一致性。真实安装包外观仍需平台手工确认
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/app/startup.rs`，`src/session/config.rs`，`examples/dev_reload.rs`，`scripts/package-macos-app.sh`，`.github/workflows/release.yml`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：否
- 变化摘要：运行环境和依赖版本未变；本轮只把对外显示名统一为 `AxShell`，把机器可读标识统一为 `ax_shell`，并增加旧 `ax_ashell` 配置目录兼容迁移
- 受影响文件：`Cargo.toml`，`Cargo.lock`，`assets/ax_shell.desktop`，旧 desktop 文件删除，`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`examples/dev_reload.rs`，`src/main.rs`，`src/backend/local.rs`，`src/session/config.rs`，`src/sync/mod.rs`，`src/app/startup.rs`，`src/app/mod.rs`，`src/app/ui.rs`，`src/app/dialogs.rs`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：任务边界明确；本轮只改项目命名、运行时路径、打包元数据和文档，不改变依赖版本、SSH/SFTP 协议或外部服务
- 开工前动作：已复查 `Cargo.toml`、`Cargo.lock`、`src/session/config.rs`、`src/app/startup.rs`、`examples/dev_reload.rs`、`scripts/package-macos-app.sh`、`.github/workflows/release.yml` 与中英文 README / docs；`rustfmt`、`cargo check`、`cargo check --example dev_reload`、`cargo test` 已完成
