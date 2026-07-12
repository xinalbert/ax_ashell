# 当前项目实施记录

## 当前目标

- 目标：为已保存 SSH 会话和分组提供不携带密码、私钥、passphrase 或代理密码的导出/导入，并在原生菜单栏与 sidebar 右键菜单中添加入口。
- 交付物：安全 share JSON payload；导出全部 saved sessions；从文件导入并默认合并到本地 sessions；菜单栏 File 入口；sidebar group/session 右键导出入口；action 接线；中英文文案；聚焦测试与常规验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/input/app_menu.rs`，`src/app/input/keybinding_recorder.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/sftp.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/main.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：系统 keychain、加密备份、导出密码或私钥、独立空分组持久化、远端同步语义调整、修改 SSH backend、修改 `Cargo.toml` / `Cargo.lock`、真实 GUI 验收。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复核 saved session/group 存储、菜单栏 action 和文件选择现有模式 | 读取 `session.rs`、`config/store.rs`、`saved_sessions.rs`、`app_menu.rs`、`layout.rs` | 分组来自 `Session.group_name`；菜单栏由 GPUI actions 接线 |
| P2 | completed | 实现无密钥 share payload、导出文件、导入合并和菜单栏 action | `rustfmt --edition 2024`、聚焦测试、`cargo check` | 导出不包含 secret 字段；导入默认生成缺失/冲突 id |
| P3 | completed | 完成文案、跟踪记录和验证收口 | `cargo test --quiet`，`git diff --check`，tracking validator | GUI 文件选择仍需手工确认 |
| P4 | completed | 在 sidebar group/session 右键菜单加入对应范围导出 | `rustfmt --edition 2024`、聚焦测试、`cargo check`、`cargo test --quiet` | 复用同一个无凭据 share JSON |

## 已完成

- 已读取 `AGENTS.md`、项目本地 fast hover skill、环境记录、当前实施记录和项目地图。
- 已确认 `Session` 包含 `group_name`、连接参数、密码、私钥、passphrase 和代理密码字段；本轮导出必须使用专用 share payload 或显式清理 secret 字段。
- 已确认本地配置的 `sessions` 是唯一保存会话集合，分组在 UI 中由 `group_name` 聚合生成，不存在独立空分组持久化模型。
- 已确认原生菜单栏位于 `src/app/input/app_menu.rs`，菜单 action 在 `src/app/views/layout.rs` 绑定到 `AxShell`。
- 已新增 `ax-shell-saved-sessions` share JSON，条目只包含 id、name、group_name、host、port、user、auth 和非敏感 proxy 参数，不包含 password、private_key_path、private_key_inline、passphrase 或 proxy_password。
- 已实现导出全部 saved SSH 到 JSON 文件；已实现从 JSON 文件导入、按连接资料去重、id 冲突时生成新 id、导入后保存配置并展开导入的非空分组。
- 已在 File 菜单加入 `Import Saved SSH...` 和 `Export Saved SSH...`，并在 root render 绑定对应 action。
- 已同步中英文状态文案。
- 已刷新项目地图中 `src/app/input/`、`src/app/input/app_menu.rs` 和 `src/app/actions/saved_sessions.rs` 的导入/导出路由说明。
- 已在 saved session 右键菜单加入 `Export SSH`，导出当前单条 SSH。
- 已在 sidebar saved group 行的展开态和折叠态右键菜单加入 `Export Group`，导出对应分组下的 SSH；复用同一个无凭据 share JSON。
- 已补充 group 过滤测试，确认按归一化 group 名称筛选。

## 验证

- 已完成：相关源码路径复核；确认不需要联网、不使用多 agent、不新增依赖、不修改配置 schema；受影响 Rust 文件 `rustfmt --edition 2024`；`cargo test --quiet saved_sessions -- --nocapture`；`cargo check`；fast hover/context 静态审计；完整 `cargo test --quiet`；`git diff --check`；tracking validator。
- 未完成：真实 GUI 菜单栏和 sidebar 文件选择手工确认。

## 风险与阻塞

- 风险：导入/导出文件选择属于真实 GUI 交互，自动化只能覆盖 payload 清理、解析与合并策略。
- 风险：本轮不保存也不导出 secret，导入到其他机器后需要用户重新输入密码或配置密钥路径。
- 无阻塞。

## 下一步

- 在真实 GUI 中确认 File 菜单导入/导出、sidebar 单条 SSH 导出、sidebar 分组导出和导入后列表刷新。

## 最后更新时间

- 2026-07-12 12:02 +0800
