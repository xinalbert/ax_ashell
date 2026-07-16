# 当前项目实施记录

## 当前目标

- 目标：为同名本地和 SSH 工作区 Tab 显示稳定实例号。
- 交付物：`Local #1` / `Local #2` 与同名 SSH 实例标签，SFTP 标签同步显示，单元测试与验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/workspace.rs`、`src/app/actions/session.rs`、`src/app/views/tab_bar.rs`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：Terminal/SFTP 连接生命周期、会话持久化 schema、Settings Tab 顺序、`Cargo.toml` / `Cargo.lock`。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 为新会话组分配稳定同名实例号并渲染至 Terminal/SFTP Tab | 工作区聚焦单元测试、`cargo check` | 实例号不随 Tab 重排改变 |
| P2 | completed | 格式化、构建、回归并收口跟踪记录 | 完整测试、差异检查、tracking validator | GUI 文本截断另需手工验证 |

## 已完成

- 已确认当前 Tab 标题仅使用 `TabGroup::title`，多个 Local 或同名已保存 SSH 会话只能由可变位置序号区分。
- 已确认会话组在创建时集中构造，适合将实例号保存在运行时组模型中，让拖动重排不影响标签。
- 已为 `TabGroup` 保存运行时实例号，并为每个会话标题维护单调递增的窗口内计数器。
- 已接入本地终端、SSH 终端与仅 SFTP 页面三个组创建入口；Terminal/SFTP 标签共同显示标题和实例号。
- 已将标签格式提取为可测试 helper，覆盖 Local、SSH 多 pane 与 SFTP 文本，并更新项目地图。

## 验证

- 已完成：受影响 Rust 文件 `rustfmt --edition 2024`；`cargo test --quiet workspace::tests`（5 项）；`cargo check`；完整 `cargo test --quiet`（212 项）；`git diff --check`；tracking docs validator。
- 未完成：真实桌面窗口的文本截断与拖放后实例号稳定性验证。

## 风险与阻塞

- 无阻塞；较长会话名加实例号可能更早触发 Tab 文本省略，需 GUI 手工确认。

## 下一步

- 手工确认较长名称下的 Tab 文本省略、Terminal/SFTP 实例对应关系，以及拖放重排后实例号不变。

## 最后更新时间

- 2026-07-16 14:20 +0800
