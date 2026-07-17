# 当前项目实施记录

## 当前目标

- 目标：核对更新后的 macOS memory sample，并让未使用的内置资源按需进入字体系统，优先消除所有内置字体的启动期注册。
- 交付物：sample 构成结论；跨平台 family 级延迟字体注册；设置页选择时加载；测试、跟踪记录和可审阅提交。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：更新后的 `vmmap` sample、`src/app/theme.rs`、`src/main.rs`、字体设置 action / 菜单、`docs/project-implementation-tracker/`、`worker.md`。
- 不在本轮范围内：修改或 vendoring GPUI/Zed renderer、更新 `Cargo.toml` / `Cargo.lock`、重做原生窗口 surface、改变终端/SFTP 协议，或在未完成三平台打包核对前把字体移出二进制。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：已结束

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 更新 sample 的内存分类与可按需资源清单 | `vmmap` sample、源代码路径审阅 | 将 IOSurface、字体注册、图标缓存和本地目录预热分开处理 |
| P2 | completed | family 级内置字体延迟注册与设置选择接线 | 聚焦测试、`cargo check`、完整测试 | 系统 UI 不注册内置字节；终端在首个可见渲染时按 family 注册 |
| P3 | completed | Main 的本地图标缓存、目录枚举和 detached transfer clone 延后 | 聚焦测试、`cargo check`、完整测试 | SFTP 打开时首次初始化；所有 SFTP 页面路由均覆盖，行渲染不执行 native lookup |
| P4 | completed | 三平台外部字体资源包决策与整体验证 | `rustfmt`、`cargo test --quiet`、静态菜单审计 | 外部字体包是可行的后续发行工程，不和本次低风险改动混合 |
| P5 | completed | tracking 校验、干净差异与窄提交 | validator、`git diff --check`、staged diff | 代码提交 `7999d30` 已创建；新构建 / 新进程 sample 是手工验收边界 |

## 已完成

- 更新 sample 的 physical footprint 为约 724.4 MiB，其中 `IOSurface` 约 395 MiB、`MALLOC` 分配约 198 MiB；主要增长仍需以 drawable 生命周期为首要线索。
- 32 个 `CAMetalLayer Display Drawable` 完全解释当前 IOSurface：29 × 2000×1400 和 3 × 2504×2710；29 个默认 detached 尺寸 drawable 至少对应约 10 组 layer 容量，必须在新进程中用开关窗口实验确认回收行为。
- 已实现字体按需注册：配置的内置 UI family 才在初始化时注册；终端 family 延迟到 `TerminalElement::prepaint` 的首个可见渲染；用户选择前不写入配置，菜单仅使用静态名称且不探测未加载内置字体。
- 发现主窗口不进 SFTP 也会同步读 28.73 MiB file-icons JSON，解码后常驻约 21.54 MiB PNG bytes，并预读/枚举本地目录；现已推迟到首次 SFTP 页面。
- 当前 `sessions.json` 的 transfer history 约 7.24 MiB，detached 初始化会产生一次即将被 workspace transfer 覆盖的 clone；本轮仅消除这次可避免 clone，持久化 schema 拆分留给后续。
- 已实现 Main 的 file-icon cache 与本地目录按 SFTP 首次进入加载；缓存仅初始化一次并沿用既有 refresh，关闭最后 terminal 自动切入 SFTP 的特殊路由也会恢复本地资源。
- 已实现 detached workspace 的 transfer history 从空集合开始，`install_workspace_transfer` 继续接收被移动的真实 transfer；不改 `ConfigStore` 的持久化 schema。
- 三个低重叠 worker 已完成交付；本地集成复核确认独立窗口的实际监控可见性路径不会重新创建 sampler。

## 验证

- 已完成：项目环境 quick scan、实施记录 / 项目地图审阅、更新 sample 审计、字体与 SFTP 延后实现、`rustfmt`、最终 `cargo check`、225 项完整测试、`git diff --check`、tracking validator、fast hover 静态审计、三平台外部字体包边界审阅、只读代码审阅和代码提交 `7999d30`。
- 未完成：新进程的 macOS memory 对比；Windows / Linux GUI 手工验收。

## 风险与阻塞

- macOS `IOSurface`、Windows swapchain、Linux WGPU surface 是每个可见原生窗口的必要基线；本轮字体优化不会消除该部分。
- GPUI 公开 API 没有字体卸载操作；已注册的 family 会保留到进程退出，因此“按需”应指首次使用时注册，不能承诺切换回系统字体后释放。
- `include_bytes!` 的未访问页通常不会进入字体解析 heap，但仍处于 executable 映像；将可选字体彻底移出映像需要同步三平台打包，先由独立审阅确认边界。
- FileIconCache 的同步解析从启动移到首次 SFTP 会把启动成本移到第一次打开页面；当前优先保证“不进入 SFTP 不占用”，更平滑的后台解析/placeholder 刷新可作为后续 UX 改进。

## 下一步

- 以全新构建 / 全新进程按“默认配置 → 首次打开 SFTP → 选择一项可选字体 → 重新选择”记录 macOS `vmmap -summary`，并按三平台清单确认 fallback 图标、目录、字体 regular/bold/italic 和终端等宽渲染无回退。

## 最后更新时间

- 2026-07-17 11:30 +0800
