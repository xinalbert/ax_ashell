# 当前项目实施记录

## 当前目标

- 目标：补齐收起侧边栏的 SAVED 分组视图，让折叠态也先显示组并支持点击展开组内 SSH
- 交付物：收起态分组渲染、折叠栏组展开交互、必要的文档修订与跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/ui.rs`，`src/session/mod.rs`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：会话模型字段、同步协议的加密/传输机制、SFTP 面板、会话选择器分组化、跨设备历史迁移工具和 GUI 手工截图验证

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 读取现有 tracking docs、环境记录和折叠侧栏实现，确认本轮修订边界 | `docs/` contract 自检，源码走查 | 已确认问题只在 `render_collapsed_sidebar` |
| P2 | completed | 让折叠侧栏改为先显示组并复用现有 `expanded_saved_groups` 展开组内会话 | `cargo check` | 已消除“展开态分组、折叠态平铺”的双轨行为 |
| P3 | completed | 同步更新用户文档、格式化、编译和 tracking docs 校验 | `rustfmt`，`cargo check`，`cargo test`，tracking docs 校验 | GUI 手工验证仍不在本轮自动执行 |

## 已完成

- 复查 `src/session/config.rs`、`src/session/mod.rs`、`src/app/dialogs.rs`、`src/app/ui.rs`、`src/app/config_sync.rs` 和 `src/sync/mod.rs`，确认当前 saved session 仍是平铺 `sessions`，同步 payload 也直接复用 `Session`
- 收敛实现路径为“给 `Session` 直接增加 `group_name` 字段”，避免另建顶层 `groups[]` 与额外同步替换逻辑
- 确认当前项目已有可复用的 dropdown menu 模式，适合给 SSH 表单加载已有组名
- 已完成上一轮 SAVED 展开态分组、SSH 表单组选项和组重命名能力
- 已确认本轮新增诉求只针对折叠侧栏 `render_collapsed_sidebar`，无需再改 `Session` 模型和同步载荷
- 已修改 `src/app/ui.rs`，让折叠侧栏也先显示组卡片，并复用现有 `expanded_saved_groups` 点击展开组内 SSH
- 已为折叠态组卡片补上文件夹图标、组名缩写和 tooltip；组内会话继续保留点击连接、tooltip 和右键菜单
- 已更新 `docs/user-guide.md`、`docs/user-guide.en.md` 与 `docs/project-implementation-tracker/project-map.md`，补充折叠态分组说明

## 验证

- 已完成：tracking docs 与环境记录复查；折叠侧栏和分组 helper 源码走查；`rustfmt --edition 2024 --config skip_children=true src/app/ui.rs`；`cargo check`；`cargo test`；`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI 手工验证未单独执行

## 风险与阻塞

- 折叠侧栏宽度有限，组头和组内会话都需要依赖缩写和 tooltip 承载信息
- 当前最稳妥路径是复用现有 `expanded_saved_groups` 状态做窄栏展开，而不是新增第二套折叠态状态机
- 组重命名入口仍保留在展开态组头，折叠态未额外塞入重命名交互，以避免窄栏过挤
- 暂无已知阻塞；若用户后续要求会话选择器也按组展示，需要再补 selector 逻辑

## 下一步

- 如需继续扩展，可下一步补折叠态组重命名入口、会话选择器分组化或组排序能力

## 最后更新时间

- 2026-07-07 12:35 CST
