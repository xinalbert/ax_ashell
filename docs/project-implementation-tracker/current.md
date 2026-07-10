# 当前项目实施记录

## 当前目标

- 目标：让超大 SFTP 目录按需加载后续页面，同时保持总内存预算和 cursor 的完整回收。
- 交付物：持久目录 cursor、显式加载更多操作、页结果追加状态、EOF/上限/切换的 cursor 回收、回归测试和实施记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/sftp.rs`，`src/terminal.rs`，`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/lifecycle/event_loop.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：自动滚动触发加载、全目录排序、文件预览 128 KiB 上限、传输协议、terminal scrollback 容量、remote-edit watcher、X11 relay、release/tag、提交。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 分页 cursor、事件和 UI 路径定位 | 已读取 SFTP worker、事件/UI state、列表虚拟化和 `russh-sftp 2.3.0` | raw directory handle 可跨多次 `READDIR` 复用 |
| P2 | completed | 持久 cursor、页追加和显式加载更多 | `rustfmt --edition 2024`、`cargo check` | 每页 250 项，保留 2,000 项/2 MiB 总预算和 30 秒读取 timeout |
| P3 | completed | 回归验证和实施记录收口 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI 多页、EOF、目录切换和总上限验证仍待执行 |

## 已完成

- 已确认普通浏览和目录预览都经由高层 `SftpSession::read_dir()`；该 API 在依赖中连续请求至 EOF 并将全部条目收集到 `Vec` 后才返回。
- 已确认 UI 在 render 时还会克隆和排序保留条目，因此只限制 backend event queue 不能限制单个目录的峰值。
- 已确认 `RawSftpSession::opendir/readdir/close` 为公开 API，可按服务端批次采集、在预算耗尽时关闭目录句柄；不需要修改锁定依赖。
- 已有目录浏览最多保留 2,000 条或 2 MiB 名称/路径数据，目录预览最多保留 200 条及 128 KiB 内容预算。
- 已确认现有短生命周期 raw session 不能继续读取下一页；`RawSftpSession::opendir` 返回的 handle 可在同一会话上重复 `readdir`。
- 已选择显式“加载更多”而非滚动触发，避免滚动/渲染重入造成重复加载；全目录排序不在本轮范围，仍只对已加载结果排序。
- 已实现每页最多 250 项的持久 cursor；`SftpEntries` 事件携带追加、是否还有更多和是否达到总预算，UI 以虚拟列表追加结果并在页脚显示“加载更多”。
- 已在 EOF、预算耗尽、目录切换、worker 关闭、worker 空闲重建和读取失败时关闭 cursor；加载失败会复位 UI 分页状态。

## 验证

- 已完成：环境预检、项目地图、分页 cursor/API/UI 接线、`rustfmt --edition 2024`、`cargo check`、定向 SFTP 测试（7 项）、`cargo test --quiet`（76 项）、`git diff --check` 和 tracking docs validator。
- 未完成：GUI 手工验证多页目录、EOF、总上限、目录切换和 idle worker 重建。

## 风险与阻塞

- 风险一：总预算达到后仍会停止加载；必须维持截断提示，不能让用户误认为目录已到 EOF。
- 风险二：raw SFTP 的单个服务端 `NAME` 响应仍须先解码；分页限制跨批次累积和 UI 保留，不替代协议级的单包字节限制。
- 风险三：持久 cursor 占用一个临时 SFTP channel；必须在 EOF、预算耗尽、导航、错误和 worker 关闭时释放。
- 无阻塞：GUI 交互行为需在桌面端人工确认。

## 下一步

- 在 GUI 中连接至少 251 项的 SFTP 目录，重复点击“加载更多”，验证 EOF 自动隐藏按钮、总上限提示，以及切换目录或空闲回收后的 cursor 回收。

## 最后更新时间

- 2026-07-10 17:41 +0800
