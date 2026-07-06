# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/mod.rs`，`src/app/ui.rs`，`src/app/dialogs.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`src/app/`，`src/session/`，`locales/`，`Cargo.toml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 排除：`.git/`，`target/`，`assets/` 批量资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `src/app/` | 主应用状态、设置页、工作区和 UI 渲染 | 需要改设置页、标题栏、侧边栏、主题、同步或主工作区行为时 | 本轮核心改动点在 `src/app/dialogs.rs` |
| `src/session/` | 会话模型与持久化配置 | 需要新增、迁移或展示配置字段时 | 本轮读取 `ConfigFile` 字段和默认值，不计划修改存储结构 |
| `locales/` | 中英文 UI 文案 | 设置项标题、分组标题或提示语变化时 | 本轮新增配置中心说明文案 |
| `docs/project-implementation-tracker/` | 本轮实施计划、项目地图与变更历史 | 真实施工前后记录计划和结论时 | 本轮切到设置页 `Custom` 配置中心任务语境 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/app/ui.rs` | 主工作区渲染和终端事件绑定 | `render_terminal_panel`，`WorkspacePage::Settings` | 修复设置页焦点、输入和终端事件抢占 |
| `src/terminal/element.rs` | 终端网格渲染和颜色转换 | `color_to_hsla`，`ansi_index_color`，`named_color`，`cell_run_style` | 调整 terminal 前景色亮度应用范围 |
| `src/app/dialogs.rs` | 设置页和弹窗渲染 | `render_settings_page`，`SettingPage`，`SettingGroup` | 调整设置页导航、Custom 页面、theme 字段布局和配置 key/default 展示 |
| `src/app/theme.rs` | 主题应用与自定义覆盖 | `apply_theme_preferences`，`apply_custom_theme`，`save_custom_appearance` | 处理自定义主题选项、保存和覆盖色应用 |
| `src/app/mod.rs` | 应用状态与输入状态 | `custom_theme_name_input`，`on_input_event` | 接入自定义主题名称输入与回车保存 |
| `src/session/config.rs` | 配置存储与默认值 | `ConfigFile`，`Default for ConfigFile`，`ConfigStore` | 梳理配置文件字段、默认值和运行态字段 |
| `locales/en.yml` | 英文 UI 文案 | `settings_custom_*`，`custom_*` | 补齐英文配置中心说明 |
| `locales/zh-CN.yml` | 中文 UI 文案 | `settings_custom_*`，`custom_*` | 补齐中文配置中心说明 |

## 常用定位

- `rg -n 'color_to_hsla|ansi_index_color|named_color|custom_font_brightness|cell_run_style' src/terminal src/app src/session`
- `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`

## 忽略与未索引

- `assets/` 未索引：图标、字体和主题资源不是本轮设置页结构调整对象
- `examples/`、`.cargo/` 未索引：本轮不改开发辅助命令
- `target/` 未索引：属于构建产物

## 刷新规则

- 刷新触发：设置页入口、主 UI 页面态、焦点事件绑定、terminal 颜色转换、theme 配置字段、locales 文案或本轮任务范围发生变化时刷新
- 最近依据：`src/terminal/element.rs`、`src/app/ui.rs`、`src/app/dialogs.rs`、`src/session/config.rs`、`src/app/theme.rs`、`locales/en.yml`、`locales/zh-CN.yml` 的实读结果

## 最后更新时间

- 2026-07-06 21:42 CST
