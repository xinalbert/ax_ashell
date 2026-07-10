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

复制和粘贴在 macOS 使用 `Cmd + C/V`，在 Linux 和 Windows 使用 `Ctrl + Shift + C/V`。所有工作区快捷键都可以在 **Key Bindings** 设置中修改。

<!-- 截图目标：../images/features/workspace-tabs-panes.png -->
<!-- 截图目标：../images/features/workspace-keybindings.png -->
