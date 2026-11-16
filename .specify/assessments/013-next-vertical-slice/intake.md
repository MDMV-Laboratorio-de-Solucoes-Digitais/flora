# Intake: 013 — Next vertical slice (Flora Seed MVP)

**Date:** 2026-11-16  
**Base:** `dev` after PR #215 (cycle 012 session authority)  
**Mode:** assessment only (no source changes/commits)

## Problem statement
Cycle 012 closed HTTP session authority (DB-backed `jti` validation) and 401 body preservation, but the core Seed communication path remains non-functional end-to-end: workspace→channel→message persistence/read is contract-broken and UI-wired to hardcoded, non-UUID channels; realtime/WS has endpoint/auth/protocol gaps (audit C4–C6, C11–C13, C22). Files are metadata-only (no binary upload/download, C7–C8). ROADMAP P0 prioritizes a verifiable core e2e slice and essential wiring.

## Candidates evaluated
| Option | Summary | Value/feasibility (80/20) |
|---|---|---|
| **A. workspace→channel→message persistence+read (REST-first) + minimal wiring** | Align contracts, add channel listing, persist/read/post in real workspace/channel context; defer WS fanout/sync/attachments | **Highest.** Direct P0 MVP value (communication usable, persisted). Backend primitives exist (`channels`/`messages` routes, repos) so scope is mostly contract/wiring/tenancy fixes + deterministic tests. |
| B. basic file upload/download binary correctness | Multipart upload, storage write/read, download streaming, quota/path safety, FE rewrite | Lower now. Requires new storage plumbing/routes and larger risk surface; current backend is intentionally metadata-only. Defer until chat/task core is verifiable. |
| C. WS session authority + minimal message contract alignment | DB session check on `/ws`, fix WS path/protocol, partial DTO fixes | Mixed. WS authority is a valid follow-up to 012 but without channel/message contract+listing fixes the user flow stays broken; not a clean vertical. Fold WS authority into a later realtime-focused slice. |

## Chosen primary slice
**A — workspace→channel→message persistence+read (REST-first) with minimal wiring fixes.** Realtime broadcast/fanout, WS protocol overhaul, attachments, and file binaries are explicit non-goals.

## Constraints / guardrails
- No source edits in this assessment; no deploy/merge/release/secrets.
- Postgres-first, VPS-cheap, no new mandatory infra/services.
- Evidence-backed claims only; NV where not executed.
