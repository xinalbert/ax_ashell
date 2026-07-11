# 当前项目实施记录

## 当前目标

- 目标：将实际生效的应用级快捷键统一纳入 Settings 的 `Key Bindings` 页面，消除终端内旧硬编码快捷键绕过配置的问题。
- 交付物：统一快捷键注册表、Settings 完整分组展示、终端内可配置 action 触发、用户文档更新和验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/input/keybinding_recorder.rs`，`src/app/actions/terminal.rs`，`src/app/views/layout.rs`，`src/main.rs`，`src/app/dialogs/settings/keybindings.rs`，`locales/`，`docs/features/workspace*.md`，跟踪文档。
- 不在本轮范围内：修改 SFTP 协议、终端字符编码规则、配置 schema、manifest/lock/release/tag；组件内部导航键仅在不属于应用级快捷键时保持局部处理。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 快捷键现状审查、环境预检和实施边界刷新 | `rg` 检索 `KeyBinding::new`、`bind_keys`、`on_key_down`、`event_matches_action`，源码复核 | 确认 Settings 仅覆盖 `WORKSPACE_ACTIONS`，终端仍有旧硬编码路径 |
| P2 | completed | 单一可配置快捷键注册表和 Settings 分组 | `rustfmt`，源码复核，`cargo check` | workspace action 与 terminal action 元数据已统一供绑定、解绑和设置页使用 |
| P3 | completed | 终端旧硬编码快捷键改为匹配配置 action | `cargo check`，源码复核 | 保留终端字符输入、Enter 重连、搜索 Escape 和 selector 上下/回车等局部控件键 |
| P4 | completed | 用户文档、本地化、验证和跟踪收口 | `git diff --check`，tracking validator，完整测试 | GUI 录制和真实快捷键触发需手工确认 |

## 已完成

- 已读取 `docs/project-env-audit/current.md`、`docs/project-implementation-tracker/current.md` 和项目地图。
- 已确认项目地图覆盖本轮涉及的 `src/app/input/`、terminal action、views layout、Settings keybindings 和用户文档，不需要结构性刷新。
- 已确认 Settings 当前渲染 `WORKSPACE_ACTIONS` 的 20 个 action；`TerminalTabKey` / `TerminalBacktabKey` 和终端内 `Alt` 系列旧快捷键未进入设置。
- 已确认全局 action 监听覆盖 `WORKSPACE_ACTIONS`，但 `on_terminal_key_down()` 内仍存在绕过配置的设置、会话、复制、粘贴、pane focus/split/close 旧路径。
- 已将快捷键元数据收敛为 `CONFIGURABLE_ACTIONS`，Settings 新增 Terminal 分组，并把 `TerminalSendTab` / `TerminalSendBacktab` 纳入配置化 context 绑定。
- 已将终端内旧硬编码的设置、会话、传输、新建 SSH、搜索、侧栏、SFTP、pane focus/split/close、复制和粘贴路径改为通过 `event_matches_action()` 匹配当前配置。
- 已移除 `main.rs` 中固定注册的 `Tab` / `Shift+Tab`，避免设置页出现可配置项但仍残留不可配置默认绑定。
- 已同步中英文 Settings 文案和工作区用户文档，说明终端焦点中的默认快捷键也可在 `Key Bindings` 设置中修改。
- 已完成 `cargo check`、94 项完整测试、`git diff --check` 和 tracking docs validator。

## 验证

- 已完成：快捷键现状审查、施工前环境记录刷新、实施计划刷新、代码实现、相关 `rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- 未完成：GUI 手工确认。

## 风险与阻塞

- 风险一：终端内 `Tab`/`Shift+Tab` 同时是 shell 输入和 GPUI context action；需要作为 Terminal 分组配置，避免破坏普通字符输入路径。
- 风险二：设置页进入时会解绑快捷键，录制/冲突检测必须覆盖新增 Terminal action，防止和 workspace action 冲突。
- 风险三：组件内部控制键如搜索 `Escape`、selector 上下/回车不属于全局应用快捷键，本轮保持局部处理以免过度配置化。
- 风险四：自动化无法确认真实 GUI 中所有平台的快捷键录制与触发效果，需手工覆盖默认值和自定义值。
- 无阻塞。

## 下一步

- 在 GUI 中打开 Settings / Key Bindings，确认 Terminal 分组显示；录制一两个 Terminal 快捷键后确认终端焦点内触发生效，`Tab` / `Shift+Tab` 仍能发送到 shell。

## 最后更新时间

- 2026-07-11 10:22 +0800
