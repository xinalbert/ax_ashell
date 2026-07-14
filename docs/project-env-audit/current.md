# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮为 SFTP 本地/远端文件列表增加独立持久化、启动预热的系统图标渲染层，涉及目标专属依赖、配置路径与平台模块，不改变 SFTP 协议或传输逻辑。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已由先前 SFTP 传输面板范围刷新为系统文件图标范围。
- changes.md：存在；已追加本轮依赖、平台预热与验证结果。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`image`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机为 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/platform.rs`，`src/app/lifecycle/init.rs`，`src/app/views/sftp_panel.rs`。
- 本轮代码入口：`src/config/store.rs`、`src/platform/file_icons.rs`（新增）、`src/platform.rs`、`src/app.rs`、`src/app/lifecycle/init.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/views/sftp_panel.rs`。
- 依赖策略：新增限定目标平台的图标接口依赖；macOS 使用 AppKit，Windows 使用 Shell API，Linux 使用 Freedesktop 图标主题解析。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024 <changed-rust-files>`、图标 key/缓存聚焦测试、`cargo check`、可用目标的 `cargo check --target`、`cargo test --quiet`、SFTP hover 静态审计、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu 和 macOS 执行 release build。
- 外部依赖：已检索 `freedesktop-icons 0.4.0`，用于 Linux 按当前主题查找图标；其余平台使用系统 SDK API。图标位图使用现有 `base64` / `serde_json` 序列化到独立 `file-icons.json`，不进入同步的 `sessions.json`。
- 证据文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`，`src/app/views/sftp_panel.rs`，`src/platform.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：当前项目不存在系统文件图标抽象；GPUI 可渲染缓存图像但不暴露系统图标 API。现有虚拟列表已使用 `uniform_list` 与共享 FastHover，要求图标查询脱离行渲染路径；本轮将以配置目录中的独立文件保存类型图标，并在启动阶段加载/预热。
- 受影响文件：`Cargo.toml`，`Cargo.lock`，`src/config/store.rs`，`src/platform.rs`，`src/platform/file_icons.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views.rs`，`src/app/views/sftp_panel.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：已完成更新。

## 开工判定

- 状态：允许开工。
- 原因：本机工具链高于仓库最低版本，CI 目标覆盖三个实现平台；系统图标查询可由目标专属 API 完成，并可由 GPUI 缓存图像渲染。
- 开工前动作：已读取项目环境、实施地图、SFTP FastHover 规则；已确认统一缓存和后台查询边界。

## 最后确认时间

- 2026-07-14 17:03 +0800
