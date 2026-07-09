# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用中的 GitHub Actions 发布矩阵扩展；本轮目标是新增 Linux ARM64、Linux `.deb` 和 macOS universal 发布产物

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“发布产物覆盖范围扩展”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GitHub Actions CI / Release workflow，Bash 打包脚本段
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`.github/workflows/ci.yml` 和 `.github/workflows/release.yml` 中 `cargo build --release --target ...`
- 本轮代码入口：`.github/workflows/ci.yml` build matrix；`.github/workflows/release.yml` build matrix、Linux package step、macOS universal package job
- 发布依据：Release workflow 在 tag 构建时用 `scripts/release_version.py` 同步版本，再按 matrix 生成平台产物并上传到 GitHub Release
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`Cargo.toml`，`assets/ax_shell.desktop`，`docs/project-implementation-tracker/project-map.md`

## 测试环境

- 测试框架：workflow YAML / shell 静态检查、tracking docs validator
- 默认测试命令：`cargo test`
- 当前实施验证命令：workflow YAML 解析，CI / Release Bash 片段静态检查，tracking docs validator
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮需要核对 GitHub-hosted runner 当前标签；真实构建需 GitHub Actions runner 执行
- 工具可用性：本机可做 workflow YAML / shell 静态检查；Linux ARM64 runner、Linux `.deb` 构建和 macOS universal 产物需 GitHub Actions 实际执行确认
- 证据文件：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`Cargo.toml`
- 本轮验证结果：workflow YAML 解析通过；Release workflow 所有 `run` 脚本经本地占位替换后通过 `bash -n`；`git diff --check` 通过；tracking docs validator 通过

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从 release highlights 格式收敛切换为发布产物矩阵扩展；运行环境仍是 GitHub Actions + Rust release build，不新增 Rust 依赖
- 受影响文件：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：已完成
- 原因：新增产物已在现有 GitHub Actions workflow 内完成，不需要修改 Rust 应用源码；Linux ARM64 有官方 runner 标签，`.deb` 用 `dpkg-deb` 组装，macOS universal 由两个 macOS `.app` artifact 组合
- 开工前动作：已复查现有 CI / Release matrix、Linux GUI 依赖安装、macOS bundle 打包段、Debian metadata 和官方 runner 文档；已确认不使用多 agent
- 完成后动作：已执行 workflow YAML 解析、Release `run` 脚本 Bash 静态检查、`git diff --check` 和 tracking docs validator；真实 GitHub Actions 发布验证留到下次 tag 发布
