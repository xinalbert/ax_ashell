# 当前项目实施记录

## 当前目标

- 目标：修复双列 SFTP 面板顶部工具区占用过高、两列样式不一致问题
- 交付物：远端 / 本地顶部结构一致、功能按钮压缩到单行、列表区保留更多可用高度、必要的跟踪记录更新、编译验证

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：SSH / SFTP 协议层、远端传输后端、侧栏改版、README / user-guide 大改、设置页结构调整

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 current plan / env 记录并确认双列 SFTP 面板的代码落点 | `docs/` contract 自检，源码走查 | 实现边界已收敛到 `src/app/mod.rs`、`src/app/ui.rs`、`src/sftp/ops.rs` |
| P2 | completed | 新增本地文件浏览状态、导航、选择和上传到当前远端目录的 helper | `cargo check` | 保持现有远端后端命令不变，复用 `UploadPaths` |
| P3 | completed | 将 SFTP 面板拆成远端 / 本地两列并完成 tracking 校验 | `cargo check`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | 继续保留现有远端下载目录选择器语义 |
| P4 | completed | 根据截图修复双列容器纵向拉伸和列表区高度塌缩 | `rustfmt --edition 2024 src/app/ui.rs`，`cargo check` | 在 `h_flex` 容器和左右 pane 上补齐 `items_stretch` / `h_full` / `overflow_hidden` |
| P5 | completed | 根据截图将远端 / 本地功能按钮压缩到同一行并统一样式 | `rustfmt --edition 2024 src/app/ui.rs`，`cargo check` | 标题独立一行，路径、上级、功能按钮合并为紧凑工具行，按钮改为图标 + tooltip |

## 已完成

- 已读取 `docs/project-implementation-tracker/project-map.md`、`docs/project-env-audit/current.md` 与当前 tracking / env 历史
- 已在 `src/app/mod.rs` 中新增本地文件浏览状态、路径输入、滚动句柄与输入订阅
- 已在 `src/sftp/ops.rs` 中实现本地目录解析、读取、刷新、选择、全选和上传到当前远端目录的 helper
- 已将 `src/app/ui.rs` 中的 SFTP 面板重组为远端 / 本地双列布局，并保留现有拖拽上传和远端下载行为
- 已同步 `locales/en.yml` 与 `locales/zh-CN.yml`，补充本地列和“上传所选”文案
- 已根据 2026-07-08 截图反馈修复双列外层 `h_flex` 未拉伸导致的左右列错位和列表区塌缩问题
- 已根据 2026-07-08 截图反馈将远端 / 本地顶部功能按钮合并到路径行，并统一为小号图标按钮加 tooltip

## 验证

- 已完成：项目地图、SFTP UI / backend 边界和相关 tracking / env 文档走查
- 已完成：`rustfmt --edition 2024 src/app/mod.rs src/app/ui.rs src/sftp/ops.rs`
- 已完成：`rustfmt --edition 2024 src/app/ui.rs`
- 已完成：`cargo check`
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：Windows / macOS GUI 手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：本轮不改远端后端协议层，只在前端补本地浏览与上传联动；若后续要做“下载到当前本地列目录”而不是弹出目录选择器，需要再单独收口下载语义
- 风险二：本轮只做代码级验证，截图反馈对应的高度和工具条问题已从布局约束层修复，但 Windows / macOS 实际窗口尺寸下仍需手工确认无挤压和遮挡

## 下一步

- 重新运行应用并确认远端 / 本地两列顶部一致，功能按钮在同一行，列表内容有足够显示高度
- 若后续要统一双栏交互语义，再把远端下载目标改成当前本地列目录

## 最后更新时间

- 2026-07-08 07:00 CST
