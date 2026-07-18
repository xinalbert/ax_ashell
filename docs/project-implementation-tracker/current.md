# 当前项目实施记录

## 当前目标

- 目标：按安全审计风险顺序修复 AxShell 的 SSH/SFTP 身份验证、弱算法降级、同步网络边界和依赖漏洞治理，并修复主机密钥确认框被连接进度遮罩拦截的交互回归。
- 交付物：可确认且可点击的服务器主机密钥信任链、禁用自动 legacy 降级的会话策略、仅 HTTPS 的同步传输与有界响应读取、更新的锁文件/CI 审计，以及定向测试和双语行为文档。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/session.rs`、`src/config/`、`src/backend/ssh.rs`、`src/backend/ssh/connection.rs`、`src/backend/ssh/legacy.rs`、`src/sftp/auth.rs`、`src/sftp/worker.rs`、`src/sftp/worker/runtime.rs`、`src/sync.rs`、`src/events.rs`、`src/app/`、`locales/`、`docs/`、`.github/workflows/ci.yml`、`Cargo.toml`、`Cargo.lock`。
- 不在本轮范围内：Telnet 的明文本质、串口安全模型、远端命令执行权限、X11 功能移除、全面密钥库迁移或 GPUI 上游架构重写。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | SSH 与 SFTP 共享的主机密钥确认、持久化与失配拒绝机制 | 主机信任存储单元测试；`cargo check`；真实 SSH/SFTP 首连、重连和密钥变更手工验收 | 未知或变更密钥默认拒绝；首次连接需确认 |
| P2 | completed | 将 legacy SSH 算法改为会话级显式 opt-in，默认只用安全算法 | 会话策略单元测试；`cargo check`；`cargo test --quiet` | 不自动回退，也不再根据连接结果写回算法模式 |
| P3 | completed | 同步 endpoint 强制 HTTPS，并限制 WebDAV/S3 响应大小 | URL/响应限额单元测试；`cargo check` | HTTP 在保存和发起请求前均被拒绝 |
| P4 | completed | 更新可修复的 RustSec 依赖并在 CI 执行锁文件审计 | `cargo audit`；`cargo check`；CI YAML 审阅 | 3 项可修复公告已消除；3 条无可用兼容修复的公告在 CI 中明确暂缓 |
| P5 | completed | 更新双语安全行为文档、环境/实施记录并完成收口验证 | tracking validator；`git diff --check`；完整 `cargo test --quiet` | 自动化收口完成，保留实机验收清单 |
| P6 | completed | 修复主机密钥确认框与连接进度遮罩的层级和点击冲突 | `rustfmt`；`cargo check`；`cargo test --quiet`；真实首次连接确认点击 | 主机密钥确认是唯一可交互模态；等待确认时不显示连接进度遮罩 |

## 已完成

- 已完成源码与 RustSec 审计：确认 SSH/SFTP callback 无条件接受服务器密钥、默认自动尝试 SHA-1/DSA/CBC/3DES legacy 模式、同步允许 HTTP 且无响应大小限制；SFTP 本地路径穿越和预览/浏览限额已有保护。
- 已完成本轮环境预检、项目地图刷新和联网研究记录；基线 `cargo test --quiet` 为 225 passed。
- P1 已完成：SSH 与 SFTP 在握手时使用同一份本地主机密钥信任记录；首次发现和密钥失配都需要用户比对 SHA-256 指纹并明确确认，超时或拒绝时连接失败。
- P2 已完成：legacy 算法仅可在单个 SSH 会话的高级选项中明确开启；终端和 SFTP 均只使用所选算法集，历史自动降级字段和连接成功后的模式回写已移除。
- P3 已完成：同步仅接受 HTTPS endpoint，WebDAV/S3 成功和错误响应均以流式读取限制在 8 MiB；无效 endpoint 不会写入本地配置。
- P4 已完成：`crossbeam-epoch`、`quinn-proto` 和 `memmap2` 已升级到修复版本，CI 新增 RustSec 审计。其余 `rsa` 和 `quick-xml` 公告受无上游补丁或当前上游版本约束影响，已在 CI 命令中以公告编号及理由显式暂缓。
- P6 已完成：对话框层移到所有应用内遮罩之后；存在活动对话框时不渲染连接进度遮罩，因此主机密钥确认成为唯一可见、可点击的模态交互。

## 验证

- 已完成：安全代码审阅、RustSec 官方公告数据库审计、依赖链初步定位、基线 `cargo test --quiet`（225 passed）；P1 的 `cargo test --quiet host_key`（6 passed）、P2 的 `cargo test --quiet legacy_ssh`（1 passed）、P3 的 `cargo test --quiet sync`（7 passed）；各步骤的 `cargo check`；RustSec 缓存数据库下的 CI 等效命令与 CI YAML 解析；完整 `cargo test --quiet`（232 passed）。
- 未完成：主机密钥确认点击的实机验收、CI 实跑，以及 macOS/Windows/Linux 的真实 SSH/SFTP/同步服务验收。

## 风险与阻塞

- 主机密钥首次信任需要 UI 明确确认，不能静默 TOFU；SSH 与 SFTP 必须使用同一份持久化信任记录。真实服务上的首连、重连、失配和关闭确认仍待手工验收。
- `rsa` Marvin 公告没有可用上游补丁，需通过关闭弱降级、主机认证和依赖上游跟进降低暴露面。
- 依赖升级可能牵动 GPUI Git 依赖，必须与认证行为修复分离验证。
- RustSec 仍报告 `rsa` 和 `quick-xml` 的已知公告，但 CI 只暂缓无可用兼容修复的三个公告 ID；任何新的漏洞公告仍会使 CI 失败。
- 主机密钥确认必须保留明确的“拒绝/信任”选择；问题在于两个模态层同时存在而非确认本身多余。

## 下一步

- 在真实 SSH/SFTP 首次连接时确认只显示主机密钥确认框，且“拒绝”和“信任密钥”都可点击；随后在三平台确认重连、密钥变化、关闭确认和显式 legacy 连接；使用 HTTPS WebDAV/S3 及 HTTP/超限响应服务完成同步边界验收，并观察 GitHub CI 首次审计运行。

## 最后更新时间

- 2026-07-18 12:44 +0800
