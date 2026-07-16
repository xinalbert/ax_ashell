[简体中文](terminal-ssh.zh.md) · [Documentation](../README.md)

# Terminal And SSH

## Local Terminals

Open a local terminal from the pinned **Local Terminal** entry in the saved-session area. Local terminals use the same tab, pane, search, and appearance controls as SSH terminals.

Choose the default local shell under **Settings > Terminal > Local Shell Profiles**. A profile has a program and zero or more arguments, with one argument per line. This supports shells such as `zsh`, `bash`, PowerShell, Command Prompt, Git Bash, and WSL. For a named WSL distribution, use `wsl.exe` as the program and enter `-d` and the distribution name on separate lines.

Profiles are editable and can be duplicated before customization. New local terminals use the selected default; splitting or reconnecting an existing local tab keeps that tab's profile.

## Create An SSH Session

1. Open **New Connection** or the session selector and choose **SSH**.
2. Enter a host, port, and username in the **Connection** section.
3. Choose password or private-key authentication in **Authentication**.
4. Optionally set a connection name or group in **Organization**.
5. Expand **Advanced SSH Options** for proxy, SFTP path, X11 forwarding, and a connection shortcut.
6. Use **Save** or **Save & Connect**.

Private-key authentication supports a key file path or inline key content and an optional passphrase.

See [Serial And Telnet](serial-telnet.md) for non-SSH terminal sessions.

## Saved Sessions

- Sessions can be grouped; sessions without a group appear under **Ungrouped**.
- Group headers can be expanded, collapsed, and renamed.
- Renaming a group updates the saved sessions assigned to it.
- **Local Terminal** is pinned above saved SSH groups and is not stored as an SSH session.
- The last-used timestamp is persisted for saved sessions.
- Each saved session can record a **Connection Shortcut** in its edit form. The shortcut opens and focuses that SSH session from the terminal or SFTP workspace.
- Connection shortcuts must use a modifier or an `F1`-`F24` key, cannot overlap another connection or a configured app shortcut, and are not included in credential-free session exports.
- Hover a saved session to see its connection shortcut and configured SFTP path.
- **Copy Session JSON** writes the same credential-free JSON used by export. Use **Import from Clipboard** in the SSH form to load one copied session; while editing, the local credentials and shortcut remain unchanged.
- Exporting a group writes the same share format as full export, so import restores every session in that group and its group name.

## Connection Behavior

- SSH sessions can use a per-session proxy or the configured global/environment proxy.
- Connection progress and retry state are shown in the workspace.
- Closing a tab or pane shuts down the backend owned by that terminal.
- Legacy SSH algorithm fallback is available for servers that require supported older algorithms.

See [Proxy And X11](proxy-x11.md) for transport and graphical forwarding settings.

<!-- Screenshot target: ../images/features/terminal-ssh-session-form.png -->
<!-- Screenshot target: ../images/features/terminal-ssh-saved-groups.png -->
