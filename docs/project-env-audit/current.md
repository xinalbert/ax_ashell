# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮修复终端重连状态持续刷新时关键词 / URL 彩色高亮闪烁问题，不修改依赖或构建配置。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在，已刷新为本轮终端高亮闪烁修复范围。
- changes.md：存在，待追加本轮验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、`portable-pty 0.9`、Tokio、`tracing`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc` / `cargo` 可用。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/backend/local.rs`。
- 本轮代码入口：`src/terminal/tab.rs`，必要时 `src/terminal/highlight.rs` / `src/terminal/element.rs`；终端 snapshot 行复用和高亮延迟刷新共同决定闪烁行为。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024 src/terminal/tab.rs`、`cargo test --quiet unchanged_rows_keep_deferred_highlights_across_full_damage`、`cargo check`、必要时完整 `cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux 和 macOS 执行 release build。
- 本轮验证结果：已完成截图现象、外部 Codex 重连输出来源、`TerminalTab` 高亮延迟刷新和 `build_visible_rows` 行复用路径审查；`rustfmt`、聚焦测试、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking validator 均通过。
- 外部依赖：无新增；无需联网。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/terminal/tab.rs`，`src/terminal/element.rs`，`src/terminal/highlight.rs`，用户截图。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：截图中的 `Reconnecting...` 和 `https://aixj.vip/responses` 来自外部 Codex 流式请求输出；该状态持续刷新会触发终端 damage。当前高亮为了性能最多每 125ms 重算一次，若 full damage 导致未变行也重建，新旧 `RenderRow` 指针不匹配，延迟窗口内彩色高亮会短暂消失。Rust 版本、依赖、CI 和测试入口均不变。
- 受影响文件：`src/terminal/tab.rs`，必要时 `src/terminal/highlight.rs` / `src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：已更新；真实 GUI 中外部请求重连场景仍需手工观察。

## 开工判定

- 状态：已完成。
- 原因：本机工具链满足仓库约束；实现只触及终端 snapshot 行复用，不需要新增依赖、联网或修改 manifest/lock。
- 完成动作：单线程实施；已完成聚焦测试、`cargo check`、完整测试、空白检查和 tracking docs validator。

## 最后确认时间

- 2026-07-13 23:09 +0800
