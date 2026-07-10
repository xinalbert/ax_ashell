[简体中文](terminal-ssh.zh.md) · [Documentation](../README.md)

# Terminal And SSH

## Local Terminals

Open a local terminal from the pinned **Local Terminal** entry in the saved-session area. Local terminals use the same tab, pane, search, and appearance controls as SSH terminals.

## Create An SSH Session

1. Open **New SSH** or the session selector.
2. Enter a host, port, and username.
3. Choose password or private-key authentication.
4. Optionally set a group and per-session proxy.
5. Use **Save** or **Save & Connect**.

Private-key authentication supports a key file path or inline key content and an optional passphrase.

## Saved Sessions

- Sessions can be grouped; sessions without a group appear under **Ungrouped**.
- Group headers can be expanded, collapsed, and renamed.
- Renaming a group updates the saved sessions assigned to it.
- **Local Terminal** is pinned above saved SSH groups and is not stored as an SSH session.
- The last-used timestamp is persisted for saved sessions.

## Connection Behavior

- SSH sessions can use a per-session proxy or the configured global/environment proxy.
- Connection progress and retry state are shown in the workspace.
- Closing a tab or pane shuts down the backend owned by that terminal.
- Legacy SSH algorithm fallback is available for servers that require supported older algorithms.

See [Proxy And X11](proxy-x11.md) for transport and graphical forwarding settings.

<!-- Screenshot target: ../images/features/terminal-ssh-session-form.png -->
<!-- Screenshot target: ../images/features/terminal-ssh-saved-groups.png -->
