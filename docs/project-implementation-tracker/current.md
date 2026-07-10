# 当前项目实施记录

## 当前目标

- 目标：在已实现 SFTP 按需建立的基础上，增加空闲断开与后续自动重连，回收长时间无交互的 SFTP 连接
- 交付物：group 级 SFTP 最后活跃时间跟踪；事件泵中的空闲断开逻辑；传输期间不回收的保护；环境记录和月度实施记录同步更新

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：SSH 自动休眠、SFTP 远程编辑会话持久化优化、`russh` / `russh_sftp` 依赖升级、终端渲染、release/tag、提交

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | SFTP 空闲断开边界与活跃条件结论 | 本地源码检索和 `src/app/actions/sftp.rs` / `src/app/lifecycle/event_loop.rs` / `src/sftp.rs` / `src/terminal.rs` 读取完成 | 活跃传输可由 `self.transfers` 判定；空闲时间戳需新增 |
| P2 | completed | group 级最后活跃时间与空闲断开实现 | `rustfmt --edition 2024 src/app.rs src/app/lifecycle/init.rs src/app/lifecycle/event_loop.rs src/app/actions/sftp.rs src/app/actions/session.rs src/app/workspace/workspace.rs src/app/dialogs/transfers.rs src/app/views/sftp_panel/transfer_panel.rs` 通过 | 仅回收无活跃传输且不可见的 SFTP 连接 |
| P3 | completed | 编译、测试与空白检查结果 | `cargo check`，`cargo test --quiet`，`git diff --check` 均通过 | 覆盖空闲断开与按需自动重连改动 |
| P4 | completed | tracking docs 校验结果 | `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` 通过 | 收口前执行 |

## 已完成

- 已完成施工前环境预检，并确认本轮不新增依赖、不改配置 schema、不联网、不使用多 agent
- 已确认上一步的 SFTP 懒连接实现已通过编译与测试，当前可继续在 group 级 handle 之上叠加空闲断开
- 已确认 `self.transfers` 中 `Running` / `Paused` 足以识别不可回收的传输中 SFTP 连接
- 已确认当前没有 SFTP 活跃时间戳，需要新增 group 级 `last_activity`
- 已确认远程编辑自动上传链路不走 `self.transfers`，本轮空闲断开需采取保守边界，优先回收不可见的 SFTP 会话
- 已新增 group 级 `sftp_last_activity`，并在 SFTP 页面打开、主动操作和后台 SFTP 事件到达时刷新活跃时间
- 已在事件泵中加入 idle sweep，默认 300 秒空闲后关闭无活跃传输且当前不可见的 SFTP 连接；后续再次使用时继续通过 ensure 自动重连

## 验证

- 已完成：环境记录读取；项目地图读取；SFTP 空闲断开边界梳理；本轮 plan-first 记录初始化；`rustfmt --edition 2024 src/app.rs src/app/lifecycle/init.rs src/app/lifecycle/event_loop.rs src/app/actions/sftp.rs src/app/actions/session.rs src/app/workspace/workspace.rs src/app/dialogs/transfers.rs src/app/views/sftp_panel/transfer_panel.rs`；`cargo check`；`cargo test --quiet`，50 个测试通过；`git diff --check`；tracking docs validator
- 未完成：GUI 手工确认空闲断开后的自动重连和远程编辑长时间驻留边界

## 风险与阻塞

- 风险一：远程编辑自动上传链路没有显式会话引用计数；当前策略通过“仅回收不可见 SFTP 页面”来保守规避，但仍建议手工验证长时间编辑场景
- 风险二：300 秒阈值是工程保守默认值，后续可能需要配置化或按用户反馈微调

## 下一步

- 如需继续降低占用，下一步可评估为远程编辑 watcher 增加会话 pin/refcount，或再讨论 SSH 休眠策略

## 最后更新时间

- 2026-07-10 10:33 +0800
