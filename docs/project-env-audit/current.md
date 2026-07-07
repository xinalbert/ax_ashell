# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮进入真实实现，已完成 `project-env-audit`，允许继续施工

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“折叠侧边栏也按组展示”的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`src/session/mod.rs`，`src/app/ui.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/session/mod.rs src/app/ui.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不依赖联网、外部服务或远程 SSH 服务器；验证边界主要是本机 Rust 工具链和 GPUI 左侧折叠栏渲染/点击链路。GUI 最终效果如需确认，仍需本机手工打开窗口查看
- 证据文件：`Cargo.toml`，`src/session/mod.rs`，`src/app/ui.rs`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变；当前任务已完成“补齐折叠侧栏也按组显示并支持点击展开”，实现集中在 `src/app/ui.rs` 的折叠态渲染与已有分组 helper 复用
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：任务边界明确；本轮只细化本地 UI 渲染和文档，不改变依赖版本、后端 SSH 协议或外部服务
- 开工前动作：已完成刷新 `docs/project-implementation-tracker/` 当前态与项目地图；已修改 `src/app/ui.rs` 并同步用户文档，已完成 `rustfmt`、`cargo check`、`cargo test` 与 tracking docs 校验；GUI 手工验证未执行
