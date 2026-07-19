# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮修复新建/编辑连接对话框在小窗口中的内容裁切，不修改连接协议、会话数据、依赖或全局 Dialog 组件。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已完成 P15 的运行环境与自动化验证记录。
- changes.md：保留既有历史，P11-P14 验证和 P15 施工前/完成记录均已追加。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、russh、russh-sftp、reqwest、Argon2id、XChaCha20-Poly1305。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`，依赖由 `Cargo.toml` 与 `Cargo.lock` 锁定。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`。
- 本轮代码入口：`src/app/dialogs.rs`、`src/app/dialogs/ssh.rs`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux x86_64/aarch64 和 macOS x86_64/aarch64 构建 release，并在独立 Linux job 安装 `cargo-audit` 审计 `Cargo.lock`。
- 外部依赖：小窗口 GUI 手工验收；本轮不新增服务端组件、协议或依赖。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`Cargo.lock`、`.github/workflows/ci.yml`、`src/app/dialogs.rs`、`src/app/dialogs/ssh.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：项目运行环境、工具链、依赖、manifest/lock 和 CI 工作流均未变；P15 已恢复安全的 `Dialog.content(...)` 延迟 builder，并在其中以显式 scroll handle 和 flex 约束建立连接页滚动区域。
- 受影响文件：`src/app/dialogs.rs`、`src/app/dialogs/ssh.rs`、`docs/`。
- 是否需要更新 `current.md` / `changes.md`：是，已补充 P15 的 crash 结论、实现结果和 GUI 验收边界。

## 开工判定

- 状态：允许开工。
- 原因：本机工具链满足仓库约束；P15 已确认问题是 GPUI Entity 借用边界而非前后台生命周期恢复。修复仅恢复延迟 form 构建并调整本地 scroll container，不改变连接业务事件。
- 开工前动作：已读取环境记录、实施记录、项目地图、manifest/lock、CI、crash report、运行日志、`show_ssh_dialog`、Dialog render、crash hook 和 lifecycle state；不新增依赖、不联网、不使用多 agent。P15 已通过格式化、编译和完整测试，待最终文档/空白校验与新的整体滚动截图验收。

## 最后确认时间

- 2026-07-19 09:20 +0800
