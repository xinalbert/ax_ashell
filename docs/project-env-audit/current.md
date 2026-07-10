# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是在已完成 SFTP 懒连接的基础上增加空闲断开，回收长时间无交互的 SFTP 连接

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 空闲断开”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`alacritty_terminal` 终端模型，`tokio` 运行时，`russh` SSH / SFTP 后端
- 版本约束：`rust-version = 1.88.0`，edition `2024`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`AGENTS.md`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`src/sftp.rs`，`src/terminal.rs`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo check`
- 当前实施验证命令：计划执行 `rustfmt --edition 2024 src/app.rs src/app/lifecycle/init.rs src/app/lifecycle/event_loop.rs src/app/actions/sftp.rs`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator
- 外部依赖：本轮只依赖本地源码事实，不需要联网
- 工具可用性：本机此前已成功执行 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；本轮待复验
- 证据文件：`Cargo.toml`，`docs/project-implementation-tracker/project-map.md`，`AGENTS.md`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变；本轮范围从“按需建立”切换到“空闲断开”，关键链路是 group 级 SFTP handle、app 层 `transfers` 活跃状态、事件泵周期检查以及 UI 可见性
- 受影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链和依赖策略未变；当前问题可限制在 app 层连接状态和事件泵，不需要修改 SSH 后端协议；`self.transfers` 已可判断进行中/暂停的传输，足以避开误断
- 开工前动作：已读取 `AGENTS.md`、环境记忆、项目地图、SFTP 懒连接实现、传输状态模型和远程编辑链路；本轮不联网，不使用多 agent
- 完成后动作：对改动文件执行 Rust 格式化、编译检查、完整测试、空白检查和 tracking docs validator；GUI 侧仍需手工确认“空闲断开后再次使用可自动重连”
