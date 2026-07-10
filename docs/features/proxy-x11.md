[简体中文](proxy-x11.zh.md) · [Documentation](../README.md)

# Proxy And X11

## Proxy Priority

SSH and SFTP connections can use:

- a per-session proxy,
- proxy environment variables loaded at startup, or
- the configured global proxy.

Supported proxy types are `socks5` and `http`, with optional username and password fields. Environment loading checks `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and lowercase variants.

## X11 Forwarding

X11 forwarding lets supported GUI programs launched on a remote SSH host display through a local X server.

Platform expectations:

- macOS: XQuartz
- Windows: VcXsrv or Xming
- Linux/Wayland: a local `DISPLAY` or Xwayland

Before connecting, confirm that the local X server is running, remote `sshd` allows `X11Forwarding yes`, and the remote application supports X11.

On Windows, the built-in launch helper prefers display `:0` and tries later displays when the corresponding port is occupied.

## Troubleshooting

- Confirm the proxy host and port are reachable without AxShell.
- Check whether a per-session proxy overrides global settings.
- Verify `DISPLAY` and the local X server before diagnosing the remote application.
- Review runtime logs for proxy negotiation or X11 relay errors.

<!-- Screenshot target: ../images/features/proxy-x11-settings.png -->
