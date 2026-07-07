# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`src/app/ui.rs`，`src/terminal/mod.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`examples/`，`src/app/`，`src/backend/`，`src/terminal/`，`docs/`，`Cargo.toml`，`.cargo/`
- 排除：`.git/`，`target/`，`assets/` 批量资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `src/terminal/` | 终端渲染、关键词高亮、输入与快照缓存 | 调整关键词高亮、URL/IP/端口识别、终端前景色覆盖规则时 | 当前核心改动点已切到 `src/terminal/highlight.rs` 的关键词 matcher，`src/terminal/element.rs` 继续负责原生颜色避让 |
| `src/app/` | 应用级搜索、窗口与工作区 UI | 需要确认搜索高亮优先级、当前 tab/viewport 映射或 UI 接线时 | 本轮只读 `src/app/search.rs` 以确认搜索高亮继续压过关键词高亮 |
| `src/session/` | 配置持久化和设置项出口 | 调整关键词高亮开关或相关配置读取时 | 本轮只读 `src/session/config.rs` 确认 `keyword_highlight` 开关逻辑未变 |
| `docs/` | 环境审计与实施跟踪记录 | 切换当前任务语境、记录实现计划和验证结果时 | 本轮需要刷新 `project-map.md`、`current.md` 与环境记录 |
| `Cargo.toml` / `.cargo/config.toml` | Rust 版本、依赖和 cargo alias | 确认构建/测试命令边界时 | 当前以 `cargo check` 和本地测试为主，不涉及依赖变更 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/terminal/element.rs` | 终端文本最终渲染层 | `cell_run_style`，`is_default_bg`，`color_to_hsla` | 修改关键词高亮覆盖条件，并保留搜索高亮优先级 |
| `src/terminal/highlight.rs` | 关键词/IP/端口/URL 匹配规则 | `highlight_cells`，`highlight_keywords`，`highlight_http_codes` | 修改关键词边界判定，收紧短词误报，同时保持 HTTP code / IP / URL / port 的专用 matcher |
| `src/terminal/mod.rs` | 终端视口快照与高亮缓存 | `RenderCell`，`RenderSnapshot`，`render_snapshot` | 确认关键词高亮缓存基于当前 viewport cells |
| `src/app/search.rs` | 搜索命中高亮与定位 | `search_highlight_map`，`perform_search` | 保证搜索高亮继续覆盖关键词高亮 |
| `src/session/config.rs` | 关键词高亮配置出口 | `keyword_highlight`，`set_keyword_highlight` | 确认开关和默认值不需变更 |

## 常用定位

- `rg -n 'keyword_highlight|highlight_cells|search_highlight_map|color_to_hsla' src`
- `rg -n 'NamedColor::Foreground|NamedColor::Background|Flags::INVERSE' src/terminal`
- `cargo check`

## 忽略与未索引

- `examples/` 未索引：本轮不涉及 `dev-reload`、示例程序或启动链路
- `src/backend/` 未索引：本轮不改 PTY/SSH 后端协议，只改渲染层高亮覆盖策略
- `assets/`、`target/` 未索引：本轮不涉及主题资源或构建产物

## 刷新规则

- 刷新触发：高亮渲染逻辑、搜索高亮优先级、终端配置开关、关键入口文件或本轮任务范围发生变化时刷新
- 最近依据：`src/terminal/highlight.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`src/app/search.rs`，`src/session/config.rs` 的实读结果

## 最后更新时间

- 2026-07-07 11:37 CST
