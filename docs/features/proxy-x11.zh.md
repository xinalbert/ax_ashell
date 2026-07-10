[English](proxy-x11.md) · [文档导航](../README.zh.md)

# 代理与 X11

## 代理优先级

SSH 和 SFTP 连接可以使用：

- 单会话代理；
- 启动时读取的代理环境变量；或
- 已配置的全局代理。

代理类型支持 `socks5` 和 `http`，可以填写用户名和密码。环境变量读取会检查 `ALL_PROXY`、`HTTPS_PROXY`、`HTTP_PROXY` 及其小写形式。

## X11 转发

X11 转发可以让远端 SSH 主机启动的兼容图形程序通过本地 X server 显示。

各平台要求：

- macOS：XQuartz
- Windows：VcXsrv 或 Xming
- Linux/Wayland：本地 `DISPLAY` 或 Xwayland

连接前需要确认本地 X server 已运行、远端 `sshd` 允许 `X11Forwarding yes`，并且远端程序支持 X11。

Windows 内置启动辅助会优先使用 display `:0`，对应端口被占用时继续尝试后续 display。

## 故障排查

- 先确认代理主机和端口在 AxShell 之外可以访问。
- 检查单会话代理是否覆盖了全局设置。
- 排查远端程序前，先确认 `DISPLAY` 和本地 X server。
- 从运行日志中查找代理协商或 X11 relay 错误。

<!-- 截图目标：../images/features/proxy-x11-settings.png -->
