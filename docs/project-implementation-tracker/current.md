# 当前项目实施记录

## 当前目标

- 目标：修复终端重连状态持续刷新时，未变化的关键词 / URL 彩色高亮在 125ms 延迟高亮窗口内短暂消失导致的闪烁。
- 交付物：full damage 下未变行的安全 `RenderRow` 复用、回归测试、自动化验证和 GUI 复测边界说明。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/tab.rs`，必要时 `src/terminal/highlight.rs` / `src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 `Cargo.toml` / `Cargo.lock`、取消高亮限频、改变外部 Codex 请求重连行为、替换 GPUI / Metal renderer、改变 PTY 流控或 SSH/SFTP 功能。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 截图输出来源、重连刷新与高亮闪烁机制定位 | 源码审查、字符串搜索 | `Reconnecting...` 来自外部 Codex 流式请求；AxShell 负责终端渲染和关键词 / URL 高亮 |
| P2 | completed | full damage / dirty damage 下内容未变行继续复用旧 `RenderRow` | 聚焦单元测试、`cargo check` | 让延迟高亮能通过行块身份保留未变行颜色 |
| P3 | completed | 格式化、自动化验证和文档收口 | `rustfmt`、聚焦测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking validator | 已通过 |
| P4 | completed | GUI 复测边界说明 | 真实 AxShell 里运行同类重连状态输出 | 自动化无法复现外部请求重连；保留为手工验收项 |

## 已完成

- 已确认截图中的 `Reconnecting... 1/5`、`Stream disconnected before completion` 和 `https://aixj.vip/responses` 不是 AxShell 自身文案，而是终端内运行的外部 Codex/请求流输出。
- 已确认 AxShell 的关键词 / URL 高亮最多每 125ms 重算一次；延迟窗口内只复用能通过 `Rc<RenderRow>` 身份证明未变的高亮。
- 已定位闪烁原因：持续重连状态可能让 terminal damage 退化为 full damage，`build_visible_rows` 会重建未变行，导致旧高亮无法映射到新行块，在下一次高亮刷新前短暂消失。
- 已修改 `build_visible_rows`：对 full damage 和 dirty rows 先逐 cell 对照当前 terminal grid，内容未变的行继续复用旧 `Rc<RenderRow>`；已新增回归测试覆盖 full damage 下未变 `ERROR` / URL 行保留延迟高亮。
- 已完成格式化、聚焦测试、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。

## 验证

- 已完成：`rustfmt --edition 2024 src/terminal/tab.rs`；`cargo test --quiet unchanged_rows_keep_deferred_highlights_across_full_damage` 1 项通过；`cargo test --quiet terminal::tab::tests` 16 项通过；`cargo check` 通过；完整 `cargo test --quiet` 165 项通过；`git diff --check` 通过；tracking docs validator 通过。
- 未完成：真实 GUI 中外部 Codex 请求重连场景手工观察未执行。

## 风险与阻塞

- 风险：保留未变行高亮会在相邻行变化影响跨行 URL 时最多保留 125ms 旧颜色；这比当前闪烁更可接受，下一次高亮刷新会纠正。
- 风险：如果外部程序实际反复改写包含彩色关键词的同一行，该行仍会等到下一次高亮刷新才重新上色；本轮目标是修复未变化行被 full damage 误清空。
- 无阻塞。

## 下一步

- 在真实 AxShell 里复现同类 `Reconnecting...` 输出，确认状态行刷新时下方红色关键词和 URL 高亮不再闪烁。

## 最后更新时间

- 2026-07-13 23:09 +0800
