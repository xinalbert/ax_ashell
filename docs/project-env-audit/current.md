# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是调整终端输入编码，让系统常见文本导航快捷键在终端中按平台习惯工作

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端原生文本导航快捷键”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`alacritty_terminal` 终端模型，`tokio` 运行时，`russh` SSH 后端
- 版本约束：`rust-version = 1.88.0`，edition `2024`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/terminal.rs`，`src/app/actions/terminal.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/terminal.rs`，`src/app/actions/terminal.rs`，`.github/workflows/ci.yml`，`AGENTS.md`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo check`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal.rs`，聚焦 `cargo test --quiet terminal::tests::`，`cargo check`，`cargo test --quiet`，`git diff --check`，tracking docs validator，均已通过
- 外部依赖：用户明确要求检索；本轮已使用官方资料确认 macOS 文本导航、Readline 控制序列和 xterm modified cursor 规则
- 工具可用性：本机已成功执行 `cargo`、`rustfmt`、`git diff --check` 与 tracking docs validator
- 证据文件：`Cargo.toml`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/research.md`，`AGENTS.md`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变；本轮范围从 Settings 信息架构整理切换为终端按键编码，需要在终端输入层增加平台文本导航映射并补测试
- 受影响文件：`src/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链和依赖策略未变；终端按键链路已定位为 `src/app/actions/terminal.rs` 调用 `src/terminal.rs::encode_key()`；本轮可在不改配置 schema、不改 PTY 后端的前提下实现
- 开工前动作：已读取 `AGENTS.md`、环境记忆、项目地图、终端输入代码和外部官方资料；本轮不使用多 agent
- 完成后动作：Rust 格式化、聚焦测试、编译检查、完整测试、空白检查和 tracking docs validator 已通过；GUI 实际按键体验仍需后续手工确认
