# 当前项目实施记录

## 当前目标

- 目标：分阶段改善高 RTT SSH 会话的输入体验；本阶段先建立不含按键内容的输入反馈延迟基线。
- 交付物：每个 SSH tab 的匿名输入到首个远端输出计时与聚合状态、超时回收、单元测试和诊断记录；后续阶段才加入提示符感知的本地输入 overlay。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/actions/terminal.rs`、`src/terminal/tab.rs`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：改变 SSH 协议或引入 Mosh 服务端、直接修改已确认终端 buffer、默认开启本地回显、Telnet/串口输入优化、保存或记录按键内容。

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
| P7 | completed | 建立 SSH 输入到远端输出的匿名反馈延迟基线 | `TerminalTab` 单元测试；`cargo check`；`cargo test --quiet` | 仅记录时间与聚合值；不记录按键内容，不改变 backend 写入顺序 |

## 已完成

- 已完成源码与 RustSec 审计：确认 SSH/SFTP callback 无条件接受服务器密钥、默认自动尝试 SHA-1/DSA/CBC/3DES legacy 模式、同步允许 HTTP 且无响应大小限制；SFTP 本地路径穿越和预览/浏览限额已有保护。
- 已完成本轮环境预检、项目地图刷新和联网研究记录；基线 `cargo test --quiet` 为 225 passed。
- P1 已完成：SSH 与 SFTP 在握手时使用同一份本地主机密钥信任记录；首次发现和密钥失配都需要用户比对 SHA-256 指纹并明确确认，超时或拒绝时连接失败。
- P2 已完成：legacy 算法仅可在单个 SSH 会话的高级选项中明确开启；终端和 SFTP 均只使用所选算法集，历史自动降级字段和连接成功后的模式回写已移除。
- P3 已完成：同步仅接受 HTTPS endpoint，WebDAV/S3 成功和错误响应均以流式读取限制在 8 MiB；无效 endpoint 不会写入本地配置。
- P4 已完成：`crossbeam-epoch`、`quinn-proto` 和 `memmap2` 已升级到修复版本，CI 新增 RustSec 审计。其余 `rsa` 和 `quick-xml` 公告受无上游补丁或当前上游版本约束影响，已在 CI 命令中以公告编号及理由显式暂缓。
- P6 已完成：对话框层移到所有应用内遮罩之后；存在活动对话框时不渲染连接进度遮罩，因此主机密钥确认成为唯一可见、可点击的模态交互。
- P7 已完成方案研究：`xiaoxingshell` 使用提示符感知的本地行缓冲和远端回显去重；Mosh 使用有 ACK/过期语义的预测 overlay。两者均表明预测层必须与确认终端状态分离，本项目先测量现有 SSH 输入反馈再修改交互语义。
- P7 已完成实现：SSH 键盘和 IME 输入会启动匿名反馈样本，首个后续输出更新最近值与平均值；连续输入合并为一个样本，30 秒无反馈的样本被丢弃，重连时清空待确认状态。日志不含按键或远端输出内容。

## 验证

- 已完成：安全代码审阅、RustSec 官方公告数据库审计、依赖链初步定位、基线 `cargo test --quiet`（225 passed）；P1 的 `cargo test --quiet host_key`（6 passed）、P2 的 `cargo test --quiet legacy_ssh`（1 passed）、P3 的 `cargo test --quiet sync`（7 passed）；P7 的 `cargo test --quiet input_feedback`（3 passed）；各步骤的 `cargo check`；RustSec 缓存数据库下的 CI 等效命令与 CI YAML 解析；完整 `cargo test --quiet`（235 passed）。
- 未完成：100/250/500 ms RTT SSH 服务上的 P7 手工采样；主机密钥确认点击的实机验收、CI 实跑，以及 macOS/Windows/Linux 的真实 SSH/SFTP/同步服务验收。

## 风险与阻塞

- 主机密钥首次信任需要 UI 明确确认，不能静默 TOFU；SSH 与 SFTP 必须使用同一份持久化信任记录。真实服务上的首连、重连、失配和关闭确认仍待手工验收。
- `rsa` Marvin 公告没有可用上游补丁，需通过关闭弱降级、主机认证和依赖上游跟进降低暴露面。
- 依赖升级可能牵动 GPUI Git 依赖，必须与认证行为修复分离验证。
- RustSec 仍报告 `rsa` 和 `quick-xml` 的已知公告，但 CI 只暂缓无可用兼容修复的三个公告 ID；任何新的漏洞公告仍会使 CI 失败。
- 主机密钥确认必须保留明确的“拒绝/信任”选择；问题在于两个模态层同时存在而非确认本身多余。
- 远端输出不保证是对输入的逐字回显；P7 仅测量从本地输入到首个后续远端输出的反馈时间，不能把它作为严格网络 RTT。对无输出或全屏应用必须超时清理，不能阻塞输入。

## 下一步

- 在 100/250/500 ms RTT 的 SSH 链路上采集匿名反馈延迟，确认超时、无回显和快速连续输入不会积压状态；随后开始 P8，实现默认关闭、仅普通 shell 提示符可用的本地输入 overlay；并继续完成既有 SSH/SFTP、同步和 CI 实机验收。

## 最后更新时间

- 2026-07-18 14:44 +0800
