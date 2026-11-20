# Converge: 084 — deduplicação de dependências lucide (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20T00:02Z — entrada 1

- **Decisão 80/20** (`.specify/assessments/084-dedupe-lucide/decision.md`,
  2026-11-19T23:47Z): `E=(1×5)/(2×1)=2,50` — valor 1 (remove dependência
  deprecada e duplicada, sem ganho funcional novo), confiança 5 (3 imports
  colados, 2 lockfiles, campo de deprecation, exports e 13/13 `iconNode`s
  idênticos nos pacotes instalados, registry consultado), esforço 2 (3 imports +
  `package.json` + 2 lockfiles), risco 1 (peer `svelte ^5` compatível; resíduo
  visual **[NV]**). Veredito **go (scoped), opção A**: trocar os 3 imports de
  `'lucide-svelte'` para `'@lucide/svelte'` preservando os 14 símbolos, remover
  `"lucide-svelte": "^1.0.1"` de `dependencies` e fixar
  `"@lucide/svelte": "1.33.0"` (sem `^`) em `devDependencies`. Alternativas
  afastadas: flag OIDC 15,0 (080), `.env.example` 10,0 (081), `cssVariables`
  5,0 (082), docs 5,0 (083), OTLP 1,25. Registry em 2026-11-19T23:47Z informa
  latest `1.76.0`; Main confirmou manter o pin contratual `1.33.0` neste ciclo
  — upgrade futuro registrado como **[NV]**.
- **Evidência determinística consolidada (orchestrator)**, base `9da1aaea6`,
  branch `feature/084-dedupe-lucide`:
  - `git diff` nos fontes → exatamente 3 linhas trocadas, uma por import:
    `apps/frontend/src/lib/components/chat/ChatLayout.svelte:10` (`Send`,
    `Sparkles`), `apps/frontend/src/lib/components/search/SearchBox.svelte:2`
    (`Search`) e `apps/frontend/src/routes/(app)/+layout.svelte:19` (11 símbolos,
    fechamento do bloco multilinha). Named imports e os 14 símbolos intactos;
    nenhum rename para `XIcon`.
  - `grep -rn "lucide-svelte" apps/frontend/src` → **zero** ocorrências;
    `grep -rn "from '@lucide/svelte'" apps/frontend/src` → exatamente 3 linhas
    (uma em cada arquivo). Os 4 componentes UI com subpath `/icons/*`
    (`dialog-content.svelte`, `dropdown-menu-checkbox-item.svelte`,
    `dropdown-menu-radio-item.svelte`, `dropdown-menu-sub-trigger.svelte`)
    permanecem byte-idênticos ao base — não editados (FR-001, SC-001).
  - `apps/frontend/package.json:22` → `"@lucide/svelte": "1.33.0"` sem caret,
    em `devDependencies`; a entrada `lucide-svelte` saiu de `dependencies`
    (`:53`) e `bits-ui`, `clsx`, `tailwind-merge` seguem intactos (FR-002,
    FR-003, SC-002).
  - Lockfiles sincronizados: `bun.lock` mantém `@lucide/svelte@1.33.0` e não
    registra mais o pacote legado; `package-lock.json` perde a entrada
    `node_modules/lucide-svelte`, mantém `node_modules/@lucide/svelte` em
    `1.33.0` com `"dev": true` preservado (FR-004).
  - Fronteira do diff e gates determinísticos (`bun run check`, `bun run lint`,
    `bun run test:unit -- --run`, `bun run build`,
    `python3 scripts/ci/check_gate_integrity.py`, auditoria de
    `git diff --name-only` e zero PRs) **verificados deterministicamente pelo
    main agent no handoff**; este agente não os reexecutou (FR-005, SC-003,
    SC-004).
- **Analyzer** (`specs/084-dedupe-lucide/analyze.md`, READ-ONLY,
  2026-11-19T23:55:33Z, verificador `/tmp/flora-sdd-084/analyze_084.py`):

  ```
  {"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}
  exit=0
  ```

  `go=True` na 1ª execução; FR-001..FR-005 e SC-001..SC-004 em `plan.md`/
  `tasks.md`; T001..T005 únicos e sequenciais; checklist revisado (15 itens
  `[x]`); `[NV]` e `go (scoped)` presentes; nenhuma correção de fonte
  necessária. Analyze não alterou dependências ou código.
- **Verificador isolado `LucideVerifier`**: **PASS-WITH-NITS** — migração
  correta nos 3 imports, no pin `1.33.0`, nos lockfiles, no grep zero e na
  fronteira de paths; nenhuma mudança de comportamento, API ou contrato de
  runtime detectada. **Nit**: `npm install --package-lock-only` injetou churn
  não relacionado em `apps/frontend/package-lock.json` (~105 linhas: entrada
  `shadcn-svelte@1.8.1`, `cn`/`commander`/`node-fetch-native` e 6 blocos
  wasm/@emnapi/tslib), fora do escopo contractual de deduplicação. **Correção
  em paralelo**: delegada a `LockFix`, que remove somente os blocos de churn
  (CUTs em `package-lock.json`) preservando as 3 mudanças contratadas —
  remoção de `lucide-svelte` de `dependencies` e da entrada
  `node_modules/lucide-svelte`, e pin de `@lucide/svelte@1.33.0` em
  `devDependencies`. Checagem somente leitura às 2026-11-20T00:02Z já não
  encontra os marcadores `shadcn-svelte`, `cn`, `commander` ou
  `node-fetch-native` no lockfile; fechamento formal do nit sujeito à
  validação final de JSON e do contrato pelo main agent.
- **Dívidas e [NV]**:
  - **Regressão visual em browser**: os 13 `iconNode`s comparados são idênticos
    entre `1.0.1` e `1.33.0`, mas não houve captura de tela nem inspeção
    visual; gates não detectam aparência de ícones **[NV]**.
  - **Grafo do bundler**: dual package residual não inspecionado após a troca
    dos 3 imports **[NV]**.
  - **Upgrade `@lucide/svelte@1.76.0`**: latest informado pelo registry em
    2026-11-19T23:47Z, divergente do pin `1.33.0` mantido por decisão de Main;
    subida exigiria nova avaliação de risco visual e de escopo **[NV]**.
  - **CI / PR / Forgejo / produção**: zero PRs no snapshot; status de CI ao
    vivo, merge, proteção de branch, aprovações e deploy não verificados
    neste ciclo **[NV]**.
  - **Reexecução local de gates/formatters**: não executada por este agente;
    cobertura determinística atribuída ao main agent **[NV]** neste registro.
  - **Fora de escopo**: 4 componentes UI já migrados, `prettier.config.js`,
    gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs,
    constituição, STATE, `specs/083-*`, DB/rede, segredos, `node_modules/`,
    `.tokens/`, fechamento de issues, leg RBAC e worktrees alheios — nenhum
    tocado neste ciclo.
- **Próxima portaria**: emitir `validate.md` (R4) e `release.md` (R7) em
  `.specify/validations/084-dedupe-lucide/` e
  `.specify/releases/084-dedupe-lucide/`, respectivamente, com R4 **APROVADO**
  e SemVer proposto `0.3.4 → 0.3.5` (patch — deduplicação mecânica de
  dependência deprecada, sem mudança de API, comportamento ou contrato de
  runtime; a branch 083 entregou `0.3.3 → 0.3.4`; ressalva: maintainer pode
  optar por minor `0.4.0`, dado o precedente 079 de mudança de superfície de
  dependência). Rollback dos 6 paths rastreados:
  `git checkout 9da1aaea6 -- apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json apps/frontend/src/lib/components/chat/ChatLayout.svelte apps/frontend/src/lib/components/search/SearchBox.svelte 'apps/frontend/src/routes/(app)/+layout.svelte'`.
  Nenhuma tag, deploy ou merge executados neste ciclo — PR a critério do
  maintainer.

## Resultado

**Converged — a implementação satisfaz spec, plan, tasks e analyzer.** O
verificador `LucideVerifier` registrou PASS-WITH-NITS; o único nit (churn não
relacionado no `package-lock.json`) está em correção paralela por `LockFix`,
sem alterar as 3 mudanças contratadas. `tasks.md` e `analyze.md` permanecem
byte-for-byte inalterados (analyze READ-ONLY). Registro criado apenas para
rastreabilidade; gates, build, testes, formatters e git não foram reexecutados
por este agente.

converge written: 1 file; gates/formatters skipped
