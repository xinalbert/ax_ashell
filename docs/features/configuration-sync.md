[简体中文](configuration-sync.zh.md) · [Documentation](../README.md)

# Configuration Sync

AxShell can synchronize saved SSH sessions and group assignments through WebDAV or S3-compatible object storage.

## Supported Backends

- WebDAV endpoint and username
- S3-compatible endpoint, region, bucket, and object key

The default remote object name is:

```text
ax_shell-sync.json
```

## Security Model

- The sync payload is encrypted locally before upload.
- The encryption password, WebDAV password, and S3 credentials remain in process memory and are not written to the local configuration file.
- Connection metadata such as endpoints, usernames, bucket, region, and object key can be saved locally.

## Upload And Download

- Upload serializes the current saved-session configuration, encrypts it, and writes it to the selected backend.
- Download decrypts the remote payload and replaces the local saved-session list and group assignments.
- Verify the selected endpoint and encryption password before downloading because the operation replaces local session data.

<!-- Screenshot target: ../images/features/configuration-sync-webdav.png -->
<!-- Screenshot target: ../images/features/configuration-sync-s3.png -->
