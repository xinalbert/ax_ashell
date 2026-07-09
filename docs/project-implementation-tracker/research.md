# 外部检索记录

## 2026-07-06 russh 依赖版本

- 时间：2026-07-07 07:57 +0800
- 检索问题：`russh`、`russh-keys`、`russh-sftp` 在 crates.io / Cargo registry 的当前版本是什么
- 检索原因：用户要求将 `russh` 升级到最新版，版本信息会随时间变化，必须查询当前 registry
- 来源列表：Cargo registry / crates.io via `cargo search russh --limit 5`；Cargo registry / crates.io via `cargo search russh-keys --limit 5`；Cargo registry / crates.io via `cargo search russh-sftp --limit 5`
- 关键结论：`russh = "0.62.2"`；`russh-keys = "0.50.0-beta.7"`；`russh-sftp = "2.3.0"`
- 对实施计划的影响：本轮目标版本定为 `russh 0.62.2`；`russh-sftp` 升级到 `2.3.0`；`russh-keys` 没有与 `russh 0.62.2` 同步的稳定线，且项目没有直接使用其 API，因此移除直接依赖并使用 `russh::keys`
- 未解决问题：未做 upstream changelog 深入分析；真实 SSH/SFTP 服务器兼容性需后续联机验证

## 2026-07-07 GitHub Release 描述生成能力

- 时间：2026-07-07 07:57 +0800
- 检索问题：GitHub Release workflow 能否同时使用自动生成 release notes 和自定义 release body
- 检索原因：用户希望发布流程自动把提交记录中的重大改动放进 Release 描述
- 来源列表：GitHub Docs `Automatically generated release notes`；`softprops/action-gh-release` README
- 关键结论：GitHub 支持自动生成 release notes；`softprops/action-gh-release` 支持 `generate_release_notes`，也支持用 `body_path` 从文件读取自定义 Release body
- 对实施计划的影响：保留 `generate_release_notes: true`，同时在 publish job 中从 git tag range 生成 `release/body.md`，再通过 `body_path: release/body.md` 注入自定义 Highlights
- 未解决问题：未在真实 tag push 后执行 GitHub Release 发布演练；最终页面拼接效果需发布时确认

## 2026-07-07 X11 forwarding cookie 替换策略

- 时间：2026-07-07 07:57 +0800
- 检索问题：SSH X11 forwarding 是否可以把远端 X11 setup 直接透明转发给本机 X server，还是必须替换 fake cookie
- 检索原因：用户询问能否不处理 cookie 直接转发；该决策影响 X11 relay 的安全边界和能否被 XQuartz 接受
- 来源列表：RFC 4254 Section 6.3.1 `x11-req`；OpenSSH portable `channels.c`
- 关键结论：`x11-req` 中的 authentication cookie 应为 fake random cookie；收到 X11 connection 后，客户端应检查 fake cookie 并替换成本机 X server 的 real cookie；把 fake cookie 原样转发给 XQuartz 通常会被拒绝，把 real cookie 直接发给远端则暴露本机 X 授权凭据
- 对实施计划的影响：`src/backend/ssh.rs` 必须实现 X11 setup packet 解析、fake cookie 校验、real cookie 替换，再进入透明双向 relay；cookie 不匹配或解析失败时关闭该 X11 channel
- 未解决问题：不同远端 sshd 对 display 编号和临时 xauth 文件的实现可能有差异，仍需真实远端联机验证

## 2026-07-07 macOS bundle version 格式约束

- 时间：2026-07-07 21:29 +0800
- 检索问题：`CFBundleShortVersionString` 和 `CFBundleVersion` 是否允许直接使用四段日期版本，例如 `2026.07.06.1`
- 检索原因：本轮要把 Git tag 做成唯一发布版本源，但同日补发 tag `vYYYY.MM.DD.N` 如果直接写入 plist，可能违反 Apple 对 bundle version 的格式要求
- 来源列表：Apple Developer Documentation `CFBundleShortVersionString`；Apple Developer Glossary `version number`；Apple Developer Glossary `build version number`
- 关键结论：`CFBundleShortVersionString` 应保持三段数字版本；`CFBundleVersion` 也必须保持纯数字、最多三段的 build version 形式，不适合直接写入四段日期 tag
- 对实施计划的影响：共享版本脚本将 `CFBundleShortVersionString` 固定为 `YYYY.MM.DD`，将 `CFBundleVersion` 改为 `YYYYMMDD` 或 `YYYYMMDD.N`，避免 tag 后缀直接进入四段 plist 版本
- 未解决问题：真实 GitHub Release 产物下载后的 Finder / 系统信息展示仍需通过一次实机安装确认

## 2026-07-09 GitHub Actions 发布 runner 覆盖

- 时间：2026-07-09 07:56 +0800
- 检索问题：当前 GitHub-hosted runners 是否支持 Linux ARM64、macOS Intel / ARM64 和 Windows ARM64 标签
- 检索原因：用户要求增加发布软件的不同系统版本，runner 标签可用性会随 GitHub Actions 平台变化，需要以官方文档为准
- 来源列表：GitHub Docs `GitHub-hosted runners reference`
- 关键结论：标准 runner 列表包含 `ubuntu-22.04-arm` / `ubuntu-24.04-arm` Linux ARM64 标签、`macos-15-intel` Intel macOS 标签、`macos-14` / `macos-15` ARM64 macOS 标签；Windows ARM64 以 `windows-11-arm` 等标签提供，但标注为 public preview
- 对实施计划的影响：本轮纳入稳定收益更高的 Linux ARM64、Linux `.deb` 和 macOS universal 产物；Windows ARM64 不并入主发布矩阵，留作后续 experimental workflow 或手动验证
- 未解决问题：Linux ARM64、`.deb` 安装体验和 macOS universal app 仍需 GitHub Actions 实际运行与下载验证
