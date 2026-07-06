# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/backend/ssh.rs`，`src/session/config.rs`，`src/app/startup.rs`，`src/app/dialogs.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`src/backend/`，`src/session/`，`src/app/`，`locales/`，`Cargo.toml`，`Cargo.lock`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 排除：`.git/`，`target/`，`assets/` 批量资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `src/backend/` | SSH / 本地后端连接、认证、终端数据转发和 X11 relay | 调整 russh API、SSH session lifecycle、X11 channel、X server endpoint 或 cookie relay 时 | 本轮核心改动点在 `src/backend/ssh.rs` |
| `src/session/` | 会话模型、代理连接与持久化配置 | SSH 代理连接、session 配置字段或 X11/local X server 设置变化时 | 配置字段仍保留 `xquartz_app_path` key，但语义已泛化为 local X server app path |
| `src/app/` | GPUI 应用状态、设置页、启动逻辑和 UI 事件 | 增加设置项、启动 helper、窗口或主题行为时 | 跨平台 local X server 启动 helper 在 `src/app/startup.rs` |
| `locales/` | 中文/英文界面文案 | 新增设置项、按钮或状态文本时 | X11 设置页文本需双语同步 |
| `Cargo.toml` / `Cargo.lock` | 依赖声明与锁定版本 | 升级 crate、确认 transitive dependency 变化时 | 当前使用 `russh 0.62.2` |
| `docs/project-implementation-tracker/` | 本轮实施计划、地图、研究记录与变更历史 | 真实施工前后记录计划和结论时 | 本轮切到 X11 本地 X server 平台适配任务语境 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/backend/ssh.rs` | SSH 连接、认证、主 shell、metrics session 和 X11 relay | `run_ssh`，`X11ForwardingState`，`local_x11_endpoints`，`LocalX11Auth`，`server_channel_open_x11`，`relay_x11_channel`，`read_rewritten_x11_setup` | 实现或调试 X11 forwarding、Windows TCP endpoint、Linux Xwayland、cookie/no-auth rewrite |
| `src/session/config.rs` | Session 与全局配置存储 | `default_local_x_server_app_path`，`default_local_x_display`，`x11_forwarding_enabled`，`x11_launch_xquartz`，`xquartz_app_path` | 读取或修改 X11/local X server 开关、默认路径和持久化字段 |
| `src/app/startup.rs` | 启动环境、日志、窗口和外部启动 helper | `launch_local_x_server_app` | macOS XQuartz、Windows VcXsrv/Xming、Linux 自定义 X server 启动 |
| `src/app/dialogs.rs` | 设置页和各类弹窗渲染 | `settings_x11`，`xquartz_app_path_input` | 增加或调整 Proxy/X11/local X server 设置控件 |
| `src/app/mod.rs` | 应用主状态和输入状态初始化 | `xquartz_app_path_input` | 新增设置页输入框状态时读取 |
| `locales/en.yml` / `locales/zh-CN.yml` | 设置页文案 | `settings_x11`，`x11_launch_xquartz`，`xquartz_app_path` | 新增或修正 X11 UI 文案时同步 |
| `docs/project-implementation-tracker/research.md` | 外部事实依据 | `RFC 4254`，`OpenSSH` | 确认 fake cookie 替换、安全边界和 russh 最新版本 |

## 常用定位

- `rg -n 'request_x11|server_channel_open_x11|X11ForwardingState|local_x11_endpoints|LocalX11Auth|read_rewritten_x11_setup|xauth|DISPLAY' src/backend/ssh.rs`
- `rg -n 'x11|XQuartz|xquartz|VcXsrv|Xming|Xwayland|local X server' src locales`
- `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`

## 忽略与未索引

- `assets/` 未索引：图标、字体和主题资源不是本轮 X11 平台适配对象
- `examples/`、`.cargo/` 未索引：本轮不改开发辅助命令
- `target/` 未索引：属于构建产物

## 刷新规则

- 刷新触发：X11 relay、local X server 平台适配、XQuartz/VcXsrv/Xming/Xwayland 设置、SSH 依赖版本、russh API 适配点或本轮任务范围发生变化时刷新
- 最近依据：`src/backend/ssh.rs`、`src/session/config.rs`、`src/app/startup.rs`、`src/app/dialogs.rs`、`src/app/mod.rs`、`locales/en.yml`、`locales/zh-CN.yml` 的实读结果

## 最后更新时间

- 2026-07-07 12:05 CST
