# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：已完成 `zed-industries/zed` 依赖统一升级与验证；当前 Zed 生态已收口到单一提交 `f9c994796ad4341649d7b8664edbdfaae8bebd5d`，本轮无需源码迁移

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为“Zed 依赖统一升级”任务的完成态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`Cargo.toml`，`Cargo.lock`
- 依赖统一策略：根项目 `gpui` / `gpui_platform` / `menu` 保持 plain git source，通过 `Cargo.lock` 统一 pin 到单一 Zed 提交，避免和 `gpui-component` 形成双 source id
- 证据文件：`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：临时副本 `cargo check --offline`，真实仓库 `cargo check --locked`，`cargo test --locked`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮已访问 Cargo git index / git checkout 获取较新 `zed-industries/zed` 提交；运行期不依赖外部服务
- 工具可用性：本机 `cargo` 可正常执行；真实仓库已完成 locked build/test 验证
- 证据文件：`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：`Cargo.lock` 已从旧 Zed 快照统一到 `f9c994796ad4341649d7b8664edbdfaae8bebd5d`；`accesskit` 升到 `0.24.1` 解开了 `gpui-component` / `gpui_windows` 冲突；`wgpu` 栈随新 Zed 快照切到 `29.0.4`；当前无需源码 API 迁移
- 受影响文件：`Cargo.toml`，`Cargo.lock`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链已满足当前依赖要求，且真实仓库 `cargo check --locked` / `cargo test --locked` 已通过，说明本轮升级可在现有 `rust-version = 1.88.0` 下落地
- 开工前动作：已复查 `Cargo.toml`、`Cargo.lock`、CI 构建入口与当前未提交 diff；已确认显式 `rev = ...` 会与 `gpui-component` 形成双 source id，因此最终保留根依赖为 plain git，并通过 `Cargo.lock` 统一 pin 到 `f9c994796ad4341649d7b8664edbdfaae8bebd5d`
