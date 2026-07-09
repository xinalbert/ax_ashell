# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是整理设置页信息架构，把过度拥挤的 General 页拆分为更清晰的设置子页面，并保持配置格式与运行时保存行为不变

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“Settings 信息架构整理”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 运行时，`russh` SSH 后端
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`，不改配置文件 schema
- 证据文件：`Cargo.toml`，`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/fonts.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`AGENTS.md`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo check`
- 当前实施验证命令：`rustfmt --edition 2024 <changed-rust-files>`，`cargo check`，`cargo test --quiet`，`git diff --check`，tracking docs validator，均已通过
- 外部依赖：本轮不需要联网检索；GUI 设置页最终交互仍需人工验证
- 工具可用性：本机已成功执行 `cargo`、`rustfmt`、`git diff --check` 与 tracking docs validator
- 证据文件：`Cargo.toml`，`docs/project-implementation-tracker/project-map.md`，`AGENTS.md`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变；本轮范围从仓库级 agent 指令切换为设置页 UI 信息架构整理，需要修改 Settings 子模块、本地化文案和跟踪记录
- 受影响文件：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链和依赖策略未变；设置页入口、现有 General 页字段和本地化键已定位；可在不改配置 schema 的前提下按现代 Rust 模块布局拆分页面
- 开工前动作：已读取 `AGENTS.md`、环境记忆、项目地图和设置页现状；本轮不联网、不使用多 agent
- 完成后动作：Rust 格式化、编译/测试、`git diff --check` 和 tracking docs validator 已通过；GUI 设置页实际交互需后续人工确认
