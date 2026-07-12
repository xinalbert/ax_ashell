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

## Local X Server Downloads

AxShell does not bundle a local X server. Install and start one before using SSH X11 forwarding, or enable AxShell's local X server launch option and point it at the installed app.

| Platform | X server | Where to get it | Notes |
| --- | --- | --- | --- |
| macOS | XQuartz | [xquartz.org](https://www.xquartz.org/) | Stable default choice for macOS. The app is normally installed as `/Applications/Utilities/XQuartz.app`. |
| macOS | MacXServer | [macxserver.com/download](https://macxserver.com/download/) or [GitHub releases](https://github.com/toddvernon/MacXServer/releases) | Modern rootless macOS X server. AxShell treats `MacXServer.app` as display `127.0.0.1:0`, matching port `6000` / display `:0`. |
| Windows | VcXsrv | [GitHub releases](https://github.com/marchaesen/vcxsrv/releases) or [SourceForge](https://sourceforge.net/projects/vcxsrv/) | Current open-source Windows option. AxShell looks for common `VcXsrv` install paths. |
| Windows | Xming | [SourceForge archive](https://sourceforge.net/projects/xming/) or [Straight Running](https://www.straightrunning.com/XmingNotes/) | Legacy Windows alternative. Some current downloads may follow the Straight Running license/download flow. |
| Linux / Wayland | X.Org / Xwayland | Install from your distribution package manager; see [X.Org](https://xorg.freedesktop.org/wiki/) and [Wayland](https://wayland.freedesktop.org/) for project context. | Prefer distro packages such as `xwayland` or the distro's X.Org server package instead of downloading standalone binaries. |

Before connecting, confirm that the local X server is running, remote `sshd` allows `X11Forwarding yes`, and the remote application supports X11.

On Windows, the built-in launch helper prefers display `:0` and tries later displays when the corresponding port is occupied.

## Troubleshooting

- Confirm the proxy host and port are reachable without AxShell.
- Check whether a per-session proxy overrides global settings.
- Verify `DISPLAY` and the local X server before diagnosing the remote application.
- Review runtime logs for proxy negotiation or X11 relay errors.

<!-- Screenshot target: ../images/features/proxy-x11-settings.png -->
