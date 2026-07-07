# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮进入真实实现，已完成 `project-env-audit`，允许继续施工

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为 macOS release 实例与 `cargo dev-reload` 开发实例输入焦点冲突修复任务的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`.cargo/config.toml`，`examples/dev_reload.rs`，`src/main.rs`，`src/app/startup.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 examples/dev_reload.rs src/app/startup.rs`，`cargo check --example dev_reload`，`cargo check`，`cargo test --example dev_reload`，`cargo run --example dev_reload`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：macOS 上若同时运行 release `.app` 与 `cargo dev-reload`，需要依赖 AppKit / NSBundle 的应用身份和前台激活行为；`dev-reload` 当前通过独立 dev bundle、`Info.plist` 中的 `LSEnvironment` 和 `open -n -a <bundle.app>` 启动链路保持独立实例身份，并避免 `open --env/--stdout/--stderr` 触发 LaunchServices `-10810`；Linux / Windows 不依赖本轮新增的 macOS bundle 隔离逻辑
- 证据文件：`Cargo.toml`，`examples/dev_reload.rs`，`src/app/startup.rs`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变，但当前任务已从 X11 local X server 适配切换为 `cargo dev-reload` 与 release `.app` 并行运行时的 macOS 应用身份/输入焦点修复；验证过程中进一步确认 `open --env/--stdout/--stderr` 会触发 LaunchServices `-10810`，最终 current 记录收敛为“dev bundle 通过独立 bundle id + `LSEnvironment` 注入实例标记，`open` 只负责启动 `.app`，窗口激活延后一帧执行”
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：任务边界明确；本轮只修改开发启动与窗口激活逻辑，不改变依赖版本或终端后端协议
- 开工前动作：刷新 `docs/project-implementation-tracker/` 当前态与项目地图，再修改 `examples/dev_reload.rs`、`src/app/startup.rs` 和开发文档并运行 `cargo check`
