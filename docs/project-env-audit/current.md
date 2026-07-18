# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮实现默认关闭的 SSH 高延迟本地输入 overlay，不修改 SSH 协议、已确认终端 buffer 或依赖图。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新为 P8 本地输入 overlay 范围。
- changes.md：保留既有历史，施工前预检和完成验证均已追加。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、russh、russh-sftp、reqwest、Argon2id、XChaCha20-Poly1305。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`，依赖由 `Cargo.toml` 与 `Cargo.lock` 锁定。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`。
- 本轮代码入口：`src/session.rs`、`src/app.rs`、`src/app/terminal.rs`、`src/app/actions/terminal.rs`、`src/app/actions/session.rs`、`src/app/dialogs/ssh.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace.rs`、`src/app/lifecycle/init.rs`、`src/terminal/tab.rs`、`src/terminal/element.rs`、`locales/`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo check`、定向单元测试、`cargo test --quiet`、`git diff --check`。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux x86_64/aarch64 和 macOS x86_64/aarch64 构建 release，并在独立 Linux job 安装 `cargo-audit` 审计 `Cargo.lock`。
- 外部依赖：真实 SSH 服务和可控 100/250/500 ms RTT 条件需要三平台手工验收；本轮不新增服务端组件或协议依赖。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`Cargo.lock`、`.github/workflows/ci.yml`、`src/session.rs`、`src/app/actions/terminal.rs`、`src/app/lifecycle/event_loop.rs`、`src/terminal/tab.rs`、`src/terminal/element.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：项目运行环境、工具链、依赖、manifest/lock 和 CI 工作流均未变；当前环境记录的任务范围从安全修复切换到 P8。现有输入路径使用 `BackendCommand::Input`，远端输出由 event loop 批处理后喂入 Alacritty terminal，适合在确认 buffer 之外维护短暂 overlay。
- 受影响文件：`src/session.rs`、`src/app.rs`、`src/app/terminal.rs`、`src/app/actions/terminal.rs`、`src/app/actions/session.rs`、`src/app/dialogs/ssh.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace.rs`、`src/app/lifecycle/init.rs`、`src/terminal/tab.rs`、`src/terminal/element.rs`、`locales/`、`docs/`。
- 是否需要更新 `current.md` / `changes.md`：是，施工前预检和完成验证均已更新。

## 开工判定

- 状态：允许开工。
- 原因：本机工具链满足仓库约束；现有会话持久化、IME composition、输入编码、输出批处理和 terminal 绘制路径均已定位。实施只增加默认关闭的 UI 状态和纯前端预测层，无法保证安全时会回退现有直通路径。
- 开工前动作：已读取环境记录、实施记录、项目地图、manifest/lock、CI、会话表单、输入编码、输出批处理和 composition 绘制路径；不新增依赖、不联网、不使用多 agent。

## 最后确认时间

- 2026-07-18 15:10 +0800
