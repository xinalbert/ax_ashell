# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮在既有行布局缓存中保存 `ShapedLine`，以移除持续输出时每帧的文本布局缓存查找，不修改依赖或构建配置。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在，已刷新为本轮终端文本绘制优化范围。
- changes.md：存在，待追加本轮验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、`portable-pty 0.9`、Tokio、`tracing`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc` / `cargo` 可用。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/backend/local.rs`。
- 本轮代码入口：`src/terminal/element.rs`，必要时 `src/terminal/tab.rs`；稳定 element state 持有行级 `RowLayout`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024`、`cargo check`、focused tests、`cargo test --quiet`、`cargo build`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux 和 macOS 执行 release build。
- 本轮验证结果：已完成 16:35 持续输出 sample、锁定 GPUI `ShapedLine` / `with_element_state` API 和现有 terminal layout cache 审查；待运行 Rust 验证与新的同负载 sample。
- 外部依赖：无新增；无需联网。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/terminal/element.rs`，`src/terminal/tab.rs`，`docs/project-implementation-tracker/research.md`，16:35 sample。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：16:35 sample 证明旧 snapshot、高亮和 row layout 构建热点已降低；剩余主要为每帧 `shape_line_by_hash` cache lookup 与 `ShapedLine::paint` / quad 提交。本轮缓存 `ShapedLine` 对象以去除前者，保留后者和现有 16ms 刷新语义。Rust 版本、依赖、CI 和测试入口均不变；工作树已有大量无关未提交改动。
- 受影响文件：`src/terminal/element.rs`，必要时 `src/terminal/tab.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；收口后记录实现、验证和 sample 对比。

## 开工判定

- 状态：允许开工。
- 原因：已读取环境/实施记录、项目地图、仓库指令和当前 dirty worktree；不需要新增依赖、联网或修改 manifest/lock。
- 完成动作：单线程实施；完成后执行 Rust 全量验证和同负载持续输出采样。

## 最后确认时间

- 2026-07-13 16:47 +0800
