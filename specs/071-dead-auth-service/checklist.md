# Quality Checklist: 071 — remover stub morto AuthService

> Revisão de **qualidade de requisitos**, não registro de implementação.

## Requisitos

- [x] FR-001 — arquivo e exports removidos.
- [x] FR-002 — `AppState` sem campo/init/import.
- [x] FR-003 — 7 construtores de teste limpos.
- [x] FR-004 — `ServiceUnavailable`, `AuthProvider`, `UserInfo` preservados.
- [x] FR-005 — rastreabilidade em STATE/artefatos.

## Critérios de sucesso

- [x] SC-001 — `grep` zero ocorrências.
- [x] SC-002 — gates do alvo verdes; pisos/jobs intactos.
- [x] SC-003 — diff restrito aos 11 arquivos + artefatos.

## Portarias

- [x] R1 `go (scoped)` antes do BLOCO 6; R3 zero perguntas; R4 validate antes de release; R7 release com SemVer/changelog/rollback.
- [x] `analyze.md` READ-ONLY; `converge.md` append-only.
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`.
