# 当前项目实施记录

## 当前目标

- 目标：为 SFTP 本地和远端文件列表接入各平台系统文件类型图标，并将缓存独立持久化到应用配置目录，避免系统查询进入虚拟列表、hover 或断联后的渲染路径。
- 交付物：`file-icons.json` 启动加载与原子更新、统一的内存渲染缓存、macOS Finder / Windows Explorer / Linux Freedesktop 图标解析、SFTP 双列表接线、回退图标、测试和验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`，`Cargo.lock`，`src/config/store.rs`，`src/platform.rs`，`src/platform/file_icons.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views.rs`，`src/app/views/sftp_panel.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：SFTP 网络协议/worker、远端内容缩略图、远端自定义文件图标、文件预览、文件关联设置和全局传输历史弹窗。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 平台能力、渲染边界、依赖和缓存方案 | 源码与 crate API 审查 | 远端和本地均按文件名/目录类型命中本机系统关联图标 |
| P2 | completed | 独立持久化图标缓存、平台后端与 SFTP 虚拟列表接线 | 聚焦测试、`cargo check`、代码审查 | 启动时读取 `file-icons.json`；缺失、损坏或平台/主题失效时预热并原子更新；渲染闭包只读缓存 |
| P3 | completed | 自动化验证与跟踪文档收口 | `cargo test --quiet`、静态 hover 审计、`git diff --check`、validator | Linux 主题和实际 GUI 图标仍需手工验收；Windows/macOS target 未安装 |

## 已完成

- 已完成环境预检、项目地图审查、SFTP FastHover 规则审查和平台可行性核实。
- 已确认 GPUI 可渲染缓存位图但未提供系统文件图标 API，因此以项目平台层统一封装；`uniform_list` 行渲染不得触发文件系统、原生 Shell 或图标主题查询。
- 已确认 macOS 使用 `NSWorkspace`、Windows 使用 `SHGetFileInfoW`，Linux 使用 Freedesktop 当前图标主题；远端和本地条目均以目录标记或扩展名取得本机关联图标，无法取得远端内容缩略图或自定义文件夹图标。
- 已完成对标检索：KDE 对慢速/远端 URL 使用扩展名 MIME 推断，Windows Shell 支持虚拟扩展名与 `SHGFI_USEFILEATTRIBUTES` 并要求后台调用。因此缓存键固定为目录、通用文件和有限扩展名，而非远端路径或本地路径。
- 已实现 `ConfigStore::file_icons_path()`，在配置根目录维护独立 `file-icons.json`。启动时同步读取兼容缓存，缺失、损坏、平台变更或 Linux 主题变更时预热并原子写入；图像数据不进入 `sessions.json` 或同步载荷。
- 已实现 macOS AppKit 分批预热、Windows Shell 后台预热（含线程 COM 初始化）和 Linux Freedesktop 主题预热。SFTP 的两个 `uniform_list` 行渲染只做内存类型键查询，缓存完成后一次刷新界面；断开 SFTP 后不受影响。

## 验证

- 已完成：`rustfmt --edition 2024`；`cargo check`；`cargo test --quiet file_icon`（5 项）；完整 `cargo test --quiet`（183 项）；SFTP hover / `uniform_list` 静态审计；`git diff --check`；tracking validator。
- 未完成：真实 macOS、Windows、Linux GUI 的系统图标、主题切换、缩放和回退视觉验收。`cargo check --target x86_64-pc-windows-msvc` / `aarch64-apple-darwin` 因本机未安装目标标准库且无 `rustup` 失败，未视为代码失败。

## 风险与阻塞

- 风险：远端文件只会显示当前客户端系统对目录或扩展名的关联图标，不能获取远端专属图标、内容缩略图或本地自定义文件夹图标。
- 风险：Linux 主题图标可用性取决于系统主题及其 PNG/SVG 资源；缺失时必须稳定回退。
- 无阻塞；跨目标编译仅受本机工具链限制。

## 下一步

- 在三端真实 GUI 中验证首启预热、二次启动缓存命中、Linux 图标主题切换和未知扩展名的回退图标。

## 最后更新时间

- 2026-07-14 17:03 +0800
