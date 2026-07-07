# 当前项目实施记录

## 当前目标

- 目标：移除 `scripts/release_version.py` 对 legacy tag 的解析，只接受 canonical `vYYYY.M.D` / `vYYYY.M.D-N`
- 交付物：release version 脚本收紧、必要的跟踪记录更新、正反例验证

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`scripts/release_version.py`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：release workflow 结构、本地打包脚本、README / user guide 大改、terminal UI、SSH / SFTP 行为、依赖升级、历史 tag 重写

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 current plan / env 记录并确认 legacy 入口残留位置 | `docs/` contract 自检，源码走查 | 已同时覆盖 `--tag` 与 Cargo 版本入口 |
| P2 | completed | 删除 legacy 解析，并拒绝带前导零的 tag / Cargo 版本段 | `python3 scripts/release_version.py ...` 正反例 | 当前只接受 `vYYYY.M.D` / `vYYYY.M.D-N` |
| P3 | completed | 更新变更记录并完成编译 / tracking 校验 | `cargo check`，tracking docs 校验 | 已明确历史旧 tag 重跑会失败 |

## 已完成

- 已读取 `docs/project-implementation-tracker/project-map.md`、`.github/workflows/release.yml`、`scripts/release_version.py` 与当前 tracking / env 记录
- 已确认 release workflow 注释和中英文开发文档主路径已经统一到 canonical tag 格式
- 已确认当前脚本仍保留 legacy 分支，且若不额外收紧 Cargo 版本校验，`2026.07.07` 这类零填充版本仍会被接受
- 已从 `scripts/release_version.py` 删除 legacy tag 分支，改为统一的 canonical 版本构造逻辑
- 已为 year / month / day 增加无前导零约束，确保 `v2026.07.07` 和 `2026.07.07` 都会被拒绝
- 已保持 `RELEASE_PUBLIC_VERSION`、`CFBundleShortVersionString` 和 `CFBundleVersion` 的派生规则不变

## 验证

- 已完成：项目地图、版本脚本和相关文档走查
- 已完成：`python3 scripts/release_version.py env --tag v2026.7.7`
- 已完成：`python3 scripts/release_version.py env --tag v2026.7.7-1`
- 已完成：`python3 scripts/release_version.py env --tag v2026.07.07`，确认失败
- 已完成：`python3 scripts/release_version.py env --tag v2026.07.07.1`，确认失败
- 已完成：`python3 scripts/release_version.py env --cargo-version 2026.07.07`，确认失败
- 已完成：`python3 scripts/release_version.py env --cargo-version-file Cargo.toml`
- 已完成：`cargo check`
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：无

## 风险与阻塞

- 阻塞：无
- 风险一：历史旧格式 tag 若重跑当前 workflow，将因脚本拒绝 legacy 格式而失败；这是本轮刻意收紧的结果

## 下一步

- 后续远端发布只使用 canonical tag：`vYYYY.M.D` / `vYYYY.M.D-N`

## 最后更新时间

- 2026-07-07 21:59 CST
