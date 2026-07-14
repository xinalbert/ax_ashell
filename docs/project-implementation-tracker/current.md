# 当前项目实施记录

## 当前目标

- 目标：参考 WinSCP，为 SFTP 下载记录提供“批量任务 + 文件明细”模型，使多选和目录递归下载可以查看所有已发现文件，而非只保留一条笼统批量记录。
- 交付物：兼容的下载文件明细持久化模型、worker 到 UI 的文件状态事件、当前文件/文件数概览、文件清单对话框、双语文案、测试与验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/sftp/model.rs`，`src/events.rs`，`src/sftp/transfer.rs`，`src/sftp/worker/runtime.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs/transfers.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：SFTP 协议、上传任务的逐文件视图、批量传输并发度、目录预扫描、单文件重试、依赖版本、`Cargo.toml` / `Cargo.lock`、CI workflow。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 现状、环境与 WinSCP 队列模型确认 | 源码审查、WinSCP 官方文档 | 顶层继续保留现有批量任务；文件在下载时动态发现，避免递归预扫描 |
| P2 | completed | 下载文件模型、事件和 worker 状态上报 | 模型/事件单元测试、`cargo check` | 只追加下载文件明细，不改变上传行为或批量暂停/取消边界 |
| P3 | completed | SFTP 传输概览与完整文件清单界面 | `cargo check`、代码审查 | 主列表保持 `uniform_list` 固定行；文件列表通过对话框查看 |
| P4 | completed | 自动化验证和文档收口 | `cargo test --quiet`、`git diff --check`、tracking validator | 真实 SFTP 多选和递归目录下载需手工确认 |

## 已完成

- 已完成环境预检、当前实施记录/项目地图审查和 SFTP 快速列表规则审查。
- 已完成 WinSCP 官方队列模型检索：顶层队列行代表后台任务；多文件任务显示总体进度与当前文件，并可查看完整文件列表。
- 已确认 AxShell 的 `DownloadPaths` 仅创建一个 `Transfer` 和一个 transfer ID；批量内文件、目录递归和进度均复用该 ID，因此 UI 无法保留文件级历史。
- 已确认文件可以在 `download_file_impl` 中逐个动态发现并上报，应用 event loop 可作为唯一的持久化 UI 状态写入点。
- 已新增兼容的 `TransferFile` / `TransferFileState`，旧传输历史缺少 `files` 时保持为空；下载路径在文件开始和结束时上报成功、跳过、失败或取消状态。
- 已把文件事件接入 event loop、持久化历史、SFTP 任务状态行与文件清单对话框；长清单通过 `uniform_list` 虚拟渲染。
- 已完成格式化、`cargo check` 及文件状态/传输筛选聚焦测试。

## 验证

- 已完成：环境/项目地图/源码审查；WinSCP 官方传输队列文档检索；`rustfmt --edition 2024`；`cargo check`；`cargo test --quiet transfer_history`（2 项）；`cargo test --quiet transfer_file_state`（1 项）；`cargo test --quiet transfer_filter`（2 项）；完整 `cargo test --quiet`（185 项）；SFTP hover/list 静态审计。
- 未完成：真实 GUI 多选/目录递归下载验收；最终 `git diff --check` 与 tracking docs validator 在本次文档收口后复跑。

## 风险与阻塞

- 风险：目录文件在递归读取时动态发现，任务运行期间已发现文件数不是预先计算的总数；完成后清单才是完整集合。
- 风险：文件明细会随传输历史一并持久化；大量目录下载会增加配置数据体积，但任务数仍沿用既有的 100 条上限。
- 无阻塞。

## 下一步

- 在真实 SFTP 环境确认多选和递归目录下载时，任务行显示当前文件，文件清单持续追加并在结束后保留所有文件状态。

## 最后更新时间

- 2026-07-14 20:09 +0800
