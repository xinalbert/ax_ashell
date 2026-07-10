[English](README.md) · [文档导航](../README.zh.md)

# 文档图片说明

面向用户的功能截图统一放在 `docs/images/features/`。

## 命名规则

- 文件名使用小写 kebab-case。
- 以对应功能页作为前缀，例如 `sftp-transfer-panel.png`。
- UI 截图优先使用 PNG。
- 截图中不要包含敏感主机、用户名、路径、凭据或终端输出。

## 添加图片

各功能页包含类似的 HTML 注释：

```html
<!-- 截图目标：../images/features/sftp-browser.png -->
```

图片加入仓库后，把注释替换为带说明的 Markdown：

```markdown
![带传输控制的 SFTP 浏览器](../images/features/sftp-browser.png)
```

界面内容与语言无关时，中英文页面可以共用图片；否则使用 `-en`、`-zh` 后缀区分。
