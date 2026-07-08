# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust 项目的真实结构重构任务；本轮目标是按功能拆分超大源文件，并在拆分后保持现有行为与编译结果

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“大文件按功能拆分”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/app/`，`src/session/`，`src/sftp/`，`src/backend/`
- 依赖统一策略：根项目 `gpui` / `gpui_platform` / `menu` 保持 plain git source，通过 `Cargo.lock` 统一 pin 到单一 Zed 提交，避免和 `gpui-component` 形成双 source id
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/app/ui.rs`，`src/app/dialogs.rs`，`src/app/mod.rs`，`src/session/mod.rs`，`src/sftp/mod.rs`，`src/backend/ssh.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要新增外部服务或联网步骤；重点是本地模块拆分后的编译与测试回归
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 13 个 Rust 测试，可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/`，`src/session/`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从 `dev_reload` 平台行为修复切换到大文件结构拆分；验证入口同步切换回全仓 `cargo check` / `cargo test`
- 受影响文件：`src/app/`，`src/session/`，`src/sftp/`，`src/backend/ssh.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链、依赖锁定和基础测试环境都已就位；本轮主要做模块边界重排，可通过本地 `cargo check` / `cargo test` 做行为不变验证
- 开工前动作：已复查 `src/app/ui.rs`、`src/app/dialogs.rs`、`src/app/mod.rs`、`src/session/mod.rs`、`src/sftp/mod.rs`、`src/backend/ssh.rs` 与现有 tracking 记录；已确认主要风险来自职责混杂和状态集中，而不是工具链或依赖阻塞
