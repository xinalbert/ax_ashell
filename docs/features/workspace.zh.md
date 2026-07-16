[English](workspace.md) · [文档导航](../README.zh.md)

# 工作区

## 标签与 Pane

AxShell 可以在一个工作区内组织本地终端、SSH 终端、SFTP 页面、监控和设置。

- 打开多个终端标签，并在拆分 Pane 中组织它们。
- 水平或垂直拆分当前 Pane。
- 在相邻 Pane 之间移动焦点。
- 关闭当前 Pane，或从标签栏切换页面。
- 标签标题有最大宽度，动态终端长标题会显示省略号。

默认只有激活标签显示状态颜色，未激活标签使用中性色。在工作区设置中启用**非激活标签显示状态色**后，所有标签都会保留状态颜色。

## 标签组与独立窗口

- 将终端或 SFTP 标签拖到另一个标签位置，可重排整个会话组。该组中的终端 Pane 与 SFTP 页面会一起移动。
- 标题相同的会话组会获得稳定的 `#` 实例号，因此多个本地终端或重复打开的已保存会话在重排后仍可区分。
- 在终端标签中使用**移到新窗口**，可将该终端组、其 Pane 和运行中的 backend 移到独立终端窗口。通过该窗口标题栏或 Window 菜单中的**返回主窗口**可移回。
- 独立窗口只展示终端；需要使用该组的 SFTP 页面、设置或监控时请先返回主窗口。存在活动或暂停的 SFTP 传输时，需先处理传输才能迁移会话组。

## 搜索

使用 `Cmd/Ctrl + F` 打开终端搜索。搜索覆盖终端缓冲区，会高亮匹配结果，并支持在结果之间跳转。

## 默认快捷键

| 操作 | macOS | Linux / Windows |
| --- | --- | --- |
| 设置 | `Cmd + ,` | `Ctrl + ,` |
| 会话选择器 | `Cmd + O` | `Ctrl + O` |
| 新建 SSH | `Cmd + N` | `Ctrl + N` |
| 传输记录 | `Cmd + T` | `Ctrl + T` |
| 搜索 | `Cmd + F` | `Ctrl + F` |
| 上一个标签 | `Cmd + Shift + {` | `Ctrl + Shift + Tab` |
| 下一个标签 | `Cmd + Shift + }` | `Ctrl + Tab` |
| 切换侧边栏 | `Cmd + S` | `Ctrl + S` |
| 聚焦 Pane | `Cmd + H/J/K/L` | `Ctrl + H/J/K/L` |
| 拆分 Pane | `Cmd + Shift + H/J/K/L` | `Ctrl + Shift + H/J/K/L` |
| 关闭 Pane | `Cmd + W` | `Ctrl + W` |

复制和粘贴在 macOS 使用 `Cmd + C/V`，在 Linux 和 Windows 使用 `Ctrl + Shift + C/V`。

终端本身获得焦点时，还可以使用这些默认快捷键：

| 操作 | 默认 |
| --- | --- |
| 发送 Tab / 反向 Tab 到 Shell | `Tab` / `Shift + Tab` |
| 从终端打开会话选择器 | `Cmd/Ctrl + Shift + O` |
| Linux 和 Windows 中的终端复制 / 粘贴 | `Ctrl + C/V` |
| 从终端聚焦 Pane | `Alt + H/J/K/L` |
| 从终端拆分 Pane | `Alt + Shift + H/J/K/L` |
| 关闭终端标签 | `Alt + Q` |

所有工作区和终端焦点快捷键都可以在 **Key Bindings** 设置中修改。

已保存 SSH 会话也可在编辑表单中设置独立连接快捷键。快捷键会在终端和 SFTP 工作区生效，并且不能与上面的可配置快捷键冲突。

<!-- 截图目标：../images/features/workspace-tabs-panes.png -->
<!-- 截图目标：../images/features/workspace-keybindings.png -->
