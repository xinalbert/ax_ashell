# 当前项目实施记录

## 当前目标

- 目标：按功能拆分项目中的超大 Rust 源文件，并在必要时继续细分，降低单文件复杂度与后续维护/回归成本
- 交付物：新的模块文件布局、更新后的 `src/app/` / `src/session/` / `src/sftp/` 等实现拆分、编译/测试验证结果，以及同步的 tracking / env / project-map 记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/`，`src/session/`，`src/sftp/`，`src/backend/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：无必要的业务行为修改、无必要的文案与配置格式变更、超出“结构拆分”范围的功能新增、GUI 手工回归

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 tracking / env 当前态并确认本轮大文件拆分边界 | `docs/` contract 走查，`src/app/` / `src/session/` / `src/sftp/` 结构复核 | 已完成当前范围确认 |
| P2 | completed | 完成 `src/app/mod.rs` 第一批按功能拆分，并评估 `src/app/ui.rs` / `src/app/dialogs.rs` 后续边界 | `cargo check`，关键文件 diff 走查 | `ui.rs` / `dialogs.rs` 仍是 GPUI 长渲染闭包，留待后续按组件边界拆分 |
| P3 | completed | 按功能拆分 `src/session/mod.rs`，必要时继续细分 `src/sftp/` / `src/backend/ssh.rs` | `cargo check`，关键文件 diff 走查 | 已拆出 session pane / saved session 逻辑和 sftp auth / path helper |
| P4 | completed | 完成测试、tracking docs 校验与 `project-map.md` 刷新 | `cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | 已完成全量测试和 tracking 校验 |

## 已完成

- 已读取 `docs/project-implementation-tracker/project-map.md`、`docs/project-env-audit/current.md` 与本月 `changes/2026/07.md`
- 已确认当前最大热点集中在 `src/app/ui.rs`、`src/app/dialogs.rs`、`src/app/mod.rs`、`src/session/mod.rs`、`src/sftp/mod.rs` 与 `src/backend/ssh.rs`
- 已完成第一轮大文件体检，确认问题核心不只是行数大，更是职责混杂和状态集中
- 已将 `src/app/mod.rs` 中的初始化逻辑拆到 `src/app/init.rs`，事件泵与后台事件处理拆到 `src/app/event_loop.rs`
- 已将 `src/app/mod.rs` 中的共享 UI / session 类型拆到 `src/app/types.rs`，工作区、连接进度和布局持久化辅助方法拆到 `src/app/workspace.rs`
- 已将 `src/session/mod.rs` 中的 pane split / focus / group activation / splitter drag 逻辑拆到 `src/session/pane.rs`
- 已将 `src/session/mod.rs` 中的 selector 与 saved session 分组/重命名逻辑拆到 `src/session/saved_sessions.rs`
- 已将 `src/sftp/mod.rs` 中的 SSH/SFTP 认证和 key 解析逻辑拆到 `src/sftp/auth.rs`
- 已将 `src/sftp/mod.rs` 中的远程路径、文件名、mtime、shell quote 和大小格式化 helper 拆到 `src/sftp/path.rs`
- 已刷新 `docs/project-implementation-tracker/project-map.md`，记录新增模块边界和定位命令
- 已完成全量测试和 tracking docs 校验

## 验证

- 已完成：项目地图、当前 env/tracking 记录与大文件结构走查
- 已完成：`rustfmt --edition 2024 src/app/mod.rs src/app/types.rs src/app/workspace.rs src/app/init.rs src/app/event_loop.rs`
- 已完成：`rustfmt --edition 2024 src/session/mod.rs src/session/pane.rs src/session/saved_sessions.rs`
- 已完成：`rustfmt --edition 2024 src/sftp/mod.rs src/sftp/auth.rs src/sftp/path.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，13 个测试全部通过
- 已完成：`project-map.md` 刷新
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI / 运行时手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：本轮会新增较多模块文件并移动大量实现，若拆分边界不稳，容易在 import / visibility / `impl AxShell` 分散时引入编译回归
- 风险二：`src/app/dialogs.rs` 和 `src/app/ui.rs` 的 GPUI 事件闭包很多，拆分时需要避免生命周期、捕获和 helper 可见性变化
- 风险三：`src/session/mod.rs` 与 `src/app/mod.rs` 共用大量状态字段，若过早做数据结构重组，风险会高于纯文件拆分；本轮优先做行为不变的实现搬移
- 风险四：仍保留既有 `block v0.1.6` future-incompat warning，来源于 GPUI / cocoa 传递依赖

## 下一步

- 后续若继续拆分，优先按组件边界处理 `src/app/ui.rs` / `src/app/dialogs.rs`，按配置分区处理 `src/session/config.rs`
- 若继续拆 `src/sftp/mod.rs`，建议先把传输实现和 archive 处理拆出，主命令循环保持薄调度层

## 最后更新时间

- 2026-07-08 10:36 CST
