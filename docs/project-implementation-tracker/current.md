# 当前项目实施记录

## 当前目标

- 目标：把 Custom 主题里的字体亮度移到 Theme 设置的全局项，并拆分为界面文字亮度和终端文字亮度。
- 交付物：全局 UI / Terminal 亮度配置与即时应用；Custom Theme Editor 不再显示或保存亮度字段；旧配置中的亮度值保持兼容迁移。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/config/model.rs`，`src/config/store.rs`，`src/app/state/appearance.rs`，`src/app/lifecycle/init.rs`，`src/app/theme.rs`，`src/app/actions/session.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/custom.rs`，`src/terminal/element.rs`，`locales/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：主题 JSON 色值、外部 `gpui-component` 源码、依赖与 `Cargo.toml` / `Cargo.lock`、真实 GUI 手工验收。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 明确亮度字段现状和迁移边界 | `rg -n "font_brightness|custom_font_brightness"` 与相关源码复核 | 当前亮度只在 custom theme draft 中保存，终端渲染读取 custom-only 亮度 |
| P2 | completed | 全局 UI / Terminal 亮度配置字段、getter/setter 与旧配置兼容 | 配置单元测试、`cargo test --quiet config::store::theme_profile_tests` | 旧 `custom_font_brightness` 和 custom draft 中的非默认亮度迁移到 terminal 全局亮度 |
| P3 | completed | Settings Theme 页新增两个亮度控件，Custom 页移除亮度输入 | 源码复核、`cargo check` | 控件即时保存并刷新主题 / 终端 |
| P4 | completed | UI 前景色亮度和终端前景色亮度分别生效 | `cargo check`、聚焦测试、真实 GUI 手工边界说明 | UI 亮度调整 theme foreground 类 token；terminal 亮度调整终端前景色 |
| P5 | completed | 格式化、完整回归和文档校验 | `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking validator | 不新增依赖，不修改 manifest/lock |

## 已完成

- 已读取 `AGENTS.md`、环境记录、实施记录、项目地图和相关 skills。
- 已确认本轮不需要联网、不使用多 agent、不新增依赖。
- 已定位亮度相关字段、Custom 页输入生成、theme 应用链路和 terminal 渲染读取点。
- 已新增全局 `ui_font_brightness` / `terminal_font_brightness` 配置和运行时状态，旧 custom 亮度会迁移到 terminal 全局亮度。
- 已在 Appearance & Theme 页 Theme 组中新增两个亮度控件；Custom Theme Editor 不再渲染亮度输入。
- 已让 UI 亮度在主题应用后调整前景色类 token，terminal 渲染读取全局 terminal 亮度。
- 已执行相关 `rustfmt`、`cargo check`、配置/theme 聚焦测试、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。

## 验证

- 已完成：本轮范围和实现路径复核；相关 `rustfmt`；`cargo check`；`cargo test --quiet config::store::theme_profile_tests`；`cargo test --quiet config::store::font_brightness_settings_tests`；`cargo test --quiet import_theme_tests`；完整 `cargo test --quiet`；`git diff --check`；tracking docs validator。
- 未完成：真实 GUI 手工确认。

## 风险与阻塞

- 风险：UI 亮度属于主题 token 后处理，需限定在文字/前景色类字段，避免误改背景导致主题色调偏移。
- 无阻塞。

## 下一步

- 提交本轮主题设置修复。

## 最后更新时间

- 2026-07-12 07:58 +0800
