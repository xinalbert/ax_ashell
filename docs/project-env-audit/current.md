# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮把 Custom 主题字体亮度迁移为 Theme 设置中的全局 UI / Terminal 亮度。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“全局字体亮度拆分”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机可用 Rust 工具链已在既有记录中确认高于最低版本。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`
- 本轮代码入口：`src/config/model.rs`，`src/config/store.rs`，`src/app/theme.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/custom.rs`，`src/terminal/element.rs`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖，不直接修改 cargo 缓存中的外部 `gpui-component` 源码。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：已执行相关 `rustfmt --edition 2024`、配置/theme 聚焦测试、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：本轮不需要联网；真实 UI/terminal 亮度视觉效果仍需后续 GUI 手工验证。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/config/model.rs`，`src/config/store.rs`，`src/app/state/appearance.rs`，`src/app/lifecycle/init.rs`，`src/app/theme.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/custom.rs`，`src/terminal/element.rs`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：项目语言、依赖管理和 CI 入口不变；旧亮度配置已从 Custom theme draft 和 legacy `custom_font_brightness` 迁移为全局 UI / Terminal 亮度配置，并接入 Theme 设置页和运行时应用。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app/` 设置与主题文件，`src/terminal/element.rs`，`locales/` 和跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务、范围、验证命令和 GUI 验证边界已切换。

## 开工判定

- 状态：允许开工
- 原因：当前问题可在仓库内配置模型、Settings UI、主题 token 后处理和 terminal 渲染读取点内完成；无需外部源码或依赖变更。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、相关 skills 和亮度/主题相关源码；确认不需要多 agent。
- 完成后动作：受影响 Rust 文件格式化、编译、定向测试、完整单测、空白检查和 tracking docs validator 已完成；真实亮度视觉效果保留手工确认。

## 最后确认时间

- 2026-07-12 07:58 +0800
