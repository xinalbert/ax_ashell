# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮将进入真实实现，必须先执行 `project-env-audit`

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为当前任务的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 证据文件：`Cargo.toml`，`.github/workflows/release.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：Linux 构建需要系统 GUI 库；GitHub Release 自动发布依赖 Actions 内置 `github.token` 和 `contents: write`；Homebrew cask 恢复仍会额外依赖 `secrets.TAP_GITHUB_TOKEN`
- 证据文件：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`Cargo.toml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变，但当前任务已从依赖迁移切换为 GitHub Release workflow 收口，需要把 current 记录中的任务语境、验证重点和密钥边界改成发布链路恢复
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目边界明确，现有构建链路已存在，当前只需在仓库内恢复 GitHub Release job，不需要新增外部平台集成
- 开工前动作：完成 `docs/project-implementation-tracker/` 计划与地图更新，再恢复 `.github/workflows/release.yml` 中的 GitHub Release 自动发布 job，并验证只保留内置 `github.token` 的活跃密钥路径
