# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮 release tag 解析收紧已完成；运行环境和依赖版本事实未变

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“只接受 canonical release tag”任务的完成态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮脚本入口：`python3 scripts/release_version.py`
- 证据文件：`Cargo.toml`，`Cargo.lock`，`scripts/release_version.py`，`.github/workflows/release.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查，加上发布版本脚本的命令行样例校验
- 默认测试命令：`cargo test`
- 当前实施验证命令：`python3 scripts/release_version.py env --tag v2026.7.7`，`python3 scripts/release_version.py env --tag v2026.7.7-1`，`python3 scripts/release_version.py env --tag v2026.07.07`，`python3 scripts/release_version.py env --cargo-version-file Cargo.toml`，`cargo check`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不依赖联网、外部服务或远程 SSH 服务器；验证边界集中在本机 Python / Rust 工具链、Cargo manifest 版本规则和 tracking docs contract
- 证据文件：`Cargo.toml`，`Cargo.lock`，`scripts/release_version.py`，`.github/workflows/release.yml`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：否
- 变化摘要：运行环境和依赖版本未变；本轮已完成 release tag / Cargo 版本解析收紧和跟踪记录更新
- 受影响文件：`scripts/release_version.py`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：已完成
- 原因：任务边界明确，且已在现有 Python / Cargo 工具链上完成脚本收紧、正反例验证、编译检查和 tracking docs 更新
- 开工前动作：已复查 `scripts/release_version.py`、`.github/workflows/release.yml`、`Cargo.toml` 与 tracking docs；已确认现有 workflow 注释保持 canonical 格式，脚本现已拒绝 legacy tag 和零填充版本段
