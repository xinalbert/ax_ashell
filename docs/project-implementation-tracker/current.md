# 当前项目实施记录

## 当前目标

- 目标：把 `Custom` 从运行时临时覆盖改成真正的可保存主题，生成真实 `ThemeConfig` 并进入 theme list，后续可直接选择；同时把可调项扩成可视化基底预设 + 详细色槽编辑
- 交付物：custom theme registry/load/save 链路、配置持久化模型、设置页可视 theme editor、中英文文案、跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/main.rs`，`src/app/theme.rs`，`src/app/mod.rs`，`src/app/dialogs.rs`，`src/terminal/element.rs`，`src/session/config.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：SSH / SFTP 协议逻辑、终端后端传输、发布脚本、外部联网检索、GUI 手工截图验证

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 current plan、项目地图和当前范围到“custom 主题注册化”任务 | `docs/` contract 自检，源码走查 | 已确认当前 custom 仍为默认主题上的手写 override，未注册进 `ThemeRegistry` |
| P2 | completed | 重构 custom theme 配置与持久化，生成真实 `ThemeConfig` 并保存到可加载 theme file | `cargo check` | 已落到配置目录 `themes/`，并接入 registry load/watch 与当前会话即时应用 |
| P3 | completed | 扩展设置页 Custom 主题编辑器，支持基底预设选择和详细可视色槽配置 | `cargo check`，文案走查 | Custom 页面现按 light/dark 与语义分组展示详细字段，并可选 base theme |
| P4 | completed | 收口 theme list 选择行为、当前主题应用和 terminal 亮度语义 | `cargo check`，源码走查 | General 主题列表已只走 registry，保存后进入 theme list，可直接重选 |
| P5 | completed | 运行格式化、测试和 tracking docs 校验并更新记录 | `rustfmt`，`cargo check`，`cargo test`，tracking docs 校验 | 代码级验证已完成；GUI 手工验证仍未执行 |

## 已完成

- `src/session/config.rs` 新增 structured custom theme draft：包含主题名、light/dark base theme、按模式 overrides 和 font brightness，并保留旧 `custom_*` 字段兼容迁移
- `src/app/theme.rs` 改成真正的 theme registry/save 链路：启动时 load embedded + user theme dir，watch `themes/`，保存时生成真实 `ThemeSet` JSON 文件，并以 `<Name> [Custom Light/Dark]` 注册到可选主题列表
- `src/app/theme.rs` 的当前 custom 主题应用逻辑已优先用 draft 即时生成 theme config，避免同名 custom theme 在 registry reload 前继续吃旧缓存
- `src/app/mod.rs` 改为动态 `custom_theme_inputs`，按 metadata 初始化完整字段，并迁移旧 `Custom Theme` 已选项到新的 light/dark registry name
- `src/app/dialogs.rs` 已移除 General 主题下拉里手工塞入的 fake custom item；Custom 页面改成真实 theme editor：主题名、light/dark base theme 选择、按模式/分组的详细色槽和亮度输入、保存/重置按钮
- `src/terminal/element.rs` 已把 terminal font brightness 语义改成“当前激活 custom theme + 当前 mode”；`src/main.rs` 已在启动时加载并监听用户 theme 目录
- `locales/en.yml` 和 `locales/zh-CN.yml` 已补 custom theme 保存、继承和 base theme 文案说明

## 验证

- 已完成：`rustfmt --edition 2024 --config skip_children=true src/app/theme.rs src/app/mod.rs src/app/dialogs.rs src/session/config.rs src/terminal/element.rs src/main.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，共 13 个测试通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI 手工验证未执行；仍需本机在设置页里目视确认“保存后立即进 theme list、后续可直接重选”的交互效果

## 风险与阻塞

- 风险一：若用户把 draft 改名后再保存，当前实现会把旧名字对应的 theme file 保留为历史主题条目，行为偏“追加保存”而不是“重命名覆盖”
- 风险二：Custom 页面旧实现目前已被注释旁路，功能不受影响；后续若要进一步清理源码，可再做一次纯删除收口
- 风险三：`block v0.1.6` 的 future-incompat warning 仍是仓库既有状态，不属于本轮 custom theme 改动

## 下一步

- 本轮若继续推进，优先做 GUI 手工验证：保存一个 custom theme，确认它进入 light/dark theme list，重启后仍可直接选择，并根据实际可用性再决定是否补 theme value 预览或删掉被旁路的旧页面代码

## 最后更新时间

- 2026-07-07 15:47 CST
