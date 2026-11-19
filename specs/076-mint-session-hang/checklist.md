# Quality Checklist: 076 — hang do setup E2E do minter de sessão

> Itens gerados sem marcações; `[x]` registra revisão de qualidade de requisitos, não implementação. Revisão em 2026-11-19T12:45Z.

## Requisitos

- [x] FR-001 — pré-compilação de `flora-mint-session` com `--features dev-mint` no `globalSetup` especificada?
- [x] FR-002 — setup E2E consome o binário já construído, sem recompilação dentro do teste?
- [x] FR-003 — `test.setTimeout(180_000)` preservado sem aumento?
- [x] FR-004 — `execFileSync` da pré-compilação fora da janela de 180s do teste?
- [x] FR-005 — código de produção, workflows e DB compartilhado intactos?

## Critérios de sucesso

- [x] SC-001 — ausência de compilação na janela de 180s verificável?
- [x] SC-002 — conclusão sem pendurar com target frio verificável?
- [x] SC-003 — diff restrito à config/setup E2E e aos artefatos 076 por `git diff --name-only`?
- [x] SC-004 — gates do alvo PASS e zero mutação de DB compartilhado?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` antes de `specify`?
- [x] R3 — zero perguntas (carry-forward: nenhuma), registradas em `clarify.md`?
- [x] R4 e R7 — registrados na rastreabilidade, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — código, workflows, ADRs, constituição, STATE, worktree 073, issues e leg RBAC fora de escopo?
- [x] `[NV]` — tempo com target frio, CI e produção separados da evidência local?
