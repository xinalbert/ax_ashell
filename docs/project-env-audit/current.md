# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：双列 SFTP 面板顶部工具区紧凑化修复已完成；运行环境和依赖版本事实未变

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“双列 SFTP 面板顶部工具区紧凑化”任务的当前态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`locales/en.yml`，`locales/zh-CN.yml`
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`src/sftp/mod.rs`，`locales/en.yml`，`locales/zh-CN.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/app/ui.rs`，`cargo check`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不依赖联网、外部服务或远程 SSH 服务器；验证边界集中在本机 Rust 工具链、SFTP 前端布局与本地文件浏览逻辑、tracking docs contract
- 工具可用性：本机无 `cargo fmt` 子命令；本轮直接使用 `rustfmt` 二进制完成格式化
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：否
- 变化摘要：运行环境和依赖版本未变；仅将 current 语境切换到双列 SFTP 面板顶部工具区紧凑化并补充本机 `rustfmt` 验证路径
- 受影响文件：`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工，且本轮顶部工具区紧凑化修复已完成
- 原因：任务边界明确，且主要集中在前端状态和布局层，不需要依赖变更或外部环境
- 开工前动作：已根据截图复查 `src/app/ui.rs` 中远端 / 本地顶部工具区结构；已确认问题集中在前端布局层，不涉及远端后端命令
