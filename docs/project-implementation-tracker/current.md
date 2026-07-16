# 当前项目实施记录

## 当前目标

- 目标：支持将一个工作区会话组移至独立应用窗口，保持终端与 SFTP 后端连接不中断。
- 交付物：可迁移事件路由、Tab 的“移到新窗口”和“返回主窗口”入口、独立 Terminal 窗口、测试与验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/events.rs`、`src/app.rs`、`src/app/lifecycle/`、`src/app/actions/session.rs`、`src/app/views/tab_bar.rs`、`src/app/workspace.rs`、`locales/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：跨窗口拖放手势、会话持久化 schema、跨进程窗口恢复、`Cargo.toml` / `Cargo.lock`。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 明确会话组迁移、事件路由与关闭所有权边界 | 架构与锁定 GPUI API 静态核对 | 迁移整个 `TabGroup`；不复制或重连 backend |
| P2 | completed | 实现可迁移的 backend 事件路由和独立窗口创建 | 路由 / runtime 聚焦测试、`cargo check` | 既有 backend sender 按 tab/group id 路由至当前所属窗口 |
| P3 | completed | 增加 Tab 入口并迁移会话组的 UI/资源所有权 | 工作区测试、手工 GUI 验收 | 独立窗口仅显示 Terminal，可返回主窗口；活跃 SFTP 传输禁止迁出 |
| P4 | completed | 完成格式化、回归和跟踪记录 | `cargo test --quiet`、差异检查、tracking validator | 保留 GUI 手工验证项 |

## 已完成

- 已确认 GPUI 的 `App::open_window` 支持在同一进程创建 `AxShell` 窗口。
- 已确认一个 workspace tab 对应整个 `TabGroup`，可含多个分屏终端与关联 SFTP；迁移粒度必须是整个组。
- 已确认当前关闭回调会关闭所属窗口全部 backend，且 backend 事件队列由窗口私有；因此必须转移资源与事件路由，不能复制 Tab。
- 已完成按终端 tab / SFTP group id 的事件路由；迁出及回迁会重注册当前接收窗口，运行中的 backend 不需重连。
- 已完成整个 `TabGroup` 的资源转移，包括 pane、terminal、空闲 SFTP handle、连接进度、密码提示与传输记录；失败时恢复源窗口所有权。
- 已完成独立窗口：只承载 Terminal，隐藏侧边栏、tab 栏、SFTP、设置和监控；窗口及标题栏显示 `名称 #实例号`，标题视觉居中。
- 已完成“返回主窗口”标题栏按钮和 Window 菜单 action；主窗口不存在时保留独立窗口。

## 验证

- 已完成：环境记录、实施记录、项目地图、Rust 格式化；事件路由、runtime 和 workspace 聚焦测试；`cargo check`；完整 `cargo test --quiet`（213 项）；`git diff --check`；tracking docs validator。
- 未完成：真实桌面 GUI 验收。

## 风险与阻塞

- 无阻塞；真实桌面环境仍需确认迁出/回迁不丢终端输出、关闭独立窗口的 backend 关闭语义，以及 macOS/Windows/Linux 标题栏中的回迁入口。

## 下一步

- 在桌面 GUI 验收迁出、回迁、关闭和标题居中交互。

## 最后更新时间

- 2026-07-16 16:05 +0800
