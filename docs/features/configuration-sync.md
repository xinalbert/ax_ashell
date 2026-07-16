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
- WebDAV and custom S3 endpoints must use HTTPS. The built-in AWS S3 endpoint also uses HTTPS.
- Every remote response, including an S3 error response, is limited to 8 MiB before it is parsed or displayed.
- The encryption password, WebDAV password, and S3 credentials remain in process memory and are not written to the local configuration file.
- Connection metadata such as endpoints, usernames, bucket, region, and object key can be saved locally.
- Trusted SSH host keys are local-only and are never included in the sync payload.

## Upload And Download

- Upload serializes the current saved-session configuration, encrypts it, and writes it to the selected backend.
- Download decrypts the remote payload and replaces the local saved-session list and group assignments.
- Verify the selected endpoint and encryption password before downloading because the operation replaces local session data.
- HTTP endpoints and oversized responses are rejected before any synchronized session data is applied.

<!-- Screenshot target: ../images/features/configuration-sync-webdav.png -->
<!-- Screenshot target: ../images/features/configuration-sync-s3.png -->
