# 当前项目实施记录

## 当前目标

- 目标：维护双语 README 的当前产品与发布入口，并将 GitHub release workflow 中残留的旧仓库链接统一为 `https://github.com/xinalbert/axshell`。
- 交付物：同步的 `README.md` / `README.zh.md`，以及链接正确的 `.github/workflows/release.yml` 注释模板。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`README.md`，`README.zh.md`，`.github/workflows/release.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：应用代码、发布流程步骤、构建矩阵、artifact 命名、依赖、`Cargo.toml`、`Cargo.lock`、GitHub 远端配置。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | README、workflow 与 Git remote 链接审查 | `rg`、`git remote -v` | `origin` 已指向用户指定的 GitHub 仓库；仅 release workflow 注释仍使用旧路径 |
| P2 | completed | 双语 README 与 release workflow 链接更新 | 双语结构/链接静态检查 | README 保持简短，补充 SFTP 下载明细和 release 入口 |
| P3 | completed | 文档和 tracking 收口 | `git diff --check`、tracking docs validator | 无 Rust 源码变动，无需构建或测试 |

## 已完成

- 已读取环境记录、实施记录、项目地图和 README 维护规范。
- 已确认 `README.md` 与 `README.zh.md` 顶部互链，主页和 Issues 链接已使用目标仓库。
- 已确认 `origin` 为 `https://github.com/xinalbert/axshell.git`；`.github/workflows/release.yml` 停用的 Homebrew Cask 模板原有三个 `xinalbert/ax_shell` 链接。
- 已在双语 README 补充批量下载文件明细和 GitHub Releases 入口，保持两种语言的章节结构一致。
- 已将 Cask 模板的两条 release URL 和 homepage 统一改为 `https://github.com/xinalbert/axshell`。

## 验证

- 已完成：README/文档索引和 release workflow 审查；Git remote 与用户提供的仓库 URL 核对；目标文件中正确 URL 检索；目标范围内旧路径检索为空；`preview.png` 存在；`git diff --check`。
- 未完成：无。

## 风险与阻塞

- Homebrew Cask job 保持停用；本轮仅维护其示例 URL，不改变启用条件或打包语义。
- 无阻塞。

## 下一步

- README 与 workflow 链接维护已完成；后续启用 Cask job 时应使用当前示例中的 GitHub 仓库路径。

## 最后更新时间

- 2026-07-14 22:37 +0800
