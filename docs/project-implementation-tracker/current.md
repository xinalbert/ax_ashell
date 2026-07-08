# 当前项目实施记录

## 当前目标

- 目标：记录 SSH 会话上次成功使用的连接兼容模式，并在后续连接时优先尝试该模式，同时保留完整 fallback 顺序
- 交付物：会话级 `last_successful_ssh_mode` 持久化字段、SSH/SFTP 共用兼容模式优先级尝试逻辑、成功后事件回写配置、格式化/编译/测试验证结果，以及同步的 tracking 记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/session/config.rs`，`src/backend/ssh.rs`，`src/sftp/auth.rs`，`src/session/mod.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：代理路径优先级缓存、认证方式自动切换、真实 SSH / SFTP 联机手工验证、UI 展示新增设置项、依赖升级

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮环境预检和实施计划刷新 | tracking docs validator | 已确认不联网、不使用多 agent |
| P2 | completed | 会话配置字段、SSH/SFTP 兼容模式优先级 helper 与连接成功回写事件 | `cargo check`，源码级 diff 检查 | 上次成功模式只调整优先级，失败仍 fallback |
| P3 | completed | 针对模式排序/配置兼容的单元测试或最小测试覆盖 | `cargo test ssh_connection_modes` | 已覆盖默认顺序和 legacy 历史优先顺序 |
| P4 | completed | 格式化、全仓编译/测试和跟踪文档校验收口 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | 真实服务器联机验证留作手工 |

## 已完成

- 已确认当前 SSH 终端连接会先试默认算法，算法协商失败后再试 legacy compatibility
- 已确认 SFTP 认证路径当前未复用 legacy fallback，适合和本轮优先级逻辑一并收敛
- 已确认 `Session` serde 配置可通过新增 `#[serde(default)]` 字段兼容旧配置
- 已新增会话级 `last_successful_ssh_mode`，认证成功后回写保存会话
- 已让 SSH 终端和 SFTP 共用同一套 default/legacy 优先顺序
- 已新增 `ssh_connection_modes_*` 单元测试覆盖排序语义

## 验证

- 已完成：源码热点只读评估
- 已完成：施工前环境预检刷新
- 已完成：`rustfmt --edition 2024 src/session/config.rs src/session/mod.rs src/backend/ssh.rs src/sftp/auth.rs src/sftp/mod.rs src/app/event_loop.rs src/terminal/mod.rs`
- 已完成：`cargo test ssh_connection_modes`，2 个定向测试通过
- 已完成：`cargo check`
- 已完成：`cargo test`，15 个测试全部通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：真实 SSH / SFTP 联机手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：历史模式不能成为唯一模式，否则服务端升级或配置变化会造成连接失败；本轮必须保留 fallback
- 风险二：SFTP 和 SSH 终端应使用同一套兼容模式顺序，否则终端加速但 SFTP 仍按旧路径失败或慢试
- 风险三：只有成功建立并认证后才应回写模式，避免把半连接成功或认证失败记录为可复用方法
- 风险四：真实 SSH / SFTP 联机行为仍需在目标服务器上手工确认

## 下一步

- 在目标服务器上手工确认首次 legacy 成功后下次优先 legacy

## 最后更新时间

- 2026-07-08 11:25 CST
