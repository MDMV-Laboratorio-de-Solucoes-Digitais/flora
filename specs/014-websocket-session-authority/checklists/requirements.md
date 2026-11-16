# Requirements Checklist: 014 — WebSocket Session Authority & Protocol Alignment

- [x] Assessment R1 `go` (slice A).
- [x] WS handshake valida sessão DB-backed (jti ativo, não expirado, user/org coerentes) com falhas genéricas.
- [x] Path/transporte alinhados para cliente (`/api/v1/ws`, token suportado conforme decisão) e nil-org tratado.
- [x] Frontend WS lida com envelopes mínimos, sem sync quebrado/retry infinito em auth, com ciclo de vida coerente.
- [x] Testes DB-gated WS/sessão e wiring CI; verificações determinísticas verdes.
- [x] Validação/release SDD, PR `dev` #217, CI verde, merge/sync.
