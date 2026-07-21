# 当前项目实施记录

## 当前目标

- 目标：修复移除 Settings 关闭确认后导致五平台 release CI 失败的编译回归。
- 交付物：恢复语言下拉所需的 `SettingField` import，并完成本地验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/dialogs/settings/general.rs`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：Settings 关闭行为、未保存表单的 dirty-state 检测、SFTP 传输关闭确认、连接表单关闭语义、其他窗口关闭路径或发布版本。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
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
| P8 | completed | 默认关闭的 SSH 会话级本地输入 overlay | 定向单元测试；`rustfmt`；`cargo check`；`cargo test --quiet`；`git diff --check` | 仅主屏、底部、已连接 SSH；不支持的输入先按顺序 flush 再直通 |
| P9 | completed | 本地输入 overlay 不失效终端行布局缓存 | `TerminalElement` 定向测试；`rustfmt`；`cargo check`；`cargo test --quiet`；`git diff --check` | composition 独立绘制；只有实际影响 row shape 的状态可失效 cache |
| P10 | completed | 同步用户新增的文档截图并清理旧图片目录入口 | 图片引用存在性、双语结构审阅、`git diff --check`、tracking docs validator | 图片不改像素；英文页面与中文页面使用同一相对路径 |
| P11 | completed | 小窗口中的新建连接表单可滚动 | `rustfmt`；`cargo check`；`cargo test --quiet`；小窗口 GUI 手工验收 | 仅限制高度和滚动表单；保存/连接操作保持原位置与语义 |
| P12 | completed | 修正受限 Dialog 高度下的表单滚动约束 | `rustfmt`；`cargo check`；`cargo test --quiet`；小窗口 GUI 截图验收 | 显式高度让滚动区获得稳定剩余空间；不改连接语义 |
| P13 | completed | 让新建连接页标题与表单整体滚动 | `rustfmt`；`cargo check`；`cargo test --quiet`；小窗口 GUI 截图验收 | 标题进入 scroll body；右上角关闭按钮保持固定 |
| P14 | completed | 修正连接页滚动容器所在的 Dialog 渲染分支 | `rustfmt`；`cargo check`；`cargo test --quiet`；小窗口 GUI 截图验收 | 使用 Dialog 内置 child scroll body，不改连接语义 |
| P15 | completed | 修复新建连接页的 GPUI 双重借用崩溃并建立安全滚动区 | `rustfmt`；`cargo check`；`cargo test --quiet`；小窗口 GUI 截图验收 | content 延迟构建，显式 scroll handle 与 flex 约束 |
| P16 | completed | 将 detached workspace 的 AppKit 窗口关闭投递到下一轮 macOS 运行循环 | `rustfmt`；`cargo check`；`cargo test --quiet`；返回主窗口 GUI 验收 | 保留 Metal drawable 清理，但不得在 GPUI `App::update` 借用期间同步关闭 |
| P17 | completed | 移除 Settings 页无条件关闭确认及其失效配置表面 | `rustfmt`；`cargo check`；`cargo test --quiet`；Settings 快捷键/标签关闭按钮 GUI 验收 | SFTP 传输关闭确认保持不变 |
| P18 | completed | 修复 Settings 通用页缺失 `SettingField` import 的五平台编译回归 | `rustfmt`；`cargo check`；`cargo test --quiet`；CI 重跑 | 不改变 P17 的直接关闭行为 |

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
- P8 已完成：SSH 高级选项新增默认关闭的会话级开关。启用后，`LocalInputBuffer` 只在已连接 SSH 主屏、滚动到底部和可见 cursor 时预测单行 ASCII、退格和左右键；Enter 发送整行并等待首个远端输出清理。粘贴、IME、Tab/Ctrl/Alt、鼠标选择、滚动和工作区迁移会先 flush；未提交行遇异步输出也会先发送，避免输入丢失。预测层不写入 Alacritty 确认 buffer。
- P9 已完成定位：`TerminalElement::cached_grid_rows` 的 `GridLayoutKey` 包含 `TerminalComposition`，但 `layout_row` 不读取 composition。每次本地键入改变 overlay 文本都会拒绝所有可见行的 `GridLayoutCache`，触发完整行 shape；composition 实际只在独立 paint 阶段使用。
- P9 已完成上游核对：Zed 在 `prepaint` 中布局确认 terminal cell，并在 `paint` 中单独 shape / 覆盖 IME marked text；其分层与本地 cache-key 收窄一致，来源记录于 `docs/project-implementation-tracker/research.md`。
- P9 已完成实现：`GridLayoutKey` 只保留 `layout_row` 实际消费的 style 和 selection；本地输入或 IME composition 文本继续在 paint 阶段独立重绘，不再使已确认可见行重新 shape。回归测试明确限定 cache key 只跟踪会改变 shaped row 的状态。
- P10 已完成预检：用户已将根 README 图片放入 `images/`、功能截图放入 `docs/features/images/`，并删除旧 `docs/images/` 说明；英文页面和文档导航仍需同步。
- P10 已完成：英文 README 与九个功能页在和中文相同的语义位置引用同一图片，双语图片均使用可读替代文字；旧 `docs/images/` 导航、所有失效截图占位注释和遗留 `preview.png` 引用均已清理。
- P11 已完成定位：`show_ssh_dialog` 只设置 560px 宽度，表单与底部操作处于非滚动 content 中；高级 SSH 选项展开后可超过视口而被裁切。项目依赖的 Dialog 支持受限高度与可滚动容器，可在本地对话框层修复。
- P11 已完成：新建/编辑连接对话框的宽度限制为 560px 或当前视口减去边距中的较小值，高度按当前视口限制；整个连接表单由独立垂直滚动区承载，因此长高级选项和底部操作不再被裁切。连接字段、焦点事件和保存/连接处理未改。
- P12 已完成定位：实际小窗口截图表明 P11 的 `max_h` 仅裁切外层，未向 content 的 flex 布局提供确定高度，滚动区无法稳定占用剩余空间。需要改为显式视口高度，保留 10% 上下边距。
- P12 已完成：连接对话框高度为当前视口的 80%，最高 800px；结合默认 10% 顶部偏移保留上下边距。表单滚动容器因此获得稳定高度，并在内容超过可用空间时显示垂直滚动条。
- P13 已完成定位：P12 的滚动容器已覆盖连接类型、认证和末尾操作，但 `Dialog.title` 仍在滚动区外。用户要求整页滚动，需将同样样式的标题移入 form 的首行，保留 Dialog 右上角关闭按钮。
- P13 已完成：固定 `Dialog.title` 已移除，`new_connection` 使用同样 base/semibold 样式成为 form 首个元素；标题、连接类型、认证、高级选项和底部操作现在共享一个连续滚动内容，右上角关闭按钮保持固定。
- P14 已完成定位：实际截图确认 P13 的标题位置正确，但 `Dialog.content(...)` 分支没有使用 Dialog 为 `.child(...)` 提供的内置 scroll body。嵌套滚动层仍按表单原始高度布局，因此被裁切而不能滚动。
- P14 已完成：连接 form 直接作为 Dialog child 渲染，固定高度下由组件的内置 `overflow_y_scrollbar()` body 管理；表单焦点、键盘事件、取消、保存和保存并连接 callback 保持不变。
- P15 已完成定位：崩溃报告显示 `.child(...)` 的 form 在 `show_ssh_dialog` 对 `AxShell` 的 update 中同步执行，并调用 `view.read(cx)`；GPUI 禁止同一 Entity 在 update 中再次 read，因此触发 `cannot read AxShell while it is already being updated`。`Dialog.content(...)` 的 builder 延迟到正常渲染路径，可避免该重入借用。
- P15 已完成：恢复 `Dialog.content(...)` 的延迟 builder，在其内部以稳定 ID 的 `ScrollHandle`、`flex_1`、`min_h_0` 和 `overflow_y_scroll()` 形成明确高度的滚动区；标题、连接类型、认证、全部高级项和末尾操作仍由同一 form 承载。
- P16 已完成定位：`a75e4cb` 在 detached workspace 返回时用 `window.defer` 调用 AppKit `performClose:`。GPUI 的 defer 在当前 `App::update` effect cycle 内执行，`performClose:` 同步进入 GPUI `on_close` 并尝试再次借用 `App`，导致 `RefCell already borrowed`；panic 穿过 Objective-C 回调后升级为无法 unwind 的进程终止。
- P16 已完成：`performClose:` 改为零延迟的 AppKit selector，进入下一轮主运行循环；移除包裹它的 GPUI `window.defer`。因此 GPUI 会先返回外层 `App::update` 并释放 `RefCell` 借用，随后关闭 callback 才能安全调用 `AsyncApp::update`；AppKit handle 不可用时仍回退到既有 `remove_window()`。
- P17 已完成定位：Settings 关闭只隐藏 Settings tab 并回到 terminal；绝大多数设置即时保存，而保留 Save 按钮的表单也没有被确认框检测。无条件确认不能防止特定数据丢失，反而每次关闭增加交互步骤。SFTP 提示不同，仅在运行或暂停传输时出现，保留后台继续和取消断连两种后果不同的行为，故不在本轮修改。
- P17 已完成：Settings 快捷键与标签关闭按钮直接调用 `close_settings_page`；删除 Settings 关闭确认 dialog、第二次快捷键动作配置、初始化状态、测试和双语文案。保留 Save 按钮的表单仍须显式保存，旧 JSON 中已废弃的确认字段按 serde 默认规则忽略并在后续保存时移除。
- P18 已完成定位：P17 删除 `settings_behavior` group 时误删 `SettingField` import，但语言下拉仍使用 `SettingField::render`。GitHub CI 的 macOS arm64/x86_64、Linux x86_64/aarch64 和 Windows x86_64 都在 release build 因同一 `E0433` 失败；RustSec audit 通过。本地 `cargo check` 已复现。
- P18 已完成：恢复 `SettingField` import，语言下拉恢复编译；Settings 关闭路径和 P17 删除的确认配置没有变化。

## 验证

- 已完成：安全代码审阅、RustSec 官方公告数据库审计、依赖链初步定位、基线 `cargo test --quiet`（225 passed）；P1 的 `cargo test --quiet host_key`（6 passed）、P2 的 `cargo test --quiet legacy_ssh`（1 passed）、P3 的 `cargo test --quiet sync`（7 passed）；P7 的 `cargo test --quiet input_feedback`（3 passed）；P8 的 `cargo test --quiet local_input`（3 passed）、`cargo test --quiet session::tests::new_session_fields_default_when_loading_existing_sessions`（1 passed）、`cargo test --quiet local_input_overlay_requires_opt_in_and_primary_screen`（1 passed）；各步骤的 `cargo check`；P8/P9/P11/P14/P15/P16 完整 `cargo test --quiet`（238 passed）与 `rustfmt`；P17/P18 完整 `cargo test --quiet`（236 passed）与 `rustfmt`；P8/P9/P11/P15/P16/P17/P18 的 `git diff --check` 和 tracking docs validator；P9 的 `cargo test --quiet grid_layout_key`（1 passed）；P10 的图片存在性、双语路径配对和旧目录引用审阅。
- 未完成：P18 的 CI 重跑；P17 的 Settings 快捷键和标签关闭按钮 GUI 验收；P16 的 macOS detached workspace 返回主窗口验收；P15 的真实小窗口 GUI 截图验收；100/250/500 ms RTT SSH 服务上的 P7/P8/P9 手工采样与交互验收；主机密钥确认点击的实机验收、CI 实跑，以及 macOS/Windows/Linux 的真实 SSH/SFTP/同步服务验收。

## 风险与阻塞

- 主机密钥首次信任需要 UI 明确确认，不能静默 TOFU；SSH 与 SFTP 必须使用同一份持久化信任记录。真实服务上的首连、重连、失配和关闭确认仍待手工验收。
- `rsa` Marvin 公告没有可用上游补丁，需通过关闭弱降级、主机认证和依赖上游跟进降低暴露面。
- 依赖升级可能牵动 GPUI Git 依赖，必须与认证行为修复分离验证。
- RustSec 仍报告 `rsa` 和 `quick-xml` 的已知公告，但 CI 只暂缓无可用兼容修复的三个公告 ID；任何新的漏洞公告仍会使 CI 失败。
- 主机密钥确认必须保留明确的“拒绝/信任”选择；问题在于两个模态层同时存在而非确认本身多余。
- 远端输出不保证是对输入的逐字回显；P7 仅测量从本地输入到首个后续远端输出的反馈时间，不能把它作为严格网络 RTT。对无输出或全屏应用必须超时清理，不能阻塞输入。
- P8 不自动识别 shell prompt，因此用户只能在普通、单行 shell 提示符处启用；密码提示、REPL、文本编辑器和全屏程序必须依赖回退路径，不能承诺预测显示正确。
- P8 仍不能把首个远端输出当作逐字回显确认；它只把该输出作为显示层失效信号。用户输入内容不会进入日志或 metrics，但会在启用模式下短暂保留于进程内内存，直至提交、flush 或清理。
- P9 保留 selection、字体、terminal snapshot、highlight 和颜色变化的布局失效；只移除了未被 `layout_row` 消费的 composition 依赖。GUI frame-time 改善仍须通过真实长 scrollback 和高频输入验收。
- P10 已确认中英文页面只引用已有图片，且无用户可见的旧 `docs/images/` 导航或空白占位；未被页面引用的 `docs/features/images/image.png` 按用户工作区内容保留，待其指定用途。
- P11 已将视口高度和宽度约束应用在本地对话框层，滚动只包围表单；input focus、键盘 Enter、取消、保存和保存并连接操作保持原实现。真实窗口最小高度仍需手工确认。
- P12 让 Dialog 的实际高度与 content 的 flex 剩余空间一致；窗口高度极小时表单以滚动代替裁切。真实 GUI 仍须确认滚动条、焦点和末尾操作可见。
- P13 保持关闭按钮在滚动区外，避免用户在长表单底部时失去关闭出口；标题和表单内容连续滚动。真实 GUI 仍须确认视觉层级和滚动范围。
- P14 的 child 构建方案已被 P15 替代，不再用于连接页；需要确认鼠标滚轮、触控板、焦点、取消、保存和保存并连接仍可用。
- P15 必须保留 `Dialog.content(...)` 的延迟构建边界，不能在 `show_ssh_dialog` 的 `AxShell` update 内同步 `view.read(cx)`；crash hook 仅用于诊断，不能作为继续运行的兜底。
- P16 不能在 GPUI `App::update` 或 `window.defer` effect 中同步调用 `performClose:`；关闭仍需在 macOS 主运行循环完成，确保 GPUI 先释放其 `App` 借用。
- P17 直接关闭 Settings 不会保存带 Save 按钮表单中未提交的 input；本轮按用户请求移除无条件确认，不实现未保存修改检测。SFTP 活跃传输关闭确认不可复用为 Settings 关闭逻辑。

## 下一步

- 推送 P18 修复并确认 CI 五个平台 release build 通过；随后继续 P17、P16、P15 与 P7/P8/P9 的 GUI 验收。

## 最后更新时间

- 2026-07-20 12:15 +0800
