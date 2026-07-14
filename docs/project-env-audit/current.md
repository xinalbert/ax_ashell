# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮为 SFTP 下载记录实现 WinSCP 式“批量任务 + 文件明细”，不改变 SFTP 协议、上传任务、依赖或 CI workflow。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已由 Windows CI tracing 编译修复范围刷新为本轮 SFTP 下载任务与文件明细范围。
- changes.md：存在；已追加本轮预检与完成验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`serde`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/events.rs`，`src/sftp/worker/runtime.rs`，`src/app/lifecycle/event_loop.rs`。
- 本轮代码入口：`src/sftp/model.rs`、`src/events.rs`、`src/sftp/transfer.rs`、`src/sftp/worker/runtime.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/actions/sftp.rs`、`src/app/dialogs/transfers.rs`、`src/app/views/sftp_panel/transfer_panel.rs`。
- 依赖策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024 <changed-rust-files>`、SFTP 传输模型/筛选聚焦测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu 和 macOS 执行 release build。
- 外部依赖：已完成 WinSCP 官方传输队列检索；不新增 crate。
- 证据文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`.github/workflows/ci.yml`，`Cargo.toml`，`src/sftp/model.rs`，`src/events.rs`，`src/sftp/transfer.rs`，`src/sftp/worker/runtime.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views/sftp_panel/transfer_panel.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：本轮目标切换为 SFTP 下载任务文件明细；现有 worker 对一批远端路径只发送一个 `TransferStarted`，目录递归内容复用该 ID。WinSCP 官方队列以任务为顶层，并为运行中的多文件任务显示当前文件和完整文件清单。
- 受影响文件：`src/sftp/model.rs`，`src/events.rs`，`src/sftp/transfer.rs`，`src/sftp/worker/runtime.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/transfers.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：已完成预检和验证更新。

## 开工判定

- 状态：允许开工。
- 原因：实现范围明确，现有 `Transfer` 已持久化且 event loop 是唯一 UI 状态写入点；可在下载路径追加兼容的文件明细事件，保持现有批量任务暂停、取消和历史管理语义。
- 开工前动作：已读取项目环境、实施记录、项目地图和 SFTP 列表规则；已完成 WinSCP 官方行为检索，不使用多 agent。

## 最后确认时间

- 2026-07-14 20:09 +0800
