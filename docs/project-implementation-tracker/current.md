# 当前项目实施记录

## 当前目标

- 目标：把终端选区刷新保护从“冻结整个终端输出”改为“只冻结选区涉及的屏幕行”，并保持 IME composition overlay 不冻结底层终端输出
- 交付物：选区行 frozen snapshot、复制优先使用 frozen text、渲染时只覆盖选中行、backend output 持续 feed、倒置索引 + live selection 行偏移校准，以及更新后的实施记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/mod.rs`，`src/app/init.rs`，`src/app/event_loop.rs`，`src/app/ui/layout.rs`，`src/app/ui/terminal_panel.rs`，`src/app/workspace.rs`，`src/session/mod.rs`，`src/session/pane.rs`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：PTY 后端协议、SSH/SFTP 连接、终端全量局部绘制缓存、系统输入法实现、GUI 自动化截图验证

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 设计并接入选区行 frozen snapshot 状态 | 源码检查，`cargo check` | 不新增依赖 |
| P2 | completed | 让 backend output 在选区存在时继续 feed，只在渲染层冻结选中行 | `cargo check`，`cargo test` | 已移除空的 deferred output 机制 |
| P3 | completed | 渲染层用 frozen row cells 覆盖当前 snapshot 中选中行 | `cargo check`，单元测试 | 未选中行继续使用实时 terminal snapshot；滚动时用倒置索引和 live selection 校准 |
| P4 | completed | 格式化、编译测试和 tracking docs 校验 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 手工验证仍需用户运行应用确认 |

## 已完成

- 已读取 `project-map.md` 和当前环境记录，本轮涉及路径均已在项目地图覆盖范围内，无需刷新项目地图
- 已确认本轮不需要联网、不使用多 agent、不新增依赖
- 已移除 active selection / IME composition 对 backend output 的全局 defer；`BackendEvent::Output` 现在始终直接 `feed`
- 已新增 `TerminalFrozenSelection` 与 `FrozenRenderCell`，在开始/拖动选区时保存选中行的 cells、highlight 和 text，复制时优先使用 frozen text
- 已在 `TerminalElement` 中只用 frozen row cells 覆盖对应行，其他行继续绘制实时 `RenderSnapshot`
- 已用“距底部第几行”的倒置索引保存 frozen cells，并在 scrollback 未满时按 `history_size` 推进；scrollback 到上限后优先用 alacritty live selection 的当前行偏移校准，避免冻结行停住
- 已在 output 后发现 live selection 被内核清除时同步清理 app 侧 frozen snapshot，避免残留假高亮
- 已删除空的 `deferred_output` 字段和 flush helper，避免后续维护误判仍存在全局冻结路径

## 验证

- 已完成：`rustfmt --edition 2024 --config skip_children=true src/app/mod.rs src/app/init.rs src/app/event_loop.rs src/app/ui/terminal_panel.rs src/app/ui/layout.rs src/app/workspace.rs src/session/mod.rs src/session/pane.rs src/terminal/input.rs src/terminal/element.rs src/terminal/mod.rs` 通过
- 已完成：`cargo check` 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 已完成：`cargo test` 通过，25 个测试全部通过；新增 UTF-16 composition range、frozen bottom index 和 history 上限 live selection 校准测试
- 已完成：tracking docs validator 通过
- 未完成：GUI 手工验证持续输出时只有选中行冻结、未选中行继续刷新；GUI 中文 IME 候选框和预编辑高亮稳定性也仍需手工确认

## 风险与阻塞

- 阻塞：无
- 风险一：GUI 手工验证未执行，仍需在真实 `Working` 流式输出下确认选中行冻结、其他行刷新和复制文本稳定
- 风险二：选区拖动过程中 frozen snapshot 只在鼠标事件到达时更新，极高速输出场景仍需目视观察是否符合预期

## 下一步

- 在 GUI 中验证：持续输出时选择中间/底部行，确认只有选中行冻结，其他行继续刷新；同时验证中文 IME composition 候选和预编辑高亮不被刷新打断

## 最后更新时间

- 2026-07-08 23:43 +0800
