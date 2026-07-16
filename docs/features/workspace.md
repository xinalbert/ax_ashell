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

## Tab Groups And Standalone Windows

- Drag a terminal or SFTP tab onto another tab position to reorder the whole session group. A group's terminal panes and SFTP page move together.
- Groups with the same title receive a stable `#` instance number, so local terminals or repeated saved-session connections remain distinguishable after reordering.
- Use **Move to New Window** on a terminal tab to move its terminal group, panes, and running backend into a dedicated terminal window. Use **Return to Main Window** in that window's title bar or Window menu to bring it back.
- A standalone window is terminal-only; return it to the main window to use its SFTP page, settings, or monitoring. Groups with active or paused SFTP transfers cannot be moved until the transfer is resolved.

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

Copy and paste use `Cmd + C/V` on macOS and `Ctrl + Shift + C/V` on Linux and Windows.

When the terminal itself has focus, these additional bindings are available:

| Action | Default |
| --- | --- |
| Send Tab / Backtab to shell | `Tab` / `Shift + Tab` |
| Open session selector from terminal | `Cmd/Ctrl + Shift + O` |
| Copy / paste in terminal on Linux and Windows | `Ctrl + C/V` |
| Focus pane from terminal | `Alt + H/J/K/L` |
| Split pane from terminal | `Alt + Shift + H/J/K/L` |
| Close terminal tab | `Alt + Q` |

All workspace and terminal-focus bindings can be changed in **Key Bindings** settings.

Saved SSH sessions can also have individual connection shortcuts in their edit form. Those shortcuts work from the terminal and SFTP workspace, and must not conflict with the configured bindings above.

<!-- Screenshot target: ../images/features/workspace-tabs-panes.png -->
<!-- Screenshot target: ../images/features/workspace-keybindings.png -->
