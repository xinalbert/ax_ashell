# 当前项目实施记录

## 当前目标

- 目标：在 saved SSH 右键菜单中增加“Open SFTP”入口，支持只打开 SFTP 页面而不创建 SSH shell tab，并确保没有可用 SSH/SFTP 上下文时 SFTP 快捷键不生效。
- 交付物：SFTP-only group session 状态、saved session 右键菜单入口、SFTP 建连/session 解析接线、快捷键防护、必要文案、回归测试和验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/workspace.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/session.rs`，`src/app/actions/sftp.rs`，`src/app/views/layout.rs`，`src/app/views/tab_bar.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`locales/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 `Cargo.toml` / `Cargo.lock`、新增依赖、改变 SFTP 协议/认证流程、实现独立 `sftp://` URI 解析、重构 saved session 列表结构。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 确认 saved SSH 右键、SFTP group、快捷键和建连链路 | 源码审查 | SFTP 目前从 group 内 SSH tab 取 session，saved 右键未提供 SFTP-only 入口 |
| P2 | completed | SFTP-only group session 状态、右键入口和快捷键防护 | `rustfmt`、聚焦测试、`cargo check` | 不创建 SSH shell tab；SFTP-only 页不响应 SFTP 快捷键 |
| P3 | completed | 项目地图、文档记录和最终验证收口 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI 右键菜单和真实 SFTP 建连需手工确认 |

## 已完成

- 已读取环境记录、实施记录、项目地图、项目本地 fast hover skill 和 saved session / SFTP / workspace / terminal 快捷键相关源码。
- 已确认 saved SSH 右键菜单当前只有复制、导出、克隆、编辑和删除。
- 已确认 SFTP worker 目前通过 group 内 SSH tab 的 `Session` 创建，因此 SFTP-only group 需要保存独立 session。
- 已确认 `ToggleSftpZoom` / `OpenTransfers` 已有 active group 与 `group.sftp` 检查，但需要保持没有 SFTP/SSH 上下文时不能误开。
- 已实现 `TabGroup::sftp_session`、saved SSH 右键 Open SFTP、SFTP-only group 建连、workspace tab 过滤空 terminal、关闭 SFTP-only 页时释放/隐藏 group、SFTP-only 页禁用 SFTP 快捷键，以及侧栏标题显示 `sftp / 会话名`。
- 已刷新 `project-map.md` 中 SFTP-only group、右键入口和路由定位说明。

## 验证

- 已完成：源码链路审查、施工计划刷新、`rustfmt`、hover/context 静态审计、`cargo check`、`cargo test --quiet workspace -- --nocapture`、`cargo test --quiet saved_sessions -- --nocapture`、`cargo test --quiet local_sftp -- --nocapture`、`cargo test --quiet`、`git diff --check`、tracking validator；标题 fallback 后已重跑 `cargo check` 和 `cargo test --quiet`。
- 待完成：无。
- 未完成：真实 GUI 中 saved SSH 右键 Open SFTP、不创建 shell tab、快捷键无 SSH/SFTP 上下文不可用需手工确认。

## 风险与阻塞

- 风险：SFTP-only group 没有 terminal pane，workspace tab、关闭逻辑和快捷键必须避免访问空 tab id。
- 风险：SFTP-only group 应复用 saved `Session`，但不能启动 SSH shell backend。
- 无阻塞。

## 下一步

- 本轮已完成；后续只需在真实 GUI 中确认 saved SSH 右键 Open SFTP、不创建 shell tab 和 SFTP-only 快捷键提示。

## 最后更新时间

- 2026-07-13 21:10 +0800
