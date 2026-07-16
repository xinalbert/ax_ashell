# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮实现 SFTP 远端文件的受管编辑与确认上传，复用既有 SSH、SFTP worker、`notify` 和默认系统打开器。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已按串口/Telnet 运行依赖和验证结果刷新。
- changes.md：已追加本轮环境与验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、Alacritty Terminal、russh、russh-sftp、`serialport 4.9.0`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`；新增 `serialport = "4"` 后锁定 `serialport 4.9.0` 及其平台相关依赖。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/init.rs`。
- 本轮代码入口：`src/app/actions/sftp.rs`、`src/app/views/sftp_panel.rs`、`src/app/workspace.rs`、`src/app/dialogs/`、`src/sftp/worker.rs`、`src/sftp/worker/runtime.rs`、`src/events.rs`。
- 平台前提：系统必须有默认文件关联。`open` 在 macOS 使用 `/usr/bin/open`，Windows 使用 `start`，Linux 依次使用 `xdg-open` / `gio open` 等；默认启动器不提供实际编辑器退出通知。文件变化由现有 `notify 8` 平台后端提供，真实行为仍依赖编辑器保存策略。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu x86_64/aarch64 和 macOS x86_64/aarch64 运行 release build；Linux runner 安装 GPUI 与 `libudev` 系统库。
- 外部服务：自动化测试不依赖真实 SFTP 或默认编辑器；真实下载、默认应用关联、原子保存、上传确认、网络中断和远端权限需在目标系统与 SFTP server 手动测试。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`src/app/actions/sftp.rs`、`src/app/views/sftp_panel.rs`、`src/app/workspace.rs`、`src/sftp/worker/runtime.rs`、`src/events.rs`、`open 5.3.6`、`notify-types 1.0.1`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：不新增依赖；已有 `open 5.1` 只能请求系统默认应用打开文件，无法报告实际编辑器关闭；已有 `notify 8` 能对目录变化提供 `Create` / `Modify` / `Remove` 等事件。
- 受影响文件：`src/app/`、`src/sftp/`、`src/events.rs`、`locales/`、`docs/features/`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；`current.md` 已更新，`changes.md` 待实现与验证完成后追加。

## 开工判定

- 状态：允许开工。
- 原因：本机 toolchain 高于仓库 MSRV；目标功能只复用已锁定的 `open`、`notify`、Tokio、GPUI、SFTP worker 和模态对话框能力，不需要依赖或运行环境迁移。worker 负责 I/O 与文件监听，SFTP 虚拟列表不增加昂贵的 hover/render 工作。
- 完成动作：已读取环境、实施记录、项目地图和 fast hover 工作流；已审查 SFTP 打开/上传、页面关闭和窗口迁移路径，并核对系统打开器和文件监听能力。实现后的自动化验证待运行。

## 最后确认时间

- 2026-07-16 18:10 +0800
