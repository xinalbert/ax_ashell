# 当前项目实施记录

## 当前目标

- 目标：将 GitHub 发布链路改成“tag 作为唯一发布版本源”，让 workflow、构建时版本、release 产物名和 macOS bundle 版本从同一套规则派生
- 交付物：共享版本解析脚本、release workflow tag 版本注入、Cargo manifest/lock 临时同步、打包脚本复用同一规则、README/development 文档更新、格式化与脚本级验证、跟踪记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`Cargo.toml`，`Cargo.lock`，`src/app/constants.rs`，`src/app/startup.rs`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：SSH / SFTP 协议行为、主题编辑器行为、终端渲染策略、UI 布局、依赖升级、真实远端仓库重命名、安装包实机安装验证、Git 历史整理

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 current plan、项目地图和环境记录到“tag 全链路版本源”任务，并确定 tag/version 映射规则 | `docs/` contract 自检，源码走查 | 输入格式最终收口为 `vYYYY.M.D` / `vYYYY.M.D-N` |
| P2 | completed | 新增共享版本解析/同步脚本，统一 tag、Cargo semver、公开显示版本与 bundle version 派生 | 脚本样例运行，源码走查 | `scripts/release_version.py` 现为唯一版本派生入口 |
| P3 | completed | 更新 release workflow，在 tag 构建时自动注入版本并同步 manifest/lock 后再编译打包 | workflow YAML 自检，脚本样例运行 | tag 构建会先同步 runner 内的 manifest/lock，再进入 `cargo build --release` |
| P4 | completed | 让本地 macOS 打包脚本复用共享版本规则，并同步更新 README/development 文档 | shell 脚本静态检查，README/doc 走查 | 本地 `.app` 打包与 GitHub Release 共用同一套版本规则 |
| P5 | completed | 运行必要脚本验证、编译检查和 tracking docs 校验 | 版本脚本样例运行，`cargo check`，tracking docs 校验 | 本轮未改 Rust 源码，因此未额外运行 `rustfmt` |

## 已完成

- 新增 `scripts/release_version.py`，统一解析规范 tag `vYYYY.M.D` / `vYYYY.M.D-N` 到 Cargo 版本、公开版本、macOS bundle 版本与 tag 文本
- release workflow 在 build job 中先解析 tag，再同步 runner 内的 `Cargo.toml` / `Cargo.lock` 根包版本，之后才编译与打包
- publish job 现复用同一套版本规则，GitHub Release 标题直接使用派生后的公开版本
- 本地 `scripts/package-macos-app.sh` 改为读取共享版本脚本，不再各自维护 plist 版本拼接逻辑
- README 与中英文 development 文档已补充 tag 格式、版本映射、Cargo 版本限制和手动 `workflow_dispatch` 的行为说明
- 已通过少量官方文档检索确认 macOS bundle 版本约束，并将 `CFBundleShortVersionString` 固定为三段日期，`CFBundleVersion` 固定为纯数字日期或 `日期.补发序号`
- 已确认 Cargo 拒绝 `2026.07.06` 这类带前导零的 semver 版本，因此规范 tag 改为与 `Cargo.toml` 一致的 Cargo 兼容格式

## 验证

- 已完成：`rg -n "GITHUB_REF_NAME|refs/tags|version|CFBundleShortVersionString|CARGO_PKG_VERSION" .github/workflows scripts src Cargo.toml`
- 已完成：release workflow、打包脚本、运行时版本显示路径的源码走查
- 已完成：`python3 scripts/release_version.py --help`
- 已完成：`python3 scripts/release_version.py env --tag v2026.7.6`
- 已完成：`python3 scripts/release_version.py env --tag v2026.7.6-1`
- 已完成：`python3 scripts/release_version.py env --cargo-version-file Cargo.toml`
- 已完成：`python3 scripts/release_version.py env --tag v2026.2.30` 失败校验
- 已完成：临时 manifest 验证 `Cargo.toml` 不接受 `2026.07.06` / `2026.07.06.1`
- 已完成：`bash -n scripts/package-macos-app.sh`
- 已完成：`.github/workflows/release.yml` YAML 静态自检
- 已完成：`cargo check`
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：真实 tag push、GitHub Release 页面展示与 macOS bundle 平台侧实机验证

## 风险与阻塞

- 阻塞：无
- 风险一：尚未做真实 tag push，因此 GitHub Release 页面上的最终展示效果仍需在远端执行一次确认
- 风险二：macOS `.app` 的 Finder / 系统信息展示仍属于平台侧实机验证范围，本轮只完成规则和脚本级收口

## 下一步

- 建议下一步直接推一次测试 tag，确认 GitHub Release 页面、资产名称和 macOS bundle 展示符合预期

## 最后更新时间

- 2026-07-07 21:25 CST
