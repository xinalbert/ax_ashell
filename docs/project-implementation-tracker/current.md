# 当前项目实施记录

## 当前目标

- 目标：按源码目录审查结论逐项收敛跨领域类型、配置职责和 app 物理模块树，使文件内容与所属模块一致并保持现有行为。
- 交付物：全应用事件模块、归位后的 SFTP/transfer 状态模型、清晰的 session/config 边界、独立 proxy/platform 实现、现代 app 子模块入口、更新后的项目地图和回归验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/main.rs`，`src/events.rs`，`src/monitoring.rs`，`src/platform.rs`，`src/platform/`，`src/app.rs`，`src/app/`，`src/backend.rs`，`src/backend/`，`src/config.rs`，`src/config/`，`src/session.rs`，`src/sftp.rs`，`src/sftp/`，`src/terminal.rs`，`src/terminal/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：改变终端、SSH、SFTP、同步或监控业务行为；修改配置 schema、依赖、manifest/lock、release/tag；重新设计 GUI 视觉或交互。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 模块边界审查、目标目录映射和环境预检记录 | 基线 `cargo check`、`git diff --check`、源码调用点统计 | 先迁移低耦合类型，再处理大量路径变更 |
| P2 | completed | `src/events.rs` 及归位后的 SFTP UI/transfer 模型 | `rustfmt`、terminal/SFTP 定向测试、`cargo check` | 事件总线脱离 terminal，SFTP 类型不再由 terminal 所有 |
| P3 | completed | session/config 类型所有权与兼容层清理 | `rustfmt`、配置/session 定向测试、`cargo check` | session 只保留连接领域类型 |
| P4 | completed | `config/store.rs` 拆分及 proxy/platform 运行时职责迁移 | `rustfmt`、proxy/X11/配置测试、`cargo check` | 不改变配置文件字段和网络行为 |
| P5 | completed | 真实 app 子模块入口、单文件目录压平和 core 类型分发 | `rustfmt`、app 相关定向测试、`cargo check` | 移除 `#[path]` 兼容声明，不新增 `mod.rs` |
| P6 | completed | `system.rs` 命名收敛、项目地图刷新和完整回归 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI 行为不变，无新增手工交互检查项 |

## 已完成

- 已完成源码目录、模块声明、公开路径、文件规模和主要调用点审查。
- 已确认主要边界问题：terminal 承载全应用事件/SFTP 类型，session 混入配置模型，config 执行 proxy/X server 运行时逻辑，app 物理目录依赖 `#[path]`，`app/core/types.rs` 成为跨功能类型集合。
- 已完成施工前环境预检；工作树基线干净，`cargo check` 和 `git diff --check` 通过。
- 已确定逐项实施顺序，不联网、不使用多 agent、不修改依赖或配置 schema。
- 已新增 `src/events.rs`，保持 256 条 Tokio 有界队列和既有事件载荷；terminal backend 只保留命令、发送端和关闭控制。
- 已将 `Transfer`、`TransferInfo`、`TransferState`、`TransferType` 迁到 `src/sftp/model.rs`，保留旧 `Cancelled` 反序列化兼容。
- 已将 `SftpUiState` 迁到 `src/app/sftp.rs`，`TerminalTab` 不再依赖 SFTP 数据模型。
- 已将 `Session`、`AuthMethod`、`SshConnectionMode` 和连接模式排序直接收敛到 `src/session.rs`。
- 已新增 `src/config/model.rs` 承载窗口、标题栏、光标和 custom theme 配置类型；全部调用点已改用直接所有权路径。
- 已删除 `src/session/config.rs` 和 `src/session/model.rs`，项目不再通过兼容层混用 session/config 类型。
- 已新增 `src/backend/proxy.rs`，通过 `ConfigStore` getter 解析 session/env/global proxy 并执行 SOCKS5/HTTP/direct 连接。
- 已新增 `src/platform.rs` 与 `src/platform/x_server.rs`，承载本地 X Server 路径、DISPLAY、Windows 空闲 display 和启动参数逻辑。
- 已将 `ConfigFile`、默认值和规范化规则迁到 `src/config/model.rs`；`src/config/store.rs` 从 1649 行降到 935 行，只保留持久化、迁移和访问器。
- 已新增 `src/app/input.rs` 和 `src/app/lifecycle.rs` 作为真实父模块入口，`src/app.rs` 不再使用 `#[path]`。
- 已将 constants/config sync/search/workspace 单文件目录压平为 `src/app/constants.rs`、`config_sync.rs`、`search.rs`、`workspace.rs`。
- 已把 `app/core/types.rs` 分发到 `pane.rs`、`workspace.rs`、`sftp.rs`、`terminal.rs`、`session_ui.rs` 和 `dialogs.rs`，并将 SearchState 与字体 helper 合并到实际功能文件。
- 已将 `src/system.rs` 更名为 `src/monitoring.rs`，全部调用点改用 monitoring 领域路径。
- 已刷新项目地图，清除旧 session/config、terminal transfer、app `#[path]` 和 system 路径记录。

## 验证

- 已完成：环境预检；P2-P5 各阶段格式化、编译和定向测试；最终 `rustfmt`、`cargo check`、`cargo test --quiet`（78 项）、`git diff --check` 和 tracking docs validator。
- 未完成：无自动化验证缺口；Windows 专用 X Server 测试和真实 proxy/X11 联机受当前平台/外部环境限制未执行。

## 风险与阻塞

- 风险一：`BackendEvent` 被 terminal、SSH、SFTP、sync 和 app event loop 共用；迁移时必须保持有界队列容量和发送语义。
- 风险二：`session::config` 有大量内部调用点；必须先建立直接所有权出口，再删除兼容层。
- 风险三：config 拆分包含平台条件编译和 proxy trait object；需逐阶段编译，避免一次性产生难定位错误。
- 剩余风险一：Windows 专用 display/启动参数测试带 `target_os = "windows"`，当前 macOS 主机未执行；代码为原逻辑纯迁移。
- 剩余风险二：proxy/X11 真实联机未执行；编译和现有 backend 测试已通过。
- 无阻塞；未修改业务协议、配置 schema、依赖或 GUI 交互。

## 下一步

- 本轮源码模块边界治理完成，可进入 review/commit；未按用户请求创建 commit。

## 最后更新时间

- 2026-07-10 21:27 +0800
