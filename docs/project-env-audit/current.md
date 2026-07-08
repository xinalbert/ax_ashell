# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用的真实功能改动；本轮目标是稳定终端中文 IME composition overlay，并把选区刷新保护从整段输出冻结改为只冻结选中行。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端 IME composition 锚点与选区行级冻结”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n / alacritty_terminal
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/app/workspace.rs`，`src/app/ui/layout.rs`，`src/app/ui/terminal_panel.rs`，`src/app/mod.rs`，`src/app/init.rs`，`src/session/mod.rs`，`src/session/pane.rs`
- 渲染依据：终端渲染由 `TerminalElement` 绘制 `RenderSnapshot`；IME 输入通过 GPUI `InputHandler` 回调进入 `set_terminal_marked_text()` / `commit_terminal_ime_text()`；候选框位置由 `terminal_ime_bounds_for_range()` 返回；选区行级冻结由 `TerminalFrozenSelection` 保存选中行 cells/text/highlights，渲染层只在对应行覆盖 frozen cells，其他行继续使用实时 snapshot
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/app/workspace.rs`，`src/app/ui/terminal_panel.rs`，`src/session/pane.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/app/mod.rs src/app/init.rs src/app/event_loop.rs src/app/ui/terminal_panel.rs src/app/ui/layout.rs src/app/workspace.rs src/session/mod.rs src/session/pane.rs src/terminal/input.rs src/terminal/element.rs src/terminal/mod.rs`，`cargo check`，`cargo test`，tracking docs validator
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；真实 IME 候选框稳定性仍需要 GUI 和系统输入法手工验证
- 工具可用性：本机 `rustc 1.96.1`、`cargo 1.96.1` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/app/event_loop.rs`，`src/app/workspace.rs`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，25 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 中文 IME 候选、高亮、持续输出刷新和选区行级冻结场景未手工验证

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从“终端中文 IME composition 锚点与高亮”扩展到“终端选区行级冻结”；运行环境不变，不新增依赖
- 受影响文件：`src/app/mod.rs`，`src/app/init.rs`，`src/app/event_loop.rs`，`src/app/workspace.rs`，`src/app/ui/layout.rs`，`src/app/ui/terminal_panel.rs`，`src/session/mod.rs`，`src/session/pane.rs`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：问题可在现有 GPUI `InputHandler`、终端 overlay 绘制层和渲染层 frozen row snapshot 内修复，不需要更换终端后端或新增依赖
- 开工前动作：已复查终端输入、IME marked text、候选框 bounds、渲染 overlay、backend output 事件分发、选区捕获和渲染覆盖路径；已确认不需要联网、不使用多 agent
