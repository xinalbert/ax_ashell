# 当前项目实施记录

## 当前目标

- 目标：将 SSH X11 forwarding 改为每个会话独立控制，未发现本机 X server 时在 SSH 新建/编辑窗口给出简短安装提示。
- 交付物：会话级 `x11_forwarding` 持久化、默认开启的表单开关、非阻塞安装提示、VcXsrv/Xming 分类与无自动启动 relay、由 `sshd` 分配远端 `DISPLAY` 的连接路径、双语说明和自动化验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/session.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/dialogs/ssh.rs`，`src/app/dialogs/settings/proxy.rs`，`src/backend/ssh.rs`，`src/backend/ssh/x11.rs`，`src/platform/x_server.rs`，`src/config/model.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/features/proxy-x11.md`，`docs/features/proxy-x11.zh.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 `Cargo.toml` / `Cargo.lock`、启动或安装第三方本地 X server、修改远端 `sshd` 配置、改变 SSH/SFTP 认证协议、引入新的 X11 crate。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 会话级 X11 配置和 SSH 表单开关 | Session serde 与表单数据流审查 | 新建和旧会话默认开启，可在编辑窗口单独关闭 |
| P2 | completed | 本机 X server 检测、Windows 分类和无自动启动 relay | `cargo check` | VcXsrv/Xming 独立识别；仅用户手动按钮能启动本机服务 |
| P3 | completed | 双语说明、回归测试与收口记录 | `cargo test --quiet`、`git diff --check`、tracking validator | Windows 和真实远端 `sshd` 仍需手工验收 |

## 已完成

- `Session.x11_forwarding` 使用 serde 默认值 `true`，旧保存会话和分享文件缺失字段时保持默认开启。
- SSH 新建、编辑和复制会话均显示会话级 X11 开关；开启且未发现 `DISPLAY` 或配置的本机 X server 时，仅显示安装提示，不阻止保存或连接。
- SSH 连接只在该会话开启时发送 `request_x11`；不再硬编码远端 `DISPLAY`，由 `sshd` 分配实际值。
- X11 relay 不再自动启动本地 X server；Settings 仅保留路径配置和用户主动的“打开 X server”操作。

## 验证

- 已完成：相关 Rust 文件 `rustfmt --edition 2024`、3 项会话/X11 聚焦测试、`cargo check`、完整 `cargo test --quiet`（171 项）、`git diff --check` 和 tracking docs validator。
- 未完成：Windows VcXsrv/Xming 和远端 GUI 的手工验收。

## 风险与阻塞

- 无阻塞；`DISPLAY` 存在只能证明可尝试连接，实际 X server 监听、xauth、远端 `X11Forwarding yes` 和 `sudo` 环境策略仍须实际环境验证。

## 下一步

- 在 Windows 分别验证 VcXsrv 和 Xming、远端 `sshd` 的 `X11Forwarding yes`、`echo $DISPLAY` 和图形程序启动；必要时检查 `sudo` 是否清除了 `DISPLAY` / `XAUTHORITY`。

## 最后更新时间

- 2026-07-14 10:57 +0800
