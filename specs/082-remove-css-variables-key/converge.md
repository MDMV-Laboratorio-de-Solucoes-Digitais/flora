# Converge: 082 — remoção da chave `cssVariables` de `components.json`

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-19T22:54Z — entrada 1

- **Decisão 80/20** (`.specify/assessments/082-remove-css-variables-key/decision.md`,
  2026-11-19T22:31Z): `E=(2×5)/(1×1)=5,0` — valor 2 (elimina chave inválida sob
  validação estrita do schema, sem mudar comportamento), confiança 5 (evidência
  local integral: linha colada, schema oficial com URL e data de acesso, parser
  Zod `$strip` do CLI 1.8.1 instalado, varredura de consumidores zero), esforço 1
  (1 linha removida + 1 vírgula ajustada em 1 arquivo), risco 1 (nenhum
  runtime/gate/CI lê a chave; o CLI já a descarta em silêncio). Alternativas
  afastadas: flag OIDC morta 15,0 (feita em 080) · `.env.example` OIDC 10,0
  (feita em 081) · `cssVariables` 5,0 (esta) · lucide 2,50 · OTLP 1,25; leg RBAC
  bloqueada por maintainer **[NV]**. Veredito **go (scoped), opção A**: em
  `apps/frontend/components.json`, remover `:7` (`"cssVariables": true`) e trocar
  `"baseColor": "zinc",` por `"baseColor": "zinc"` na linha `:6` — sem tocar em
  `style`, `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`,
  `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate, CI, DB,
  rede ou segredos. `style` permanece porque está declarado no schema oficial e
  apenas deprecado em v4 (`specs/078-shadcn-config-paths` preservou a mesma
  decisão por escopo mínimo).
- **Evidência determinística consolidada (orchestrator)**, base `8f24519fe`,
  branch `feature/082-remove-css-variables-key`, gate PASS, zero PRs:
  - `git diff -- apps/frontend/components.json` = 1 path, `+1/−2`: sai a linha
    `:7` (`"cssVariables": true`) e a vírgula de `:6`
    (`"baseColor": "zinc",` → `"baseColor": "zinc"`). `:2` (`$schema`), `:3`
    (`"style": "default"`), `:5` (`tailwind.css`), o valor `zinc` e o bloco
    `aliases` (`:9-14`) permanecem byte-idênticos ao base.
  - `python3 -c` com `json.load` em `apps/frontend/components.json` →
    `set(obj["tailwind"]) == {"css","baseColor"}` é `True` e `cssVariables` está
    ausente em todos os níveis do objeto carregado (SC-001).
  - `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` →
    nenhuma correspondência em `apps/frontend/components.json` nem em
    código/script/CI; restam só os registros históricos em
    `.specify/memory/STATE.md:626,644` e `specs/078-*` /
    `.specify/assessments/078-*` (SC-002).
  - `cd apps/frontend && bunx prettier --check components.json` → exit 0 (tabs,
    aspas duplas, sem trailing comma — `prettier.config.js:5-7`);
    `bun run check && bun run lint && bun run test:unit -- --run && bun run build`
    → sem regressão, 42 testes green;
    `python3 scripts/ci/check_gate_integrity.py` → gate PASS. Nenhum desses
    comandos lê `cssVariables` em runtime (SC-003).
  - `git diff --name-only` lista somente `apps/frontend/components.json`, os 5
    artefatos `.specify/assessments/082-remove-css-variables-key/`
    (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) e os
    documentos do ciclo em `specs/082-remove-css-variables-key/`; zero mutação de
    gate, CI, scripts, workflows, lockfiles, `node_modules/`, ADRs, specs
    numeradas, DB, rede ou segredos; zero PRs (SC-004).
- **Analyzer** (`analyze.md`, READ-ONLY, 2026-11-19T22:41:36Z, verificador
  `/tmp/flora-sdd-082/analyze_082.py`):

  ```
  {"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
  exit=0
  ```

  `go=True` na 1ª execução; FR-001..FR-004 e SC-001..SC-004 em `plan.md`/`tasks.md`,
  T001..T004 únicos e sequenciais; checklist revisado (14 itens `[x]`), `[NV]` e
  `go (scoped)` presentes. `errors=[]`; nenhuma correção de fonte necessária;
  Analyze não alterou arquivos de configuração.
- **Verificador isolado `CssVerifier`**: **PASS** sem findings — remoção exata de
  `:7` e da vírgula de `:6`, `tailwind` com exatamente `{css, baseColor}`,
  `style`/`$schema`/`aliases` preservados byte-idênticos, diff restrito a
  `apps/frontend/components.json` (`+1/−2`) e risco funcional em runtime nulo (o
  CLI descarta a chave via `$strip`). Nenhuma nit registrada.
- **Dívidas e [NV]**:
  - **Validação online do schema**: validação do arquivo editado contra
    `https://shadcn-svelte.com/schema.json` não executada — exigiria rede + edição;
    a decisão se sustenta no schema colado (URL e data de acesso 2026-11-19:
    `tailwind.properties = {css, baseColor, config}`, `required =
    ["css","baseColor"]`, `additionalProperties = false`, nenhum `cssVariables`
    em qualquer nível) e no parser `$strip` local do CLI 1.8.1 **[NV]**.
  - **CI e produção**: status de CI ao vivo, PRs, Forgejo, produção, merge, tag,
    deploy e fechamento de issues não verificados neste ciclo (zero PRs, sem
    merge/deploy/tag) **[NV]**.
  - **Leg RBAC / aprovação maintainer**: leg RBAC bloqueada; qualquer mudança em
    ADRs, constituição, `FLOORS` ou workflows depende de aprovação maintainer —
    ciclo separado, fora deste escopo **[NV]**.
  - **Consumidores externos do `components.json`**: repos/ferramentas fora do
    checkout podem esperar `cssVariables`; nenhum leitor oficial depende da chave
    (`schema-Bq2uExa4.d.mts:410-414`, `$strip`) — consumo externo não inferível
    localmente **[NV]**.
  - **Registros históricos**: `.specify/memory/STATE.md:626,644`,
    `specs/078-*` e `.specify/assessments/078-*` conservam menções a
    `cssVariables` — fora do escopo por SC-002/SC-004, não alterados **[NV]**.
- **Próxima portaria**: emitir `validate.md` (R4) e `release.md` (R7) em
  `.specify/validations/082-remove-css-variables-key/` e
  `.specify/releases/082-remove-css-variables-key/`, respectivamente, com SemVer
  proposto `0.3.2 → 0.3.3` (a branch 081 entregou `0.3.1 → 0.3.2`) e rollback de
  1 path: `git checkout 8f24519fe -- apps/frontend/components.json`. Nenhuma tag,
  deploy ou merge executados neste ciclo — PR a critério do maintainer.

## Resultado

**Converged — a implementação satisfaz spec, plan, tasks e analyzer.** O
verificador `CssVerifier` registrou PASS sem findings. Nenhuma tarefa foi anexada
a `tasks.md` (permanece byte-for-byte inalterado); `analyze.md` continua
READ-ONLY. Registro criado apenas para rastreabilidade; gates, build, testes,
formatters e git não foram reexecutados por este agente.

converge written: 1 file; gates/formatters skipped
