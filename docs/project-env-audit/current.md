# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮只维护 README 与用户文档的双语图片引用和已删除图片目录的导航，不修改应用代码、依赖或运行环境。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已完成 P10 双语文档图片同步的环境验证记录。
- changes.md：保留既有历史，施工前预检和完成验证均已追加。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、russh、russh-sftp、reqwest、Argon2id、XChaCha20-Poly1305。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`，依赖由 `Cargo.toml` 与 `Cargo.lock` 锁定。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`。
- 本轮文档入口：`README.md`、`README.zh.md`、`docs/README*.md`、`docs/features/`、`images/`、`docs/features/images/`。

## 测试环境

- 测试框架：Markdown 图片/链接存在性审阅、`git diff --check`、tracking docs validator。
- 默认测试命令：图片/链接存在性检查、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux x86_64/aarch64 和 macOS x86_64/aarch64 构建 release，并在独立 Linux job 安装 `cargo-audit` 审计 `Cargo.lock`。
- 外部依赖：无；本轮不新增服务端组件、协议或图片生成依赖。
- 证据文件：`AGENTS.md`、`README.md`、`README.zh.md`、`docs/README*.md`、`docs/features/`、`images/`、`docs/features/images/`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：项目运行环境、工具链、依赖、manifest/lock 和 CI 工作流均未变；P10 已同步 README 和九个功能页的中英文图片引用，并清理旧 `docs/images/` 导航和截图占位。
- 受影响文件：`README*.md`、`images/`、`docs/`。
- 是否需要更新 `current.md` / `changes.md`：是，已补充完成验证和未使用图片的保留边界。

## 开工判定

- 状态：允许开工。
- 原因：仅改 Markdown 和已有 PNG 引用；中英文页面和图片目录的真实位置已定位，不需要联网或工具链变更。
- 开工前动作：已读取环境记录、实施记录、项目地图、README 双语页面、功能页对应关系和图片文件类型；不新增依赖、不使用多 agent。图片存在性、双语引用配对、`git diff --check` 和 tracking docs validator 均已通过。

## 最后确认时间

- 2026-07-18 22:35 +0800
