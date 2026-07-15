# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮实现前台活动终端/UI 的有界高刷新率校准，不枚举显示器 Hz，不新增依赖，不改变后台、深睡、终端或 SFTP 架构。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新并完成高刷新率自适应实现验证。
- changes.md：已追加本轮环境与验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、Alacritty Terminal、russh、russh-sftp。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`。
- 包管理器：`cargo`；本机已确认 `rustc 1.96.1`、`cargo 1.96.1`。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/state/lifecycle.rs`。
- 本轮代码入口：`src/app/state/runtime.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/init.rs`、`src/app/views/layout.rs`。
- 依赖策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；复用锁定 GPUI 的平台帧源、VSync / VRR 与热压力保护。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、帧节奏聚焦测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu x86_64/aarch64 和 macOS x86_64/aarch64 运行 release build；Linux runner 安装 GPUI 所需系统库。
- 外部依赖：上游 GPUI / WGPU 帧调度事实已联网确认；真实 60Hz / 120Hz / VRR 功耗测量需要对应显示器和平台。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`.github/workflows/ci.yml`、`docs/resource-lifecycle.md`、`src/app/state/runtime.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/init.rs`、`src/app/views/layout.rs`、锁定 GPUI `window.rs` / `gpui_wgpu` 和上游源码链接。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：已实现前台活动刷新自适应：有前台 terminal/UI 变化时至多请求 3 个 GPUI animation frame，按实测呈现间隔在 60–120Hz 之间选择合帧周期。窗口 resize/move、失焦或系统恢复会清除样本；空闲、后台和深睡仍保留原有低频策略。
- 受影响文件：`src/app/state/runtime.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/init.rs`、`src/app/views/layout.rs`、`docs/resource-lifecycle*.md`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；两者已更新。

## 开工判定

- 状态：已完成自动化验证。
- 原因：应用层不直接依赖平台显示器 API；用 GPUI `request_animation_frame` 的有界三帧校准得到实际帧间隔，并在状态变化后按 8.333ms 至 16ms 合帧。空闲、后台、深睡和上游 GPUI 热压力 / 非活动窗口保护保持原样。
- 完成动作：已读取环境记录、实施记录、项目地图和相关源码；已联网、未使用多 agent；已运行受影响 Rust 文件格式化、帧节奏聚焦测试（6 项）、`cargo check`、完整 `cargo test --quiet`（200 项）、`git diff --check` 和 tracking docs validator。三平台 60Hz / 120Hz / VRR 实机帧率、CPU、GPU 与功耗验证仍待执行。

## 最后确认时间

- 2026-07-15 12:10 +0800
