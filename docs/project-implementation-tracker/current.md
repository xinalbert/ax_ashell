# 当前项目实施记录

## 当前目标

- 目标：稳定终端交互期间的显示，避免 Codex 流式 `Working` 输出在选区存在或中文 IME 组合中时打断当前交互
- 交付物：`src/terminal/mod.rs` 中的延迟输出缓冲、`src/app/event_loop.rs` 中的交互期输出延后应用逻辑、`src/terminal/input.rs` 中的交互结束冲刷，以及更新后的实施/环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/terminal/input.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：行级 layout cache、局部 dirty-rect 绘制、GPUI 渲染模型重构、Codex 本体界面修改

## 当前状态

- 阶段：验证中
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新环境/实施记录到 IME 与选区稳定性任务 | tracking docs validator | 已确认 `Working` 是终端内文本，不是仓库外 UI |
| P2 | completed | 活动终端交互期的输出延迟应用 | `cargo check`，`cargo test` | 交互锁包含选区和 `terminal_marked_text` |
| P3 | completed | 交互结束后的输出冲刷与 IME 状态保留 | `cargo check`，`cargo test` | 后端输出不再直接清空组合态 |
| P4 | in_progress | 文档收口、验证与提交拆分 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 体验仍需手工验证 |

## 已完成

- 已确认“`Working` 被刷掉”本质仍是终端正文交互被流式输出打断，而不是仓库外另一个独立视图在刷新
- 已确认当前 `BackendEvent::Output` 会在事件泵里直接喂给 `alacritty_terminal`，这会让选区和 IME 组合态一起暴露在持续输出之下
- 已确认当前实现还会在任意输出到来时清空 `terminal_marked_text`，这是中文候选被打断的直接风险点
- 已实现活动终端在选区存在或 IME 组合中时只缓存输出，不立刻改写终端 buffer；交互结束后再冲刷积压输出
- 已补齐选区清空、IME 提交/清除、pane 切换和 backend 重连时的缓冲释放与清理边界

## 验证

- 已完成：源码级确认终端输出、IME 标记文本、输入处理器注册与 render snapshot 的关联路径
- 已完成：`rustfmt --edition 2024 src/terminal/mod.rs src/terminal/input.rs src/app/event_loop.rs src/session/pane.rs src/session/mod.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`
- 未完成：tracking docs validator
- 未完成：GUI 终端选区与中文 IME 手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：延迟应用输出会让交互中的活动终端短时间停留在旧画面，交互结束后再一次性追平
- 风险二：若用户长期按住选区或长期停留在 IME 组合态，活动终端会累计待冲刷输出，但只影响当前活动 tab
- 风险三：这轮实现不会改变 `alacritty_terminal` 自身在真实内容变更后的 selection 语义，只是把变更延后到交互结束

## 下一步

- 已完成“交互期延迟应用输出 + 结束后冲刷”的最小闭环
- 下一步重点验证：`Working` 持续输出时选区是否稳定、中文候选框是否不再被流式输出打断

## 最后更新时间

- 2026-07-08 17:07 +0800
