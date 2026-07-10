[简体中文](README.zh.md) · [Documentation](../README.md)

# Documentation Images

Store user-facing screenshots under `docs/images/features/`.

## Naming

- Use lowercase kebab-case filenames.
- Prefix the filename with the related guide, for example `sftp-transfer-panel.png`.
- Prefer PNG for UI screenshots.
- Keep sensitive hosts, usernames, paths, credentials, and terminal output out of screenshots.

## Adding An Image

Each feature page contains an HTML comment such as:

```html
<!-- Screenshot target: ../images/features/sftp-browser.png -->
```

After adding the image file, replace the comment with descriptive Markdown:

```markdown
![SFTP browser with transfer controls](../images/features/sftp-browser.png)
```

Use the same image in both language pages when the interface content is language-neutral. Otherwise add language suffixes such as `-en` and `-zh`.
