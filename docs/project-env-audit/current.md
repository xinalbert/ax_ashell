# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮调整终端 URL/路径的快捷键激活视觉提示，不修改依赖或构建配置。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新为终端链接激活视觉范围。
- changes.md：已追加本轮验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、`portable-pty 0.9`、Tokio、`tracing`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc` / `cargo` 可用。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/sftp.rs`，`src/sftp/worker/runtime.rs`。
- 本轮代码入口：`src/app/terminal.rs`，`src/app/actions/terminal.rs`，`src/app/views/terminal_panel.rs`，`src/terminal/element.rs`；命中 URL/路径后，仅在 Command（macOS）或 Ctrl（其他平台）按下时显示可激活视觉。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024 <changed-rust-files>`、终端链接修饰键聚焦测试、`cargo check`、`cargo test --quiet`、终端 hover 静态审计、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux 和 macOS 执行 release build。
- 本轮验证结果：修饰键门控、即时 modifier-change 刷新和行布局缓存收敛已完成；`rustfmt`、聚焦测试、`cargo check`、完整测试、hover 静态审计和空白检查通过。
- 外部依赖：无新增；无需联网。
- 证据文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`Cargo.toml`，`src/app/terminal.rs`，`src/app/actions/terminal.rs`，`src/app/views/terminal_panel.rs`，`src/terminal/element.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：终端 URL/路径命中后，仅在 Command（macOS）或 Ctrl（其他平台）按下时显示下划线和手型指针；普通 hover 保持原终端文本外观。GPUI modifier-change 事件使按下或松开快捷键时无需移动鼠标即可切换视觉，链接 hover 也不再使文本行布局缓存失效。
- 受影响文件：`src/app.rs`，`src/app/terminal.rs`，`src/app/actions/terminal.rs`，`src/app/views/terminal_panel.rs`，`src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：已更新。真实 GUI 下按下/松开修饰键后的视觉切换仍需手工验证。

## 开工判定

- 状态：允许开工。
- 原因：本机工具链满足仓库约束；实现局限于既有 GPUI 修饰键事件、终端 hover 状态和绘制缓存，不需要联网、新依赖或 manifest/lock 修改。
- 完成动作：单线程实施完成；已执行格式化、链接视觉聚焦测试、`cargo check`、完整测试、hover 静态审计、空白检查和 tracking docs validator。

## 最后确认时间

- 2026-07-14 14:34 +0800
