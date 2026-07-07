# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`examples/dev_reload.rs`，`src/backend/local.rs`，`src/terminal/mod.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`examples/`，`src/app/`，`src/backend/`，`src/terminal/`，`docs/`，`Cargo.toml`，`.cargo/`
- 排除：`.git/`，`target/`，`assets/` 批量资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `examples/` | 开发辅助命令与本地 runner | 调整 `cargo dev-reload` 的构建、重启、日志或启动方式时 | 本轮核心改动点在 `examples/dev_reload.rs`，包括 macOS dev bundle、`LSEnvironment` 和启动语义 |
| `src/app/` | GPUI 应用启动、窗口创建、标题栏和设置逻辑 | 调整窗口激活、实例标识、标题栏或应用级状态时 | 本轮核心改动点在 `src/app/startup.rs` |
| `src/backend/` | 本地/SSH 后端与 PTY/SSH 输入输出转发 | 需要确认是否存在后端跨进程输入共享时 | 本轮只读 `src/backend/local.rs` 排除后端串线 |
| `src/terminal/` | 终端 tab、后端命令与输入分发 | 需要确认单进程内 backend 共享语义时 | 本轮只读 `src/terminal/mod.rs` 排除跨进程共享 |
| `docs/` | 开发说明与跟踪记录 | 修改 dev-reload 行为或记录实施过程时 | `docs/development*.md` 需同步说明 macOS 开发 bundle 行为 |
| `Cargo.toml` / `.cargo/config.toml` | Cargo 入口与别名 | 确认 `cargo dev-reload` 指向的真实命令时 | 当前 `cargo dev-reload` 映射到 `run --example dev_reload --` |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `examples/dev_reload.rs` | restart-based dev runner | `DevReload::start_app`，`binary_path`，`prepare_macos_app_bundle` | 修改开发实例启动方式、环境变量注入、日志与 macOS bundle 隔离 |
| `src/app/startup.rs` | 应用启动与窗口创建 | `open_main_window`，`current_instance_kind`，`current_window_app_id`，`should_force_app_activation` | 修正窗口 app id、前台激活与开发实例标题 |
| `src/main.rs` | GPUI 应用初始化入口 | `main`，`on_reopen` | 确认应用没有单实例 IPC 或额外 reopen 路由 |
| `src/backend/local.rs` | 本地 PTY 后端 | `spawn_local_terminal` | 排除不同进程之间共享同一 PTY writer |
| `src/terminal/mod.rs` | 终端 tab 和 backend 抽象 | `BackendTx`，`TerminalTab::set_backend` | 确认 backend 共享只发生在单进程内部 |
| `docs/development.md` / `docs/development.en.md` | 开发命令说明 | `开发期自动重载` / `Restart-Based Dev Reload` | 同步说明 macOS 下 dev-reload 的独立开发 bundle 行为 |

## 常用定位

- `rg -n 'dev-reload|dev_reload|start_app|binary_path|launch_path' examples src docs`
- `rg -n 'activate\\(|activate_window|makeKeyAndOrderFront|bundleIdentifier|app_id' src ~/.cargo/git/checkouts/zed-a70e2ad075855582/24c5b37/crates/gpui*`
- `cargo check --example dev_reload`
- `cargo check`

## 忽略与未索引

- `assets/` 未索引：本轮不改图标资源本身，只复用已有 `.icns`
- `src/backend/ssh.rs` 未索引：本轮问题与 SSH/X11 无关
- `target/` 未索引：属于构建产物，但运行时会在其下生成临时开发 bundle

## 刷新规则

- 刷新触发：`cargo dev-reload` 行为、应用启动路径、窗口激活、macOS bundle 身份、文档说明或本轮任务范围发生变化时刷新
- 最近依据：`examples/dev_reload.rs`，`src/app/startup.rs`，`src/main.rs`，`src/backend/local.rs`，`src/terminal/mod.rs`，`docs/development.md`，`docs/development.en.md` 的实读结果

## 最后更新时间

- 2026-07-07 11:00 CST
