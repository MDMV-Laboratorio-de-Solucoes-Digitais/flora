# Requirements Checklist: 014 — WebSocket Session Authority & Protocol Alignment

- [x] Assessment R1 `go` (slice A).
- [ ] WS handshake valida sessão DB-backed (jti ativo, não expirado, user/org coerentes) com falhas genéricas.
- [ ] Path/transporte alinhados para cliente (`/api/v1/ws`, token suportado conforme decisão) e nil-org tratado.
- [ ] Frontend WS lida com envelopes mínimos, sem sync quebrado/retry infinito em auth, com ciclo de vida coerente.
- [ ] Testes DB-gated WS/sessão e wiring CI; verificações determinísticas verdes.
- [ ] Validação/release SDD, PR `dev`, CI verde, merge/sync.
