# Converge: 079 — instalação pinada do CLI `shadcn-svelte`

**Data:** 2026-11-19T21:18Z · **Tipo:** append-only; registro inicial · **Baseline:** `ea5cd2069` · **Branch:** `feature/079-install-shadcn-svelte` · **Gate:** PASS · **PRs:** 0

## Decisão

- `go (scoped)` — opção A registrada em `.specify/assessments/079-install-shadcn-svelte/decision.md` (2026-11-19T20:57Z): em `apps/frontend`, `bun add -D -E shadcn-svelte@1.8.1`; sem `init`/`add`/`apply`.
- 80/20: `E=(3×5)/(2×1)=7,50` (valor 3 — toolchain local reproduzível; confiança 5 — registry + docs + estado local; esforço 2 — 1 linha + lockfile; risco 1 — diff mínimo). Alternativas afastadas: leg RBAC (bloqueada); Tauri/OTLP (dependem de ADR/maintainer).
- FR-001..FR-005 e SC-001..SC-004 de `define.md`, preservados em `spec.md`/`plan.md`/`tasks.md`; T001..T005 únicos e sequenciais.

## Evidências de fechamento

- `bun add -D -E shadcn-svelte@1.8.1` executado em `apps/frontend`.
- Diff rastreado restrito a `apps/frontend/package.json` (+1 linha em `devDependencies`) e `apps/frontend/bun.lock` (+9 linhas); `node_modules/` permanece ignorado e local.
- `bun pm ls shadcn-svelte` → `shadcn-svelte@1.8.1` (versão exata, sem `^`/`~`).
- `bun run shadcn-svelte --help` → exit 0; subcomandos `init`, `add`, `apply` e `registry build` listados pelo bin local (nenhum executado).
- `bun install --frozen-lockfile --dry-run` → exit 0, sem diff de lockfile.
- `bun run check`, Prettier (`prettier --check .` via `bun run lint`), `bun run lint`, `bun run test:unit` (42/42) e `bun run build` → exit 0, sem regressão.
- `python3 scripts/ci/check_gate_integrity.py` (raiz, sem DB/rede) → PASS; gate do alvo PASS; base de comparação `ea5cd2069`.
- Fontes primárias (2026-11-19): registry npm/bun (`1.8.1`, licença MIT) e docs oficiais (`installation/sveltekit`, `cli`, `components-json`).

## Analyzer

`analyze.md` (READ-ONLY, 2026-11-19T21:03:12Z, verificador `/tmp/flora-sdd-079/analyze_079.py`):

```
{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}
exit=0
```

`go=True` na 1ª execução; FR-001..FR-005, SC-001..SC-004 e T001..T005 presentes em `plan.md` e `tasks.md`; checklist com 15 itens `[x]`, `[NV]` e `go (scoped)` presentes. Nenhuma correção de fonte necessária; Analyze não alterou arquivos de código.

## Verificador — PASS

Verificador isolado `ShadcnVerifier`: **PASS**. `components.json`, `src/routes/layout.css`, os 51 componentes de `src/lib/components/ui/` e `src/lib/utils.ts` byte-idênticos ao base `ea5cd2069`; nenhum `init`/`add`/`apply` executado.

## Dívidas / [NV]

- **CLI não executado além de `--help`**: `init`, `add` e `apply` deliberadamente não rodados — sobrescreveriam `src/routes/layout.css` e `components.json` e poderiam reinstalar os 51 componentes (docs oficiais, 2026-11-19). **[NV]**
- **CI e produção**: status de CI ao vivo, PRs, Forgejo e produção não verificados neste ciclo (zero PRs, sem merge/deploy); `.github/workflows/ci.yml` roda `bun install` sem `--frozen-lockfile`, mas o lockfile é rastreado e vai commitado junto. **[NV]**
- **Leg RBAC / aprovação maintainer**: leg RBAC bloqueada; mudanças em `rust.yml`/`FLOORS`/ADRs dependem de aprovação maintainer — ciclo separado, fora deste escopo. **[NV]**
- **Upgrade futuro de versão**: pinagem `-E` é decisão registrada (as demais devDependencies usam caret); upgrade, dist-tag `next` (`1.0.0-next.19`) e adoção de caret ficam fora deste ciclo. **[NV]**

## Próxima portaria

`validate.md` (R4) → `release.md` (R7) → PR → CI verde ao vivo → merge deliberado em `dev/`.

## Resultado

**Converged — a implementação satisfaz spec, plan, tasks e analyzer.** Nenhuma tarefa foi anexada a `tasks.md` (permanece byte-for-byte inalterado); `analyze.md` continua READ-ONLY. Registro criado apenas para rastreabilidade; gates, build, testes e formatters não foram reexecutados por este agente.
