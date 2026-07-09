# 当前项目实施记录

## 当前目标

- 目标：打开或切回 SFTP 页面时，远端路径默认定位到对应 SSH shell 当前工作目录
- 交付物：捕获 VS Code / iTerm2 / OSC 7 工作目录 escape sequence；SSH 后端提供独立 `pwd -P` 查询兜底；SFTP 页面打开和切回时按已知 shell 工作目录导航

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal.rs`，`src/backend/ssh.rs`，`src/backend/local.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace/workspace.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：安装或修改远端 shell integration、重写 SFTP 协议层、真实 SSH/SFTP 联机验证、GUI 手工验收

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 环境预检和实施计划切换到 SFTP 工作目录定位任务 | tracking docs validator | 已确认不新增依赖 |
| P2 | completed | 捕获终端 shell integration 工作目录并缓存到 SSH tab | `cargo check`，单元测试 | 支持 VS Code `OSC 633;P;Cwd=...`、iTerm2 `OSC 1337;CurrentDir=...` 和 OSC 7 |
| P3 | completed | SSH 后端新增独立 `pwd -P` 查询兜底并把结果转成事件 | `cargo check` | 不向用户交互 shell 写入命令 |
| P4 | completed | SFTP 页面打开或切回时按缓存/查询结果跳转远端目录 | `cargo check`，源码检查 | 查询失败保持现有 SFTP 路径 |
| P5 | completed | 格式化、编译测试和文档收口 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI/真实 SSH 验证仍需实机确认 |

## 已完成

- 已完成施工前环境预检，确认项目仍为 Rust / GPUI 桌面应用
- 已确认 VS Code 捕获 CWD 的主路径是 shell integration OSC 序列，不是终端主动注入可见命令
- 已因用户要求查看 VS Code 捕获方法而检索官方 VS Code terminal shell integration 文档，并记录到 `docs/project-implementation-tracker/research.md`
- 已在 `TerminalTab::feed()` 捕获 CWD 序列，并加入跨输出 chunk 的短缓冲
- 已新增 `BackendCommand::QueryWorkingDirectory`、`WorkingDirectoryChanged` 和 `WorkingDirectoryResolved`
- 已在 SSH 后端用独立 session exec `pwd -P` 查询兜底；本地后端对该命令 no-op
- 已在打开 SFTP 页面、切回已有 SFTP tab、收到 CWD 事件时同步远端 SFTP 路径

## 验证

- 已完成：`rustfmt --edition 2024 src/terminal.rs src/backend/ssh.rs src/backend/local.rs src/app/workspace/workspace.rs src/app/lifecycle/event_loop.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，30 个测试全部通过
- 已完成：tracking docs validator
- 未完成：GUI 手工验证，真实 SSH/SFTP 连接验证

## 风险与阻塞

- 风险一：精确跟随用户交互 shell 的 `cd` 依赖远端 shell integration 输出；若 shell 没有输出 VS Code / iTerm2 / OSC 7 CWD 序列，只能走兜底查询
- 风险二：`pwd -P` 兜底通过独立 SSH exec session 执行，不污染用户终端，但不等价于用户当前交互 shell 已 `cd` 后的位置
- 风险三：真实 GUI 与远端 shell integration 行为仍需在实际 SSH/SFTP 连接中确认

## 下一步

- 执行 `git diff --check`，修复发现的问题后提交本轮改动

## 最后更新时间

- 2026-07-09 13:57 +0800
