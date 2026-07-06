# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`src/session/mod.rs`，`src/backend/ssh.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`Cargo.toml`，`README.md`，`src/app/`，`src/session/`，`src/terminal/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 排除：`.git/`，`.cargo/registry/`，`.cargo/git/`，`target/`，`assets/fonts/`，`assets/icons/`，`locales/`，生成产物与外部依赖源码缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `Cargo.toml` | 仓库依赖、Rust 版本和包元数据 | 确认技术栈、版本约束、依赖能力或 crate / bin 命名时 | 本轮需要修改 crate 名、作者/维护者文本和打包产物路径 |
| `src/main.rs` | 应用入口与 action 注册 | 需要确认全局快捷键或应用启动流时 | 标签交互问题通常不在这里 |
| `src/app/` | 主界面状态、布局和交互实现 | 修复 UI、面板、弹窗行为，或迁移应用结构体/窗口文案时 | 本轮需要改 `AxAshell` 结构体名、窗口标题和 UI 文案 |
| `src/session/` | 会话配置与连接入口 | 需要确认会话配置、同步文件名、SSH 会话数据来源时 | 本轮需要改配置目录名与同步文件名 |
| `src/backend/` | 本地终端与 SSH 后端实现 | 修复连接、认证、协议兼容性或终端进程环境变量时 | 本轮需要改 `TERM_PROGRAM` 等运行时标识 |
| `src/terminal/` | 终端渲染和鼠标键盘输入 | 问题涉及终端区选择、滚动、链接 hover 或主视图类型名时 | 本轮需要同步 `AxAshell` 类型引用 |
| `assets/` | 桌面入口、图标和主题等打包资源 | 需要改应用资源名、桌面入口或 bundle 资源时 | 本轮需要重命名 desktop / icon 资源并修正引用 |
| `.github/workflows/` | CI、发布和打包流水线 | 需要改发布产物名、bundle 名或 Homebrew cask 时 | 本轮需要统一发布产物与下载链接中的应用名 |
| `scripts/` | 本地打包和辅助脚本 | 需要改 bundle 名、资源复制路径或安装脚本时 | 本轮需要改 macOS 打包脚本中的 app / icon / bundle id |
| `docs/project-env-audit/` | 项目环境当前态与历史 | 开工前预检或环境事实变化时 | 需保持当前态 |
| `docs/project-implementation-tracker/` | 本轮实施计划、地图与变更历史 | 真实施工前后记录计划和结论时 | 本轮需刷新到 current contract |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `Cargo.toml` | crate 元数据与打包入口 | `name`，`authors`，`package.metadata.deb.assets` | 统一 crate 名、二进制名与安装路径 |
| `src/app/mod.rs` | 主应用结构体与状态中心 | `AxAshell` | 将主应用类型迁移到 `AxAshell` 并同步跨模块引用 |
| `src/app/startup.rs` | 应用启动、日志目录、窗口标题和图标加载 | `AxAshell::new`，`window.set_window_title`，`include_bytes!` | 统一启动时文案、日志目录与图标文件路径 |
| `src/session/config.rs` | 配置目录和同步文件命名 | `config_path`，`default_s3_object_key` | 将配置目录与同步文件名切到 `ax_ashell` 版本 |
| `.github/workflows/release.yml` | 发布产物与 Homebrew cask | `bin`，`STAGE`，`APP`，`CASK_FILE` | 统一发布包名、bundle 名、cask 名与下载链接 |
| `assets/ax_ashell.desktop` | Linux 桌面入口资源 | `Name`，`Exec`，`Icon`，`StartupWMClass` | 需要重命名文件并同步桌面入口字段 |

## 常用定位

- `rg -n --hidden --glob '!.git' "\\bax_ashell\\b|AxAshell|AX_ASHELL" .`
- `rg -n "ax_ashell|AxAshell|AX_ASHELL" Cargo.toml README.md README.en.md src assets scripts .github/workflows`
- `cargo check`

## 忽略与未索引

- `target/`、`.cargo/registry/`、`.cargo/git/` 未索引：属于构建产物或外部依赖缓存，不作为项目源码路由索引
- `assets/fonts/`、`assets/themes/` 未索引：与本轮品牌标识迁移无关
- `examples/` 只按需展开：本轮仅需改 `examples/dev_reload.rs` 中的运行目标名和帮助文案

## 刷新规则

- 刷新触发：crate / bin 命名、资源文件名、发布脚本、关键入口或本轮范围发生变化时刷新
- 最近依据：`rg --files` 全仓清点，结合 `Cargo.toml`、`src/app/`、`src/session/config.rs`、`assets/`、`scripts/package-macos-app.sh` 与 `.github/workflows/release.yml` 的实读结果

## 最后更新时间

- 2026-07-06 16:23 CST
