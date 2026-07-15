# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮实现系统唤醒的跨平台 MVP 兜底，不接入 macOS、Windows 或 Linux 原生电源事件，不改变终端或 SFTP 架构。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新并完成系统恢复 MVP 验证。
- changes.md：已追加本轮环境与验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、Alacritty Terminal、russh、russh-sftp。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`。
- 包管理器：`cargo`；本机已确认 `rustc 1.96.1`、`cargo 1.96.1`。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/state/lifecycle.rs`。
- 本轮代码入口：`src/app/state/lifecycle.rs`、`src/app/state/monitoring.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace.rs`、`src/events.rs`、`src/backend/ssh.rs`、`src/app/actions/sftp.rs`。
- 依赖策略：MVP 不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；正式原生电源事件阶段再评估 AppKit、`WM_POWERBROADCAST` 和 Linux D-Bus 接入。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、聚焦生命周期/恢复测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu x86_64/aarch64 和 macOS x86_64/aarch64 运行 release build；Linux runner 安装 GPUI 所需系统库。
- 外部依赖：不需要新增联网事实；用户提供的官方电源事件参考只影响正式跨平台阶段，当前 MVP 可由本地 `Instant` 调度间隙实现。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`.github/workflows/ci.yml`、`docs/resource-lifecycle.md`、`src/app/lifecycle/event_loop.rs`、`src/app/state/lifecycle.rs`、`src/app/state/monitoring.rs`、`src/app/workspace.rs`、`src/events.rs`、`src/backend/ssh.rs`、`src/app/actions/sftp.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：已实现 suspend/resume MVP：以单调时钟和墙上时钟的 10 秒事件泵间隙判定可能恢复，恢复后隔离旧 remote probe、重置本机网络采样基线、仅检查当前可见 SSH、延迟重建空闲 SFTP。现有窗口前台/后台/深睡状态机、SSH/SFTP 有界关闭和 SFTP work pin 继续复用；仍不接入 OS 原生电源事件。
- 受影响文件：`src/app/state/lifecycle.rs`、`src/app/state/monitoring.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace.rs`、`src/events.rs`、`src/backend/ssh.rs`、`src/app/actions/sftp.rs`、`docs/resource-lifecycle*.md`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；两者已更新。

## 开工判定

- 状态：已完成自动化验证。
- 原因：现有单一事件泵可检测长时间未调度；生命周期、监控和 backend event chain 已定位。MVP 不依赖不稳定的原生窗口句柄或 Linux init 系统，并已验证恢复判定、旧远程采样结果隔离及当前页单次探测协议可编译、测试。
- 完成动作：已读取环境记录、实施记录、项目地图和相关源码；未联网、未使用多 agent；已运行受影响 Rust 文件格式化、4 项恢复相关测试、`cargo check`、完整 `cargo test --quiet`（194 项）、`git diff --check` 和 tracking docs validator。三平台实机睡眠/唤醒验证仍待执行。

## 最后确认时间

- 2026-07-15 11:09 +0800
