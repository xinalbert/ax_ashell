# 当前项目实施记录

## 当前目标

- 目标：让终端 URL 和路径只在对应激活快捷键按下时显示下划线和手型指针。
- 交付物：修饰键驱动的终端链接视觉状态、即时按键切换、缓存失效优化、回归测试和验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/terminal.rs`，`src/app/actions/terminal.rs`，`src/app/views/terminal_panel.rs`，`src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 `Cargo.toml` / `Cargo.lock`、修改 URL/路径检测或 Command/Ctrl+单击打开行为、引入新的 hover 或终端渲染依赖。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | URL/路径 hover、修饰键状态和绘制缓存诊断 | 源码与 GPUI modifier-change API 审查 | 当前无条件显示下划线和手型 |
| P2 | completed | 修饰键触发的下划线与手型、缓存键收敛 | 终端链接聚焦测试、`cargo check` | Command 对应 macOS，Ctrl 对应其他平台 |
| P3 | completed | 自动化验证与记录收口 | `rustfmt`、完整测试、hover 审计、tracking validator | 已完成 |

## 已完成

- 已确认 URL/路径命中信息、Command/Ctrl+单击激活逻辑与修饰键状态已存在。
- 已确认绘制层的下划线和 pane 层的手型仅检查 hover，未检查激活修饰键；GPUI `on_modifiers_changed` 可在不移动鼠标时刷新状态。
- URL/路径命中后仅在激活修饰键按下时显示下划线和手型；普通 hover 保持终端文本视觉。
- 修饰键按下/松开由 `on_modifiers_changed` 即时通知；链接 hover 从文本行布局缓存键移除，避免光标移动重建行布局。

## 验证

- 已完成：终端 hover、修饰键状态、绘制缓存和 GPUI modifier-change API 审查；`rustfmt`、链接聚焦测试 3 项、缓存键测试 1 项、`cargo check`、完整 `cargo test --quiet`（174 项）、hover 静态审计和 `git diff --check`。
- 已完成：tracking docs validator。
- 未完成：真实 GUI 下 macOS Command / 非 macOS Ctrl 按下和松开时的下划线、手型和单击打开验收。

## 风险与阻塞

- 风险：必须在真实 GUI 确认修饰键按下和松开时无需移动鼠标就能准确切换下划线与指针。
- 无阻塞。

## 下一步

- 在真实 GUI 中验证 macOS Command / 非 macOS Ctrl 按下和松开时的下划线、手型与既有单击打开行为。

## 最后更新时间

- 2026-07-14 14:34 +0800
