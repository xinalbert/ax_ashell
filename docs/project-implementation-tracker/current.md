# 当前项目实施记录

## 当前目标

- 目标：修正终端 frozen selection 仍覆盖旧文字的问题，使其只记住选择高亮的 viewport 行列和复制文本快照，终端文字内容继续实时刷新
- 交付物：移除 frozen cell / frozen highlight 覆盖路径；选择背景独立于 live cell 绘制并固定在 captured viewport 行列；复制仍使用选择时缓存文本；补充单元测试锁定“无旧文字冻结、高亮仍保留”；同步环境与实施记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：终端 emulator 底层 selection 算法重写、非终端 TextView 选择、复制格式富文本、GUI 自动化测试、弹窗/SSH/可复制文本等前序功能改动

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位旧文字冻结残留来源 | 源码检查 | `layout_grid()` 仍通过 `frozen_cells_by_position()` 替换 live cell，并保留 frozen highlights |
| P2 | completed | 移除 frozen cell / frozen highlight 覆盖路径 | `cargo check` | `TerminalFrozenSelection` 只保留 tab、selection 和 copy text |
| P3 | completed | 让选择背景独立于 live cell 绘制 | 定向单元测试 | 刷新后即使 selected 位置没有 live cell，也会继续画固定 selection rect |
| P4 | completed | 格式化、编译、测试和 tracking 收口 | `rustfmt`，`cargo check`，`cargo test --quiet`，`git diff --check`，tracking docs validator | GUI 手工持续输出拖选仍需实机 |

## 已完成

- 已确认上一版虽然固定了 viewport 坐标，但仍保存 `FrozenRenderCell` 并在 `layout_grid()` 里用旧 cell 覆盖实时 cell，这是“以前的冻结”继续出现的直接原因
- 已确认选择背景目前只在遍历到 live `snapshot.cells` 时绘制，刷新后选区位置如果没有 live cell，高亮就会消失或看起来被刷新
- 已确认复制路径优先返回 `TerminalFrozenSelection.text`，可在不冻结文字渲染的前提下继续保留选择时文本快照
- 已移除 `FrozenRenderCell`、frozen highlights、`frozen_cells_by_position()` 和 `frozen_highlights_by_position()`；渲染层只画固定 selection 背景，不再覆盖旧文字 cell
- 已新增 `selection_background_rects()`，让 selection 背景按 captured viewport selection 独立于 live cell 绘制；刷新后 selected 位置即使没有字符也会保留高亮背景

## 验证

- 已完成：源码检查；`rustfmt --edition 2024 src/terminal.rs src/terminal/element.rs src/app/actions/terminal.rs` 通过；`cargo check` 通过；`cargo test --quiet frozen_ -- --nocapture` 通过，3 个相关测试全部通过；`cargo test --quiet` 通过，46 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过
- 未完成：GUI 手工持续输出拖选验证

## 风险与阻塞

- 风险一：选择背景从 cell 遍历中拆出后，需要避免和非默认背景 / inverse 背景重复绘制导致颜色覆盖顺序错误
- 风险二：窗口 resize 后固定 viewport row/col 需要裁剪到当前 rows/cols，否则可能越界
- 风险三：自动测试无法确认真实鼠标拖选体验，仍需 GUI 手工验证

## 下一步

- 在 GUI 中手工确认持续输出时选区高亮固定、终端文字继续实时刷新、复制内容仍为选择时文本

## 最后更新时间

- 2026-07-09 23:37 +0800
