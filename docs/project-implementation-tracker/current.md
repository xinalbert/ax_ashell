# 当前项目实施记录

## 当前目标

- 目标：修复 SFTP 进入某个远端文件夹超时或报错后，远端列表后续点击失效的问题
- 交付物：SFTP 远端目录导航失败时保持上一轮成功目录可用、路径输入不被失败目标污染、列表点击/刷新/父级跳转仍可继续使用，以及更新后的实施记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/sftp/ops.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：SFTP 认证、上传下载传输实现、SSH 连接策略、SFTP 表格排序和传输面板样式重构

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位 SFTP 目录导航失败后的状态污染路径 | 源码检查 | 已确认 `navigate_sftp()` 先改 `current_path`，失败后只发 status |
| P2 | completed | 目录读取成功后再提交 current path，失败保留旧目录和 entries | `cargo check` | 不改 SFTP 后端协议；失败只更新 status |
| P3 | completed | 格式化、编译测试和跟踪文档校验 | `rustfmt`，`cargo test`，tracking docs validator | GUI 手工验证仍需用户运行应用确认 |

## 已完成

- 已确认远端列表点击在 `src/app/ui/sftp_panel.rs` 中最终调用 `select_sftp_entry()` / `navigate_sftp()`
- 已确认 `src/sftp/ops.rs` 当前会在发送 `ListDir` 后立即把 `SftpUiState.current_path` 改成目标目录
- 已确认 `src/sftp/mod.rs` 的 `ListDir` 失败分支只发送 `SftpStatus`，不会发送成功路径或 entries，因此 UI 会保留旧 entries 但路径已经变成失败目录
- 已确认修复可限定在 UI state 语义：只在收到 `SftpEntries` 成功事件后提交路径，失败保留上一轮成功目录
- 已修改 `navigate_sftp()`，目录点击只发送 `ListDir` 并更新状态栏，不再提前写入 `current_path` 或同步路径输入框
- 已给远端目录浏览增加 30 秒超时，并让后续 `ListDir` 使用新的 SFTP channel，避免坏目录污染主浏览 session

## 验证

- 已完成：源码级确认远端目录导航、`SftpEntries` / `SftpStatus` 事件回写路径
- 已完成：`git status --short`
- 已完成：`rustfmt --edition 2024 --config skip_children=true src/sftp/ops.rs src/sftp/mod.rs src/app/event_loop.rs src/terminal/mod.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，18 个测试全部通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI 手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：路径输入框在 pending 导航时不立刻显示目标路径，需由状态栏错误信息反馈失败目标
- 风险二：真实超时场景需要 GUI 或远端环境复现；本机编译测试只能覆盖状态 wiring

## 下一步

- 由用户在 GUI 中复现“坏目录超时/报错后旧目录内容仍可点击”

## 最后更新时间

- 2026-07-08 22:05 +0800
