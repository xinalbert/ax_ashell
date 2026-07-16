# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮修复 SSH/SFTP 服务器身份验证、遗留算法降级、同步端点安全边界、同步响应限额与依赖漏洞治理。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新为本轮安全修复范围。
- changes.md：保留既有历史，本轮施工前预检已追加。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、russh、russh-sftp、reqwest、Argon2id、XChaCha20-Poly1305。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`，依赖由 `Cargo.toml` 与 `Cargo.lock` 锁定。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`。
- 本轮代码入口：`src/session.rs`、`src/backend/ssh.rs`、`src/backend/ssh/connection.rs`、`src/backend/ssh/legacy.rs`、`src/sftp/auth.rs`、`src/sync.rs`、`src/events.rs`、`src/app/`、`.github/workflows/ci.yml`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo check`、定向单元测试、`cargo test --quiet`、`git diff --check`。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux x86_64/aarch64 和 macOS x86_64/aarch64 构建 release，并在独立 Linux job 安装 `cargo-audit` 审计 `Cargo.lock`。
- 外部依赖：真实 SSH/SFTP 服务器和 GUI 主机密钥确认需要三平台手工验收；临时目录中的 `cargo-audit` 仅用于本轮审计，不进入项目依赖。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`Cargo.lock`、`.github/workflows/ci.yml`、`src/backend/ssh.rs`、`src/sftp/auth.rs`、`src/sync.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：工具链和 Cargo 工作流未变；CI 已新增 RustSec 审计。锁文件将 `crossbeam-epoch`、`quinn-proto` 和 `memmap2` 升级到修复版本；其余 `rsa` 与 `quick-xml` 公告受无补丁或当前上游约束影响，已以明确 ID 暂缓并保留后续复审。
- 受影响文件：`src/session.rs`、`src/backend/`、`src/sftp/`、`src/sync.rs`、`src/events.rs`、`src/app/`、`.github/workflows/ci.yml`、`Cargo.lock`、`locales/`、`docs/`。
- 是否需要更新 `current.md` / `changes.md`：是，均已在施工前更新。

## 开工判定

- 状态：允许开工。
- 原因：本机工具链满足仓库约束；主机密钥、legacy、同步边界、依赖审计和文档更新均已完成自动化验证。依赖升级保持为最小补丁级锁文件变更。
- 开工前动作：已读取环境记录、实施记录、项目地图、CI、锁文件与相关安全路径；已使用 RustSec 官方公告数据库完成基线与修复后审计；不使用多 agent。

## 最后确认时间

- 2026-07-18 12:55 +0800
