# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮新增已保存 SSH 会话与分组的无密钥、无密码导出和导入，并在原生菜单栏和 sidebar 右键菜单提供入口。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“saved SSH share import/export”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1` 可用。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/input/app_menu.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`
- 本轮代码入口：`src/app/input/app_menu.rs`，`src/app/actions/saved_sessions.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`，`src/app.rs`，`src/session.rs`，`locales/`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖；复用现有 `serde_json`、`rfd` 和 GPUI action/menu 机制。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：已执行 `rustfmt --edition 2024` 覆盖受影响 Rust 文件，新增聚焦单元测试，`cargo check`，fast hover/context 静态审计，完整 `cargo test --quiet`，`git diff --check`，tracking docs validator。
- 外部依赖：无；不需要联网。真实 GUI 中菜单栏导入/导出、sidebar group/session 右键导出文件选择和导入后列表刷新仍需手工确认。
- 证据文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`Cargo.toml`，`Cargo.lock`，`src/app/input/app_menu.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/sftp.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/session.rs`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：项目语言、依赖管理、CI 入口和测试命令不变；本轮在 app action 层增加无凭据 share 文件导入/导出，并在菜单栏与 sidebar 右键菜单接入导出入口，不新增依赖、不改变 SSH backend。
- 受影响文件：`src/app/input/app_menu.rs`，`src/app/input/keybinding_recorder.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/sftp.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/main.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务和验证范围已切换。

## 开工判定

- 状态：允许开工
- 原因：保存会话和分组当前都由 `Session` 序列化承载，分组是 `group_name` 字段；导出可构建专用 share payload 并剔除密码、私钥、passphrase 和代理密码，导入可复用 `ConfigStore` 的 sessions 读写路径。
- 开工前动作：已读取 `AGENTS.md`、项目本地 fast hover skill、环境记录、实施记录、项目地图、manifest 和相关源码；确认不需要联网或多 agent。
- 完成后动作：Rust 格式化、聚焦测试、`cargo check`、fast hover/context 静态审计、完整测试、空白检查和 tracking docs validator 已完成；真实 GUI 菜单栏和 sidebar 右键文件选择仍需手工确认。

## 最后确认时间

- 2026-07-12 12:02 +0800
