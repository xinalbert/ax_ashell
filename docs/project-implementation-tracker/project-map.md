# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`.github/workflows/release.yml`，`src/backend/ssh.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`.github/workflows/release.yml`，`Cargo.toml`，`Cargo.lock`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 排除：`.git/`，`.cargo/registry/`，`.cargo/git/`，`target/`，`assets/`，`locales/`，生成产物与外部依赖源码缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `.github/workflows/release.yml` | 跨平台构建、artifact 上传与 GitHub Release 发布 | 需要修改 tag 触发构建、release asset 上传或 cask 分发时 | 本轮重点恢复 GitHub Release 发布，继续停用 cask |
| `Cargo.toml` | 仓库依赖、Rust 版本和包元数据 | 需要切换依赖来源、版本约束或 crate 功能时 | 当前不是本轮主要修改点 |
| `Cargo.lock` | 实际锁定的依赖版本与来源 | 需要确认生效 commit、registry 包版本或依赖切换结果时 | 当前存在上轮依赖迁移后的未提交改动，需要避免误混入 |
| `docs/project-env-audit/` | 项目环境当前态与历史 | 开工前预检或环境事实变化时 | 本轮需刷新为依赖迁移语境 |
| `docs/project-implementation-tracker/` | 本轮实施计划、地图与变更历史 | 真实施工前后记录计划和结论时 | 本轮需刷新到 `alacritty_terminal` 官方化任务 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `.github/workflows/release.yml` | 发布工作流入口 | `build`，`publish`，`cask` | 控制 artifact 与 Release asset 的边界，确保只恢复 GitHub Release |
| `docs/project-implementation-tracker/current.md` | 本轮实施 current 态 | `当前目标`，`活动计划`，`验证` | 保证当前任务切换到 release workflow 收口 |
| `docs/project-env-audit/current.md` | 当前环境与 CI 约束 | `测试环境`，`外部依赖` | 记录 `github.token` 与 `secrets.TAP_GITHUB_TOKEN` 的边界 |

## 常用定位

- `rg -n 'publish|cask|upload-artifact|download-artifact|action-gh-release|github.token|TAP_GITHUB_TOKEN' .github/workflows/release.yml`
- `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`

## 忽略与未索引

- `target/`、`.cargo/registry/`、`.cargo/git/` 未索引：属于构建产物或外部依赖缓存，不作为项目源码路由索引
- `src/`、`assets/` 未索引：本轮只做 release workflow 收口，不涉及应用代码与资源

## 刷新规则

- 刷新触发：发布链路、artifact/release 边界、本轮范围或关键 CI job 发生变化时刷新
- 最近依据：`.github/workflows/release.yml`、`docs/project-env-audit/current.md` 与当前工作区状态的实读结果

## 最后更新时间

- 2026-07-06 17:43 CST
