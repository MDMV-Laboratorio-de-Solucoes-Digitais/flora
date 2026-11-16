# Decision: 013 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice
**A. Workspace→channel→message persistence+read (REST-first) with minimal wiring fixes.** Delivers the highest-value Seed capability still missing: a tenant-safe, persisted communication path usable from `apps/frontend` in a selected workspace, without new infra and without depending on broken realtime fanout.

## Justification (evidence)
- P0 priority: `OBJETIVO.md`/`ROADMAP.md` require persisted canais/mensagens and a verifiable core e2e slice.
- Feasibility: backend already implements channel/message persistence and core routes (`flora-api/src/routes/channels.rs`), but is blocked by missing channel listing, dropped `thread_id`, path-tenancy gap (C19), and FE DTO/ID wiring (C4,C5,C12; hardcoded non-UUID channels).
- Higher ROI than B/C: (B) needs binary storage plumbing/routes (currently metadata-only JSON, C7–C8); (C) fixes WS authority/protocol but leaves chat unusable without contract+listing fixes.

## Implementation checklist (file-by-file, planned)

### Backend
- `flora-api/src/routes/channels.rs` — add `GET /` (`list_channels`) with `workspace_id` query; require auth/org/membership; verify workspace belongs to org; return `Vec<ChannelResponse>`. Persist `thread_id` in `post_message` (`CreateMessageRequest` + `Message` field). Enforce `channel_id` path matches `msg.channel_id` in `edit_message`/`delete_message` (C19). Add/adjust unit tests for `ChannelResponse`/`MessageResponse` serialization key sets.
- `flora-core` traits/repositories for channels/messages (`traits/*channel*`, `traits/*message*`, `repositories/*channel*`, `repositories/*message*`, model/schema as needed) — add `list_by_workspace`/`find_by_workspace_id` if absent; ensure thread persistence supported by model/DB (verify migration/schema; NV if schema change required — prefer using existing column if present).
- `flora-api/src/routes/mod.rs` — no route-shape change expected beyond channels router behavior (confirm).

### Frontend
- `apps/frontend/src/lib/api/mappers.ts` (new) or equivalent — `toChannel`, `toMessage` mapping snake_case→FE model, `fileAttachments: []`.
- `apps/frontend/src/lib/types/models.ts` — align/extend types only as needed for mapped model (keep mapping explicit; avoid reintroducing drift).
- `apps/frontend/src/lib/components/chat/ChatLayout.svelte`, `.../thread/ThreadReply.svelte` — use real `channelId` prop/selection, call list/post endpoints with correct payload (`thread_id` for replies), render mapped messages; remove hardcoded `default-channel`/placeholder assumptions; gate or hide broken attachment-send path for 013.
- `apps/frontend/src/routes/(app)/channels/+page.svelte`, `apps/frontend/src/lib/state/MessageStore.svelte` (+ any channel state/store added) — minimal workspace→channel list/create/select flow feeding chat; remove static demo IDs.
- `apps/frontend/src/routes/(app)/dms/+page.svelte` — mark NV/placeholder-safe (no fake UUID calls) unless trivially aligned.
- `apps/frontend/src/routes/(app)/+layout.svelte` — replace hardcoded "Recent Channels" with real list or neutral empty state (minimal).

### Tests / docs
- `flora-tests/src/contract/messaging.rs` (replace `Ok(())` stubs) and/or DB-gated tests — DTO key snapshot, create/list channels by workspace, post/list messages, thread_id persisted/read, edit/delete rejects mismatched `channel_id` path (C19), cross-org isolation.
- `apps/frontend/tests/unit/*` — mapper unit tests (`sender_id→authorId`, `thread_id`, `fileAttachments` default, no crash on render shape).
- `apps/frontend/tests/integration/*` (as applicable) — REST chat flow against mocked fetch.
- `docs/dev-smoke.md` / assessment evidence — optional manual REST smoke steps (NV if not run).

## Deterministic verification plan
1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings` (or scoped crates if workspace lints heavy; record)
3. `cargo test -p flora-core --lib`
4. `cargo test -p flora-tests --lib`
5. DB-gated (Postgres 18, disposable/alt-port ok): `FLORA_DATABASE__POSTGRES_URL=... cargo test -p flora-tests contract::messaging::db_ -- --ignored --test-threads=1` and include in CI `db-tests` job when added.
6. `cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build`
7. NV (environment/explicit): full smoke with running API+DB in default ports, real OIDC/JWKS, WS realtime fanout/delivery, e2e browser flow, `apps/web`.

## Risks
- **Schema uncertainty:** `thread_id`/message schema support must be confirmed; if migration needed, keep 013 to existing columns or split (NV risk).
- **Scope creep:** realtime fanout, attachments, channel permissions/DM model, pagination correctness (C22) can expand quickly — enforce non-goals.
- **FE blast radius:** chat/layout/store changes can regress tasks/auth shell; mitigate with mapper isolation and existing deterministic checks.
- **Tenancy edge cases:** list-by-workspace must not leak other org data; rely on workspace ownership + membership checks.

## Non-goals (013)
- WS broadcast/subscription registry, Valkey fanout, `/messages/sync`, WS session DB authority, reconnect/backoff redesign (C6,C20,C25 partial) — follow-up.
- File binary upload/download/storage, attachment persistence/linking (C7,C8,C13).
- DMs UX, channel membership/roles UI, search/notifications wiring, task listing semantics fix (C11), cross-tenant workspace_id hardening for tasks/files (C18) except incidental, OIDC e2e, `apps/web`, deploy/merge.

## Debloat notes
- REST-first on Postgres avoids adding/depending on realtime infra for correctness; no new mandatory services. Favor a small mapper module over generating/duplicating types. Defer pagination/cursor polish until needed.
