# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮统一 AxShell 下拉、菜单行和长列表的 fast hover 路径，并补齐 Theme Editor Base Theme 下拉的 lazy candidate builder。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“全局快速 hover 统一 + Base Theme lazy 下拉”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机可用 Rust 工具链已在既有记录中确认高于最低版本。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`
- 本轮代码入口：`src/app/hover.rs`，`src/app/dialogs/settings/fast_menu.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/ssh.rs`，`src/app/dialogs/selector.rs`，`src/app/views/sidebar.rs`，`src/app/views/layout.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖，不直接修改 cargo 缓存中的外部 `gpui-component` / `gpui` 源码。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：已执行受影响 Rust 文件 `rustfmt --edition 2024`、`cargo check`、完整 `cargo test --quiet`、fast hover 审计搜索；提交前继续执行 `git diff --check` 和 tracking docs validator。
- 外部依赖：本轮不需要联网；真实 GUI hover 手感仍需手工确认。
- 证据文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`src/app/hover.rs`，`src/app/dialogs/settings/fast_menu.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/ssh.rs`，`src/app/dialogs/selector.rs`，`src/app/views/sidebar.rs`，`src/app/views/layout.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：项目语言、依赖管理、CI 入口和测试命令不变；新增项目本地 fast hover skill 与共享 `src/app/hover.rs`，Settings 下拉复用 lazy/virtual fast menu，selector/sidebar/transfer 等长列表改为 `uniform_list` 可见行渲染，自绘 SFTP / saved session 右键菜单复用 fast hover。
- 受影响文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/`，`src/app/hover.rs`，`src/app/dialogs/`，`src/app/views/`，`src/app/actions/`，`src/app.rs`，`src/app/lifecycle/init.rs`，跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务、范围和验证命令已切换。

## 开工判定

- 状态：允许开工
- 原因：当前问题可在 app 层共享 hover、Settings fast menu、selector/sidebar/SFTP 渲染路径内完成；无需外部源码、依赖变更或联网资料。
- 开工前动作：已读取 `AGENTS.md`、项目本地 fast hover skill、环境记录、实施记录、项目地图和相关源码；确认不需要多 agent。
- 完成后动作：格式化、`cargo check`、完整 `cargo test --quiet`、fast hover 审计、空白检查和 tracking docs validator。

## 最后确认时间

- 2026-07-12 10:05 +0800
