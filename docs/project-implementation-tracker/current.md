# 当前项目实施记录

## 当前目标

- 目标：增强运行日志可观测性，增加日志/崩溃目录入口、启动摘要日志，并扩大日志保留窗口
- 交付物：About 页面日志目录入口、启动摘要 `tracing` 日志、更合理的默认日志 filter 和保留数量、格式化/编译/测试验证结果，以及同步的 tracking 记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/startup.rs`，`src/app/dialogs.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：捕获所有 stdout/stderr、dev-reload 日志合并、第三方库 tracing 接管、GUI 手工回归、依赖升级

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮环境预检、实施计划和地图范围确认 | tracking docs validator | 不联网、不使用多 agent |
| P2 | completed | 日志/崩溃目录公开入口和 About 页面按钮 | `cargo check`，源码级检查 | 只打开目录，不读取或上传日志内容 |
| P3 | completed | 启动摘要日志、默认 filter 收紧、日志保留数量提升 | `cargo check`，源码级检查 | 保持 tracing 全局 subscriber 结构 |
| P4 | completed | 文案同步和收口验证 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 打开目录需手工验证 |

## 已完成

- 已确认当前日志系统是全局 tracing subscriber，但只覆盖 `tracing::*` 调用
- 已确认 `src/app/startup.rs` 负责运行日志和 crash report 路径，`src/app/dialogs.rs` About 页面适合放目录入口
- 已在 About 页面显示运行日志和崩溃报告目录，并提供打开目录按钮
- 已将默认日志 filter 调整为 `ax_shell=info,warn`，并在启动后写入版本、平台、配置目录、日志目录和保留数量摘要
- 已将运行日志保留数量从 6 个提升到 48 个
- 已同步中英文文案和开发文档日志说明

## 验证

- 已完成：日志实现只读评估
- 已完成：施工前环境预检刷新
- 已完成：`rustfmt --edition 2024 src/app/startup.rs src/app/dialogs.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，15 个测试全部通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI 打开日志目录 / 崩溃报告目录手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：默认 filter 收紧不能隐藏应用自身 `info` 日志
- 风险二：打开目录入口需要处理目录不存在或打开失败，不能影响设置页渲染
- 风险三：GUI 打开目录行为仍需用户手工验证
- 风险四：这仍不是全 stdout/stderr 捕获方案；只有 `tracing::*` 进入主应用日志

## 下一步

- 在应用内手工打开 About 页面，确认两个目录按钮能正常唤起系统文件管理器

## 最后更新时间

- 2026-07-08 11:34 CST
