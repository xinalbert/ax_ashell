# 当前项目实施记录

## 当前目标

- 目标：整理 Settings 的信息架构，把拥挤的 General 页拆为职责清晰的设置子页面，降低后续维护成本
- 交付物：Settings 页面装配更新；General 设置拆分后的子模块；新增/更新本地化文案；项目地图、环境记录和实施记录同步更新

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：配置 schema 修改、Settings 组件库替换、Proxy/Sync 表单深度重构、运行时行为变更、release/tag、提交

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 设置页现状与环境预检记录 | `docs/project-env-audit/current.md` 已刷新 | 确认 General 页混合外观、字体、终端、工作区、监控、语言和重置入口 |
| P2 | completed | Settings General 拆分为 focused pages / modules | `rustfmt --edition 2024 <changed-rust-files>` 通过 | 保持配置保存逻辑原样，只调整页面归类 |
| P3 | completed | 本地化文案与项目地图刷新 | tracking docs validator 通过 | 新增页面标题和关键文件索引 |
| P4 | completed | 编译、测试和 diff 校验结果 | `cargo check`，`cargo test --quiet`，`git diff --check` 通过 | GUI 手工验证未执行 |

## 已完成

- 已把原 `src/app/dialogs/settings/general.rs` 拆为 `appearance.rs`、`font_page.rs`、`terminal.rs`、`workspace.rs`、`monitoring.rs` 和 `language.rs`
- 已更新 `src/app/dialogs/settings.rs` 页面顺序：Appearance、Fonts、Terminal、Workspace、Monitoring、Language、Custom、Sync、Proxy、Key Bindings、Help、About
- 已补充中英文设置页标题文案
- 已刷新 `docs/project-implementation-tracker/project-map.md`，移除旧 General 页索引并加入新的 focused settings 子页面
- 已同步环境记录与月度实施记录

## 验证

- 已完成：`rustfmt --edition 2024 src/app/dialogs/settings.rs src/app/dialogs/settings/appearance.rs src/app/dialogs/settings/font_page.rs src/app/dialogs/settings/terminal.rs src/app/dialogs/settings/workspace.rs src/app/dialogs/settings/monitoring.rs src/app/dialogs/settings/language.rs`；`cargo check`；`cargo test --quiet`；`git diff --check`；tracking docs validator
- 未完成：GUI 设置页手工点击验证

## 风险与阻塞

- 风险一：拆分页面会改变 Settings 左侧导航结构，需要人工确认实际信息架构是否符合预期
- 风险二：本轮只整理 General 页，`sync.rs` 与 `proxy.rs` 仍是相对较大的表单文件，可后续按同样方式继续拆
- 风险三：GUI 手工验证尚未执行，最终视觉间距和默认打开页需要在真实应用中确认

## 下一步

- 在真实 GUI 中打开 Settings，检查新侧栏顺序、各下拉/开关保存行为、Language 切换和 reset layout 入口

## 最后更新时间

- 2026-07-10 07:35 +0800
