# Tasks: 116 — identidade real de autores no chat

**Data**: 2026-11-23 · **Spec**: `specs/116-chat-author-identity/spec.md`

- T-001: Add batch `UserRepository::find_by_ids`, `resolve_sender_name` + unit tests, and author-enriched `MessageResponse`/`MessageEventPayload` across `channels.rs` & `websocket.rs` [FR-001 / SC-001]
- T-002: Add `db_messages_include_author_display_name_across_post_list_and_catchup` contract test and lockstep floor `messaging: 4 -> 5` (79 total) [FR-002 / SC-002]
- T-003: Implement `chatAuthor.ts` (TDD `chatAuthor.test.ts`), update `Message`/mappers/`MessageStore`/`ChatLayout.svelte` and unit tests [FR-003 / SC-003]
- T-004: Assert real author identities (`Alice Admin`, `Bob Builder`, zero truncated UUID) in `seed-flow.e2e.ts` Step 3 [FR-004 / SC-004]
- T-005: Author `tenant` Comprehension Briefing, governance/append-only docs, test roadmap/evidence, R4/R7 (`0.9.0`), and independent ACR [FR-005 / SC-005]
