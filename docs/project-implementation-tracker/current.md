# 当前项目实施记录

## 当前目标

- 目标：升级 Rust 依赖集合并将项目 MSRV 收口到当前依赖要求
- 交付物：更新后的 `Cargo.toml` / `Cargo.lock`、Rust `1.88.0` MSRV 文档、最新编译与测试验证结果、实施跟踪与环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`，`Cargo.lock`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 不在本轮范围内：大规模源码迁移、UI 行为改动、release workflow 改造、未经验证的 git 依赖前移

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新 tracking / env 当前态并确认本轮升级边界 | `docs/` contract 自检，`Cargo.toml` / `Cargo.lock` 走查 | 现有 `project-map.md` 已覆盖 `Cargo.toml` / `Cargo.lock`，无需刷新地图 |
| P2 | completed | 用 Cargo registry 结果识别还能升级的依赖集合 | `cargo update --dry-run`，`cargo info`，crates.io API 查询 | 已区分 lockfile 小升级、manifest 大版本升级和需要源码迁移的候选 |
| P3 | completed | 落地可安全升级的 lockfile / manifest 调整 | `git diff Cargo.toml Cargo.lock`，`cargo check --locked` | 已将 `rust-version` 提升到 `1.88.0`，匹配 `image` / `time` 当前要求 |
| P4 | completed | 完成测试与 tracking docs 校验 | `cargo check --examples --locked`，`cargo test --locked`，tracking docs 校验 | 未做 GUI / 平台手工验证 |

## 已完成

- 已读取 `docs/project-implementation-tracker/project-map.md`、`docs/project-env-audit/current.md` 与现有 tracking / env 历史
- 已确认本轮接续当前未提交的 `Cargo.lock` / docs 改动，不回滚前一轮 `anyhow`、`open`、`uuid` 升级
- 已确认项目地图覆盖本轮依赖文件范围，无需刷新 `project-map.md`
- 已执行全量和逐包 `cargo update --dry-run`，并通过 crates.io API / `cargo info` 识别 manifest 级升级候选
- 已将项目 MSRV 从 `1.85.0` 提升到 `1.88.0`，同步 `docs/development.md` 与 `docs/development.en.md`
- 已升级直依赖约束：`directories 6`、`portable-pty 0.9`、`rfd 0.17`、`rust-i18n 4`、`thiserror 2`、`notify 8`、`zip 8`、`reqwest 0.13`
- 已更新 lockfile：前一轮 `anyhow` / `open` / `uuid` 小升级继续保留，本轮补充 `time 0.3.53`、`reqwest 0.13.4`、`zip 8.6.0` 等解析结果
- 已尝试 `chacha20poly1305 0.11` / `hmac 0.13` / `rand 0.10` / `sha2 0.11`，确认需要源码 API 迁移，已回退该组升级

## 验证

- 已完成：项目地图、当前 env/tracking 记录、现有未提交 diff 走查
- 已完成：`cargo update --dry-run`
- 已完成：逐包 `cargo update --dry-run -p <dependency>`
- 已完成：`cargo info time@0.3.53`、`cargo info reqwest@0.13.4`
- 已完成：crates.io API 直依赖最新版本扫描
- 已完成：`cargo check --locked`
- 已完成：`cargo check --examples --locked`
- 已完成：`cargo test --locked`
- 已完成：tracking docs 校验
- 未完成：运行时 / GUI 手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：本轮未做 GUI / 平台手工回归；依赖升级已通过编译、examples 和单元测试，但仍需后续实际启动验证
- 风险二：加密相关依赖的新主版本会改变 `rand` / `hmac` API，已明确不纳入本次无源码迁移升级
- 风险三：仍保留既有 `block v0.1.6` future-incompat warning，来源于 GPUI / cocoa 传递依赖

## 下一步

- 如需继续清理依赖，单独规划加密依赖 API 迁移或 GPUI git 依赖前移
- 如需发布，使用 Rust `1.88.0` 或更新工具链构建

## 最后更新时间

- 2026-07-08 08:08 CST
