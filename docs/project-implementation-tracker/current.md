# 当前项目实施记录

## 当前目标

- 目标：统一升级项目内 `zed-industries/zed` 依赖链到单一较新提交，并确认升级的真实代价与源码迁移需求
- 交付物：更新后的 `Cargo.toml` / `Cargo.lock`、真实仓库 `cargo check` / `cargo test` 验证结果、实施跟踪与环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`，`Cargo.lock`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：GUI 手工回归、无必要的功能改动、跨仓库 `gpui-component` 源码修改、超过本轮验证范围的大规模源码迁移

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 tracking / env 当前态并确认本轮 Zed 升级边界 | `docs/` contract 自检，`Cargo.toml` / `Cargo.lock` 走查 | 现有 `project-map.md` 已覆盖依赖文件与 GPUI/UI 入口，无需刷新地图 |
| P2 | completed | 在临时副本验证 Zed 升级的真实风险点 | `/private/tmp` 副本 `cargo check --offline` | 已确认“只升根项目依赖”会造成双版本 `gpui`；统一整条链到单一较新 Zed 提交后无需源码迁移 |
| P3 | completed | 在真实仓库统一 `zed-industries/zed` 依赖提交并更新 lockfile | `git diff Cargo.lock`，`rg -n 'git\+https://github.com/zed-industries/zed#' Cargo.lock`，`cargo check --locked` | 根项目继续保留 plain git source，`Cargo.lock` 已统一到 `f9c994796ad4341649d7b8664edbdfaae8bebd5d` |
| P4 | completed | 完成测试与 tracking docs 校验 | `cargo test --locked`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | GUI 手工验证不在本轮范围 |

## 已完成

- 已读取 `docs/project-implementation-tracker/project-map.md`、`docs/project-env-audit/current.md` 与现有 tracking / env 历史
- 已确认现有项目地图覆盖 `Cargo.toml` / `Cargo.lock` 及 `src/app/` 等 GPUI/UI 耦合入口，无需刷新 `project-map.md`
- 已在临时副本验证两种路径：只 pin 根项目 Zed 提交会引入双版本 `gpui`；统一到单一 Zed source id 后 `cargo check --offline` 通过
- 已确认本轮真实风险中心在依赖图统一，而不是已知的大规模源码迁移
- 已确认根项目若显式写 `rev = ...`，会与 `gpui-component` 的 plain git source 形成两套 `gpui` 类型宇宙；最终方案是保留根依赖为 plain git，并让 `Cargo.lock` 统一 pin 到单一提交
- 已在真实仓库将 `Cargo.lock` 统一到 `f9c994796ad4341649d7b8664edbdfaae8bebd5d`，并确认 `accesskit` / `wgpu` 等传递依赖随新 Zed 快照更新
- 已完成真实仓库 `cargo check --locked`、`cargo test --locked`，当前无需源码改动

## 验证

- 已完成：项目地图、当前 env/tracking 记录、现有未提交 diff 走查
- 已完成：临时副本 `cargo check --offline`
- 已完成：`rg -n 'git\+https://github.com/zed-industries/zed#' Cargo.lock`，确认仅剩 `#f9c994796ad4341649d7b8664edbdfaae8bebd5d`
- 已完成：真实仓库 `cargo check --locked`
- 已完成：真实仓库 `cargo test --locked`，13 个测试全部通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：运行时 / GUI 手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：若正式仓库只修改根项目 `gpui/gpui_platform/menu` 指向而不统一整条 Zed 生态，`gpui-component` 会引入第二套 `gpui`，导致类型和 trait 全面不兼容
- 风险二：当前根 `Cargo.toml` 不能单独显式 pin `rev = ...`；若要在 manifest 层固定具体 Zed 提交，必须让 `gpui-component` 与根依赖共享完全相同的 source id
- 风险三：本轮只做编译与测试验证，不含 GUI 手工回归；窗口、渲染、字体和菜单行为仍需后续实机确认
- 风险四：仍保留既有 `block v0.1.6` future-incompat warning，来源于 GPUI / cocoa 传递依赖

## 下一步

- 如需把 Zed 提交固定到 manifest 层，先处理 `gpui-component` 与根依赖的 source id 一致性，再决定是否继续写 `rev = ...`
- 视需要补做 GUI 手工回归，重点确认窗口、菜单、渲染、字体和输入行为
- 若继续前移 Zed 提交，优先先在临时副本验证 `gpui-component` 是否仍与目标快照保持同一依赖宇宙

## 最后更新时间

- 2026-07-08 09:13 CST
