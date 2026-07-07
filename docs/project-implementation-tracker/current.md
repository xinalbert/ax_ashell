# 当前项目实施记录

## 当前目标

- 目标：修复 macOS 上 release 版与 `cargo dev-reload` 开发版同时运行时，开发窗口输入落到 release 实例的问题，并恢复 dev-reload 开发实例的独立 App 启动链路
- 交付物：独立的 dev-reload 实例标记、macOS 开发 app bundle 启动路径、开发实例前台激活修复、更新后的开发文档与实施跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`examples/dev_reload.rs`，`src/app/startup.rs`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：`src/backend/` 终端后端协议、release 打包脚本、X11 forwarding、真实多平台 GUI 联调

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位输入串到另一实例的根因，确认是否为后端跨进程路由 | 源码检查 | 结论是 macOS 应用身份 / 前台激活问题，不是终端后端共享 |
| P2 | completed | 为 `dev-reload` 注入独立实例标记并在 macOS 下走独立开发 bundle | `cargo check --example dev_reload` | 开发实例通过环境变量与独立 bundle id 区分 |
| P3 | completed | 修正窗口打开时的前台激活与实例可见标识 | `cargo check` | 开发实例打开时显式 `cx.activate(true)`，窗口标题区分为 `ax_ashell [dev]` |
| P4 | completed | 同步开发文档与跟踪文档 | tracking docs 校验 | 文档说明 macOS 下 dev-reload 会使用独立开发 app bundle |
| P5 | completed | 修复 macOS `open` 启动链路，消除 `-10810` 并恢复独立实例隔离 | `cargo test --example dev_reload`，`cargo run --example dev_reload`，用户实机确认 | 最终方案为独立 bundle id + `LSEnvironment` + 延后一帧执行窗口激活 |

## 已完成

- 检查 `examples/dev_reload.rs`、`src/app/startup.rs`、`src/backend/local.rs`、`src/terminal/mod.rs` 和 GPUI macOS 平台代码，排除“两个进程共享同一终端后端”的可能
- 确认 GPUI 在 macOS 上窗口级 `app_id` 为空实现，实际应用身份取决于 `NSBundle.mainBundle().bundleIdentifier`
- 在 `examples/dev_reload.rs` 中为开发实例注入 `AX_ASHELL_INSTANCE_KIND=dev-reload` 与独立 `AX_ASHELL_APP_ID`
- 在 macOS 下为 `cargo dev-reload` 准备独立的 `target/<profile>/ax_ashell-dev.app` 启动载体，避免和 release `.app` 共用应用身份
- 将 `dev-reload` 的 macOS 启动命令收敛为 `open -n -a <bundle.app>`，移除会触发 LaunchServices `-10810` 的 `open --env/--stdout/--stderr` 参数组合
- 将实例环境变量落入 dev bundle 的 `Info.plist:LSEnvironment`，保持真正的 `CFBundleExecutable` 仍是 `ax_ashell`，避免破坏 macOS 应用身份隔离
- 在 `src/app/startup.rs` 中基于实例标记设置窗口 `app_id`、区分开发窗口标题，并把 `activate/focus` 延后一帧执行，确保开发实例真正拿到前台输入焦点
- 更新中英文开发文档，说明 macOS 下 dev-reload 会通过独立开发 app bundle 启动

## 验证

- 已完成：`rustfmt --edition 2024 examples/dev_reload.rs src/app/startup.rs`；`cargo check --example dev_reload`；`cargo check`；`cargo test --example dev_reload`；`cargo run --example dev_reload` 启动后不再出现 `_LSOpenURLsWithCompletionHandler ... error -10810`；用户实机确认 dev-reload 与 release 实例现已独立隔离
- 未完成：无

## 风险与阻塞

- 当前修复依赖 macOS 下通过独立 bundle id 隔离开发实例；若用户绕过 `dev-reload` 直接手工运行裸 debug 二进制，仍可能只依赖前台激活逻辑而不是 bundle 隔离
- 暂无已知阻塞；若后续再改 `dev-reload` macOS 启动链路，需要同时回归验证 `_LSOpenURLsWithCompletionHandler`、Dock 图标和前台输入焦点

## 下一步

- 若后续继续调整 `dev-reload`，优先保持独立 bundle id、`LSEnvironment` 和延后一帧执行窗口激活这三个约束

## 最后更新时间

- 2026-07-07 11:00 CST
