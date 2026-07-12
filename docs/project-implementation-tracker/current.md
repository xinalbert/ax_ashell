# 当前项目实施记录

## 当前目标

- 目标：统一 AxShell Settings 下拉、菜单行和长列表的快速 hover 路径，减少 Settings / selector / sidebar / SFTP 菜单 hover 期间的全量渲染和重复候选构建。
- 交付物：共享 `src/app/hover.rs` fast hover API；Settings `fast_menu` lazy + virtual list；UI/Terminal font candidate 缓存；SSH group、selector、saved sidebar、SFTP transfer 和右键菜单迁移到 fast hover / `uniform_list`；Theme Editor Base Theme 下拉改为 lazy candidate builder。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/`，`src/app/hover.rs`，`src/app/dialogs/`，`src/app/views/`，`src/app/actions/`，`src/app.rs`，`src/app/lifecycle/init.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：主题 JSON 色值、外部 `gpui-component` / `gpui` 源码、`Cargo.toml` / `Cargo.lock`、Settings 页面结构重写、真实 GUI 手工验收。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 新增项目本地 fast hover skill 和 `src/app/hover.rs` 共享 API | skill 内容复核，项目地图刷新 | 后续 dropdown/list/menu hover 优先复用 `FastHoverOptions` / `FastHoverExt` |
| P2 | completed | Settings fast menu 支持 lazy candidate cache 和长菜单 `uniform_list` | `cargo check` | 字体菜单和 Base Theme 下拉避免页面 render 时构建候选 |
| P3 | completed | selector、saved sidebar、transfer history、SSH group 和自绘 context menu 迁移到 fast hover / `uniform_list` | fast hover 审计搜索，`cargo test --quiet` | 移除相关 `DropdownMenu` / `PopupMenuItem` / `context_menu` 路径 |
| P4 | completed | 运行格式化、编译、完整测试和文档校验 | `rustfmt`，`cargo check`，`cargo test --quiet`，`git diff --check`，tracking validator | GUI 手感仍需手工确认 |

## 已完成

- 已读取 `AGENTS.md`、项目本地 `.agents/skills/ax-ashell-fast-hover/SKILL.md`、环境记录和当前实施记录。
- 已确认本轮不需要联网、不使用多 agent、不新增依赖。
- 已新增 `.agents/skills/ax-ashell-fast-hover/` 并在 `AGENTS.md` 指向该项目本地 skill。
- 已新增 `src/app/hover.rs`，集中提供 `FastHoverOptions`、`FastHoverTokens`、`FastHoverExt`、`list_fast_hover_options`。
- 已让 Settings `fast_menu` 复用共享 fast hover tokens，并对长菜单切到 `uniform_list`；lazy 菜单候选在一次 popover 会话内缓存。
- 已缓存 Settings UI font names 和 terminal monospace font filtering 结果，避免 hover / reopen 热路径重复枚举和测量字体。
- 已将 Theme Editor Base Theme 下拉从 `fast_settings_menu` 切换为 `fast_settings_menu_lazy`，展开时再按当前模式构建主题候选。
- 已将 SSH group 下拉迁移到 Settings fast menu。
- 已将 selector saved-session 列表、saved sidebar 展开 / 折叠列表和 SFTP transfer history 切到 `uniform_list` 可见行渲染。
- 已将 SFTP / saved session 右键菜单改为自绘 fast hover 菜单行，避免 package context menu hover 路径叠加。
- 已执行受影响 Rust 文件 `rustfmt --edition 2024`、`cargo check`、完整 `cargo test --quiet` 和 fast hover 审计搜索；提交前继续执行空白检查和 tracking docs validator。

## 验证

- 已完成：源码路径复核；`rustfmt`；`cargo check`；完整 `cargo test --quiet`；fast hover 审计搜索；`git diff --check`；tracking docs validator。
- 未完成：真实 GUI 手工确认。

## 风险与阻塞

- 风险：自动化能覆盖编译和基本逻辑，但 hover 体感、popover 位置和真实 GUI 鼠标快速扫动仍需手工验收。
- 无阻塞。

## 下一步

- 创建 Git commit；随后在真实 GUI 中确认 Settings 下拉、SSH group、selector、sidebar、SFTP 右键菜单和 transfer history 的 hover 手感。

## 最后更新时间

- 2026-07-12 10:05 +0800
