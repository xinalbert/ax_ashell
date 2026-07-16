[简体中文](serial-telnet.zh.md) · [Documentation](../README.md)

# Serial And Telnet

AxShell can save and reopen serial-console and Telnet sessions alongside local terminals and SSH connections. They use the same terminal tabs, panes, search, and keybinding controls.

## Create A Connection

1. Open **New Connection** or the session selector.
2. Choose **Serial** or **Telnet** under **Connection Type**.
3. Set an optional connection name and group.
4. Review the connection-specific fields, then select **Save** or **Save & Connect**.

Saved entries appear with the rest of the saved sessions. Their selector detail shows either the serial port and baud rate or the Telnet host and port.

## Serial Console

When the form opens or switches to **Serial**, AxShell detects available local serial ports. Use **Refresh** after connecting a device, then select a detected port or enter one manually.

Configure baud rate, data bits, parity, stop bits, and flow control for the device. Defaults are `115200 8N1` with no flow control. Disconnecting or closing the terminal releases the port; use the normal reconnect action after resolving a cable, device, or port-in-use problem.

## Telnet

Enter a Telnet host and port. The default port is `23`. AxShell performs conservative Telnet negotiation so the remote side can receive terminal-size updates and interactive terminal input remains usable.

Telnet is not encrypted. Use it only for trusted networks and systems that explicitly require it; prefer SSH whenever the server supports SSH.

## SSH-Only Features

SFTP pages, remote system monitoring, X11 forwarding, SSH private-key/password authentication, and SSH connection-health recovery apply only to SSH sessions. A serial or Telnet session remains a terminal connection and does not open an SFTP page.

For local shells and SSH setup, see [Terminal And SSH](terminal-ssh.md). For tabs, panes, and standalone terminal windows, see [Workspace](workspace.md).

<!-- Screenshot target: ../images/features/serial-telnet-session-form.png -->
