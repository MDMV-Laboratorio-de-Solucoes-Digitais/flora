# Problem: 013

## Core problem
SMB users cannot complete the Seed communication happy path in `apps/frontend` because channel/message contracts and wiring do not match the persisted backend: channels are not discoverable/creatable in context, messages cannot be listed/posted against real UUID channels, and response/request field names diverge (C4,C5,C12,C13). This blocks the ROADMAP P0 goal of a verifiable `workspace → canal → mensagem` slice with persistence and read.

## Impact
- **Product:** core collaboration value (chat) is effectively unusable despite backend persistence primitives.
- **Correctness/security-adjacent:** missing path-segment validation on message edit/delete (C19) weakens tenancy clarity; silent field drops (`thread_id`, unknown fields) hide bugs.
- **Operability/VPS:** no new infra needed to fix; continuing to add features atop broken contracts increases rework and test noise.

## Root causes (evidenced)
1. Backend API surface incomplete for FE needs (no channel list) and request handling drops `thread_id`.
2. DTO naming/shape mismatch (snake_case backend vs camelCase FE models) with no mapping layer.
3. FE hardcodes placeholder channel IDs and has no workspace→channel selection flow.
4. Realtime stack (WS path/auth/protocol/fanout) is separately broken, so REST must be the reliable path for 013.

## Success criteria (013)
- A member can, in a selected org/workspace: list/create a channel, post a message, and read persisted messages (including a reply/thread id when provided) via REST with consistent contracts and tenancy enforcement.
- Deterministic tests pin DTOs, persistence/read, and key tenancy checks; no dependence on real OIDC/WS fanout.
