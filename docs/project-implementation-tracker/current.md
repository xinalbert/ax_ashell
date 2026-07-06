# 当前项目实施记录

## 当前目标

- 目标：让 `custom_font_brightness` 调节真正作用到 terminal 内容前景色，包括 ANSI、256 色和 truecolor 输出
- 交付物：更新后的 `src/terminal/element.rs`、通过的格式化/编译验证、同步后的实施跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/element.rs`，`src/session/config.rs`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：配置字段迁移、主题渲染逻辑、SSH / SFTP 逻辑、release workflow、README 改写

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位 terminal 前景色转换链路和亮度未覆盖的颜色类型 | 源码检查 `src/terminal/element.rs` | `NamedColor::Foreground` 生效，ANSI/Indexed/Spec 未生效 |
| P2 | completed | 在 terminal 颜色转换阶段对前景色统一应用 `custom_font_brightness` | 源码检查 `src/terminal/element.rs` | 不调整背景色，避免破坏背景和 selection |
| P3 | completed | 格式化、编译与跟踪文档校验 | `rustfmt --edition 2024 --config skip_children=true src/terminal/element.rs` 通过；`cargo check` 通过；tracking docs 校验通过 | GUI 手工验证需用户运行应用确认 |

## 已完成

- 读取 `src/terminal/element.rs`，确认 terminal 默认前景色走 `cx.theme().foreground`，会受 custom theme 亮度影响
- 确认 `AnsiColor::Spec`、`AnsiColor::Indexed` 和多数 `NamedColor` 使用硬编码颜色，当前不会受亮度调节影响
- 确认背景色也走同一转换函数，需要只对 foreground 路径应用亮度，避免背景色被一起调整
- 修改 `src/terminal/element.rs`，将 `custom_font_brightness` 应用到 terminal 前景色、ANSI/256 色、truecolor、关键词/搜索高亮以及自定义 block 的最终前景色

## 验证

- 已完成：源码定位；`rustfmt --edition 2024 --config skip_children=true src/terminal/element.rs`；`cargo check`；tracking docs 校验
- 未完成：GUI 手工验证

## 风险与阻塞

- 只应调节 terminal 前景色，不调节背景色、selection、光标色和边框色
- 高亮文字颜色已按前景色处理；GUI 中仍需确认极端亮度下 ANSI 色可读性

## 下一步

- 在应用中打开 Custom 主题，将 `custom_font_brightness` 调到低/高值，观察彩色 `ls`、`grep --color` 或 truecolor 测试输出

## 最后更新时间

- 2026-07-06 21:51 CST
