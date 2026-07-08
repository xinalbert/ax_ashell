# 当前项目实施记录

## 当前目标

- 目标：将自定义主题里的亮度控制收口为终端显示亮度，并把允许范围调整为 0.60-1.20
- 交付物：`src/app/theme.rs` 中移除 theme 生成阶段的全局亮度改写、`src/session/config.rs` 中的亮度夹取范围、`locales/` 中的设置文案和更新后的实施/环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/theme.rs`，`src/session/config.rs`，`locales/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：新增配置键迁移、改变 terminal 颜色算法、改变页面级主题字段、GUI 手工回归

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮环境预检和实施计划刷新 | tracking docs validator | 当前工作区已有菜单栏和光标未提交改动，本轮只追加亮度作用域收口 |
| P2 | completed | 亮度作用域从 theme 生成收口到 terminal 渲染 | `cargo check`，`cargo test` | 已移除 `apply_brightness` 对 colors/highlight 的改写，保留 terminal 前景色亮度逻辑 |
| P3 | completed | 亮度范围和设置文案调整为 0.60-1.20 / 终端字体亮度 | `cargo check` | 保持既有配置字段名，避免配置迁移 |
| P4 | completed | 格式化、编译、测试和文档校验 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 设置页和终端视觉效果留作手工验证 |

## 已完成

- 已确认 `font_brightness` 当前同时参与 custom theme 生成和 terminal 前景色渲染
- 已确认用户期望是控制命令行显示部分，而不是整个页面
- 已确认本轮不新增配置键，继续复用现有 `font_brightness` 字段并改变语义文案
- 已从 custom theme 生成阶段移除全局亮度改写，避免影响页面级 theme colors/highlight
- 已将结构化 custom theme 和旧兼容 `custom_font_brightness` 的夹取范围统一为 0.60-1.20
- 已将设置文案改为 `终端字体亮度` / `Terminal Font Brightness`

## 验证

- 已完成：源码级确认亮度当前有 theme 生成和 terminal 渲染两条路径
- 已完成：`rustfmt --edition 2024 --config skip_children=true src/app/theme.rs src/session/config.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，18 个测试全部通过
- 已完成：tracking docs validator
- 未完成：GUI 设置页和终端亮度视觉验证

## 风险与阻塞

- 阻塞：无
- 风险一：已有用户配置中大于 1.20 的值会在读取或保存时被夹取到 1.20，属于本轮预期变化
- 风险二：不改配置键名会减少迁移成本，但旧文档或用户记忆里的“字体亮度”需要通过 UI 文案纠正
- 风险三：GUI 中设置页提示和 terminal 视觉效果仍需要手工确认

## 下一步

- 用户可在 GUI 中确认设置页文案，以及亮度调整只影响 terminal 输出文字、不影响页面整体主题颜色

## 最后更新时间

- 2026-07-08 15:23 +0800
