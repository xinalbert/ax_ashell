# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是排查并修复非 macOS CI 构建失败，失败点收敛为 `src/app/lifecycle/startup.rs` 中窗口图标 `include_bytes!` 的资源路径少退一级目录

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“非 macOS CI 资源路径修复”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 运行时，`russh` SSH 后端
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`，`src/app/lifecycle/startup.rs`
- 本轮代码入口：`src/app/lifecycle/startup.rs`
- 资源入口：`assets/icons/terminal_icon_all_formats/terminal_icon_256.png`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/lifecycle/startup.rs`，`assets/icons/terminal_icon_all_formats/terminal_icon_256.png`

## 测试环境

- 测试框架：Rust 编译检查、CI release 构建矩阵
- CI 测试命令：`.github/workflows/ci.yml` 对每个 matrix target 执行 `cargo build --release --target ${{ matrix.target }}`
- 本轮失败平台：`windows-x86_64`，`linux-x86_64`，`linux-aarch64`
- 失败原因：`include_bytes!("../../assets/icons/terminal_icon_all_formats/terminal_icon_256.png")` 从 `src/app/lifecycle/startup.rs` 相对解析时实际指向 `src/assets/...`，实际资源在仓库根目录 `assets/`，正确路径需要回到包根目录后再进入 `assets/`
- 当前修复：改用 `include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/terminal_icon_all_formats/terminal_icon_256.png"))`，避免资源引用依赖源文件目录层级
- 本机可用性：本机可执行 `rustfmt`、`cargo check`、`git diff --check`；`rustup` 不在 PATH 中，未能列出已安装 target
- 本轮验证结果：`rustfmt --edition 2024 src/app/lifecycle/startup.rs` 通过；`cargo check` 通过；`git diff --check` 通过；资源文件存在性检查通过
- 未完成验证：`cargo check --target x86_64-unknown-linux-gnu` 在下载 crates 阶段多次失败，均未进入项目编译；错误为 crates.io 下载 `Transferred a partial file`，与 CI 中的资源路径错误不是同类问题

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变，但 `current.md` 语境从上一轮 SFTP 路径任务切换到 CI 非 macOS 资源路径修复；验证重点切换到 `include_bytes!` 编译期路径和 CI matrix 构建
- 受影响文件：`src/app/lifecycle/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目边界清晰，失败日志明确指向编译期资源路径；修复不需要新增依赖或调整 CI
- 开工前动作：已复查 `Cargo.toml`、`.github/workflows/ci.yml`、`src/app/lifecycle/startup.rs` 与资源文件实际位置
- 完成后动作：已执行本机格式化、host 编译检查、diff 空白检查和资源存在性检查；跨 Linux target 编译受本机 crates.io 下载中断阻塞
