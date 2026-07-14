# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮已完成双语 README 维护和 release workflow 仓库链接修正，不改变运行时、构建或发布执行逻辑。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已刷新为 README 与 release workflow 链接维护完成状态。
- changes.md：存在；已追加本轮预检和完成验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Cargo。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`。
- 包管理器：`cargo`。
- 构建 / 运行入口：`src/main.rs`。
- 本轮入口：`README.md`，`README.zh.md`，`.github/workflows/release.yml`。
- 依赖策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：文档链接静态检查、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu 和 macOS 执行 release build；本轮不修改 CI 执行步骤。
- 外部依赖：无；已通过 Git remote 和本地 workflow 内容核对仓库地址。
- 证据文件：`AGENTS.md`，`README.md`，`README.zh.md`，`.github/workflows/release.yml`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：双语 README 现在说明 SFTP 批量下载文件明细并提供 GitHub Releases 入口；release workflow 停用 Homebrew Cask 模板中的两个下载 URL 和 homepage 已统一为 `https://github.com/xinalbert/axshell`。
- 受影响文件：`README.md`，`README.zh.md`，`.github/workflows/release.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：已完成。

## 开工判定

- 状态：允许开工。
- 原因：范围只涉及文档和停用 workflow 注释，现有项目地图已覆盖所有目标文件。
- 开工前动作：已读取项目环境、实施记录、项目地图和 README 维护规范；不需要联网或多 agent。

## 最后确认时间

- 2026-07-14 22:37 +0800
