# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮“tag 作为唯一发布版本源”的发布链路已完成收口；运行环境和依赖版本事实未变

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“tag 全链路版本源”任务的完成态，并补充一次官方文档检索结论

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/app/constants.rs`，`src/app/startup.rs`，`.github/workflows/release.yml`，`scripts/package-macos-app.sh`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`python3 scripts/release_version.py --help`，`python3 scripts/release_version.py env --tag v2026.7.6`，`python3 scripts/release_version.py env --tag v2026.7.6-1`，`python3 scripts/release_version.py env --cargo-version-file Cargo.toml`，`python3 scripts/release_version.py env --tag v2026.2.30` 失败校验，`bash -n scripts/package-macos-app.sh`，`.github/workflows/release.yml` YAML 静态自检，`cargo check`，tracking docs 校验
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮只做了少量官方文档检索，用于确认 macOS `CFBundleShortVersionString` / `CFBundleVersion` 的格式约束；其余验证边界仍是本机 Rust/Python 工具链、release workflow、manifest/lock 同步逻辑、运行时版本显示和 macOS bundle 版本派生。真实 GitHub Release 执行与安装包展示仍需平台手工确认
- 证据文件：`Cargo.toml`，`Cargo.lock`，`src/app/constants.rs`，`src/app/startup.rs`，`scripts/package-macos-app.sh`，`.github/workflows/release.yml`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：否
- 变化摘要：运行环境和依赖版本未变；本轮已完成 release workflow、manifest/lock 临时同步、本地打包脚本与文档的统一版本收口；规范 tag 已改为与 `Cargo.toml` 一致的 Cargo 兼容格式，并补充确认了 macOS bundle version 的格式边界
- 受影响文件：`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`Cargo.toml`，`Cargo.lock`，`src/app/constants.rs`，`src/app/startup.rs`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：已完成
- 原因：任务边界明确，且已完成共享版本规则、workflow 注入、本地打包脚本复用、文档同步与本机验证
- 开工前动作：已复查 `Cargo.toml`、`Cargo.lock`、`src/app/constants.rs`、`src/app/startup.rs`、`scripts/package-macos-app.sh`、`.github/workflows/release.yml` 与中英文 README / development 文档；当前剩余边界仅为远端 tag push 和平台实机确认
