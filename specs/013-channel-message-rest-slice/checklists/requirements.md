# Requirements Checklist: 013 — Channel & Message REST Slice (80/20)

- [x] Assessment R1 `go` (`.specify/assessments/013-next-vertical-slice/decision.md`).
- [x] `GET /api/v1/channels?workspace_id=` com checks de auth/membership/workspace-org.
- [x] `thread_id` persistido em mensagens; edit/delete rejeitam path `channel_id` divergente.
- [x] Frontend consome DTOs snake_case mapeados e usa IDs reais de workspace/canal.
- [x] Fluxo mínimo workspace→canal→mensagem (listar/criar/postar/ler) funcional via REST.
- [x] Testes DB-gated de messaging e verificações determinísticas verdes.
- [x] Validação/release SDD, PR `dev`, CI verde (#216, 6/6), merge/sync.
