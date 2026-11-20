# Feature Specification: remover a chave `cssVariables` de `components.json` — 082

**Feature Branch**: `feature/082-remove-css-variables-key`
**Created**: 2026-11-19T22:36Z
**Status**: Approved (`decision.md` — go (scoped), opção A, E=5,0)
**Input**: "remover a chave `cssVariables` de `apps/frontend/components.json`" — decidido em `.specify/assessments/082-remove-css-variables-key/decision.md`; tratado como correção de metadado de config, sem mudança de comportamento nem de contrato de leitura.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — `components.json` válido sob validação estrita do schema (Priority: P1)

Como maintainer ou agente que valida `apps/frontend/components.json` contra o schema oficial do shadcn-svelte, quero que o objeto `tailwind` contenha apenas as chaves declaradas (`css`, `baseColor`), para que a validação estrita (`additionalProperties = false`) não reprove um arquivo que hoje carrega um metadado inexistente.

**Why this priority**: é o cerne do ciclo. O schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19) declara `tailwind.properties = {css, baseColor, config}`, `tailwind.required = ["css","baseColor"]` e `tailwind.additionalProperties = false`; `cssVariables` não existe em nenhum nível. `apps/frontend/components.json:7` declara `"cssVariables": true`, e 078 preservou a chave por escopo mínimo (registrado em `.specify/memory/STATE.md:626`). O CLI instalado `shadcn-svelte@1.8.1` usa Zod `$strip` (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`) e descarta a chave em silêncio — o dano é metadado inválido e enganoso, não comportamento de runtime.

**Independent Test**: `python3 -c` com `json.load` em `apps/frontend/components.json` → `set(obj["tailwind"]) == {"css","baseColor"}` é `True` e `cssVariables` está ausente em todos os níveis (SC-001); nenhuma linha `cssVariables` permanece no arquivo (SC-002). A remoção de `:7` e o ajuste da vírgula em `:6` bastam; nenhum outro arquivo precisa mudar.

**Acceptance Scenarios**:

1. **Given** `tailwind = {css, baseColor, cssVariables}` em `apps/frontend/components.json:4-8`, **When** a linha `:7` é removida e a vírgula de `:6` é ajustada, **Then** `tailwind = {css, baseColor}` e `json.load` confirma `set(...) == {"css","baseColor"}` com `cssVariables` ausente em todos os níveis.
2. **Given** um validador estrito do schema oficial, **When** ele lê o arquivo editado, **Then** não há mais chave `cssVariables` em nenhum nível para ser rejeitada por `additionalProperties = false`; `style` permanece porque está declarado no schema (apenas deprecado em v4).

---

### User Story 2 — Restante do arquivo byte-idêntico e formatação preservada (Priority: P1)

Como maintainer, quero que `$schema`, `style`, `tailwind.css`, o valor de `baseColor` e todo o bloco `aliases` permaneçam byte-idênticos, e que a formatação do Prettier local continue válida, para que a correção não altere nada além da chave inválida.

**Why this priority**: FR-003 e SC-002/SC-003 são a cerca de compatibilidade. O diff permitido é de 1 linha removida + 1 vírgula ajustada em 1 arquivo. Qualquer reindentação, troca de `style`, alteração de `baseColor` (valor) ou reordenação de `aliases` transformaria uma correção de metadado em mudança de config que exige reavaliação. `prettier.config.js:5-7` fixa `useTabs: true`, `singleQuote: true` (apenas JS/TS) e `trailingComma: 'none'` — o JSON deve usar tabs, aspas duplas e nenhuma trailing comma.

**Independent Test**: `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` não retorna `apps/frontend/components.json`; `cd apps/frontend && bunx prettier --check components.json` termina com exit 0; `git diff` do arquivo mostra somente a remoção de `:7` e a vírgula de `:6`.

**Acceptance Scenarios**:

1. **Given** a edição aplicada, **When** o diff é auditado, **Then** `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), `"baseColor": "zinc"` (`:6`, sem vírgula) e `aliases` (`:9-14`) permanecem byte-idênticos ao base `8f24519fe`.
2. **Given** a configuração local do Prettier, **When** `bunx prettier --check components.json` é executado, **Then** exit 0 (tabs, aspas duplas, sem trailing comma), sem reformatar o arquivo.

---

### User Story 3 — Diff estreito, gates determinísticos e reversível (Priority: P2)

Como maintainer, quero que o diff rastreado fique restrito a `apps/frontend/components.json` e aos artefatos do ciclo 082, com gate em PASS e zero mutação de runtime, CI, DB ou segredos, para que o ciclo seja auditável e reversível pela restauração de um único path.

**Why this priority**: FR-004 e SC-003/SC-004 são a fronteira da mudança. O custo é de 1 linha removida + 1 vírgula em 1 arquivo; qualquer efeito colateral — tocar `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate, workflows, lockfiles, specs numeradas — transformaria a correção em mudança de contrato que exige reavaliação. Nenhum comando executado lê `cssVariables` em runtime; o CLI já descarta a chave.

**Independent Test**: `git diff --name-only` lista somente `apps/frontend/components.json` e os artefatos do ciclo (5 de assessment + 6 documentos); `python3 scripts/ci/check_gate_integrity.py` permanece PASS; `cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run && bun run build` segue sem regressão. A restauração de `components.json` ao base devolve `cssVariables` e a vírgula sem force-push.

**Acceptance Scenarios**:

1. **Given** a edição aplicada, **When** o diff é auditado contra `8f24519fe`, **Then** nenhum outro path é listado além de `apps/frontend/components.json` e dos artefatos 082, e nenhum outro arquivo é modificado.
2. **Given** os gates determinísticos, **When** executados sem DB, rede ou segredos, **Then** permanecem PASS e nenhum deles lê `cssVariables` em runtime.

---

### Edge Cases

- **`cssVariables` ausente ou já removido**: FR-001/FR-002 são idempotentes — nada a remover/ajustar e SC-001/SC-002 já satisfeitos. A vírgula de `:6` só é ajustada se a chave existir; sem ela, `"baseColor": "zinc"` já é o estado final.
- **Chave `style`**: não remover. Está declarada no schema (`schema-Bq2uExa4.d.mts:409`; `properties.style` online) e apenas deprecada em v4; removê-la seria mudança de contrato fora do escopo.
- **`config` no `tailwind`**: o schema admite `config` como propriedade opcional; o arquivo atual não a declara e não deve ser adicionada (FR-003 preserva as chaves existentes, SC-001 exige exatamente `{css, baseColor}`).
- **Validadores externos/offline**: repos ou ferramentas fora do checkout podem esperar `cssVariables`; nenhum leitor oficial depende dela (`$strip` no CLI). Consumo externo é `[NV]`, sem inferência além da evidência local.
- **Validação online do arquivo editado**: exigiria rede + edição; a decisão se sustenta no schema colado (URL e data de acesso) e no parser `$strip` local. `[NV]`.
- **Reformatação automática**: qualquer reindentação ou alteração fora de `:6-7` invalida FR-003/FR-004.
- **Worktree/specs alheias e STATE**: `specs/078-*`, `.specify/assessments/078-*` e `.specify/memory/STATE.md` conservam menções históricas a `cssVariables` — fora do escopo (SC-002/SC-004); a varredura de SC-002 distingue esses registros de qualquer ocorrência em `components.json`, código, script ou CI.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: remover a linha `apps/frontend/components.json:7` (`"cssVariables": true`) do objeto `tailwind`. A chave não existe em nenhum nível do schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `tailwind.additionalProperties = false`) e é descartada por `$strip` no parser Zod do CLI instalado (`schema-Bq2uExa4.d.mts:410-414`).
- **FR-002**: ajustar a linha `apps/frontend/components.json:6` de `"baseColor": "zinc",` para `"baseColor": "zinc"` (remover a vírgula que separava `baseColor` da chave agora excluída), preservando o valor `zinc`. Resultado: `tailwind` com exatamente `css` e `baseColor`, JSON válido e sem trailing comma (`prettier.config.js:7`).
- **FR-003**: preservar byte-idênticos `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), o valor de `tailwind.baseColor` (`:6`) e todo o bloco `aliases` (`:9-14`). Manter a formatação do Prettier local: tabs (`prettier.config.js:5`), aspas duplas obrigatórias em JSON (`singleQuote` em `:6` vale para JS/TS) e nenhuma trailing comma (`:7`). **Não remover `style`**: ele está declarado no schema (`schema-Bq2uExa4.d.mts:409`; `properties.style` online) e apenas deprecado em v4.
- **FR-004**: limitar o diff rastreado a `apps/frontend/components.json` (1 linha removida + 1 vírgula ajustada) e aos 5 artefatos `.specify/assessments/082-remove-css-variables-key/`; nenhum outro arquivo de código, config, doc, gate, CI, lockfile ou `node_modules/` é modificado; base `8f24519fe` preservada em todo o resto. Nenhuma dependência nova, segredo ou mutação de DB/rede. Os documentos do próprio ciclo em `specs/082-remove-css-variables-key/` acompanham o diff conforme SC-003.

### Key Entities

N/A — a mudança é uma correção de metadado em `apps/frontend/components.json` (config do shadcn-svelte); nenhuma entidade de dados, schema, migração ou contrato de leitura é introduzido. O objeto `tailwind` passa a ter exatamente `css` e `baseColor`; `style`, `$schema` e `aliases` permanecem byte-idênticos.

## Success Criteria *(mandatory)*

- **SC-001**: `python3 -c` com `json.load` em `apps/frontend/components.json` retorna `set(obj["tailwind"]) == {"css", "baseColor"}` → `True`, e `cssVariables` está ausente em todos os níveis do objeto carregado. `[NV]` depende da edição.
- **SC-002**: varredura determinística por `cssVariables` no checkout (excluindo `node_modules`/`.git`) não retorna `apps/frontend/components.json` nem qualquer arquivo de código/script/CI; apenas os registros históricos já existentes em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*` permanecem. `$schema`, `style`, `baseColor`, `css` e `aliases` seguem byte-idênticos.
- **SC-003**: em `apps/frontend`, `bunx prettier --check components.json` termina com exit 0; `bun run check`, `bun run lint`, `bun run test:unit -- --run` (`package.json:15` define `test:unit` como `vitest`, que exige `--run` para execução única) e `bun run build` seguem sem regressão; `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. Nenhum desses comandos lê `cssVariables` em runtime. `[NV]` não executados nesta avaliação.
- **SC-004**: `git diff --name-only` lista somente `apps/frontend/components.json` e os artefatos 082 — os 5 de assessment em `.specify/assessments/082-remove-css-variables-key/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) e os 6 documentos do ciclo em `specs/082-remove-css-variables-key/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`); zero mutação de gate, CI, scripts, workflows, lockfiles, `node_modules/`, ADRs, specs numeradas, DB, rede ou segredos; zero PRs abertos.

## Scope / Out of scope

**Dentro**: `apps/frontend/components.json` — remoção de `:7` (`"cssVariables": true`) e ajuste da vírgula em `:6` (`"baseColor": "zinc",` → `"baseColor": "zinc"`); os 6 documentos do ciclo em `specs/082-remove-css-variables-key/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`); e os 5 artefatos de assessment em `.specify/assessments/082-remove-css-variables-key/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`).

**Fora**: `style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, `.specify/memory/STATE.md`, `specs/078-*`, specs/082-* (exceto os artefatos do ciclo), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.

## Assumptions

- O schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19) declara `tailwind.properties = {css, baseColor, config}`, `required = ["css","baseColor"]`, `additionalProperties = false` e não contém `cssVariables` em nenhum nível; `style` é declarado e apenas deprecado em v4.
- O CLI `shadcn-svelte@1.8.1` (`apps/frontend/package.json:39`) usa parser Zod com `$strip` (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`): a chave é descartada em silêncio, sem erro de runtime.
- Nenhum consumidor local lê `cssVariables`: varredura determinística sem correspondência em código/script/CI (registros históricos em STATE e 078 permanecem, fora do escopo).
- `prettier.config.js:5-7` fixa `useTabs: true`, `singleQuote: true` (JS/TS) e `trailingComma: 'none'`; o arquivo atual já segue esse formato (tabs, aspas duplas, sem trailing comma).
- A base de comparação do diff é `8f24519fe` (HEAD da branch `feature/082-remove-css-variables-key` no início do ciclo), com gate PASS e zero PRs.
- `.specify/feature.json` é estado local ignorado (`.specify/.gitignore:6`), reescrito a cada troca de feature; hoje registra `specs/082-remove-css-variables-key` (a divergência 081→082 está resolvida em `clarify.md`).

## [NV]

- Execução dos gates determinísticos (item 1 `json.load`, item 2 `grep`, item 3 `bunx prettier --check`, item 4 `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build`, item 5 `python3 scripts/ci/check_gate_integrity.py` e `git diff --name-only`) — fica com o main agent após a edição; não executada nesta avaliação (marcadores em SC-001 e SC-003).
- Validação online do arquivo editado contra `https://shadcn-svelte.com/schema.json` — exigiria rede + edição; a evidência local (`$strip` + schema colado com URL e data de acesso) já sustenta a decisão.
- Consumidores externos do `components.json` (repos fora do checkout podem esperar `cssVariables`) e CI em produção — nenhum leitor oficial depende da chave (`schema-Bq2uExa4.d.mts:410-414`).
- Status de CI ao vivo, merge, PRs, Forgejo, produção e fechamento de issues — não verificados nesta spec; nenhuma inferência além da evidência local e de `decision.md`.
