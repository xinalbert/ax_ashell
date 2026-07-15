# 当前项目实施记录

## 当前目标

- 目标：消除持续终端输出中新建或变更行先以 ANSI 前景色绘制、125ms 后才补关键词 / URL 色的明显跳色，同时保持大范围变更时的 CPU 限频保护。
- 交付物：可报告真正重建行的可视快照构建结果、按 `Rc<RenderRow>` 重排行级高亮缓存的增量高亮器、小变更同步高亮与大变更 125ms 校正策略、回归测试和验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，必要时 `src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 不在本轮范围内：关键词规则和配色、终端后端 / PTY 协议、GPUI 绘制 API、配置 schema、依赖版本、`Cargo.toml` / `Cargo.lock`、CI workflow，以及按列范围优化无换行进度条的第二阶段工作。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 环境、现有 `Rc<RenderRow>` 复用与高亮缓存边界确认 | 预检、项目地图、`tab.rs` / `highlight.rs` 源码审查 | 现有 125ms 到期刷新由 app event loop 驱动；不需新增定时器或依赖 |
| P2 | completed | `VisibleRowBuild` 元数据与按行身份重排的高亮缓存 | terminal 单元测试、`cargo check` | 滚屏 `TermDamage::Full` 只作为候选检查范围，不等价于全屏高亮失效 |
| P3 | completed | 小范围同步识别及大范围 125ms 校正接线 | 跳色 / 滚屏 / WRAPLINE / resize 回归测试 | 同步路径只扫描真正重建行及相连逻辑行；复杂大变更继续批处理 |
| P4 | completed | 格式化、完整验证和记录收口 | `cargo test --quiet`、`cargo build`、`git diff --check`、tracking validator | 真实 GUI 持续输出与无换行进度条需手工采样 |

## 已完成

- 已完成 Rust 2024 / Cargo 施工前预检；本机 `rustc 1.96.1`、`cargo 1.96.1` 满足仓库 `rust-version = 1.88.0`。
- 已确认 `build_visible_rows` 已逐 cell 验证滚屏复用的 `Rc<RenderRow>`，但没有把真正重建行上报给高亮器。
- 已确认 `HighlightCache` 当前按视口行号保存，而 `TermDamage::Full` 会触发整屏高亮重算；这正是滚屏仍会出现延迟跳色的缺口。
- 已确认 app event loop 会检查 `TerminalTab::highlight_refresh_due`，因此大范围 125ms 校正会自然请求 UI 刷新。
- 已让 `build_visible_rows` 返回真正重建行；最多 4 行时当帧同步识别关键词 / HTTP / IP / 端口和相关 `WRAPLINE` URL 链，随后仅保留这些行的 125ms 轻量校正。
- 已将 `HighlightCache` 改为按 `Rc<RenderRow>` 身份重排；滚屏中已验证复用的行继续保留关键词和 URL 色，不再因 `TermDamage::Full` 强制全屏重算。
- 已将 URL 处理收紧为仅构建受影响的逻辑换行链；行缓存迁移使用 `Rc` 指针哈希定位，避免滚屏路径的线性嵌套查找。

## 验证

- 已完成：环境预检、实施记录与项目地图审查、终端调用链审查；`rustfmt --edition 2024 src/terminal/tab.rs src/terminal/highlight.rs`；terminal tab 聚焦测试 19 项；terminal highlight 聚焦测试 20 项；`cargo check`；完整 `cargo test --quiet`（190 项）；`cargo build`；`git diff --check`；tracking docs validator。
- 未完成：真实 GUI 下连续换行输出、跨 `WRAPLINE` URL、resize / alternate screen 与无换行 `\r` 进度条的颜色首帧和 CPU sample 验收。

## 风险与阻塞

- 风险：跨 `WRAPLINE` URL 的两端可能因滚屏或换行标志变化而失去逻辑行上下文；同步路径必须使用当前和前一帧的 wrap 边界扩展受影响行。
- 风险：无换行的 `\r` 进度条会持续重建同一行；本轮先完成行级同步，后续仅在新 sample 证明仍是热点时保留 `LineDamageBounds` 列范围做局部识别。
- 无阻塞。

## 下一步

- 在真实终端负载采样中确认颜色不再跳变，并仅在无换行进度条仍显著占用 CPU 时启动列范围增量识别第二阶段。

## 最后更新时间

- 2026-07-15 09:49 +0800
