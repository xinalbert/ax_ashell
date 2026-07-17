# Worker Registry

## Current Goal

- Goal: Completed updated-sample triage and on-demand AxShell resource initialization without changing GPUI renderer ownership.
- Coordinator: main thread
- Last Updated: 2026-07-17 11:30 CST

## Reuse Rules

- Reuse only when task context matches (`goal_id` or `task_slice`) and there is at least 1 strong + 1 weak signal, unless exact `owned_paths` + `deliverable` match allows direct reuse.
- Prefer reachable live agents over file-only records.
- Do not use wide `owned_paths` as the main reuse signal for research/chat/no-file tasks.
- Exclude `reuse_hint=do-not-reuse` rows from normal scoring unless the current request explicitly asks to resume them.
- Mark uncertain liveness as `suspected-stale` before replacement; promote to `stale` only after explicit checks.

## Agents

| name | agent_id | status | goal_id | task_slice | responsibility | owned_paths | workstream | execution_lane | worker_class | reuse_hint | deliverable | deliverable_kind | task_mode | dependency_boundary | session_id | reachability | last_heartbeat_at | last_checked_at | ttl_hint | progress_marker | overlap_keywords | notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |


## History

| name | agent_id | final_status | goal_id | task_slice | summary | closed_at | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| coordinator | main-thread | completed | 20260717-memory | diagnosis-integration | Delivered a runtime- and source-backed explanation; no application code was changed. | 2026-07-17 09:00 CST | Coordination record retained for future deduplication. |
| runtime-profiler | /root/runtime_profiler | completed | 20260717-memory | runtime-memory-sampling | Sample footprint is 245.7 MiB, led by IOSurface and malloc; a separate installed AxShell process is also running. | 2026-07-17 09:00 CST | One-shot validation; no files changed. |
| lifecycle-reviewer | /root/lifecycle_reviewer | completed | 20260717-memory | source-lifecycle-audit | Identified unconditional full-process sysinfo sampling, embedded font baseline, and conditional cache/list retention paths. | 2026-07-17 09:00 CST | One-shot source review; no files changed. |
| sampler-worker | /root/sampler_worker | completed | 20260717-window-memory | selective-system-sampling | Replaced full sysinfo refresh with selective CPU/memory sampling and made the sampler lazy. | 2026-07-17 10:17 CST | Focused test and cargo check passed; one-shot source owner. |
| detached-init-worker | /root/detached_init_worker | completed | 20260717-window-memory | detached-init-slimming | Added initialization mode, skipped detached icon/local-directory prewarming, and released stale main workspace globals. | 2026-07-17 10:17 CST | Full test suite and cargo check passed; one-shot source owner. |
| renderer-reviewer | /root/renderer_reviewer | completed | 20260717-window-memory | renderer-boundary-validation | Confirmed per-window native presentation targets on all supported platforms and supplied manual verification guidance. | 2026-07-17 10:17 CST | No local files changed; one-shot reviewer. |
| coordinator | main-thread | completed | 20260717-window-memory | integration-and-tracking | Integrated sampler, detached initialization, lifecycle cleanup, platform-boundary review, and validation records. | 2026-07-17 10:25 CST | Full Rust suite, diff check, and tracking validator passed; target-platform GUI profiling remains manual. |
| bundle-boundary-reviewer | /root/bundle_boundary_reviewer | completed | 20260717-lazy-resources | external-font-bundle-boundary | Phase 2 is feasible with `Cow::Owned`, but requires all release/dev packaging routes and resource lookup; P1 remains the current low-risk delivery. | 2026-07-17 11:08 CST | No files changed; one-shot review, do not reuse. |
| font-deferred-loader | /root/font_deferred_loader | completed | 20260717-lazy-resources | deferred-embedded-fonts | Replaced startup-wide font registration with UI / terminal first-use registration and static lazy-menu candidates. | 2026-07-17 11:10 CST | rustfmt, cargo check, focused tests, full 224-test suite, diff check, and hover audit passed; one-shot source owner. |
| sample-resource-auditor | /root/sample_resource_auditor | completed | 20260717-lazy-resources | updated-sample-audit | Identified drawable dominance, a 21.54 MiB decoded file-icon cache, startup local-directory enumeration, and redundant detached transfer cloning. | 2026-07-17 11:10 CST | No files changed; one-shot audit, do not reuse. |
| sftp-lazy-resource-loader | /root/sftp_lazy_resource_loader | completed | 20260717-lazy-resources | deferred-local-sftp-resources | Deferred file-icon cache, local SFTP directory enumeration, and the detached transfer-history clone to actual SFTP/transfer use. | 2026-07-17 11:12 CST | rustfmt, focused test, cargo check, full 225-test suite, and diff check passed; one-shot source owner. |
| lazy-font-reviewer | /root/lazy_font_reviewer | completed | 20260717-lazy-resources | deferred-font-review | Approved first-use registration order and App-scoped cache; identified tracker-map refresh and residual manual-test caveat. | 2026-07-17 11:26 CST | Read-only review; no blocking defect, do not reuse. |
| coordinator | main-thread | completed | 20260717-lazy-resources | integration-and-tracking | Integrated first-use fonts, delayed SFTP resources, route coverage, validation, and the behavior commit. | 2026-07-17 11:30 CST | `7999d30` contains only source behavior; tracking records remain a separate documentation commit. |
