# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮将实际生效的应用级快捷键统一纳入 Settings 的 `Key Bindings` 配置页。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取并刷新为“快捷键设置完整化”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = 1.88.0`、edition `2024`；本机已记录可用 `rustc`/`cargo` 高于最低版本。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`
- 本轮代码入口：`src/app/input/keybinding_recorder.rs`，`src/app/actions/terminal.rs`，`src/app/views/layout.rs`，`src/main.rs`，`src/app/dialogs/settings/keybindings.rs`。
- 依赖策略：复用现有 GPUI `KeyBinding`、action 和 settings 录制逻辑，不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：已执行相关 `rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：无；静态与单元测试不需要真实 SSH/SFTP 服务，真实 GUI 快捷键录制与触发仍需手工确认。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`，`src/app/input/keybinding_recorder.rs`，`src/app/actions/terminal.rs`，`src/app/views/layout.rs`，`src/main.rs`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：项目语言、依赖、工具链、CI 和测试入口不变；当前任务只涉及应用级快捷键注册表、Settings 展示和终端内旧硬编码快捷键接入。
- 受影响文件：快捷键注册/设置页、终端 key down 处理、全局 action 监听、用户文档与跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务、验证范围和风险记录已切换。

## 开工判定

- 状态：允许开工
- 原因：本轮修复可限制在现有 keybinding/action/settings 模块内，不需要新增依赖、修改配置 schema 或接触外部服务。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图和快捷键/Settings/terminal key handling 相关源码；确认无需联网和多 agent。
- 完成后动作：已完成格式化、编译、94 项完整测试、空白检查和 tracking validator；真实 GUI 快捷键录制与终端触发作为手工验证项保留。

## 最后确认时间

- 2026-07-11 10:22 +0800
