# 当前项目实施记录

## 当前目标

- 目标：将 X11 forwarding 的本地 X server 支持从 macOS XQuartz 扩展到 Windows Xming / VcXsrv，以及 Linux X11 / Wayland(Xwayland) 环境
- 交付物：跨平台本地 X server 启动 helper、跨平台默认 X server 应用路径、Windows/Linux endpoint 与 xauth 适配、设置页通用文案、通过的格式化/编译验证、同步后的实施跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/startup.rs`，`src/backend/ssh.rs`，`src/session/config.rs`，`src/session/mod.rs`，`src/app/mod.rs`，`src/app/dialogs.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：安装 Xming/VcXsrv/XQuartz、修复远程 GUI 程序自身缺少 X11 支持、真实 Windows/Linux 联机验证、release workflow、README 改写

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 读取现有 X11 配置、UI、XQuartz 启动 helper 和 X11 relay | 源码检查 | 现有字段仍以 `xquartz_app_path` 命名，需兼容迁移为通用语义 |
| P2 | completed | 泛化本地 X server 默认路径和启动逻辑 | `cargo check` | macOS XQuartz、Windows VcXsrv/Xming、Linux 不强制启动 |
| P3 | completed | 扩展 relay 的 DISPLAY、endpoint 和 xauth 查找 | `cargo check` | Wayland 走 Xwayland 提供的 `DISPLAY` |
| P4 | completed | 更新设置页文案、格式化、编译和跟踪文档校验 | `rustfmt`；`cargo check`；tracking docs 校验 | GUI 联机验证需用户环境 |

## 已完成

- 读取当前 `src/backend/ssh.rs`，确认 X11 relay 已经有 `request_x11`、fake cookie 替换和 channel/socket relay
- 读取当前 `src/session/config.rs`、`src/app/startup.rs`、`src/app/dialogs.rs` 和 locales，确认 UI 仍显示 XQuartz 专名
- 将本地 X server 启动 helper 泛化为跨平台：macOS 用 `open -g` 启动 `.app`，Windows 启动 VcXsrv/Xming executable，Linux 可留空直接使用 `DISPLAY`
- Windows 默认路径优先探测 `VcXsrv\vcxsrv.exe`，其次 `Xming\Xming.exe`；默认本地 display 为 `127.0.0.1:0`
- Linux / Wayland 使用环境变量 `DISPLAY`；Wayland 下依赖 Xwayland 提供 X11 display
- X11 relay 在找不到本机 `xauth` real cookie 时，改写为 no-auth setup，支持关闭 access control 的本地 X server
- 设置页文案从 XQuartz 专名改成 Local X Server，说明 macOS / Windows / Linux-Wayland 的对应方式

## 验证

- 已完成：源码级确认当前平台支持边界；`rustfmt --edition 2024 --config skip_children=true src/session/config.rs src/app/startup.rs src/session/mod.rs src/app/mod.rs src/app/dialogs.rs src/backend/ssh.rs`；`cargo check`；tracking docs 校验
- 未完成：Windows/Linux GUI 联机验证；Windows/Linux target cross compile

## 风险与阻塞

- Windows Xming/VcXsrv 若关闭 access control，relay 会 fallback 到 no-auth setup；若开启 MIT cookie，则仍需本机可用 `xauth`
- Linux Wayland 不是 X11 协议本身；X11 forwarding 仍需 Xwayland 提供 `DISPLAY`
- 现有配置 key 为 `xquartz_app_path`，本轮应兼容保留，避免破坏已有用户配置

## 下一步

- 在 Windows 上分别用 VcXsrv/Xming、在 Linux X11 和 Wayland(Xwayland) 下做实机验证

## 最后更新时间

- 2026-07-07 12:05 CST
