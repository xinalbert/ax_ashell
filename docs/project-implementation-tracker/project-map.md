# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`src/app/ui.rs`，`src/session/mod.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`src/app/`，`src/session/`，`src/sync/`，`locales/`，`docs/`，`Cargo.toml`
- 排除：`.git/`，`target/`，`assets/` 批量资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `src/session/` | SSH 会话模型、表单保存、连接入口 | 改 saved session 持久化字段、表单回填、会话重命名和连接行为时 | 本轮主数据模型和会话分组 helper 在这里落地 |
| `src/app/` | 侧栏、设置页、弹窗和工作区 UI | 调整 SAVED 区布局、组展开/重命名交互、SSH 新建/编辑弹窗时 | `ui.rs` 负责 SAVED 区渲染，`dialogs.rs` 负责 SSH 表单，`mod.rs` 持有输入与界面状态 |
| `src/sync/` | 会话配置加密同步 payload | 判断新增会话字段是否会自动进入同步上传/下载时 | 本轮预计不改传输逻辑，只依赖 `Session` 序列化扩展 |
| `locales/` | 中英文界面文案 | 新增组相关 label、按钮和提示文案时 | 需要同步 `en.yml` 和 `zh-CN.yml` |
| `docs/` | 用户文档、环境审计和实施跟踪 | 记录本轮分组功能、验证边界和使用方式时 | 本轮需刷新环境记录、project tracker 和 user guide |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/session/config.rs` | 会话序列化和本地 `sessions.json` 持久化 | `Session`，`ConfigFile.sessions`，`ConfigStore::sessions/upsert/remove/save` | 给会话增加 `group_name` 字段并保持向后兼容 |
| `src/session/mod.rs` | SSH 表单提交、saved session helper 和连接动作 | `connect_ssh`，`reset_ssh_form`，`load_session_into_form`，`session_detail` | 读取/保存组名，生成分组视图和组重命名 helper |
| `src/app/mod.rs` | UI 状态和输入事件路由 | `AxAshell` fields，`DialogKind`，`on_input_event` | 新增组输入框、组重命名输入和展开态状态 |
| `src/app/dialogs.rs` | 新建/编辑 SSH 弹窗 | `show_ssh_dialog` | 给 SSH 表单增加组输入与已有组下拉加载 |
| `src/app/ui.rs` | 左侧 SAVED 区和折叠侧栏渲染 | `render_sidebar`，`render_collapsed_sidebar` | 展开态和折叠态都按组渲染 SAVED，会话卡片交互和组重命名都在这里接线 |
| `docs/user-guide.md` / `docs/user-guide.en.md` | 用户使用说明 | `### 新建 SSH 会话`，`配置同步` | 补充 SAVED 分组和组选择行为说明 |

## 常用定位

- `rg -n 'Session|sessions\\(|upsert\\(|group_name|connect_ssh|load_session_into_form' src/session src/app`
- `rg -n 'saved|saved session|copy_connection_info|clone|edit|delete' src/app/ui.rs src/app/dialogs.rs`
- `cargo check`

## 忽略与未索引

- `src/backend/` 未索引：本轮不改 SSH/PTTY 后端协议或连接实现
- `src/terminal/` 未索引：本轮不改终端渲染和输入逻辑
- `assets/`、`target/` 未索引：本轮不涉及静态资源或构建产物

## 刷新规则

- 刷新触发：saved session 结构、SAVED 侧栏布局、SSH 表单字段、同步载荷边界或用户文档范围发生变化时刷新
- 最近依据：`src/session/config.rs`，`src/session/mod.rs`，`src/app/mod.rs`，`src/app/dialogs.rs`，`src/app/ui.rs`，`src/app/config_sync.rs`，`src/sync/mod.rs` 的实读结果

## 最后更新时间

- 2026-07-07 12:35 CST
