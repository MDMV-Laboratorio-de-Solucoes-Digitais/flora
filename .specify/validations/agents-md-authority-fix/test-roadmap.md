# Test roadmap — AGENTS.md authority fix

**Escopo:** correção documental, sem mudança de código, workflow, gate, migration ou credencial. Consumidor: qualquer agente ou desenvolvedor que usa `AGENTS.md` como guia operacional do repositório.

**Pré-condições:** worktree limpa criada de `github/dev` (`ab9e95dead`); nenhum path de outra sessão tocado.

|#|Ação|Resultado esperado|Automatrizável?|Superfície|
|---|---|---|---|---|
|1|`head -1 AGENTS.md; grep -nE '^## ' AGENTS.md`|`# Repository Guidelines` e exatamente 8 headings na ordem normativa|Sim|CLI|
|2|`grep -Ein 'manual-only\|10 legs\|77 pisos\|no deploy on push\|informative only\|There is no .oidc_mock' AGENTS.md`|Nenhuma correspondência|Sim|CLI|
|3|`sed -n '42,54p' scripts/ci/check_gate_integrity.py` e soma dos pisos|11 legs, soma 89; distribuição do texto igual a `FLOORS`|Sim|CLI|
|4|`grep -nE 'push:|workflow_dispatch:' .github/workflows/docker-*.yml`|Os quatro workflows de deploy têm `push` e `workflow_dispatch`|Sim|CLI|
|5|`test -f flora-tests/src/oidc_mock.rs; grep -n 'Ed25519' flora-tests/src/oidc_mock.rs`|Artefato existe e declara JWKS Ed25519|Sim|CLI|
|6|`grep -n 'expect(findings' apps/frontend/tests/e2e/seed-flow.e2e.ts`|Violações axe falham o smoke; o job `e2e-browser` continua advisory|Sim|CLI|
|7|`git diff --check`|RC 0|Sim|CLI|
|8|Required checks no head exato do PR (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`)|Verdes ou skipped permitido (`dependency-review` em PR-only); `mirror`/`build-and-push` são advisory|Sim|GitHub Actions|

**Plano de capturas:** saídas de terminal coladas em `evidence.md`; não há UI alterada por esta mudança, portanto não há captura de tela (justificativa registrada: mudança é puramente documental e não observável na interface).

**Rollback:** revert do commit na branch; nenhum efeito em runtime, CI ou produção.
