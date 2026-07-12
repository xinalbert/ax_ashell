# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮修正 macOS X11 本地 X server Browse 选择器，避免 `.app` 应用包无法被选择。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“macOS 本地 X server Browse 选择器修正”任务完成状态。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1` 可用。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/platform/x_server.rs`，`src/backend/ssh/x11.rs`
- 本轮代码入口：`src/app/actions/session.rs`，`src/app/dialogs/settings/proxy.rs`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖；复用现有 `rfd::AsyncFileDialog`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：`rustfmt --edition 2024 src/app/actions/session.rs`、`cargo check`、`git diff --check` 和 tracking docs validator。
- 外部依赖：无；不需要联网。真实 GUI 中 macOS 文件对话框选择 `.app` 仍需手工确认。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/app/actions/session.rs`，`src/app/dialogs/settings/proxy.rs`，`docs/project-implementation-tracker/project-map.md`，本机 `rfd 0.17.2` 源码。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：项目语言、依赖管理、CI 入口和测试命令不变；本轮只调整 macOS X server app Browse 的文件对话框模式，不改变配置 schema、display 解析或 SSH X11 relay。
- 受影响文件：`src/app/actions/session.rs`，跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务和验证范围已切换。

## 开工判定

- 状态：允许开工
- 原因：当前 macOS Browse 使用 `pick_folder()`，而 `.app` 应用包在 macOS 对话框中可能不能按普通目录选择；`rfd 0.17.2` 提供 macOS 专用 `pick_file_or_folder()`，可同时允许文件和目录。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、manifest、X server Browse action 和 `rfd` 0.17.2 对话框 API；确认不需要联网或多 agent。
- 完成后动作：Rust 格式化、`cargo check`、空白检查和 tracking docs validator 均已完成；真实 GUI Browse 选择 `.app` 仍需手工确认。

## 最后确认时间

- 2026-07-12 13:27 +0800
