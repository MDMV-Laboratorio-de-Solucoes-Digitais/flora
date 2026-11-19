# Requirements Quality Checklist — 076

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T12:45Z.

## Completude

- [x] FR-001 define a pré-compilação de `flora-mint-session` com `--features dev-mint` no `globalSetup` do Playwright, antes dos testes.
- [x] FR-002 define que o setup E2E consome o binário já construído, sem disparar `cargo run` que recompile dentro do teste.
- [x] FR-003 preserva `test.setTimeout(180_000)` em `apps/frontend/tests/e2e/seed.setup.ts:14`, sem aumento.
- [x] FR-004 mantém o `execFileSync` da pré-compilação fora da janela de 180s do teste.
- [x] FR-005 exclui alterações em código de produção, workflows e DB compartilhado.

## Clareza e consistência

- [x] SC-001 é verificável: nenhuma compilação ocorre dentro da janela de 180s do teste.
- [x] SC-002 é verificável: o setup E2E conclui sem pendurar com target frio.
- [x] SC-003 é verificável por `git diff --name-only`: diff restrito à config/setup E2E e aos artefatos do ciclo 076.
- [x] SC-004 é verificável: gates do alvo permanecem PASS e zero mutação de DB compartilhado.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004, FR-005 e SC-001, SC-002, SC-003, SC-004 — sem intervalos.
- [x] Evidência local (0.00s / 0.39s com target quente, 30.18s com cache) é distinguida de `[NV]` para tempo com target frio, CI e produção.

## Edge cases

- [x] E se o target estiver frio? A pré-compilação roda no `globalSetup`, fora da janela de 180s do teste.
- [x] E se `cargo run` continuar em `seed.setup.ts:36-53`? Com o binário pré-construído ele apenas executa; o arquivo permanece intacto.
- [x] E se o build falhar no `globalSetup`? `stdio: 'inherit'` expõe a falha e o Playwright aborta antes dos testes.
- [x] E se houver risco de DB compartilhado? Nenhum comando de seed é alterado; a fronteira de paths permanece explícita.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` antes da escrita.
- [x] R3 zero perguntas carry-forward, registradas em `clarify.md`.
- [x] R4 e R7 respeitados na rastreabilidade, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: código, workflows, ADRs, constituição, STATE, worktree 073, issues e leg RBAC.
