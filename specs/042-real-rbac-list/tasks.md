# Tasks: 042 — Real RBAC role listing

- [x] T001 Research (c042-01): minimal GET /roles slice + DTO + auth model approved.
- [x] T002 Implement `list_roles` real (AuthExtractor + require_org + membership + can_list_roles).
- [x] T003 Add `RoleResponse` DTO (omits tenant/timestamps) + `can_list_roles` pure helper.
- [x] T004 Add 3 pure unit tests (permission gate allow/deny, DTO omission).
- [x] T005 Gates green: fmt/clippy/deny/test (3 rbac tests), fresh isolated check (330 crates).
- [x] T006 Cycle 041 F-1 resolved as concurrency false-positive; validate updated.
- [ ] T007 Author-blind audit on frozen snapshot + validate/release/research/ROADMAP + commit.
