# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust 项目的真实代码修复任务；本轮需先执行并刷新 `project-env-audit`，再修改 `dev-reload` 行为

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“`cargo dev-reload` Windows 重载顺序修复”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / alacritty_terminal / russh
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`examples/dev_reload.rs`，`docs/development.md`，`docs/development.en.md`
- 依赖统一策略：根项目 `gpui` / `gpui_platform` / `menu` 保持 plain git source，通过 `Cargo.lock` 统一 pin 到单一 Zed 提交，避免和 `gpui-component` 形成双 source id
- 证据文件：`Cargo.toml`，`.cargo/config.toml`，`examples/dev_reload.rs`，`docs/development.md`，`.github/workflows/ci.yml`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 examples/dev_reload.rs`，`cargo check --example dev_reload`，`cargo test --example dev_reload`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮已访问 Cargo git index / git checkout 获取较新 `zed-industries/zed` 提交；运行期不依赖外部服务
- 工具可用性：本机 `cargo` 可正常执行；`dev_reload` example 存在独立单元测试，可直接做局部验证
- 证据文件：`examples/dev_reload.rs`，`Cargo.toml`，`.github/workflows/ci.yml`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从依赖统一升级切换到 `cargo dev-reload` 的 Windows 平台行为修复；验证入口同步切换到 `examples/dev_reload.rs` 的局部编译和测试
- 受影响文件：`examples/dev_reload.rs`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链、`cargo` alias 与 `dev_reload` example 都已就位；当前问题局限在本地 runner 逻辑，可通过局部编译和测试完成验证
- 开工前动作：已复查 `.cargo/config.toml`、`examples/dev_reload.rs`、开发文档和现有 tracking 记录；已确认现有实现与 Windows 分支注释冲突，并已按局部编译/测试路径完成修正与验证
