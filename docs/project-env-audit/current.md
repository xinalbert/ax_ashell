# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：Rust 依赖集合升级已完成；项目 MSRV 提升到 `1.88.0`

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“Rust 依赖集合升级”任务的完成态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`Cargo.toml`，`Cargo.lock`
- 证据文件：`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`cargo update --dry-run`，`cargo check --locked`，`cargo check --examples --locked`，`cargo test --locked`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮需要访问 Cargo registry / git index 获取最新兼容解；运行期不依赖外部服务
- 工具可用性：本机 `cargo` 可正常执行；本轮不需要 `cargo fmt`
- 证据文件：`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：否
- 变化摘要：依赖集合升级完成；项目最低 Rust 版本由 `1.85.0` 提升到 `1.88.0`
- 受影响文件：`Cargo.toml`，`Cargo.lock`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工，且本轮依赖升级已完成
- 原因：本轮依赖和 MSRV 调整已通过 `cargo check --locked`、`cargo check --examples --locked` 与 `cargo test --locked`
- 开工前动作：已复查 `Cargo.toml`、`Cargo.lock`、CI 构建入口与当前未提交 diff；已执行 Cargo dry-run / info / crates.io API 检查并落地可安全升级集合
