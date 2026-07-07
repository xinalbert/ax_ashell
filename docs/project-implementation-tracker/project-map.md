# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`src/app/ui.rs`，`src/app/theme.rs`，`src/session/mod.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`src/app/`，`src/session/`，`src/terminal/`，`src/sync/`，`locales/`，`docs/`，`Cargo.toml`
- 排除：`.git/`，`target/`，`assets/` 批量资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `src/session/` | 配置持久化、会话模型和本地配置目录路径 | 改 custom theme 持久化字段、theme file 输出路径、配置兼容逻辑时 | 本轮 custom theme draft/registry file 模型主要在这里落地 |
| `src/app/` | 侧栏、设置页、弹窗、主题与工作区 UI | 调整 Custom 页面、theme list、主题应用逻辑和工作区动作时 | `theme.rs` 负责注册/生成/apply，`dialogs.rs` 已改成 metadata-driven custom theme editor，`mod.rs` 负责动态输入状态 |
| `src/terminal/` | 终端渲染、颜色和交互 | custom theme brightness 或终端颜色语义需要与主题联动时 | 本轮只改 `element.rs` 的 custom brightness 语义，不改 PTY 或输入链路 |
| `src/sync/` | 会话配置加密同步 payload | 判断新增会话字段是否会自动进入同步上传/下载时 | 本轮预计不改传输逻辑，只依赖 `Session` 序列化扩展 |
| `src/main.rs` | 应用初始化入口 | 增加 custom theme watch/load、补入口初始化顺序时 | 本轮需要在 `gpui_component::init` 后加载并 watch 用户 theme 目录 |
| `locales/` | 中英文界面文案 | 新增 custom theme 分组、提示、保存说明和错误消息时 | 需要同步 `en.yml` 和 `zh-CN.yml` |
| `docs/` | 环境审计和实施跟踪 | 记录本轮 custom theme 注册化和验证边界时 | 本轮需刷新 env audit 与 project tracker；用户文档可视结果稳定后再决定是否补充 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/session/config.rs` | 本地配置文件模型、路径和 getter/setter | `ConfigFile`，`ConfigStore::load/save`，config path helpers | 新增 custom theme draft、registry file 路径和兼容迁移 |
| `src/app/theme.rs` | 主题注册、当前主题应用和 custom theme 逻辑 | `load_embedded_themes`，`load_user_themes`，`apply_theme_preferences`，`save_custom_appearance` | 本轮已改成“真实 ThemeConfig + theme file 持久化 + registry 即时应用/监听” |
| `src/app/mod.rs` | 全局 UI 状态和设置页输入实体 | `AxAshell` fields，`new(...)`，`on_input_event` | custom theme 编辑器字段数量会增多，输入状态与回车保存链路在这里 |
| `src/app/dialogs.rs` | 设置页渲染 | `render_settings_page`，Custom page groups/items | General 页主题下拉现只走 registry；Custom 页已扩成基底 preset + light/dark 分组色槽编辑 |
| `src/terminal/element.rs` | terminal 前景色与高亮渲染 | `layout_grid`，`cell_run_style`，`color_to_hsla` | 把亮度语义从“全局 custom 数值”收敛到“当前激活的 custom theme” |
| `src/main.rs` | 应用启动初始化顺序 | `main()` | 新增用户 theme 文件初始加载和 watch 入口 |

## 常用定位

- `rg -n 'custom_theme|ThemeRegistry|load_embedded_themes|apply_theme_preferences|save_custom' src/app src/session src/main.rs`
- `rg -n 'custom_font_brightness|color_to_hsla|layout_grid' src/terminal`
- `rg -n 'ThemeConfig|ThemeSet|try_parse_color|watch_dir|default_light_theme|default_dark_theme' ~/.cargo/git/checkouts/gpui-component-*`
- `cargo check`

## 忽略与未索引

- `src/backend/` 未索引：本轮不改 SSH/PTTY 后端协议或连接实现
- `assets/`、`target/` 未索引：本轮不涉及静态资源或构建产物

## 刷新规则

- 刷新触发：custom theme 持久化模型、theme file 注册策略、设置页字段分组、theme list 行为、terminal 亮度语义或用户文档范围发生变化时刷新
- 最近依据：`src/main.rs`，`src/app/theme.rs`，`src/app/mod.rs`，`src/app/dialogs.rs`，`src/session/config.rs`，`src/terminal/element.rs` 与 `gpui-component` theme registry/schema 的实读结果；已结合 `cargo check` / `cargo test` 验证当前实现

## 最后更新时间

- 2026-07-07 15:47 CST
