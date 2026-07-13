# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮已完成 SFTP 本地文件浏览器的全局默认本地目录配置，不修改依赖或构建配置。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在，已刷新为本轮 SFTP 默认本地目录设置范围。
- changes.md：存在，已追加本轮预检和验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、`portable-pty 0.9`、Tokio、`tracing`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc` / `cargo` 可用。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/backend/local.rs`。
- 本轮代码入口：`src/config/model.rs`，`src/config/store.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs/settings/proxy.rs`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024`、`cargo test --quiet local_sftp -- --nocapture`、`cargo check`、`cargo test --quiet`、`cargo build`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux 和 macOS 执行 release build。
- 本轮验证结果：相关 Rust 文件格式化、`cargo test --quiet local_sftp -- --nocapture`、`cargo check`、`cargo test --quiet`、`cargo build`、`git diff --check` 和 tracking validator 已通过。
- 外部依赖：无新增；无需联网。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/config/model.rs`，`src/config/store.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs/settings/proxy.rs`，`src/sync.rs`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：本轮仅新增本机 SFTP 默认目录配置和 Settings UI；运行时、依赖、manifest/lock、CI 入口和 sync payload 结构不变。新增配置是本机 `ConfigFile` 字段，不加入 WebDAV/S3 `SyncPayload`。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/`，`docs/features/sftp*.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；已完成记录实现、验证和 GUI 手工验证边界。

## 开工判定

- 状态：已完成。
- 原因：本机工具链满足仓库约束；现有配置、Settings 和 SFTP 本地浏览链路足以完成实现；不需要新增依赖、联网或修改 manifest/lock。
- 完成动作：单线程实施；已执行 Rust 验证、构建、空白检查和 tracking docs 校验。

## 最后确认时间

- 2026-07-13 17:40 +0800
