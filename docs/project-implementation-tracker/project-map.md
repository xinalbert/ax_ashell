# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`src/session/mod.rs`，`src/backend/ssh.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`Cargo.toml`，`README.md`，`src/app/`，`src/session/`，`src/terminal/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 排除：`.git/`，`.cargo/registry/`，`.cargo/git/`，`target/`，`assets/fonts/`，`assets/icons/`，`locales/`，生成产物与外部依赖源码缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `Cargo.toml` | 仓库依赖、Rust 版本和包元数据 | 确认技术栈、版本约束、依赖能力时 | 本轮用于确认 GPUI / gpui-component / Rust 版本 |
| `src/main.rs` | 应用入口与 action 注册 | 需要确认全局快捷键或应用启动流时 | 标签交互问题通常不在这里 |
| `src/app/` | 主界面状态、布局和交互实现 | 修复 UI、面板、弹窗行为时 | 本轮不改，SSH 兼容问题不在这里 |
| `src/session/` | 会话配置与连接入口 | 需要确认 SSH 会话数据来源、用户输入和后端启动流程时 | 本轮只读，定位 `open_ssh_session` 到 `backend::ssh` 的链路 |
| `src/backend/` | 本地终端与 SSH 后端实现 | 修复连接、认证、协议兼容性或远程指标拉取问题时 | 本轮核心范围，SSH 握手与认证逻辑在这里 |
| `src/terminal/` | 终端渲染和鼠标键盘输入 | 问题涉及终端区选择、滚动、链接 hover 时 | 本轮不改 |
| `docs/project-env-audit/` | 项目环境当前态与历史 | 开工前预检或环境事实变化时 | 需保持当前态 |
| `docs/project-implementation-tracker/` | 本轮实施计划、地图与变更历史 | 真实施工前后记录计划和结论时 | 本轮需刷新到 current contract |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/backend/ssh.rs` | SSH 握手、认证与远程采样后端 | `connect_and_authenticate`，`private_keys_with_algs`，`ClientHandler` | 为老服务器加入算法兼容 fallback，并增强 `NoCommonAlgo` 诊断信息 |
| `Cargo.toml` | 依赖版本声明 | `russh`，`ssh-key` | 确认当前协议栈能力边界和是否需要改依赖 |
| `src/session/mod.rs` | SSH 会话创建入口 | `open_ssh_session`，`connect_ssh` | 确认用户输入最终如何进入 SSH backend |
| `docs/project-env-audit/current.md` | 环境当前态 | 运行环境、测试环境 | 刷新上轮残留信息 |

## 常用定位

- `rg -n "connect_and_authenticate|authenticate_publickey|authenticate_password|NoCommonAlgo" src/backend src/session`
- `rg -n "Preferred|DH_G14_SHA1|DH_G1_SHA1|AES_128_CBC|TRIPLE_DES_CBC|ALL_MAC_ALGORITHMS" ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/russh-0.49.2/src`
- `cargo check`

## 忽略与未索引

- `target/`、`.cargo/registry/`、`.cargo/git/` 未索引：属于构建产物或外部依赖缓存，不作为项目源码路由索引
- `assets/` 大部分未展开：本轮问题不涉及主题资源、字体和图标内容
- `examples/` 未展开：与本轮 SSH 兼容性修复无关

## 刷新规则

- 刷新触发：SSH 握手逻辑、算法配置、连接入口或本轮范围发生变化时刷新
- 最近依据：`rg --files` 全仓清点，结合 `src/backend/ssh.rs`、`src/session/mod.rs`、`Cargo.toml` 和 `russh` 源码的实读结果

## 最后更新时间

- 2026-07-06 15:05 CST
