# Plan: 013 — Channel & Message REST Slice (80/20)

**Spec:** `specs/013-channel-message-rest-slice/spec.md`  
**Assessment:** `.specify/assessments/013-next-vertical-slice/`

## Backend
1. `flora-api/src/routes/channels.rs`: add `ListChannelsQuery` and `GET /` handler (auth+membership+workspace ownership), persist `thread_id` in `post_message`, enforce `msg.channel_id == channel_id` in edit/delete.
2. Reuse existing `ChannelRepository::find_by_workspace_id`, `Message` model/repo `thread_id` (schema already present).

## Frontend
1. Add `apps/frontend/src/lib/api/mappers.ts` (snake_case channel/message → UI types).
2. Align `Message`/channel types and `MessageStore` usage; update `ChatLayout`/`ThreadReply` to real channelId, REST fetch/post with `thread_id`, mapped DTOs.
3. Replace channels page with minimal workspace/channel list + create + select feeding chat; neutralize hardcoded recents in layout for slice safety.

## Tests
1. `flora-tests/src/contract/messaging.rs`: replace stubs with DB-gated tests (channel list/create, message post/list, thread_id, path mismatch, cross-org isolation) reusing contract auth harness patterns.
2. Frontend unit tests for mappers/flow as feasible within scope.

## Verification
- fmt/check/clippy, `cargo test -p flora-core --lib`, `cargo test -p flora-tests --lib`, DB-gated messaging tests on disposable PG18, frontend check/lint/test:unit(server)/build.
- NV: smoke on default ports (5432 conflict env), OIDC e2e, realtime delivery, browser e2e.
