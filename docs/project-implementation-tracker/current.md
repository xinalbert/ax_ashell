# 当前项目实施记录

## 当前目标

- 目标：让 macOS X11 本地 server 同时支持 XQuartz 和 `/Applications/MacXServer.app`，MacXServer 场景固定走 TCP display `127.0.0.1:0`。
- 交付物：macOS 本地 X server 默认候选识别 MacXServer；配置路径为 MacXServer 时 display 解析不再使用 XQuartz launchd `DISPLAY`；启动后返回正确 display；设置页文案提示 XQuartz / MacXServer；相关聚焦测试和文档记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/platform/x_server.rs`，`src/app/lifecycle/startup.rs`，`src/app/actions/session.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 SSH X11 fake-cookie relay 协议、实现 keyboard-interactive 或 Xauthority 管理、修改 `Cargo.toml` / `Cargo.lock`、真实 MacXServer / XQuartz GUI 联机验收。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复核当前 macOS X11 启动、display 解析和 relay endpoint | 读取 `x_server.rs`、`startup.rs`、`x11.rs`、Settings 源码 | 当前 relay 已支持 TCP 6000，但会被环境 `DISPLAY` 影响 |
| P2 | completed | 增加 MacXServer 路径识别、display 固定和 Settings 文案 | `rustfmt --edition 2024`，聚焦测试，`cargo check` | 用户安装路径为 `/Applications/MacXServer.app` |
| P3 | completed | 完成验证和 tracking docs 校验 | `git diff --check`，tracking validator | GUI 联机保留为手工验证 |

## 已完成

- 已读取 `AGENTS.md`、环境记录、当前实施记录和项目地图。
- 已复核当前 macOS `launch_local_x_server_app()` 只执行 `open -g <app>`，理论上可启动任意 `.app`，但返回 display 仍来自 `default_display()`。
- 已复核当前 `resolve_display()` 优先使用环境 `DISPLAY`；若本机已有 XQuartz，可能得到 launchd socket，导致配置 MacXServer 后 relay 仍先连 XQuartz。
- 已确认 `local_x11_endpoints()` 对 `127.0.0.1:0` 会直接走 TCP port 6000，符合 MacXServer README。
- 已联网记录 MacXServer 官方 README / plan 证据：MacXServer port 6000 对应 display `:0`。
- 已在 `src/platform/x_server.rs` 增加 MacXServer bundle 名识别、macOS 默认候选和固定 display `127.0.0.1:0`。
- 已让 macOS 本地 X server 启动后返回 `resolve_display(path, true)`，因此配置 MacXServer 时不会继续沿用 XQuartz launchd `DISPLAY`。
- 已调整本地 X server 选择器起始目录，优先从当前配置路径的父目录打开，便于选择 `/Applications/MacXServer.app` 或 XQuartz。
- 已更新中英文 X11 设置文案，说明 macOS 支持 XQuartz / MacXServer，并提示 MacXServer 路由到 `127.0.0.1:0`。

## 验证

- 已完成：源码路径复核；联网确认 MacXServer display/port 行为；确认不需要新增依赖、不修改配置 schema；`rustfmt --edition 2024 src/platform/x_server.rs src/app/lifecycle/startup.rs src/app/actions/session.rs` 通过；`cargo test --quiet macxserver` 通过 2 项；`cargo check` 通过，仅保留既有 `block v0.1.6` future-incompat warning；`git diff --check` 通过；tracking docs validator 通过。
- 未完成：真实 GUI 中启动 MacXServer / XQuartz 并通过远端 `xterm` 或 `xclock` 做 X11 forwarding 联机验证。

## 风险与阻塞

- 风险：MacXServer 的 Xauthority/no-auth 行为和具体远端 X11 程序兼容性仍需真实联机确认；本轮自动化只能覆盖路径/display 解析。
- 无阻塞。

## 下一步

- 在真实 GUI 中将本地 X server 应用设置为 `/Applications/MacXServer.app`，启动 MacXServer 后用远端 X11 程序确认窗口能回连本机 TCP 6000。

## 最后更新时间

- 2026-07-12 13:15 +0800
