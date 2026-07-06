# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮进入真实实现，已完成 `project-env-audit`，允许继续施工

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为 X11 local X server 跨平台适配任务的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/backend/ssh.rs`，`src/session/config.rs`，`src/app/startup.rs`，`src/app/dialogs.rs`，`locales/en.yml`，`locales/zh-CN.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/session/config.rs src/app/startup.rs src/session/mod.rs src/app/mod.rs src/app/dialogs.rs src/backend/ssh.rs`，`cargo check`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：运行时若使用 X11 forwarding，macOS 需要 XQuartz，Windows 需要 VcXsrv 或 Xming，Linux X11/Wayland 需要本机 `DISPLAY` / Xwayland；MIT cookie 模式需要可用 `xauth`；远程 `sshd_config` 需允许 `X11Forwarding yes`，远程 GUI 程序自身需要 X11 支持
- 证据文件：`.github/workflows/ci.yml`，`Cargo.toml`，`src/backend/ssh.rs`，`src/session/config.rs`，`src/app/startup.rs`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变，但当前任务已从 macOS XQuartz / russh X11 relay 扩展为 Windows Xming/VcXsrv 与 Linux X11/Wayland 平台适配，需要把 current 记录的范围和验证重点改为 local X server 启动、DISPLAY endpoint、xauth/no-auth fallback 和设置页文案
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：任务边界明确，依赖 `russh 0.62.2` 已在本地可用；本轮只改跨平台 local X server 适配，不改变依赖版本
- 开工前动作：刷新 `docs/project-implementation-tracker/` 当前态与项目地图，再修改配置、启动 helper、SSH 后端和文案并运行 `cargo check`
