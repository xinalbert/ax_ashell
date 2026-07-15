# 项目施工前预检

## 项目边界

- 类型：明确子项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮仅优化终端可视行高亮增量路径，不改变关键词规则、PTY 协议、依赖、配置或 CI workflow。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新为终端同步增量高亮范围。
- changes.md：存在；已追加本轮预检和完成验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Alacritty Terminal、Tokio、`serde`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`。
- 包管理器：`cargo`；本机已确认 `rustc 1.96.1`、`cargo 1.96.1`。
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/event_loop.rs`，`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`src/terminal/element.rs`。
- 本轮代码入口：`src/terminal/tab.rs` 与 `src/terminal/highlight.rs`；`src/terminal/element.rs` 仅用于确认行布局缓存是否需要兼容调整。
- 依赖策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo build`、tracking docs validator。
- 默认验证命令：`rustfmt --edition 2024 src/terminal/tab.rs src/terminal/highlight.rs`、terminal 聚焦测试、`cargo check`、`cargo test --quiet`、`cargo build`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu 和 macOS 执行 release build。
- 外部依赖：不需要新的联网事实；当前锁定 `alacritty_terminal 0.26.0` 的 `TermDamage` 语义和已有项目实现足以确定本轮方案。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/lifecycle/event_loop.rs`，`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`src/terminal/element.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：当前记录从 SFTP 下载文件明细切换到终端高亮性能修复。现有 `build_visible_rows` 已可用 `Rc<RenderRow>` 安全复用滚屏行，但高亮缓存仍按视口行号处理 `TermDamage::Full`，导致新增行在 125ms 限频窗口内缺少语义颜色。
- 受影响文件：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，必要时 `src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是，已完成预检和验证记录。

## 开工判定

- 状态：允许开工。
- 原因：可视行构建、高亮缓存和到期 UI 刷新链路均已定位；只需在现有行级缓存之上增加重建行元数据与身份重排，无需新的运行时、定时器或依赖。
- 开工前动作：已读取环境记录、实施记录、项目地图和终端模块；本轮不联网、不使用多 agent。

## 最后确认时间

- 2026-07-15 09:49 +0800
