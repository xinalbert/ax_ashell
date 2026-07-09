# 当前项目实施记录

## 当前目标

- 目标：修复非 macOS CI 构建失败；三个失败平台均在 `src/app/lifecycle/startup.rs` 的窗口图标 `include_bytes!` 编译期资源读取处找不到 `terminal_icon_256.png`
- 交付物：将非 macOS 窗口图标资源引用改为基于 Cargo 包根目录解析，避免源文件目录层级变化导致 CI 构建失败

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/lifecycle/startup.rs`，`assets/icons/terminal_icon_all_formats/terminal_icon_256.png`，`.github/workflows/ci.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：CI matrix 调整、release workflow 调整、图标资源重生成、Windows/Linux GUI 手工验收、依赖版本升级

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位 CI 失败根因，确认三个平台是同一个编译期资源路径问题 | CI 日志与源码路径检查 | `include_bytes!` 相对 `startup.rs` 解析，原路径少退一级目录 |
| P2 | completed | 将窗口图标资源路径改为包根目录路径 | `rustfmt --edition 2024 src/app/lifecycle/startup.rs`，`cargo check` | 使用 `env!("CARGO_MANIFEST_DIR")` 保持跨平台稳定 |
| P3 | completed | 同步环境与实施跟踪记录，补充本机验证边界 | tracking docs validator，`git diff --check` | Linux target 本机检查停在 crates.io 下载中断，未进入项目编译 |

## 已完成

- 已确认 `windows-x86_64`、`linux-x86_64`、`linux-aarch64` 三个 CI 失败日志指向同一处：`include_bytes!("../../assets/icons/terminal_icon_all_formats/terminal_icon_256.png")`
- 已确认该路径从 `src/app/lifecycle/startup.rs` 相对解析时实际指向 `src/assets/...`，而真实资源位于仓库根目录 `assets/`
- 已将图标资源引用改为 `include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/terminal_icon_all_formats/terminal_icon_256.png"))`
- 已刷新环境记录和实施跟踪记录，并在项目地图中补充当前编译期图标资源入口

## 验证

- 已完成：`rustfmt --edition 2024 src/app/lifecycle/startup.rs`
- 已完成：`cargo check`
- 已完成：`git diff --check`
- 已完成：`test -f assets/icons/terminal_icon_all_formats/terminal_icon_256.png`
- 已完成：tracking docs validator
- 未完成：`cargo check --target x86_64-unknown-linux-gnu` 多次停在 crates.io 下载中断，未进入项目代码编译；Windows/Linux GitHub Actions matrix 需要推送后复跑确认

## 风险与阻塞

- 风险一：本机普通 `cargo check` 在 macOS host 上不会编译 `#[cfg(not(target_os = "macos"))]` 分支；真正覆盖仍依赖 GitHub Actions 的 Windows/Linux 构建
- 风险二：本机 Linux target 验证受 crates.io 下载中断阻塞，无法在本机完成非 macOS target 编译

## 下一步

- 推送后复跑 CI，确认 `windows-x86_64`、`linux-x86_64`、`linux-aarch64` 不再卡在窗口图标 `include_bytes!` 路径

## 最后更新时间

- 2026-07-09 21:30 +0800
