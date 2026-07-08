# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust 桌面应用的真实功能改动；本轮目标是记住 SSH 连接成功使用的兼容模式，并在后续连接中优先尝试该模式，同时保留完整 fallback。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SSH 成功连接方法优先级缓存”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh / russh-sftp
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/session/config.rs`，`src/backend/ssh.rs`，`src/sftp/auth.rs`，`src/session/mod.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/session/config.rs`，`src/backend/ssh.rs`，`src/sftp/auth.rs`，`src/session/mod.rs`，`src/app/event_loop.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 ...`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；真实 SSH / SFTP 联机效果需要目标服务器手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/session/`，`src/backend/`，`src/sftp/`，`src/app/event_loop.rs`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从上一轮代码整理切换到 SSH 连接优先级缓存；运行环境不变，验证入口仍为全仓 `cargo check` / `cargo test`
- 受影响文件：`src/session/config.rs`，`src/backend/ssh.rs`，`src/sftp/auth.rs`，`src/session/mod.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链、依赖锁定和基础测试环境都已就位；本轮主要改动是连接尝试顺序和配置回写，可通过格式化、编译、单元测试与源码级检查验证
- 开工前动作：已复查 SSH 终端默认/legacy fallback、SFTP 认证路径、会话配置持久化和后端事件分发；已确认不需要联网、不使用多 agent
