# 当前项目实施记录

## 当前目标

- 目标：让前台活动终端和 UI 刷新跟随实际显示帧节奏，最高 120Hz，同时保持空闲、后台和深睡的既有资源策略。
- 交付物：有界帧节奏采样状态、前台活动合帧间隔适配、单元测试、双语资源策略说明和验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/state/runtime.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/views/layout.rs`、`docs/resource-lifecycle.md`、`docs/resource-lifecycle.zh.md`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：`Cargo.toml` / `Cargo.lock`、GPUI 或 WGPU 依赖升级、平台私有显示器枚举、常驻动画循环、用户可配置帧率、后台/深睡时的高刷新、SSH/SFTP 架构调整。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 锁定 GPUI 帧回调、VRR / VSync 及平台刷新策略结论 | 上游源码与 WGPU 文档核对 | 不在 AxShell 内新增显示器 Hz 枚举 |
| P2 | completed | 有界帧节奏采样状态和前台事件泵接线 | 6 项单元测试、`cargo check` | 三个 GPUI animation frame 仅用于新活动 burst 校准 |
| P3 | completed | 根视图采样、双语资源策略与边界文档 | 代码审查、文档检查 | 无活动时不请求 animation frame |
| P4 | completed | 格式化、聚焦/完整测试、空白检查和跟踪记录收口 | `rustfmt`、`cargo test`、`cargo check`、validator | 真实 60/120Hz / VRR GUI 验收保留 |

## 已完成

- 已确认前台终端/UI 事件泵固定以 16ms 合并刷新，空闲前台为约 1Hz 保活，后台为 250ms，深睡为 1s。
- 已确认锁定 GPUI 仅在窗口 dirty 时重建场景；上游对非活跃窗口和热压力已有 30Hz / 60Hz 保护。
- 已确认 WGPU FIFO VSync 和 macOS / Linux 平台帧源已由 GPUI 负责；应用层无需读取或持久化显示器刷新率。
- 已实现三帧校准、60–120Hz 合帧钳制、2 秒样本过期和窗口移动/缩放、失焦、系统恢复时的样本失效。
- 已保留 idle 33ms、后台 250ms、深睡 1s 和 GPUI 的直接输入 / VRR、非活动窗口、热压力保护路径。

## 验证

- 已完成：环境记录、实施记录、项目地图、当前事件泵/运行时/根视图和锁定 GPUI 帧回调路径审查；上游联网研究；受影响 Rust 文件 `rustfmt --edition 2024`、帧节奏测试（6 项）、`cargo check`、完整 `cargo test --quiet`（200 项）、`git diff --check` 与 tracking docs validator。
- 未完成：60Hz / 120Hz / VRR 实机采样。

## 风险与阻塞

- 风险：持续满载输出在 120Hz 显示器上可增加前台 CPU/GPU 消耗；本轮只保证无活动、后台和深睡不增加常驻工作。
- 风险：跨屏切换和某些 VRR / 合成器组合可能暂时给出不稳定帧间隔；无效或过期样本必须回退现有 16ms。
- 无阻塞。

## 下一步

- 在 macOS、Windows 和 Linux 的 60Hz / 120Hz / VRR 显示器上采集前台持续输出与静止窗口的 FPS、CPU、GPU 和功耗；确认跨屏移动后下一 burst 重新校准。

## 最后更新时间

- 2026-07-15 12:10 +0800
