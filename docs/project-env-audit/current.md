# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮进入真实实现，已完成 `project-env-audit`，允许继续施工

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：刷新为设置页 `Custom` theme 配置中心任务的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 证据文件：`Cargo.toml`，`src/app/dialogs.rs`，`src/session/config.rs`，`locales/en.yml`，`locales/zh-CN.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/app/dialogs.rs`，`cargo check`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：设置页结构依赖 `gpui-component` 的 `Settings` / `SettingPage` / `SettingGroup`；本轮不涉及联网或外部服务
- 证据文件：`.github/workflows/ci.yml`，`Cargo.toml`，`src/app/dialogs.rs`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变，但当前任务已从 `dev-reload` 工具切换到主应用设置页 theme 配置 UI 结构调整，需要把 current 记录的范围和验证重点改为 `src/app/dialogs.rs`、`src/session/config.rs` 与本地化文案
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：任务边界明确，主要调整设置页导航分组、theme 相关配置 key/default 展示和文案，不改变配置存储格式
- 开工前动作：刷新 `docs/project-implementation-tracker/` 当前态与项目地图，再调整设置页分组结构
