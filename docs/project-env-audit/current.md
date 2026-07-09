# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用；本轮目标是打开或切回 SFTP 页面时，让远端路径默认同步到对应 SSH shell 的当前工作目录

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 默认定位 shell 工作目录”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 运行时，`russh` SSH 后端
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`
- 本轮代码入口：`src/terminal.rs`，`src/backend/ssh.rs`，`src/backend/local.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace/workspace.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/main.rs`，`src/app.rs`，`src/terminal.rs`，`src/backend/ssh.rs`，`src/backend/local.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace/workspace.rs`，`docs/project-implementation-tracker/project-map.md`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal.rs src/backend/ssh.rs src/backend/local.rs src/app/workspace/workspace.rs src/app/lifecycle/event_loop.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 运行 `cargo check --all-targets` 和 `cargo test --all`
- 外部依赖：无新增运行依赖；完整行为验证需要真实 SSH/SFTP 连接和远端 shell integration 输出
- 工具可用性：本机可执行 `rustfmt`、`cargo check`、`cargo test` 与 tracking docs validator；`cargo fmt` 子命令未安装
- 证据文件：`Cargo.toml`，`.github/workflows/release.yml`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，30 个测试全部通过；tracking docs validator 通过

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变，但 `current.md` 语境从 SFTP 传输交互修正切换到 SFTP 打开时同步远端 shell 工作目录；验证重点扩展到终端 OSC 捕获、SSH 后端 exec fallback 和 workspace 页面切换事件
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目边界清晰，运行环境稳定，本轮不依赖新增依赖；用户明确要求参考 VS Code 捕获方法，已将实现收敛为捕获 shell integration OSC 序列，并用独立 SSH exec 查询作兜底
- 开工前动作：已复查 `src/terminal.rs`、`src/backend/ssh.rs`、`src/backend/local.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace/workspace.rs` 与现有 tracking 文档
- 开工前动作：已确认不向用户交互 shell 注入可见 `pwd` 命令；优先解析 VS Code / iTerm2 / OSC 7 CWD escape sequence
- 完成后动作：已执行 `cargo check`、`cargo test` 和 tracking docs validator；GUI / 真实 SSH/SFTP 手工验证仍需在实际连接中确认
