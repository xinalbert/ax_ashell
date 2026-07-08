# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用的真实功能改动；本轮目标是修复 SFTP 目录读取超时或报错后远端列表继续点击失效的问题。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 目录失败状态恢复”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n / alacritty_terminal
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/sftp/ops.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`
- 渲染依据：SFTP 远端列表点击从 `src/app/ui/sftp_panel.rs` 调用 `select_sftp_entry()`，目录导航由 `navigate_sftp()` 发送 `SftpCommand::ListDir`，成功结果由 `BackendEvent::SftpEntries` 回写 `SftpUiState`
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/sftp/ops.rs`，`src/sftp/mod.rs`，`src/app/event_loop.rs`，`src/app/ui/sftp_panel.rs`，`src/terminal/mod.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/sftp/ops.rs src/sftp/mod.rs src/app/event_loop.rs src/terminal/mod.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；真实目录超时复现仍需要 GUI 和远端 SFTP 环境
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/sftp/ops.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 对“超时目录失败后仍可点击旧目录内容”的行为未手工验证

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从“SFTP 列表排序和传输面板”继续收敛到“SFTP 目录导航失败后的 UI 状态恢复”；运行环境不变，不新增依赖
- 受影响文件：`src/sftp/ops.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：问题可通过调整 SFTP UI 状态提交时机修复，不需要更换 SFTP 后端或新增依赖
- 开工前动作：已复查远端列表点击、目录导航命令、`SftpEntries` / `SftpStatus` 事件回写路径；已确认不需要联网、不使用多 agent
