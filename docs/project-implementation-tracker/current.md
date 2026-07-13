# 当前项目实施记录

## 当前目标

- 目标：消除持续终端输出时每帧对未变 text run 调用 `shape_line_by_hash` 的主线程开销，同时保持终端文本、颜色和交互行为正确。
- 交付物：行级 `ShapedLine` 缓存、完整失效语义、回归测试、构建验证和持续输出 sample 对比。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/element.rs`，必要时 `src/terminal/tab.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 `Cargo.toml` / `Cargo.lock`、替换 GPUI / Metal renderer、缓存 GPU command buffer 或离屏纹理、降低终端文本/ANSI/cursor 的刷新频率、改变 PTY 流控或既有 SFTP 功能。

## 当前状态

- 阶段：实施中
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 16:35 sample 热点、GPUI `ShapedLine` 生命周期和缓存键审查 | sample 调用树、锁定 GPUI `text_system.rs` / `line.rs` | `shape_line_by_hash` 多为 cache lookup；`ShapedLine::paint` 仍每帧提交 glyph |
| P2 | in_progress | `RowLayout` 中预生成并保存每个 text run 的 `ShapedLine` | element focused tests、`cargo check` | 内容、字体、字号、主题/亮度和 row highlights 变化时重建；选择/IME/hover 实时绘制 |
| P3 | pending | 格式化、完整 Rust 验证和文档收口 | `cargo test --quiet`、`cargo build`、`git diff --check`、tracking validator | 保留 dirty worktree 中无关改动 |
| P4 | pending | 同负载持续输出 sample 对比 | macOS `sample` | 预期显著降低 `shape_line_by_hash` 样本；`paint_line` / `paint_quad` 仍会存在 |

## 已完成

- 16:35 debug sample 的 `render_snapshot` 为 36、`highlight_rows_incremental` 为 30，均显著低于早期基线；旧 `layout_grid` 已消失。
- 16:35 的主要路径为 `TerminalElement::paint` 361 样本，其中 `BatchedTextRun::paint_with_row_offset` 187、`shape_line_by_hash` 184、`ShapedLine::paint` 173 和 `paint_quad` 184。
- 已确认锁定 GPUI 的 `ShapedLine` 是可克隆的布局和 decoration 数据；其 `paint` 不依赖原 text 字符串。`with_element_state` 可跨帧持有此缓存。

## 验证

- 已完成：环境、dirty worktree、16:35 sample 和锁定 GPUI `ShapedLine` / line-layout cache API 审查。
- 未完成：缓存实现、Rust 回归/构建、持续输出 sample 复测和 GUI 手工验收。

## 风险与阻塞

- 风险：缓存必须随 text run 的内容、font family / size、ANSI/主题颜色、装饰和 keyword/search 颜色失效，否则可能出现错误字形或陈旧颜色。
- 风险：即使消除 shaping lookup，`ShapedLine::paint` 仍逐 glyph 提交 GPU primitive；本轮不能承诺消除 `paint_line` / `paint_quad` 主路径。
- 无阻塞。

## 下一步

- 在 `RowLayout` 构建阶段生成 `ShapedLine` 并直接绘制缓存对象，再以相同持续输出负载复测。

## 最后更新时间

- 2026-07-13 16:47 +0800
