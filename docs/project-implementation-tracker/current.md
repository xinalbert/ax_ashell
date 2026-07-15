# 当前项目实施记录

## 当前目标

- 目标：让 SFTP 打开行为可预测且只加载一次初始目录。
- 交付物：保存会话的远端目录恢复、固定路径优先级、显式“打开终端当前目录”动作和自动化验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/config/model.rs`、`src/config/store.rs`、`src/app.rs`、`src/app/actions/sftp.rs`、`src/app/workspace.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/init.rs`、`src/app/views/sftp_panel.rs`、`src/sftp/worker.rs`、`src/sftp/worker/runtime.rs`、`locales/`、`docs/features/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：`Cargo.toml` / `Cargo.lock`、SFTP 协议、传输模型、会话导入导出格式、终端 CWD 捕获协议。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 确认初始目录双请求根因和主流客户端目录优先级 | 代码路径与官方文档核对 | 当前 home 列举与终端 CWD 同步并行 |
| P2 | completed | 保存远端目录、单一初始列举和显式终端目录动作 | 3 项聚焦测试、`cargo check` | 优先级为活动重连路径、固定路径、上次目录、home |
| P3 | completed | 格式化、完整测试、追踪记录收口 | `rustfmt`、`cargo test`、validator | GUI 行为保留手工验收 |

## 已完成

- 已确认 worker 连接后按 `sftp_path` 或 home 列举目录，而 SFTP 页面切入同时会自动同步 SSH 终端 CWD，导致出现两次目录列举和 home 闪现。
- 已完成外部检索：WinSCP 支持每站点初始远端目录及“记住上次目录”；Cyberduck 将当前目录到终端的关联设计为显式动作。
- 已确定本轮目录优先级：固定 `SFTP 路径`、保存会话的上次远端目录、服务端 home；终端当前目录仅由用户显式触发。
- 已新增本机 `last_remote_sftp_paths`，仅在成功接收目录列表后记录已保存会话的远端目录；删除或导入替换会话时自动清理失效记录。
- 已将连接重建路径传入 worker 启动参数，移除启动后的第二次 `ListDir`；worker 现在只列举一次选定的初始目录。
- 已移除进入 SFTP 页与终端 CWD 事件的隐式同步，并在远端地址栏加入显式“打开终端当前目录”按钮。

## 验证

- 已完成：环境记录、实施记录、项目地图、SFTP worker/workspace/配置路径审查；WinSCP 与 Cyberduck 官方文档检索；受影响 Rust 文件 `rustfmt --edition 2024`；启动路径、worker 初始路径和远端路径存储各 1 项聚焦测试；`cargo check`；完整 `cargo test --quiet`（202 项）；`git diff --check`；tracking docs validator。
- 未完成：真实 GUI 连接、目录恢复和显式终端目录跳转验收。

## 风险与阻塞

- 风险：旧配置没有远端目录记录，必须无缝回退服务端 home。
- 风险：SFTP 连接重建必须带上已知当前目录，但不能再通过独立命令重复列举。
- 无阻塞。

## 下一步

- 在真实 SSH/SFTP 服务器上确认固定路径、上次远端目录、home 回退和工具栏“打开终端当前目录”的顺序及无 home 闪现。

## 最后更新时间

- 2026-07-15 16:32 +0800
