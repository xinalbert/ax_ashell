# 当前项目实施记录

## 当前目标

- 目标：为 SSH 连接增加老服务器算法兼容 fallback，并把 `No common algorithm` 的具体协商失败类型与双方算法列表暴露出来
- 交付物：`russh` 客户端兼容旧服务器的算法配置、包含 `kind/ours/theirs` 的诊断日志与错误信息、编译验证结果、更新后的实施跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/backend/ssh.rs`，`Cargo.toml`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 不在本轮范围内：GUI 布局、标题栏行为、监控仪表盘设置、SFTP 功能实现、终端渲染逻辑

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮 SSH 兼容性任务的范围、地图与 current 态刷新 | `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py <repo-root>` | 已将 current 态和项目地图切换到 SSH 兼容性任务 |
| P2 | completed | 老服务器算法 fallback 与协商失败诊断增强 | `cargo check` | 默认失败后自动切到 legacy compatibility；`No common algorithm` 会包含 `kind/ours/theirs` |
| P3 | completed | 收口验证与记录更新 | `rustfmt --edition 2024 --config skip_children=true src/backend/ssh.rs && cargo check && python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` | 格式化、编译和 tracking docs 校验均已通过 |

## 已完成

- 读取项目环境与实施跟踪 skill 约束，确认本轮需要先做环境预检与 plan-first 跟踪
- 刷新 `docs/project-env-audit/` 与 `docs/project-implementation-tracker/` 到当前 contract
- 确认现状：ashell 使用 `russh`，不是系统 `ssh`；当前连接层直接使用 `client::Config::default()`，因此只带默认首选算法，不会对老服务器做兼容 fallback
- 确认 `russh` 实际内置了 `diffie-hellman-group14-sha1`、`diffie-hellman-group1-sha1`、`ecdh-sha2-nistp256`、`aes*-cbc`、`3des-cbc` 等老算法，但默认 `Preferred` 没把它们放进首选列表
- 修改 `src/backend/ssh.rs`：默认先用当前安全算法集发起握手；若命中 `RusshError::NoCommonAlgo`，自动重连并启用 legacy compatibility 算法列表
- 修改 `src/backend/ssh.rs`：legacy mode 追加 `ecdh-sha2-nistp*`、`diffie-hellman-group14-sha1`、`diffie-hellman-group1-sha1`、`aes*-cbc`、`3des-cbc` 以及 `ssh-dss`
- 修改 `src/backend/ssh.rs`：把协商失败的 `kind/ours/theirs` 解包为更具体的日志与最终错误文本，便于判断是 KEX、host key、cipher 还是 MAC 没交集
- 运行 `rustfmt`、`cargo check` 和 tracking docs 校验脚本，确认本轮修改可编译且跟踪文档满足 contract

## 验证

- 已完成：读取 `src/backend/ssh.rs`、`Cargo.toml` 以及 `russh` 源码中的默认 `Preferred`、可用 KEX / cipher / MAC 集合；确认本机 `cargo` 可用
- 已完成：`rustfmt --edition 2024 --config skip_children=true src/backend/ssh.rs` 通过；`cargo check` 通过；`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` 通过
- 未完成：未对真实老 SSH 服务器做联机验证；尚未确认目标服务器是否还需要更激进的 DSA / 认证兼容修补

## 风险与阻塞

- 若远端服务器只支持极旧且不安全的组合，兼容模式需要在“能连上”和“不过度放宽默认安全边界”之间做取舍
- 本轮可以增强 `NoCommonAlgo` 的可观测性，但无法在本地脱离目标服务器完全复现所有老旧 SSH 配置
- 若服务器依赖的是 `ssh-dss` host key 或更特殊的认证路径，可能还需要第二轮兼容补丁

## 下一步

- 用目标老 SSH 服务器实测；如果仍失败，直接根据新日志里的 `kind/ours/theirs` 继续补第二轮兼容

## 最后更新时间

- 2026-07-06 15:11 CST
