# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是修正上一版 frozen selection 仍覆盖旧终端文字的问题；目标语义调整为只固定选择高亮的 viewport 行列和复制文本快照，终端文字内容继续实时刷新

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端选区只固定高亮，不冻结文字 cell”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 运行时，`russh` SSH 后端
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`，`src/terminal.rs`，`src/terminal/element.rs`
- 本轮代码入口：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`src/app/lifecycle/event_loop.rs`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo check`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal.rs src/terminal/element.rs src/app/actions/terminal.rs`，`cargo check`，定向单元测试，`cargo test --quiet`，`git diff --check`，tracking docs validator
- 外部依赖：真实鼠标拖选、持续输出期间的选区视觉稳定性和剪贴板行为仍需 GUI 手工确认；本机自动验证只覆盖编译和现有/新增测试
- 工具可用性：本机已成功执行 `rustfmt`、`cargo check`、`cargo test --quiet frozen_ -- --nocapture`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator
- 证据文件：`Cargo.toml`，`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变，但 `current.md` 语境从“固定 viewport 行列且冻结 cell”切换到“只固定选择背景，不冻结文字 cell”；验证重点转到渲染层 selection rect 与 frozen selection 数据结构
- 受影响文件：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：根因已定位到渲染层仍使用 `frozen_cells_by_position()` 覆盖 live cell，且选择背景依赖 live cell 遍历；本轮不新增依赖，风险集中在终端渲染层
- 开工前动作：已复查 `TerminalFrozenSelection`、`capture_active_terminal_frozen_selection()`、`TerminalElement::layout_grid()` 和复制优先 frozen text 的路径
- 完成后动作：`rustfmt`、`cargo check`、定向单元测试、`cargo test --quiet`、`git diff --check` 与 tracking docs validator 已通过；GUI 手工持续输出拖选仍需实机确认
