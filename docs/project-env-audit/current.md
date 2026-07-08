# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust 桌面应用的真实日志可观测性增强任务；本轮目标是提升运行日志可发现性、启动诊断信息和日志保留窗口，同时保持现有 tracing 全局结构。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“运行日志可观测性增强”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / tracing / tracing-subscriber / tracing-appender
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/app/startup.rs`，`src/app/dialogs.rs`，`src/app/constants.rs`，`src/main.rs`
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/app/startup.rs`，`src/app/dialogs.rs`，`src/main.rs`，`docs/development.md`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 ...`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；打开日志目录按钮的 GUI 行为需要用户手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从上一轮 SSH 连接模式缓存切换到日志可观测性增强；运行环境不变，验证入口仍为格式化、全仓编译、全仓测试和 tracking docs 校验
- 受影响文件：`src/app/startup.rs`，`src/app/dialogs.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：日志系统已基于 `tracing_subscriber` 全局初始化；本轮是低风险增强，可通过编译、测试和源码级检查验证
- 开工前动作：已复查 `init_logging()`、panic hook、About 页面和开发文档中的日志说明；已确认不需要联网、不使用多 agent
