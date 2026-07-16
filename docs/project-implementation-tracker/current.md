# 当前项目实施记录

## 当前目标

- 目标：重构新建 SSH 连接表单，使核心连接信息、认证、组织和高级选项清晰分层。
- 交付物：分区 SSH 表单、明确字段文案、保留现有代理/SFTP/X11/快捷键行为，以及验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/dialogs/ssh.rs`、`src/app/lifecycle/init.rs`、`locales/en.yml`、`locales/zh-CN.yml`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：SSH backend 行为、会话持久化 schema、跳板机/ProxyJump、压缩/keepalive 等尚未实现的 SSH 参数、`Cargo.toml` / `Cargo.lock`。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 重构新建 SSH 表单的分区、字段文案与高级选项层级 | Rust 格式化、`cargo check` | 保留现有 Session 字段与交互 |
| P2 | completed | 完成回归、差异检查与跟踪记录 | `cargo test --quiet`、tracking validator | GUI 交互仍需手工确认 |

## 已完成

- 已确认现有 Session 模型已经支持密码/私钥、代理、SFTP 初始目录、X11 和连接快捷键，无需扩展 schema。
- 已确认 backend 未实现跳板机、ProxyJump、压缩、用户态 keepalive、Agent forwarding 等行为；本轮不展示无效参数。
- 已完成 Termius、MobaXterm 与 Royal TS 的只读信息架构调研：核心连接字段优先，分组独立于连接参数，代理/X11 等进入高级层级。
- 已完成表单重构：`Host | Port` 同行、用户名独占下一行；连接、认证、组织和高级 SSH 选项以独立卡片区分。
- 已将连接名称、保存分组、选择既有分组、私钥口令和 SFTP 初始目录替换为明确的中英文文案；高级选项默认收起，编辑含高级配置的既有会话时自动展开。

## 验证

- 已完成：项目环境、实施记录、项目地图、SSH 表单/会话模型和快速 hover 规范的静态核对；受影响 Rust 文件 `rustfmt --edition 2024`；`cargo check`；完整 `cargo test --quiet`（212 项）；`git diff --check`；tracking docs validator。
- 未完成：真实桌面窗口的表单可视高度、键盘 Tab 顺序、密码/私钥切换、分组菜单和高级选项展开验收。

## 风险与阻塞

- 无阻塞；表单分区后的实际可视高度、键盘 Tab 顺序和弹窗滚动体验需要 GUI 手工确认。

## 下一步

- 在桌面应用中手工确认新建和编辑 SSH 会话的分区、保存和连接流程。

## 最后更新时间

- 2026-07-16 14:37 +0800
