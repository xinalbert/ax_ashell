# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮进入真实实现，已完成 `project-env-audit`，允许继续施工

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为终端关键词完整匹配收敛任务的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`.cargo/config.toml`，`src/terminal/element.rs`，`src/terminal/highlight.rs`，`src/app/search.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal/highlight.rs`，`cargo test keyword_highlight`，`cargo check`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不依赖联网、外部服务或平台专有运行时；验证边界主要是本机 Rust 工具链和终端关键词匹配逻辑。若需要 GUI 目视确认，则只涉及本机终端窗口内容显示，不改变后端协议
- 证据文件：`Cargo.toml`，`src/terminal/element.rs`，`src/terminal/highlight.rs`，`src/app/search.rs`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变，但当前任务已从“原生颜色避让”继续推进到“关键词完整匹配”；本轮实现集中在 `src/terminal/highlight.rs` 的 matcher，目标是减少短词和标识符内部的误报
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：任务边界明确；本轮只修改终端关键词 matcher 语义，不改变依赖版本、配置结构或后端协议
- 开工前动作：刷新 `docs/project-implementation-tracker/` 当前态与项目地图，再修改 `src/terminal/highlight.rs` 并运行 `rustfmt`、`cargo test`、`cargo check`
