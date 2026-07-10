# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮按源码目录审查结论做行为保持的模块边界治理。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已复核并刷新为“源码模块边界治理”任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` / `gpui_component` UI，`alacritty_terminal` 终端模型，`tokio` runtime，`russh` / `russh-sftp` 后端。
- 版本约束：仓库声明 `rust-version = 1.88.0`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1` 可用。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/events.rs`，`src/monitoring.rs`，`src/platform.rs`，`src/app.rs`，`src/app/`，`src/backend.rs`，`src/backend/`，`src/config.rs`，`src/config/`，`src/session.rs`，`src/sftp.rs`，`src/sftp/`，`src/terminal.rs`，`src/terminal/`
- 依赖统一策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；使用现代 `foo.rs` / `foo/bar.rs` 模块布局，不新增 `mod.rs`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 以多平台 `cargo build --release --target ...` 为主，未声明独立 test job。
- 当前实施验证命令：每个阶段执行 `rustfmt --edition 2024 <changed-rust-files>`、相关定向测试和 `cargo check`；最终执行 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：不需要联网；本轮是模块路径和职责迁移，不改变协议、配置 schema 或 GUI 交互。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`，`docs/project-implementation-tracker/project-map.md`，本轮涉及的 `src/` 模块入口与调用点。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行时、依赖和工具链不变；源码新增 events/platform/proxy 模块，session/config/app 模块边界完成迁移，`system.rs` 更名为 `monitoring.rs`。
- 受影响文件：`src/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/`
- 是否需要更新 `current.md` / `changes.md`：是；源码入口和测试范围已变化，当前态、历史与项目地图均已刷新。

## 开工判定

- 状态：允许开工
- 原因：所有模块迁移已完成；`cargo check`、78 项完整测试和 `git diff --check` 通过。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、Cargo/CI 证据和相关源码；已确认无需联网和多 agent。
- 完成后动作：项目地图和当前实施记录已更新；tracking docs validator 已通过。

## 最后确认时间

- 2026-07-10 21:27 +0800
