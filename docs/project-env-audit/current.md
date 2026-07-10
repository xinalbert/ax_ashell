# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮为 SFTP 超大目录增加显式分页加载，同时保持已落地的单次保留数据上限。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已复核并刷新为“SFTP 目录分页加载”任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 多线程运行时，`russh` SSH / SFTP 后端。
- 版本约束：`rust-version = 1.88.0`，edition `2024`；本机为 `rustc 1.96.1` / `cargo 1.96.1`。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/sftp.rs`，`src/terminal.rs`，`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/lifecycle/event_loop.rs`，`locales/`
- 依赖统一策略：复用已锁定的 `russh-sftp` 公共 `RawSftpSession` API，在 worker 内保存目录句柄作为 cursor；按页 `READDIR`，不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：CI 配置当前以构建为主，未声明独立 test job。
- 当前实施验证命令：`rustfmt --edition 2024` 覆盖修改 Rust 文件、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：不需要联网；完整交互验证需要连接一个包含多页条目的远端目录，确认加载更多、EOF、上限截断和目录切换时的 cursor 回收正常。
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/sftp.rs`，`src/terminal.rs`，`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/lifecycle/event_loop.rs`，`locales/en.yml`，`locales/zh-CN.yml`，已锁定 `russh-sftp 2.3.0` 源码，`AGENTS.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行时与工具链未变；SFTP worker 现在保存 raw directory cursor，每次显式“加载更多”至多追加 250 项。cursor 在 EOF、2,000 项/2 MiB 总预算、目录切换、worker 关闭、空闲回收和读取失败时关闭；UI 保持虚拟列表并追加页结果，不预取完整目录。
- 受影响文件：`src/sftp.rs`，`src/terminal.rs`，`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/lifecycle/event_loop.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：依赖公开了可复用目录句柄的 raw API；可在不预取完整目录的前提下按需继续 `READDIR`，并在总预算时关闭 cursor。
- 开工前动作：已读取 `AGENTS.md`、环境记录、项目地图、SFTP worker、事件处理、SFTP UI 状态和列表渲染路径；已确认本机 Rust 工具链及锁定依赖源码可用。
- 完成后动作：已执行格式化、编译、完整测试、空白检查和 tracking docs validator；GUI 侧仍需验证多页目录、EOF、总上限、切换路径和 worker 回收。

## 最后确认时间

- 2026-07-10 17:41 +0800
