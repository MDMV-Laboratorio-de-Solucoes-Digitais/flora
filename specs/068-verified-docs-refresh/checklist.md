# Quality Checklist: 068 — refresh verificado de documentação

> Esta checklist é **revisão de qualidade de requisitos**, não registro de implementação.
> Itens marcados `[x]` significam que o requisito foi revisado e está pronto para execução.

## Requisitos

- [x] FR-001 — contagens do índice têm fonte determinística declarada (`find` da própria linha 4).
- [x] FR-002 — nota do PROGRESSO preserva o histórico e marca **[NV]** o que é remoto/futuro.
- [x] FR-003 — notas de OBJETIVO/ROADMAP são apêndices datados, sem reescrita do corpo.
- [x] FR-004 — comentário do RBAC contradito por código e STATE; sem ambiguidade de autoridade.
- [x] FR-005 — um arquivo por commit e rastreabilidade em STATE/validate/release.

## Critérios de sucesso

- [x] SC-001 — cada número publicado tem comando `find` correspondente em `research.md`.
- [x] SC-002 — gates listados em `plan.md` são os mesmos do gate autoritativo do alvo e do CI (ADR-0003).
- [x] SC-003 — fronteira de paths explícita e verificável por `git diff --name-only`.

## Portarias

- [x] R1 — `decision.md` contém `go (scoped)` antes do BLOCO 6.
- [x] R3 — zero esclarecimentos pendentes (vide `clarify.md`).
- [x] R4 — `validate.md` será aprovado antes de `release.md`.
- [x] R7 — `release.md` conterá SemVer, changelog com FR/T e rollback.
- [x] `analyze.md` é READ-ONLY e será executado após `tasks.md`, antes da implementação.
- [x] `converge.md` é append-only.
- [x] Nenhum comando `/speckit.validate` ou `/speckit.release` será invocado — ambos são artefatos.
