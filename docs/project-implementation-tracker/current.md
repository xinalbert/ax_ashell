# 当前项目实施记录

## 当前目标

- 目标：将仓库内项目标识从 `ashell` 统一迁移到 `ax_ashell`，同步修正代码标识、包名、资源名、脚本、CI、用户可见文案以及 GitHub 仓库地址；停用当前不想启用的、依赖外部 token / 密钥的发布链路；并将 README 与用户可见版本规则收敛到日期版本策略
- 交付物：统一后的 `ax_ashell` 代码/文档/资源引用、指向 `https://github.com/xinalbert/ax_ashell` 的 GitHub 链接、切换到 `terminal_icon_all_formats` 的图标来源、停用 `publish` / `cask` 后的 release workflow、双语 README、日期版本展示策略、编译验证结果、更新后的实施跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`，`Cargo.lock`，`README.md`，`README.en.md`，`build.rs`，`assets/`，`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`examples/dev_reload.rs`，`src/` 中包含 `ax_ashell` 标识或版本展示的代码文件，仓库内指向旧 GitHub 地址的文档/脚本文件，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 不在本轮范围内：与改名无关的功能行为修复、SSH 协议兼容策略、监控仪表盘逻辑、标题栏交互逻辑重构

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮改名任务的范围、current 态与项目地图刷新 | `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py <repo-root>` | 已将 current 态切换到 `ashell -> ax_ashell` 标识迁移任务 |
| P2 | completed | 代码、资源、脚本、CI 与文档中的 `ashell` 标识统一迁移到 `ax_ashell`，并将 GitHub 链接改到 `https://github.com/xinalbert/ax_ashell` | `rg -n --hidden --glob '!.git' 'github\\.com/rust-kotlin/ashell|\\bashell\\b|Ashell|ASHELL' .` | 需要同时处理文本替换、GitHub 链接更新与资源/文件名重命名 |
| P3 | completed | 编译校验、README 收口、日期版本展示策略、停用依赖外部密钥的发布步骤、剩余引用扫描与跟踪文档收口 | `cargo check && rg -n 'secrets\\.|token:' .github/workflows && python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | 需要区分“对外日期版本展示”和“内部 semver 构建版本” |

## 已完成

- 读取项目环境与实施跟踪 skill 约束，确认本轮需要先做环境预检与 plan-first 跟踪
- 刷新 `docs/project-env-audit/` 与 `docs/project-implementation-tracker/` 到当前 contract
- 扫描仓库内 `ashell` / `Ashell` / `ASHELL` 与旧 GitHub 地址引用，确认范围覆盖 crate 名、结构体名、窗口标题、TERM_PROGRAM、同步文件名、资源文件名、打包脚本、GitHub Actions、README 文案与仓库链接
- 确认存在需要同步重命名的资源文件：`assets/ax_ashell.desktop`、`assets/icons/ax_ashell.icns`、`assets/icons/ax_ashell.ico`、`assets/icons/ax_ashell.png`
- 已将 Windows / macOS / Linux / 运行时图标来源切换到 `assets/icons/terminal_icon_all_formats/`
- 已确认 `.github/workflows/release.yml` 中显式依赖外部密钥的是 Homebrew cask 发布使用的 `secrets.TAP_GITHUB_TOKEN`
- 已将双语 README 精简为项目入口页，补充 fork 来源、当前仓库地址、发布状态与日期版本规则
- 已将内部包版本起点切到 `2026.7.6`，并补充设置页 / macOS bundle 的对外日期版本展示映射

## 验证

- 已完成：扫描 `Cargo.toml`、`README.md`、`src/`、`assets/`、`.github/workflows/release.yml` 与脚本中的现有 `ax_ashell` 标识及旧 GitHub 地址；确认本机 `cargo` 可用；`cargo check` 通过；`rg -n 'secrets\\.|token:' .github/workflows` 仅剩注释中的 `secrets.TAP_GITHUB_TOKEN`；tracking docs 校验通过
- 未完成：未做 GUI 手工验证

## 风险与阻塞

- crate / bin 名、桌面入口、应用 bundle 名与资源文件路径存在联动，若替换不彻底会直接导致编译或打包失败
- `AxAshell` 结构体名和 `ax_ashell` 字面量同时存在，必须区分大小写变体，避免误把其他标识改坏
- 用户要求的 `YYYY.MM.DD.1` 不符合 Cargo semver，必须通过“对外日期显示 / 内部兼容版本”双层策略实现
- 若未来重新启用 `publish` / `cask`，需要恢复对应 job，并重新配置 GitHub Release 与 Homebrew tap 所需权限 / token

## 下一步

- 按功能边界拆分并提交本轮改动

## 最后更新时间

- 2026-07-06 17:21 CST
