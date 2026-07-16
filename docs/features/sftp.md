[简体中文](sftp.zh.md) · [Documentation](../README.md)

# SFTP

## Open The SFTP Page

Open SFTP from an active SSH session. The page combines a remote browser, a local browser, and transfer state for that session.

## Session SFTP Path

When creating or editing a saved SSH session, set **SFTP Path (optional)** to choose the remote directory opened by that session's SFTP page. An absolute path is used directly; `~` and relative paths are resolved from the server's home directory.

When SFTP opens without an explicit target, AxShell uses the saved session's **SFTP Path (optional)** when it is set; otherwise it opens the server home directory. A path explicitly opened from the terminal takes priority and is resolved before the first directory listing, so the page does not briefly show the configured directory or home first. If an existing SFTP page reconnects after its idle worker is reclaimed, it restores the directory already shown on that page.

Use **Open Terminal Directory** beside the remote path bar when you explicitly want to browse the connected SSH terminal's current directory. It and a path entered in the remote path bar open the requested target directly when they create the SFTP connection. Changing directories in the terminal does not automatically move the SFTP browser.

## Local Directory Memory

Set the global default under **Settings > Connections > SFTP > Default Local Directory**. New or unsaved connections start there. Leave it empty to use the user home directory.

For a saved SSH session, the local browser reopens its own last successfully opened directory instead of the global default. If a remembered directory is deleted or unreadable, AxShell opens the default local directory without replacing the remembered value.

These local paths stay on the current computer and are not included in WebDAV or S3 session sync.

## File Operations

- Browse remote directories and show or hide hidden files.
- Sort loaded entries and navigate by path.
- Upload files or folders.
- Download files or folders; directory downloads use a temporary archive when appropriate.
- Drag remote files into the local pane to download them first. The current GPUI runtime does not support dragging files out of AxShell to Finder or Explorer, including files that already exist locally.
- Create folders and recursively delete selected paths.
- Open a remote file in the system editor and upload changes after save.
- Preview supported files and bounded directory contents.

## Large Directories

Remote listings load on demand instead of reading the entire directory into the UI.

- Each page displays up to 250 additional entries.
- Use **Load More** while more entries are available.
- A listing keeps at most 2,000 entries or 2 MiB of retained name/path data.
- When the safety budget is reached, AxShell stops loading and shows a truncation state instead of presenting the result as end-of-directory.

## Transfers

Transfer tasks support pause, resume, and cancel. Completed, failed, interrupted, and active tasks are shown in transfer history, which keeps up to 100 records.

Closing an SFTP page with active work uses the configured confirmation flow so ongoing transfers are not discarded silently.

<!-- Screenshot target: ../images/features/sftp-browser.png -->
<!-- Screenshot target: ../images/features/sftp-transfer-panel.png -->
