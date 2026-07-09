# 当前项目实施记录

## 当前目标

- 目标：扩展 GitHub Release 发布产物覆盖范围，新增 Linux ARM64 构建、Linux `.deb` 安装包和 macOS universal `.app` 压缩包
- 交付物：更新后的 `.github/workflows/ci.yml` / `.github/workflows/release.yml` 构建矩阵与打包步骤、中英文发布文档说明、同步后的实施记录和环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`docs/development.md`，`docs/development.en.md`，`README.md`，`README.en.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：Windows ARM64 preview runner、老 Linux glibc / musl 兼容包、Homebrew cask 发布、Rust 应用源码功能改动

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复查当前 CI / Release 矩阵、Debian metadata 和 GitHub runner 可用性 | 源码检查，GitHub 官方 runner 文档核对 | Windows ARM64 标为后续实验项 |
| P2 | completed | CI / Release 增加 Linux ARM64，Release Linux 产物增加 `.deb` | workflow YAML 静态检查，Bash 静态检查 | 不改 Rust 源码 |
| P3 | completed | Release 增加 macOS universal zip 组合 job | workflow YAML 静态检查，Bash 静态检查 | 依赖两个 macOS 架构 artifact |
| P4 | completed | 更新发布文档和跟踪记录 | docs 检查，tracking docs validator | 真实 tag 发布验证留到 GitHub Actions |

## 已完成

- 已读取 `project-map.md`、当前实施记录和环境记录，确认 `.github/workflows/`、`README`、`docs/development` 和 tracking 文档均在项目地图覆盖范围内
- 已复查当前 Release/CI 只覆盖 `windows-x86_64`、`linux-x86_64`、`macos-aarch64`、`macos-x86_64`
- 已确认 `Cargo.toml` 已有 Debian metadata，但当前 GitHub Release workflow 尚未输出 `.deb`
- 已查 GitHub 官方 runner 文档，确认 Linux ARM64 runner 标签可用；Windows ARM64 为 public preview，暂不并入主发布矩阵
- 已确认本轮不新增 Rust 依赖、不修改应用源码、不使用多 agent
- 已在 CI 和 Release matrix 中新增 `linux-aarch64`
- 已让 Linux Release 产物同时上传 `.tar.gz` 和 `.deb`，其中 `.deb` 使用 `dpkg-deb` 在 workflow 内组装，暂不引入 `cargo-deb` 依赖
- 已新增 `macos-universal` job，从 `macos-aarch64` 与 `macos-x86_64` artifacts 组合 universal `.app`，重新 ad-hoc codesign 后上传 universal zip
- 已同步 README 与中英文开发文档的发布产物清单

## 验证

- 已完成：当前 workflow / manifest / Debian metadata 源码检查
- 已完成：GitHub-hosted runner 官方文档核对
- 已完成：`.github/workflows/ci.yml` 和 `.github/workflows/release.yml` YAML 解析
- 已完成：Release workflow 所有 `run` 脚本经本地占位替换后通过 `bash -n`
- 已完成：`git diff --check`
- 已完成：tracking docs validator
- 未完成：真实 GitHub Actions tag 发布验证、Linux `.deb` 安装体验验证、macOS universal 下载后启动验证

## 风险与阻塞

- 阻塞：无
- 风险一：Linux ARM64 和 macOS universal 需要 GitHub Actions 实际 runner 执行才能最终确认；本机只能做静态校验
- 风险二：手写 `.deb` 控制文件需要在真实 runner 上确认安装体验；本轮先保持最小 Debian metadata
- 风险三：macOS universal 重新 ad-hoc codesign 后仍需下载实测 Gatekeeper / Finder 展示

## 下一步

- 推送后观察 CI / Release workflow；下次 tag 发布后下载验证 Linux `.deb` 和 macOS universal `.app`

## 最后更新时间

- 2026-07-09 08:08 +0800
