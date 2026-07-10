# 当前项目实施记录

## 当前目标

- 目标：让终端输入继承常见系统文本导航习惯，支持 macOS `Command+←/→` 行首/行尾、macOS `Option+←/→` 按词移动，并保留 Windows/Linux `Ctrl+←/→` 的终端序列行为
- 交付物：`src/terminal.rs` 平台文本导航按键映射；相关单元测试；环境记录、检索记录和月度实施记录同步更新

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal.rs`，`src/app/actions/terminal.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：全局 keybinding 系统重构、终端设置页新增选项、IME 组合输入逻辑、PTY 后端协议、release/tag、提交

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 官方资料与本地按键链路结论 | `docs/project-implementation-tracker/research.md` 已更新 | 已确认 macOS 文本导航、Readline 序列和 xterm modified cursor 规则 |
| P2 | completed | 平台文本导航映射实现 | `rustfmt --edition 2024 src/terminal.rs` 通过 | 只在终端编码层增加别名，避免改变 UI 动作层 |
| P3 | completed | 单元测试覆盖快捷键字节序列 | `cargo test --quiet terminal::tests::` 通过 | 覆盖 macOS 映射与现有 `Ctrl+Arrow` |
| P4 | completed | 编译、完整测试与文档校验结果 | `cargo check`，`cargo test --quiet`，`git diff --check`，tracking docs validator 均通过 | GUI 手工验证保留为后续风险 |

## 已完成

- 已完成施工前环境预检，并确认本轮不新增依赖、不改配置 schema
- 已检索官方资料并确定映射策略：macOS `Command+←/→` 转为 Readline `C-a/C-e`，macOS `Option+←/→` 转为 Readline `M-b/M-f`，Windows/Linux `Ctrl+←/→` 保留 xterm modified cursor
- 已确认 `src/app/actions/terminal.rs` 将 GPUI `KeyDownEvent` 交给 `src/terminal.rs::encode_key()`，本轮主修改点在终端编码层
- 已修改 `src/terminal.rs`，新增平台文本导航映射 helper 和跨平台可测的 `TerminalPlatform` 分支；已补充快捷键字节序列单元测试

## 验证

- 已完成：官方资料检索；本地代码链路定位；环境记录刷新；`rustfmt --edition 2024 src/terminal.rs`；`cargo test --quiet terminal::tests::`，11 个测试通过；`cargo check`；`cargo test --quiet`，50 个测试通过；`git diff --check`；tracking docs validator
- 未完成：GUI 手工按键验证

## 风险与阻塞

- 风险一：macOS `Option` 在非英文键盘上可能用于输入字符，因此本轮只对 `Option+Arrow` 做显式文本导航别名，不全局启用 `option_as_meta`
- 风险二：真实 shell/readline/zsh 绑定可能被用户配置覆盖；本轮只能保证发送的字节序列符合常见约定
- 风险三：GUI 手工按键验证需要在真实 macOS、Linux 或 Windows 会话中确认

## 下一步

- 在真实 GUI 中确认 macOS `Command+←/→`、macOS `Option+←/→` 和 Windows/Linux `Ctrl+←/→` 的终端输入手感

## 最后更新时间

- 2026-07-10 08:00 +0800
