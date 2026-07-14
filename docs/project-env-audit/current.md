# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮把 X11 forwarding 移至 `Session`，并只提示缺失的本机 X server，不自动安装或启动它。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新为会话级 X11 forwarding 与本机 X server 检测范围。
- changes.md：已追加本轮验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、`portable-pty 0.9`、Tokio、`tracing`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc` / `cargo` 可用。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/backend/ssh.rs`。
- 本轮代码入口：`src/session.rs`，`src/app/dialogs/ssh.rs`，`src/backend/ssh/x11.rs`，`src/platform/x_server.rs`；会话选择是否请求 X11，本机环境仅决定是否显示安装提示和 relay 的连接目标。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo test --quiet new_session_fields_default_when_loading_existing_sessions`、`cargo test --quiet saved_sessions_share_export_omits_credentials_and_key_material`、`cargo check`、完整 `cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux 和 macOS 执行 release build。
- 本轮验证结果：已完成 Rust 格式化、3 项聚焦测试、`cargo check`、完整 `cargo test --quiet`（171 项）、`git diff --check` 和 tracking validator。
- 外部依赖：无新增；无需联网。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/session.rs`，`src/app/dialogs/ssh.rs`，`src/backend/ssh.rs`，`src/backend/ssh/x11.rs`，`src/platform/x_server.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：全局 `x11_forwarding_enabled` 和自动启动设置已移除；`Session.x11_forwarding` 默认 `true` 并决定每个 SSH 是否发送 X11 request。未检测到本地 `DISPLAY` 或配置的 X server 路径时，表单只显示安装提示。远端 `DISPLAY` 不再被固定为 `localhost:10.0`。
- 受影响文件：`src/session.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/dialogs/ssh.rs`，`src/app/dialogs/settings/proxy.rs`，`src/backend/ssh.rs`，`src/backend/ssh/x11.rs`，`src/platform/x_server.rs`，`src/config/model.rs`，`src/config/store.rs`，`locales/`，`docs/features/proxy-x11*.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：均已更新。Windows/Xming/VcXsrv、XQuartz 和远端 SSHD 仍需用户侧实机验证。

## 开工判定

- 状态：已完成。
- 原因：本机工具链满足仓库约束；变更局限于现有会话、X11 relay 和表单路径，不需要联网、新依赖或 manifest/lock 修改。
- 完成动作：已完成自动化验证；保留 GUI 和远端环境的手工验证边界。

## 最后确认时间

- 2026-07-14 10:57 +0800
