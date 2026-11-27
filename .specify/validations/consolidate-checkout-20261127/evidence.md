# Evidence — consolidação de checkout (2026-11-27)

**Branch:** `fix/consolidate-checkout-20261127` @ `de8578130e` (9 commits desde `b44b53cdea`) · **Base:** `github/dev=b44b53cdea`.

> **Nota de superposição (27/11, ACR round 1 — LOW):** o cabeçalho anterior registrava `c755c1c74a` / 7 commits. Esse registro histórico permanece aqui como superado pelo head final `de8578130e` / 9 commits; não foi apagado nem reescrito.

Varredura de alegações obsoletas — comando corrigido na ACR round 1 (MEDIUM; substitui a forma com alternação escapada, que era vacua e omitia o runbook):

```bash
grep -Ein 'manual-only|deploy é manual|Nenhum workflow dispara|no deploy on push|10 legs|77 pisos|six DB-gated' AGENTS.md README.md docs/governance/deploy-coolify-cloudflare.md
```

| Passo | Comando/ação | Saída observada | Resultado |
|---|---|---|---|
| 1 | `git status --short` na worktree consolidada | Somente `?? .omp/plans/` e `?? libws_lint_probe3.rmeta`; todo trabalho útil está em 9 commits | PASS |
| 2 | `git diff --check` | RC 0 | PASS |
| 3 | `grep -Ein '…' AGENTS.md README.md docs/governance/deploy-coolify-cloudflare.md` (comando exato no bloco acima) | 1 match: `docs/governance/deploy-coolify-cloudflare.md:3` — "**deploy é manual e humano**. Nenhum workflow dispara em `push`..." (achado HIGH da ACR round 1; runbook fora do Target destes dois documentos) | `[EV]` achado aberto |
| 4 | `grep -nE '^## ' AGENTS.md` | 8 headings na ordem exigida (linhas 3, 9, 27, 41, 78, 89, 105, 112) | PASS |
| 5 | `git diff --numstat` | `STATE.md` +96/−0, `PROGRESSO.md` +48/−0: append-only puro; demais arquivos com edições explícitas | PASS |
| 6 | `find .specify/queue/consolidation-archive-20261127 -type f \| wc -l` | 18 arquivos (MANIFEST + 17 fontes); hashes conferidos pelo agente responsável | PASS |
| 7 | `git ls-files .specify/validations \| cut -d/ -f3 \| sort -u \| wc -l` | 119 diretórios; nenhum copiado porque já estavam presentes e idênticos à origem | PASS |
| 8 | `git diff --name-only github/dev..HEAD -- .github/workflows/ scripts/ci/check_gate_integrity.py flora-core/ flora-api/ flora-tests/ apps/frontend/ Cargo.toml` | Nenhum arquivo alterado — branch é somente documentação/config template | PASS |
| 9 | `python3 scripts/ci/check_gate_integrity.py` (3 tentativas locais, inclusive com `CARGO_TARGET_DIR` reaproveitado) | Timeout por rebuild local completo (900 s); **não é RED de política**. Registro autoritativo em 26/11 no head exato: `gate-integrity: PASS`, 11 legs / 89 pisos, RC 0 | `[NV]` local / `[EV]` histórico no head |
| 10 | `python3 scripts/ci/check_deploy_workflows.py` | PASS nas execuções anteriores no mesmo head; reexecução local não concluída pelo mesmo timeout | `[NV]` local / `[EV]` histórico |

**Decisão de venue (0.8):** GitHub Actions é a autoridade; o trabalho é documental e o branch não altera nenhum arquivo que o gate avalia. Não insisti no gate local para não consumir o notebook compartilhado (memória disponível <2 GiB nas tentativas). O PR dependerá dos checks remotos no head exato, que serão observados ao vivo antes de qualquer merge.

**Preservação:** variante manual-only dos quatro workflows e do validador, upgrade Spec Kit 1.4.3 e gate antigo ficam no archive local (não aplicados). Política vigente (`github/dev`, PR #440) e gate 11/89 permanecem intactos. Runbook, INBOX de 21/11, `.omp/plans/` e `libws_lint_probe3.rmeta` foram trazidos para a worktree consolidada; os dois últimos permanecem untracked por serem scratch/artifact.

**[NV]:** CI remoto no head `de8578130e`, aprovação/promoção e efeitos de deploy. **[EV]:** ACR round 1 em 27/11 — `needs_change`, 3 achados (§ abaixo).

## ACR round 1 — 27/11/2026

Veredito: **`needs_change`**, 3 achados. Registro datado e imutável; a correção abaixo não reescreve o histórico.

| # | Sev. | Achado | Disposição em 27/11 |
|---|---|---|---|
| 1 | HIGH | `docs/governance/deploy-coolify-cloudflare.md` reintroduz política manual-only (linhas 3, 14, 21, 89), contradizendo `github/dev` (PR #440) e `README.md:61`. Autoridade vigente: deploy automático por push/merge (`dev` → staging, `main` → produção), `workflow_dispatch` como backup; gate 11 legs / 89 pisos. | Fora do Target destes dois documentos: achado **aberto**, competência do dono do runbook. A varredura corrigida detecta o match em `:3` (passo 3). |
| 2 | MEDIUM | A varredura nos passos 3 de `evidence.md` e `test-roadmap.md` empregava alternação escapada (pipe literal em ERE), portanto vacua, e não incluía o runbook no escopo. | **Corrigido:** alternação ERE real, `-n` e três arquivos, conforme bloco acima e passo 3 de ambos os documentos. |
| 3 | LOW | `evidence.md` fixado em `c755c1c74a` / 7 commits em vez do head final `de8578130e` / 9 commits. | **Corrigido** no cabeçalho, no passo 1 e na nota `[NV]`; o registro anterior permanece preservado na nota de superposição. |

**Correção aplicada nesta rodada:** achados 2 e 3; achado 1 registrado como aberto. Vereditos dos passos 1, 2 e 4–10 inalterados (PASS / `[NV]` / `[EV]` histórico conforme tabela); apenas a contagem de commits do passo 1 foi alinhada ao head final. Nenhuma evidência datada foi apagada.
