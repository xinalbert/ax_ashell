# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮已让 macOS X11 本地 server 同时识别 XQuartz 和 `/Applications/MacXServer.app`，并避免 MacXServer 被 XQuartz 的 `DISPLAY` 抢路由。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“macOS MacXServer 本地 X server 支持”任务完成状态。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1` 可用。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/platform/x_server.rs`，`src/backend/ssh/x11.rs`
- 本轮代码入口：`src/platform/x_server.rs`，`src/app/lifecycle/startup.rs`，`src/app/actions/session.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖；复用现有 `open -g`、DISPLAY 解析和 X11 relay。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：`rustfmt --edition 2024` 覆盖受影响 Rust 文件、macOS X server helper 聚焦测试、`cargo check`、`git diff --check` 和 tracking docs validator。
- 外部依赖：已联网查看 MacXServer 官方 README / plan 文档确认其 server 监听 port 6000、display `:0`；真实 GUI 中 MacXServer 启动、X11 relay 和远端 xterm/xclock 仍需手工确认。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/platform/x_server.rs`，`src/app/lifecycle/startup.rs`，`src/backend/ssh/x11.rs`，`src/app/actions/session.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：项目语言、依赖管理、CI 入口和测试命令不变；本轮只调整 macOS 本地 X server path/display 选择和 Settings 文案，不改变 SSH X11 fake-cookie relay 协议。
- 受影响文件：`src/platform/x_server.rs`，`src/app/lifecycle/startup.rs`，`src/app/actions/session.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务和验证范围已切换。

## 开工判定

- 状态：允许开工
- 原因：MacXServer 使用标准 TCP display `:0` / port 6000；当前 relay 已支持 TCP 6000，但 macOS display 选择会优先使用环境 `DISPLAY`，可能连到 XQuartz launchd socket。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、manifest、X server helper、startup、X11 relay 和 Settings 源码；已联网确认 MacXServer display 行为；确认不需要多 agent。
- 完成后动作：Rust 格式化、聚焦测试、`cargo check`、空白检查和 tracking docs validator 均已完成；真实 GUI MacXServer 联机仍需手工确认。

## 最后确认时间

- 2026-07-12 13:15 +0800
