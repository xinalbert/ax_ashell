# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用的真实功能改动；本轮目标是将自定义主题里的亮度控制收口到终端显示区域，并把允许范围调整为 0.60-1.20。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端字体亮度作用域和范围收口”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/app/theme.rs`，`src/session/config.rs`，`src/terminal/element.rs`，`locales/en.yml`，`locales/zh-CN.yml`
- 渲染依据：custom theme 生成由 `src/app/theme.rs` 负责，终端前景色亮度仍由 `src/terminal/element.rs` 在 terminal 渲染阶段读取 `active_custom_font_brightness` 处理
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/app/theme.rs`，`src/session/config.rs`，`src/terminal/element.rs`，`locales/en.yml`，`locales/zh-CN.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 ...`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；设置页文案和终端亮度视觉效果需要 GUI 手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/theme.rs`，`src/session/config.rs`，`src/terminal/element.rs`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；GUI 设置页和终端亮度视觉效果未手工验证

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从终端光标自动反差色切换到终端字体亮度作用域和范围收口；运行环境不变，验证入口仍为格式化、全仓编译、全仓测试和 tracking docs 校验
- 受影响文件：`src/app/theme.rs`，`src/session/config.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：亮度作用域可通过移除 theme 生成阶段的全局改写并保留 terminal 渲染阶段处理完成，不需要新增依赖或改配置格式
- 开工前动作：已复查当前 custom theme 生成、配置读取/写入夹取范围、设置页文案和 terminal 前景色亮度路径；已确认不需要联网、不使用多 agent
