# 当前项目实施记录

## 当前目标

- 目标：将项目名称改为 `AxShell`，并统一代码、Cargo 包、二进制、打包元数据、运行时目录、同步默认文件名和中英文文档中的当前项目标识
- 交付物：`AxShell` 显示名、`ax_shell` 机器名、Linux desktop 文件重命名、macOS bundle 元数据、release workflow 产物名、旧 `ax_ashell` 配置迁移、双语 README/docs 更新、格式化编译验证、跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`，`Cargo.lock`，`assets/ax_shell.desktop`，删除旧 desktop 文件，`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`examples/dev_reload.rs`，`src/` 当前项目标识引用，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：SSH / SFTP 协议行为、主题编辑器行为、终端渲染策略、依赖升级、真实远端仓库重命名、GUI 自动截图验证、安装包实机安装验证

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 current plan、项目地图和环境记录到“AxShell 项目改名”任务 | `docs/` contract 自检，源码走查 | 不联网，不使用多 agent |
| P2 | completed | 统一 Cargo 包名、Rust 类型名、动作 namespace、二进制名、desktop 文件、bundle 元数据和 release workflow 产物名 | 残留旧名检索，`cargo check` | 历史变更日志中的旧名保留 |
| P3 | completed | 新增旧 `~/.config/ax_ashell` 到 `~/.config/ax_shell` 的首次启动迁移兼容 | `cargo test`，源码走查 | 只复制缺失的 `sessions.json` 和 `themes/`，不删除旧目录 |
| P4 | completed | 同步中英文 README、用户指南和开发打包文档 | README/doc 链接与命令检索 | 保持 README 简洁，不重写结构 |
| P5 | completed | 运行格式化、编译、测试、残留名称检索和 tracking docs 校验 | `rustfmt`，`cargo check`，`cargo check --example dev_reload`，`cargo test`，`rg`，tracking docs 校验 | GUI/安装包实机验证不在自动流程内 |

## 已完成

- 已确认本轮采用 `AxShell` 作为用户可见显示名，`ax_shell` 作为 Cargo 包、二进制、配置目录、同步默认对象名和 CI artifact 的机器名
- 已完成机械替换 `AxAshell -> AxShell`、`ax_ashell -> ax_shell`、`AX_ASHELL -> AX_SHELL`，并把 `assets/ax_ashell.desktop` 重命名为 `assets/ax_shell.desktop`
- 已保留历史实施日志里的旧名称，不改写过去记录
- 已在 `src/session/config.rs` 新增旧配置目录兼容迁移：新目录缺失时复制旧 `sessions.json` 与 `themes/`
- 已把 `src/app/startup.rs`、`scripts/package-macos-app.sh` 和 `.github/workflows/release.yml` 的显示名收口为 `AxShell`
- 已把 macOS release/app bundle 可见文件名改为 `AxShell.app`，内部可执行文件保持 `ax_shell`
- 已修复 `.github/workflows/release.yml` 和 `examples/dev_reload.rs` 中遗漏的旧 `ax_ashell` 硬编码，使 release packaging 与 dev-reload 都回到 `AxShell` / `ax_shell`
- 已同步中英文 README、user guide 和 development 文档，并更新 GitHub Release 发布状态说明

## 验证

- 已完成：`rustfmt --edition 2024 --config skip_children=true src/app/startup.rs src/backend/local.rs src/session/config.rs examples/dev_reload.rs src/main.rs src/app/mod.rs src/app/ui.rs src/app/dialogs.rs src/app/keybinding_recorder.rs src/app/config_sync.rs src/app/search.rs src/app/theme.rs src/session/mod.rs src/sftp/mod.rs src/sftp/ops.rs src/sync/mod.rs src/terminal/element.rs src/terminal/input.rs`
- 已完成：`cargo check`
- 已完成：`cargo check --example dev_reload`
- 已完成：`cargo build --bin ax_shell`
- 已完成：`.github/workflows/release.yml` 的 YAML 解析和 `Generate release highlights` 的 `bash -n` 静态检查
- 已完成：`cargo test`，共 13 个测试通过
- 已完成：残留旧名检索；非历史区域只保留旧 `ax_ashell` 配置目录迁移代码和升级说明
- 已完成：tracking docs 校验通过
- 未完成：平台级手工验证 `AxShell.app`、Linux desktop entry 和 GitHub Release artifact 的最终展示

## 风险与阻塞

- 风险一：真实远端仓库地址和本地目录名不在本轮自动重命名范围内；文档中的新仓库地址是假定后续仓库同步改名后的目标地址
- 风险二：旧配置目录迁移采用复制而非移动，避免破坏回退，但用户同时运行旧版和新版时可能产生两个配置目录
- 风险三：macOS `.app`、Linux `.desktop` 和 GitHub Release artifact 的真实安装展示仍需平台手工确认

## 下一步

- 如需完整发布验证，下一步在目标平台手工构建并打开 `AxShell.app`、Linux desktop entry 和 GitHub Release artifact

## 最后更新时间

- 2026-07-07 19:35 CST
