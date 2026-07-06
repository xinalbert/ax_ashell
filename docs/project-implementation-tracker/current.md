# 当前项目实施记录

## 当前目标

- 目标：恢复 GitHub Actions 中基于 tag 的 GitHub Release 自动发布链路，同时继续停用 Homebrew cask 等依赖额外外部密钥的平台发布
- 交付物：更新后的 `.github/workflows/release.yml`、通过的 workflow 配置复核结果、更新后的实施跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`.github/workflows/release.yml`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 不在本轮范围内：`Cargo.toml` / `Cargo.lock` 依赖迁移、Homebrew cask 恢复、README 改写、应用代码功能修改

## 当前状态

- 阶段：实施中
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | in_progress | 将跟踪文档 current 态、项目地图与环境记录切换到 GitHub Release workflow 收口任务 | `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | 需要从上一个 `alacritty_terminal` 迁移任务切回发布流程语境 |
| P2 | pending | 恢复 `publish` job，使用内置 `github.token` 自动创建 / 更新 GitHub Release 并附带构建产物 | 复核 `.github/workflows/release.yml` 语义 | 继续保留 `cask` 注释停用，不引入额外 secret |
| P3 | pending | 收口验证与 tracking docs 同步 | `rg -n 'secrets\\.|token:' .github/workflows/release.yml && python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | 需要确认活跃密钥路径仅使用仓库内置 `github.token` |

## 已完成

- 读取项目环境与实施跟踪 skill 约束，确认本轮需要先刷新 current 态再修改 workflow
- 复查当前 `.github/workflows/release.yml`，确认现状为：`build` 保留、`publish` 和 `cask` 被整体注释停用
- 明确本轮目标只恢复 GitHub Release 自动发布，不恢复 Homebrew cask

## 验证

- 已完成：读取当前 workflow，确认构建产物仍会通过 `actions/upload-artifact` 存入 Actions artifact
- 未完成：尚未恢复 `publish` job；尚未重新做一轮 token / secret 路径扫描与 tracking docs 校验

## 风险与阻塞

- `actions/upload-artifact` 产物不会自动出现在 GitHub Release，必须显式增加 release 上传步骤
- 若 `publish` 恢复方式处理不当，可能出现“已有 Release 时重复创建失败”或“找不到 artifact 文件”的问题
- 用户当前明确不考虑 cask，因此必须严格保持 `cask` 停用，不能重新引入 `secrets.TAP_GITHUB_TOKEN`

## 下一步

- 修改 `release.yml`：保留 `build` job，恢复 `publish` job 用内置 `github.token` 自动创建 / 更新 GitHub Release，并继续注释停用 `cask`

## 最后更新时间

- 2026-07-06 17:43 CST
