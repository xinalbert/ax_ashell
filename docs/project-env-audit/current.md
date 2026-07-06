# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：本轮将进入真实实现，必须先执行 `project-env-audit`

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为当前任务的 current 态

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh
- 版本约束：`rust-version = 1.85.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 证据文件：`Cargo.toml`，`src/app/startup.rs`，`README.md`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：Linux 构建需要系统 GUI 库；集成标题栏行为受 GPUI 平台层实现影响；当前策略为 macOS 保留集成标题栏并由应用层控制可拖区域，非 macOS 直接回退到系统原生标题栏，不再依赖额外的 Windows 原生拖窗 helper
- 证据文件：`.github/workflows/ci.yml`，`Cargo.toml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行环境事实未变，但当前任务已扩展到 `ax_ashell` 标识迁移、README 收口、日期版本展示策略与发布 workflow 收敛，因此需要刷新 current 记录中的任务语境、影响范围与验证重点
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目边界明确，Cargo/Rust 工具链可用，问题可在仓库内独立修复
- 开工前动作：完成 `docs/project-implementation-tracker/` 计划与地图更新，再统一修改 crate 名、运行时文案、资源文件名、脚本和 CI 中的 `ax_ashell` 引用，收口双语 README 与日期版本策略，并停用依赖外部 token 的发布步骤
