# Evidence — AGENTS.md authority fix

**Data:** 2026-11-27 · **Worktree:** `.specify/queue/worktrees/agentsmd-correction-20261127` (limpa, criada de `github/dev@ab9e95dead`) · **Branch:** `docs/agents-md-authority-fix`.

|Passo|Comando/ação|Saída observada|Resultado|
|---|---|---|---|
|1|`head -1 AGENTS.md; grep -nE '^## ' AGENTS.md`|`# Repository Guidelines`; headings nas linhas 3, 9, 27, 41, 78, 89, 105, 112|PASS|
|2|`grep -Ein 'manual-only\|10 legs\|77 pisos\|no deploy on push\|informative only\|There is no .oidc_mock' AGENTS.md`|Nenhuma correspondência|PASS|
|3|`sed -n '42,54p' scripts/ci/check_gate_integrity.py` + soma por `awk`|`legs=11 total=89`; distribuição `auth 25, messaging 5, websocket 9, search 8, notifications 6, files 8, tasks 4, workspace 3, rbac 5, bootstrap 13, hitl 3`|PASS|
|4|`grep -lE '^  push:'` e `grep -lE '^  workflow_dispatch:'` nos quatro `.github/workflows/docker-*.yml`|`4` e `4`|PASS|
|5|`test -f flora-tests/src/oidc_mock.rs; grep -n 'Ed25519'`|Artefato presente; `:4` declara JWKS `Ed25519 OKP`|PASS|
|6|`grep -n 'expect(findings' apps/frontend/tests/e2e/seed-flow.e2e.ts`|`:59 expect(findings, summary).toEqual([])`|PASS|
|7|`git diff --check`|`RC=0`; `AGENTS.md` 5+/5−|PASS|
|8|Required checks no head exato do PR|[NV] até observação ao vivo no GitHub Actions; `dependency-review` é PR-only e `skip` permitido|NV|

Sem capturas de tela: a mudança é exclusivamente documental e não produz efeito observável na interface. Nenhum build, teste Rust, teste frontend ou gate agregado foi executado localmente — a mudança não toca arquivos que esses gates avaliam; a autoridade é o CI remoto no head do PR.
