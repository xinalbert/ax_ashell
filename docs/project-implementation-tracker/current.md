# 当前项目实施记录

## 当前目标

- 目标：修复终端在快速 resize 且发生底部滚动时复用旧行快照导致的越界崩溃。
- 交付物：同尺寸快照守卫、覆盖 resize 与滚动叠加情形的回归测试、自动化验证与跟踪记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/tab.rs`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：终端布局策略调整、关键字高亮算法改造、macOS 崩溃上报机制、现有 SFTP 受管编辑任务。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 基于 crash/runtime 日志定位越界条件 | 崩溃位置、resize 时序与缓存代码核对 | 旧快照可在不同尺寸下进入滚动复用，访问 `len` 本身 |
| P2 | completed | 限制滚动行复用只使用同尺寸快照，并补充回归测试 | 聚焦单元测试 | 保留同尺寸下既有行复用优化 |
| P3 | completed | 格式化、完整测试、静态差异与跟踪文档校验 | `cargo check`、`cargo test --quiet`、`git diff --check`、tracking validator | GUI resize 需在 macOS 手工确认 |

## 已完成

- 关联四份 crash 报告与运行日志，确认两次首次 panic 均来自 `src/terminal/tab.rs:859` 的 `previous_rows[row + scroll_rows]`。
- 确认快速 resize 期间当前屏幕行数会从 44 变为 45、从 27 变为 28；旧快照行数较小但仍可进入滚动复用路径。
- 确认随后出现的 `panic in a function that cannot unwind` 是首次 panic 穿过 macOS 回调边界后的次生崩溃。
- 滚动复用仅接受 `rows`、`cols` 都匹配的旧快照；新增旧快照 4 行、当前终端 5 行且 history 增量为 1 的回归测试。

## 验证

- 已完成：crash/runtime 日志、快照构建路径、现有底部滚动复用测试与环境记录审查；Rust 修复与对应回归测试实现；`rustfmt`、聚焦测试、`cargo check`、完整 `cargo test --quiet`（222 项）、`git diff --check` 和 tracking docs validator。
- 未完成：macOS GUI 快速 resize 与 PTY 输出并发的手工验收。

## 风险与阻塞

- 守卫会在 resize 帧放弃一次滚动行复用并重建当前可见行，属于正确性优先的短暂性能退化；同尺寸滚动路径保持原有优化。
- 自动化测试可覆盖缓存尺寸不匹配，仍无法完全替代 macOS 窗口拖动与 PTY 输出并发的真实 GUI 验收。

## 下一步

- 在 macOS GUI 中拖动终端窗口并持续输出内容，确认窗口不会退出且终端内容稳定。

## 最后更新时间

- 2026-07-16 23:24 +0800
