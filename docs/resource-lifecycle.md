[中文](resource-lifecycle.zh.md)

# Resource Lifecycle and Deep Sleep

## Goal

Reduce CPU, network, and repaint work while AxShell is unfocused without interrupting SSH commands, local PTYs, or SFTP transfers. Focus changes come from GPUI window activation events; AxShell does not create a polling thread to detect focus.

## State Machine

```text
Foreground --window deactivates--> Background --timeout--> DeepSleep
    ^                                  |                    |
    +----------window activates--------+--------------------+
```

- Foreground: normal terminal, cursor, monitoring, and theme refresh rates.
- Background: stop local/remote monitoring, theme polling, and cursor blinking immediately. Backend events still drain and rendering is coalesced at a lower rate so SSH output cannot accumulate in memory or a channel.
- DeepSleep: retain the same low-frequency event pump for backend events and required SFTP idle cleanup. Focus is never polled; the system window activation event restores foreground work immediately.

The default deep-sleep delay is 5 minutes after the window loses focus. Settings allow Off, 1, 5, 15, or 30 minutes. Off disables deep sleep only; background throttling still applies.

## Phase One: Safe Throttling

Phase one does not close SSH, PTY, or SFTP workers. It provides:

- A persistent deep-sleep timeout setting with a 5-minute default.
- `Foreground / Background / DeepSleep` state driven by `observe_window_activation`.
- No monitoring samples, theme polling, or cursor blinking while backgrounded or asleep.
- Continued backend draining with coalesced lower-frequency terminal and UI refreshes.
- A throttled SFTP idle-reclaim check that retains the existing five-minute idle behavior.

It intentionally does not pause remote commands, close local shells, disconnect SSH, or pause/cancel transfers. Phase one adds no deep-sleep SFTP close rule; existing idle reclamation remains unchanged, and remote-edit watcher pin/refcount protection is deferred to phase two.

## Later Defenses

### Phase Two: SFTP pins and deep-sleep reclamation

Each SFTP group gains explicit pins/refcounts. Transfers, remote-edit watchers, sync, directory work, and preview downloads hold a pin. Only an unpinned, unfocused group that has reached deep sleep may release its worker. Refocus reconnects on demand, never all pages at once.

Implementation semantics: a pin is acquired before a command is queued, so work waiting for the worker is protected too. Short work releases on completion; transfers and auto-uploads release when their child task ends; a remote-edit watcher releases when the editor closes or the worker is explicitly closed. Explicit close, transfer cancellation, and reconnect may still force worker shutdown. The ordinary five-minute background idle rule remains unchanged; deep sleep immediately reclaims an unpinned, non-current group.

### Phase Three: SSH, PTY, and query task ownership

Implemented. A terminal backend now owns both its command channel and a non-blocking shutdown controller, so tab close, reconnect, and a natural `Closed` event converge on one path. The SSH primary task receives `Close`, waits for up to two seconds, and is aborted only after that timeout. Remote monitoring and CWD queries are held in the primary session's `JoinSet` and aborted/joined when it exits. Local PTY shutdown kills the shell first, then a background reaper joins its reader and writer without blocking the UI.

Window close and the application Quit menu both call `shutdown_all_backends()`, including SFTP handles; layout saving remains synchronous during the close request. This phase does not recursively terminate background process trees started by a shell, and cannot guarantee the two-second graceful window if the OS force-kills the application.

### Phase Four: Process exit and system sleep

The cross-platform resume MVP is implemented. A gap of at least 10 seconds in the app event pump, measured with both monotonic and wall clocks, is treated as a possible resume. This intentionally remains a conservative fallback: it can also detect a debugger pause or severe scheduling stall, but it avoids assuming that every OS exposes a safe native power hook through GPUI.

On a possible resume, AxShell invalidates older monitoring requests, refreshes the system theme once, and marks live SSH connections as needing validation. Only the SSH tab in the currently visible terminal context receives one bounded five-second health check; success clears the marker, while failure uses the existing user-driven reconnect path. AxShell never reconnects all SSH tabs automatically. A dropped SSH interactive session cannot be resumed by standard SSH; use `tmux` or `screen` on the remote host when command continuity is required.

Idle SFTP workers are marked for lazy recreation, while workers with active pins, transfers, remote editing, or other queued work are left untouched. The next user operation creates a fresh SFTP worker if needed. AxShell does not restart transfers or promise resume semantics after system sleep.

The formal platform integration remains future work: macOS `NSWorkspace` sleep/wake notifications, Windows `WM_POWERBROADCAST`, and Linux logind `PrepareForSleep` should publish a shared power event. Linux native integration must remain optional because logind is not universal. It should complement, not replace, the MVP fallback.

## Resource Policy

| Resource | Background | DeepSleep | Resume |
| --- | --- | --- | --- |
| SSH terminal / local PTY | Keep running | Keep running | Continue unchanged; bounded cleanup on close/reconnect |
| Backend events | Low-frequency drain | Low-frequency drain | Refresh immediately |
| Terminal paint / cursor | Coalesced paint; no blink | Lower-rate paint; no blink | Normal refresh |
| Local and remote monitoring | No new samples | No new samples | Drop old samples; health-check only the visible SSH tab, then resume normal sampling |
| Follow-system theme | No polling | No polling | Sync once |
| SFTP transfers | Continue | Continue | No automatic restart or resume |
| Idle SFTP worker | Existing timeout applies | Phase two evaluates pins | Mark stale and reconnect on next user operation |

## Verification Boundary

- Unit tests: state transitions, disabled deep sleep, resume-gap detection, and stale monitoring result isolation.
- Local checks: `rustfmt`, `cargo check`, `cargo test --quiet`, and `git diff --check`.
- GUI checks: monitoring stops after focus loss; five minutes reaches deep sleep; refocus restores monitoring and terminal rendering; background SSH output remains bounded.
- Connected checks: closing a tab or window while SSH is connecting or while a remote probe/CWD query is running should exit within two seconds or log an abort; closing and reconnecting a local shell must not leave reader/writer threads behind.
- Platform checks: on macOS, Windows, and Linux, test sleep, hibernate where available, network loss during sleep, active terminal output, an idle SFTP page, and a pinned transfer. Confirm that only the visible SSH tab receives a check and that no connection or transfer is restarted automatically.
