# Quality Checklist: 072 — refresh verificado de estado

> Unit tests for requirements writing. Itens gerados sem marcação; a revisão abaixo os avalia.

## Requisitos

- [x] FR-001 — os quatro documentos e as seis contagens estão definidos com âncora explícita?
- [x] FR-002 — os ciclos 069/070 e o intervalo 001–070 estão cobertos sem reabrir 137/137?
- [x] FR-003 — a invariante append-only é verificável por comando?
- [x] FR-004 — todas as afirmações remotas são `[NV]`?
- [x] FR-005 — STATE.md e index.md da raiz estão na rastreabilidade?

## Critérios de sucesso

- [x] SC-001 — contagens e apêndice datados verificáveis por `grep`/`find`?
- [x] SC-002 — fronteira de paths explícita e verificável por `git diff --name-only`?
- [x] SC-003 — ausência de deleções nos quatro documentos verificável por `git diff --numstat`?
- [x] SC-004 — gate do alvo inalterado e PASS?

## Portarias

- [x] R1 `go (scoped)` antes de `specify`.
- [x] R3 no máximo 3 perguntas, uma por vez (aqui: zero).
- [x] R4 validate aprovado antes de release.
- [x] R7 SemVer + changelog por IDs + rollback.
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`.
- [x] Código/CI/worktree 071 fora de escopo.
