[English](configuration-sync.md) · [文档导航](../README.zh.md)

# 配置同步

AxShell 可以通过 WebDAV 或 S3 兼容对象存储同步已保存 SSH 会话及其分组信息。

## 支持的后端

- WebDAV 地址和用户名
- S3 兼容 endpoint、region、bucket 和 object key

默认远端对象名为：

```text
ax_shell-sync.json
```

## 安全模型

- 同步载荷在上传前先在本地加密。
- WebDAV 和自定义 S3 地址必须使用 HTTPS；内置 AWS S3 地址同样使用 HTTPS。
- 每个远端响应（包括 S3 错误响应）在解析或显示前都限制为 8 MiB。
- 加密口令、WebDAV 密码和 S3 凭据只保留在当前进程内，不写入本地配置文件。
- endpoint、用户名、bucket、region 和 object key 等连接参数可以保存到本地。
- 已信任的 SSH 主机密钥仅保存在本机，绝不会进入同步载荷。

## 上传与下载

- 上传会序列化当前已保存会话配置，加密后写入所选后端。
- 下载会解密远端载荷，并替换本机已保存会话列表及分组信息。
- 下载会替换本地会话数据，操作前应确认 endpoint 和加密口令正确。
- HTTP 地址和超出上限的响应会被拒绝，且不会应用任何同步会话数据。

<!-- 截图目标：../images/features/configuration-sync-webdav.png -->
<!-- 截图目标：../images/features/configuration-sync-s3.png -->
