# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用的真实功能改动；本轮目标是让活动终端在选区存在或中文 IME 组合中时延迟应用流式输出，避免 Codex `Working` 文本打断当前交互。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端交互期延迟应用输出”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n / alacritty_terminal
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/terminal/input.rs`
- 渲染依据：终端 snapshot 与绘制由 `src/terminal/mod.rs` 和 `src/terminal/element.rs` 驱动；后台输出、刷新时机与 IME 组合态保留由 `src/app/event_loop.rs` 和 `src/terminal/input.rs` 协同处理
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/terminal/mod.rs`，`src/terminal/element.rs`，`src/terminal/input.rs`，`src/app/event_loop.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal/mod.rs src/terminal/input.rs src/app/event_loop.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；终端选区与中文 IME 稳定性仍需要 GUI 手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/terminal/mod.rs`，`src/terminal/input.rs`，`src/app/event_loop.rs`
- 本轮验证结果：`rustfmt --edition 2024 src/terminal/mod.rs src/terminal/input.rs src/app/event_loop.rs src/session/pane.rs src/session/mod.rs` 通过；后续对 `src/terminal/input.rs` 的选区回归修正也已单独通过 `rustfmt --edition 2024 src/terminal/input.rs`、`cargo check` 和 `cargo test`；总计 18 个测试全部通过；tracking docs 校验待执行；GUI 终端选区与中文 IME 稳定性未手工验证

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从“减少无效刷新”收窄到“交互期间延迟应用输出”；运行环境不变，但实现入口改为事件泵、终端输入和终端 tab 状态
- 受影响文件：`src/terminal/mod.rs`，`src/terminal/input.rs`，`src/app/event_loop.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：本轮实现仍基于现有终端 tab、事件泵和 IME 标记文本路径增加缓冲与冲刷，不需要新增依赖或改绘制模型
- 开工前动作：已复查终端输出事件、IME 标记文本、选区状态与 render snapshot 路径；已确认不需要联网、不使用多 agent
