[简体中文](workspace.zh.md) · [Documentation](../README.md)

# Workspace

## Tabs And Panes

AxShell can keep local terminals, SSH terminals, SFTP pages, monitoring, and settings in one workspace.

- Open multiple terminal tabs and group them in split panes.
- Split the focused pane horizontally or vertically.
- Move focus between adjacent panes.
- Close the focused pane or activate another tab from the tab bar.
- Tab titles are width-limited and use ellipsis for long dynamic terminal titles.

Inactive tabs use a neutral indicator by default. Enable **Color inactive tabs** in workspace settings to keep status colors visible on every tab.

## Search

Open terminal search with `Cmd/Ctrl + F`. Search covers the terminal buffer, highlights matches, and lets you move between results.

## Default Keybindings

| Action | macOS | Linux / Windows |
| --- | --- | --- |
| Settings | `Cmd + ,` | `Ctrl + ,` |
| Session selector | `Cmd + O` | `Ctrl + O` |
| New SSH | `Cmd + N` | `Ctrl + N` |
| Transfers | `Cmd + T` | `Ctrl + T` |
| Search | `Cmd + F` | `Ctrl + F` |
| Previous tab | `Cmd + Shift + {` | `Ctrl + Shift + Tab` |
| Next tab | `Cmd + Shift + }` | `Ctrl + Tab` |
| Toggle sidebar | `Cmd + S` | `Ctrl + S` |
| Focus pane | `Cmd + H/J/K/L` | `Ctrl + H/J/K/L` |
| Split pane | `Cmd + Shift + H/J/K/L` | `Ctrl + Shift + H/J/K/L` |
| Close pane | `Cmd + W` | `Ctrl + W` |

Copy and paste use `Cmd + C/V` on macOS and `Ctrl + Shift + C/V` on Linux and Windows. All workspace bindings can be changed in **Key Bindings** settings.

<!-- Screenshot target: ../images/features/workspace-tabs-panes.png -->
<!-- Screenshot target: ../images/features/workspace-keybindings.png -->
