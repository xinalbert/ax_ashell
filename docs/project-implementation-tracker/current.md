# 当前项目实施记录

## 当前目标

- 目标：修正终端选择在持续输出/刷新时跟随内容移动的问题，使其像中文 IME composition 一样记住当时 viewport 行列位置和文本快照，不随后续终端刷新重新映射
- 交付物：终端 frozen selection 改为固定 viewport row/col；复制仍使用选择时缓存文本；补充单元测试锁定持续输出后选区位置不移动；同步环境与实施记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`src/app/lifecycle/event_loop.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：终端 emulator 底层 selection 算法重写、非终端 TextView 选择、复制格式富文本、GUI 自动化测试、前面未提交的弹窗/SSH/可复制文本改动

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位当前 frozen selection 跟随刷新移动的原因 | 源码检查 | 当前通过 bottom-index、history delta 和 live selection remap 让选区随内容上移 |
| P2 | completed | 将 frozen selection 存储改为固定 viewport row/col | `cargo check` | 类似 IME composition 的 anchor row/col，不再根据 history/display offset 重映射 |
| P3 | completed | 更新渲染和复制语义测试 | 定向单元测试 | 复制继续使用选择时缓存文本，渲染保持固定位置 |
| P4 | completed | 格式化、编译、测试和 tracking 收口 | `rustfmt`，`cargo check`，`cargo test --quiet`，`git diff --check`，tracking docs validator | GUI 手工持续输出拖选仍需实机 |

## 已完成

- 已确认当前 `TerminalFrozenSelection` 保存 bottom index、history size 和 display offset，并通过 `remap_frozen_bottom_index_for_snapshot()` / `remap_frozen_selection()` 在输出刷新后移动 frozen cell 与选区
- 已确认 IME composition 使用 `anchor_row` / `anchor_col` 直接按 viewport 坐标绘制，不跟随终端 history 刷新重映射
- 已确认复制路径优先返回 `TerminalFrozenSelection.text`，可保留选择时文本快照
- 已将 `FrozenRenderCell` 改为保存固定 `row` / `col`，并将 frozen highlights 改为 `(row, col)` key
- 已移除 frozen selection 对 history delta、bottom-index 和 live selection 的位置重映射；有 frozen selection 时渲染层不再 fallback 到底层 live selection
- 已取消 backend output 触发的 frozen selection 自动清理；用户输入、粘贴、点击等显式清理路径仍会清理 frozen selection

## 验证

- 已完成：源码检查；`rustfmt --edition 2024 src/terminal.rs src/terminal/element.rs src/app/actions/terminal.rs src/app/lifecycle/event_loop.rs` 通过；`cargo check` 通过；`cargo test --quiet frozen_ -- --nocapture` 通过，3 个相关测试全部通过；`cargo test --quiet` 通过，46 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过
- 未完成：GUI 手工持续输出拖选验证

## 风险与阻塞

- 风险一：如果 alacritty 底层 live selection 在输出后自动变化，渲染层固定位置和底层 selection 可能短暂不一致；复制必须继续优先使用 frozen text 快照
- 风险二：窗口 resize 后固定 viewport row/col 需要裁剪到当前 rows/cols，否则可能越界
- 风险三：自动测试无法确认真实鼠标拖选体验，仍需 GUI 手工验证

## 下一步

- 在 GUI 中手工确认持续输出时选区视觉固定、复制内容仍为选择时文本

## 最后更新时间

- 2026-07-09 23:14 +0800
