# 当前项目实施记录

## 当前目标

- 目标：实现不依赖原生平台 API 的系统 suspend/resume MVP 兜底，在长时间未调度后安全恢复应用状态，避免旧监控结果、假活连接和恢复风暴。
- 交付物：基于长调度间隙的恢复检测、幂等恢复 reducer、远程监控代次隔离、仅活动 SSH 的单次健康探测、活动 SFTP 的可能失效标记、单元测试和双语资源生命周期文档。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/state/lifecycle.rs`、`src/app/state/monitoring.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace.rs`、`src/events.rs`、`src/backend/ssh.rs`、`src/app/actions/sftp.rs`、`docs/resource-lifecycle.md`、`docs/resource-lifecycle.zh.md`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：macOS `NSWorkspace`、Windows `WM_POWERBROADCAST`、Linux logind D-Bus 原生事件；自动重连 SSH；自动重启或续传 SFTP；终端/SFTP 架构重构；依赖、`Cargo.toml`、`Cargo.lock`、CI workflow 与退出流程重构。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 环境预检、现有生命周期/监控/SSH/SFTP 边界和 MVP 范围 | 环境记录、项目地图、源码审查 | 固定 10 秒调度间隙作为跨平台恢复兜底，避免误判后台节流 |
| P2 | completed | 恢复 reducer、调度间隙检测和监控代次隔离 | 4 项恢复相关单元测试、`cargo check` | 旧 probe 结果不能影响恢复后的当前页面 |
| P3 | completed | 当前上下文单次健康检查与 SFTP 可能失效状态 | SSH/SFTP action 与 event-loop 审查、完整测试 | 不自动重连、不批量采样、不续传 |
| P4 | completed | 文档、完整验证与记录收口 | `cargo test --quiet`、`git diff --check`、tracking validator | 原生三平台电源事件作为正式阶段后续工作 |

## 已完成

- 已完成 Rust 2024 / Cargo 施工前预检；本机 `rustc 1.96.1`、`cargo 1.96.1` 满足仓库 `rust-version = 1.88.0`。
- 已确认 `Foreground / Background / DeepSleep` 使用 GPUI 窗口激活事件，后台 250ms、深睡 1s；尚未接入 OS suspend/resume。
- 已确认 SSH/SFTP 关闭使用 2 秒 timeout/abort，SFTP worker 使用 work pin；这些机制不需要重构。
- 已确认系统恢复后风险集中在过期 remote probe、监控 in-flight 标记、SFTP server-side handle 假活和恢复风暴。
- 恢复兜底同时比较单调时钟和墙上时钟的 10 秒事件泵间隙；恢复后监控代次递增，旧 remote probe 结果被忽略。
- 仅当前可见 terminal SSH 会发起一次 5 秒 SSH session-open 健康检查。检查事件还绑定 terminal backend 代次，用户重连后的旧结果不能关闭或改写新连接。
- 空闲 SFTP worker 被标记为下次用户操作时重建；有 work pin、活动/暂停传输、远程编辑或排队操作的 worker 保持不动，且不会自动恢复传输。

## 验证

- 已完成：环境预检、实施记录与项目地图审查、生命周期/监控/SSH/SFTP worker 恢复边界审查、受影响 Rust 文件 `rustfmt --edition 2024`、恢复相关单元测试（4 项）、`cargo check`、完整 `cargo test --quiet`（194 项）、`git diff --check` 与 tracking docs validator。
- 未完成：macOS、Windows、Linux 的睡眠、可用时休眠、睡眠期间网络变化、活动 SSH、空闲 SFTP 页面和带 pin 传输的实机验收；正式原生电源事件接入。

## 风险与阻塞

- 风险：长调度间隙是通用兜底，不能区分系统睡眠、调试器暂停或极端主线程阻塞；正式阶段必须接入每个平台原生电源事件。
- 风险：标准 SSH 不能恢复断开的交互 shell；MVP 只能提示并提供已有的用户主动重连路径。
- 风险：SFTP 传输的安全续传需要单独的断点、远端大小校验和覆盖策略设计；本轮仅标记可能失效，不自动重新开始。
- 无阻塞。

## 下一步

- 在三平台实机按资源生命周期文档执行睡眠/唤醒与断网矩阵；之后单独设计 `PowerEvent::Suspend/Resume` 的原生事件适配层，继续保留本轮 10 秒兜底。

## 最后更新时间

- 2026-07-15 11:09 +0800
