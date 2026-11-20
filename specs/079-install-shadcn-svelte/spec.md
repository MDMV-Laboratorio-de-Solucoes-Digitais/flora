# Feature Specification: instalação pinada do CLI `shadcn-svelte` — 079

**Feature Branch**: `feature/079-install-shadcn-svelte`
**Created**: 2026-11-19T20:58Z
**Status**: Approved (`decision.md` — go scoped, opção A)
**Input**: "Instalar shadcn-svelte" — pedido explícito do usuário, tratado como autorização para declarar o CLI como devDependency local e reproduzível em `apps/frontend`, sem executar `init`/`add`/`apply`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — CLI local com versão exata (Priority: P1)

Como maintainer ou agente que precisa executar `shadcn-svelte` no frontend, quero que o bin esteja declarado em `devDependencies` com a versão exata `1.8.1`, para que não dependa de `bun x shadcn-svelte@latest` (rede e versão flutuante) e o toolchain seja reproduzível a partir do lockfile.

**Why this priority**: é o cerne do ciclo. `grep -rn "shadcn"` em `apps/frontend/package.json` e `apps/frontend/bun.lock` retorna zero correspondências: o CLI é hoje efêmero. O registry (npm + bun, 2026-11-19) reporta `dist-tags.latest = 1.8.1`, licença MIT e bin `shadcn-svelte`; `1.8.1` foi publicado em 2026-11-11T21:25Z. A ausência de pinagem permitiria que uma versão futura alterasse a saída do gerador e invalidasse os 51 componentes já commitados.

**Independent Test**: executar `bun pm ls shadcn-svelte` em `apps/frontend` e confirmar `shadcn-svelte@1.8.1` no workspace; em paralelo, `grep -n '"shadcn-svelte"' apps/frontend/package.json apps/frontend/bun.lock` confirma a versão exata, sem `^` nem `~`.

**Acceptance Scenarios**:

1. **Given** `apps/frontend/package.json:19-48` sem entrada `shadcn-svelte`, **When** `bun add -D -E shadcn-svelte@1.8.1` é executado em `apps/frontend`, **Then** `devDependencies` contém `"shadcn-svelte": "1.8.1"` — exato, sem caret ou tilde — e nenhuma dependência de runtime é alterada.
2. **Given** a versão `1.8.1` declarada, **When** `bun pm ls shadcn-svelte` é executado no workspace do frontend, **Then** a saída reporta `shadcn-svelte@1.8.1` e o valor é idêntico ao registrado em `bun.lock`.

---

### User Story 2 — Bin resolvido localmente e lockfile consistente (Priority: P1)

Como maintainer ou agente que invoca o CLI, quero que `bun run shadcn-svelte --help` funcione sem `bun x` e que `bun install --frozen-lockfile --dry-run` termine sem produzir diff, para que a resolução seja local, rastreável e estável em CI.

**Why this priority**: o valor da instalação se concretiza apenas se o bin existir no projeto e o lockfile espelhar `package.json`. `apps/frontend/bun.lock` usa `lockfileVersion: 2` e o workspace `""` replica as listas de dependências; a entrada de `shadcn-svelte` deve incluir as 4 transitivas (`cn`, `postcss`, `commander`, `node-fetch-native`). Sem isso, a CI permaneceria dependente de re-resolução de versão.

**Independent Test**: em `apps/frontend`, `bun run shadcn-svelte --help` lista `init`, `add`, `apply` e `registry build` (bin local); `bun install --frozen-lockfile --dry-run` termina com exit 0 e sem alterar `bun.lock`; `grep` confirma a entrada do pacote e as 4 transitivas no lockfile.

**Acceptance Scenarios**:

1. **Given** o pacote instalado em `apps/frontend/node_modules`, **When** `bun run shadcn-svelte --help` é executado, **Then** o comando é resolvido pelo bin local (sem `bun x`) e exibe os subcomandos `init`, `add`, `apply` e `registry build`; nenhum deles é executado neste ciclo.
2. **Given** `package.json` e `bun.lock` atualizados, **When** `bun install --frozen-lockfile --dry-run` roda em `apps/frontend`, **Then** não há diff de lockfile nem mudança em `node_modules` rastreado, e `bun run check`, `bun run lint` (com `prettier --check .`), `bun run test:unit` e `bun run build` seguem sem regressão.

---

### User Story 3 — Diff estreito, artefatos intactos e reversível (Priority: P2)

Como maintainer, quero que a mudança fique restrita a `apps/frontend/package.json` (1 linha) e `apps/frontend/bun.lock`, além dos artefatos do ciclo 079, com os gates do alvo em PASS e nenhuma mutação de DB, rede (fora do registry), segredos ou arquivos do ciclo 078, para que o ciclo seja auditável e reversível pela reversão dos 2 paths.

**Why this priority**: FR-003, FR-004 e FR-005 e SC-004 são a cerca do ciclo. Um toolchain de geração não justifica reescrever `components.json`, o CSS global ou os 51 componentes — a documentação oficial descreve `init` como inicialização de projeto novo que sobrescreve o CSS global, recria `components.json` e reinstala componentes. A formatação local do Prettier e a integridade do gate precisam permanecer intactas.

**Independent Test**: `git diff --name-only ea5cd2069..HEAD` contém somente `apps/frontend/package.json`, `apps/frontend/bun.lock` e os artefatos de `specs/079-install-shadcn-svelte/`; `git diff --exit-code ea5cd2069 -- apps/frontend/components.json apps/frontend/src/routes/layout.css apps/frontend/src/lib/utils.ts apps/frontend/src/lib/components/ui` confirma byte-identicidade; `python3 scripts/ci/check_gate_integrity.py` (raiz, sem DB/rede) permanece PASS.

**Acceptance Scenarios**:

1. **Given** as edições aplicadas, **When** o diff é auditado contra a base `ea5cd2069`, **Then** `package.json` tem exatamente 1 linha adicionada em `devDependencies`, `bun.lock` registra a resolução, e `components.json`, `layout.css`, `utils.ts`, os 51 `.svelte`, `prettier.config.js`, `vite.config.ts`, gate/scripts/workflows, ADRs, constituição, STATE, DB/rede e `.tokens/` permanecem byte-idênticos e intocados.
2. **Given** os gates determinísticos do alvo, **When** executados sem DB e sem rede (exceto o registry já autorizado), **Then** permanecem PASS, sem credenciais ou segredos novos, e a reversão dos 2 paths (`package.json` + `bun.lock`) restaura o estado original sem force-push ou reescrita de histórico.

---

### Edge Cases

- **Execução de `bun add -D -E shadcn-svelte@1.8.1`**: fora da avaliação (escopo somente leitura); é o passo de implementação autorizado por `decision.md`. Sua execução e a dos comandos de verificação são **[NV]** nesta spec.
- **Versão flutuante (`@latest`) ou caret (`^1.8.1`)**: rejeitados. `@latest` repete o problema atual; o caret (opção B) permitiria minor/patch futuro alterar a saída do CLI. A pinagem `-E` é intencional, embora as demais devDependencies usem caret — **[NV]** para upgrade futuro de versão.
- **`init`, `add` ou `apply` "para validar"**: deliberadamente não executados; sobrescreveriam `src/routes/layout.css` e `components.json` e poderiam reinstalar os 51 componentes (docs oficiais, 2026-11-19). São rabbit holes registrados em `concept.md`.
- **Instalação na raiz em vez de `apps/frontend`**: fora de escopo; o CLI é ferramenta do workspace do frontend e a resolução de bin deve ocorrer ali.
- **Formatação de `package.json`**: `bun add` pode reescrever o arquivo; `prettier.config.js:5-8` exige tabs e `trailingComma: 'none'`. `singleQuote: true` vale para JS/TS, não para JSON, onde aspas duplas são obrigatórias. Conferir com `bun run lint` (`prettier --check .`).
- **Tag `next` (`1.0.0-next.19`)**: dist-tag separada no registry; não é `latest` e está fora deste ciclo.
- **CI ao vivo, merge e produção**: status não verificado nesta spec; `.github/workflows/ci.yml:21-30` roda `bun install` sem `--frozen-lockfile`, mas o lockfile é rastreado e vai commitado junto.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: declarar `"shadcn-svelte": "1.8.1"` (exato, sem `^` e sem `~`) em `devDependencies` de `apps/frontend/package.json`, gerado por `bun add -D -E shadcn-svelte@1.8.1` executado em `apps/frontend` (`latest = 1.8.1` no registry em 2026-11-19).
- **FR-002**: registrar a resolução em `apps/frontend/bun.lock` (entrada do pacote + 4 deps transitivas `cn`, `postcss`, `commander`, `node-fetch-native`), mantendo lockfile rastreado e consistente com `package.json`.
- **FR-003**: preservar byte-idênticos `$schema`, `style`, `tailwind` e `aliases` de `apps/frontend/components.json:1-15`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/` e `src/lib/utils.ts`; não executar `init`, `add` nem `apply` (docs oficiais: `init` sobrescreve o CSS global, recria `components.json` e reinstala componentes).
- **FR-004**: manter a formatação de `package.json` conforme `prettier.config.js:5-8` (tabs, sem trailing commas; aspas duplas obrigatórias em JSON) e não introduzir segredos, `.tokens/`, mudanças de gate/scripts/workflows nem mutação de DB/rede fora do registry.
- **FR-005**: limitar o diff rastreado a `apps/frontend/package.json` + `apps/frontend/bun.lock` (além dos 5 artefatos 079); `node_modules/` permanece ignorado e local.

### Key Entities

N/A — a mudança é de declaração de dependência de ferramenta (JSON + lockfile); nenhuma entidade de dados, schema de banco ou migração é introduzida. O "bin `shadcn-svelte`" é o contrato observável, não uma entidade persistida.

## Success Criteria *(mandatory)*

- **SC-001**: `bun pm ls shadcn-svelte` reporta `shadcn-svelte@1.8.1` no workspace `apps/frontend`; `grep` em `package.json`/`bun.lock` confirma a versão exata, sem caret. `[NV]` depende da instalação.
- **SC-002**: `bun run shadcn-svelte --help` (em `apps/frontend`) lista `init`, `add`, `apply` e `registry build`: bin resolvido localmente, sem `bun x`. `[NV]` depende da instalação.
- **SC-003**: `bun install --frozen-lockfile --dry-run` em `apps/frontend` termina sem diff de lockfile; `bun run lint` (inclui `prettier --check .`) e `bun run check` sem regressão de formatação/tipo. `[NV]` depende da instalação.
- **SC-004**: `git diff --stat` mostra só `apps/frontend/package.json` (1 linha adicionada) e `apps/frontend/bun.lock`, mais os 5 artefatos 079; gate do alvo PASS; zero mutação de DB/segredos; `components.json` e os 51 componentes byte-idênticos ao base `ea5cd2069`.

## Scope / Out of scope

**Dentro**: `apps/frontend/package.json` (1 linha em `devDependencies`) e `apps/frontend/bun.lock` (entrada do pacote + 4 transitivas), os artefatos do ciclo 079 (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`) e os 5 artefatos de assessment já registrados em `.specify/assessments/079-install-shadcn-svelte/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`).

**Fora**: `components.json`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/`, `src/lib/utils.ts`, `prettier.config.js`, `vite.config.ts`; gate/scripts/workflows (inclui `scripts/ci/check_gate_integrity.py`), `.github/workflows/`, supply-chain; ADRs, constituição, `.specify/memory/STATE.md`; `specs/079-*` além dos artefatos aqui listados; DB/rede fora do registry, segredos, `.tokens/`; fechamento de issues, leg RBAC, upgrade de versão, tag `next` e execução de `init`/`add`/`apply`.

## Assumptions

- O pedido do usuário é autorização explícita para instalar; `shadcn-svelte` é ferramenta de geração, não dependência de runtime ⇒ devDependency.
- `dist-tags.latest = 1.8.1`, licença MIT, bin `shadcn-svelte` e 4 transitivas (`cn ^0.3.0`, `postcss ^8.5.10`, `commander ^14.0.0`, `node-fetch-native ^1.6.4`) foram confirmados no registry npm/bun em 2026-11-19; a publicação de `1.8.1` é de 2026-11-11T21:25Z.
- `components.json:1-15` já está correto desde o ciclo 078 (`tailwind.css = src/routes/layout.css`, sem a chave deprecada `config`); `src/lib/utils.ts` já tem `cn` e existem 51 componentes em `src/lib/components/ui/`.
- `init` é para projeto novo e sobrescreve o CSS global, recria `components.json` e reinsta componentes; `components.json` é opcional e só exigido pelo CLI (docs oficiais, 2026-11-19).
- A base de comparação do diff é `ea5cd2069` (HEAD da branch `feature/079-install-shadcn-svelte` no início do ciclo); o único acesso de rede autorizado é o registry para a instalação.

## [NV]

- Execução de `bun add -D -E shadcn-svelte@1.8.1` e dos comandos de verificação (`bun pm ls shadcn-svelte`, `bun run shadcn-svelte --help`, `bun install --frozen-lockfile --dry-run`, `bun run check`, `bun run lint`, `bun run test:unit`, `bun run build`, `python3 scripts/ci/check_gate_integrity.py`) — dependem da instalação; nenhuma bloqueia o `go` (decision.md).
- `init`/`add`/`apply` deliberadamente não executados: sobrescreveriam CSS e `components.json` e poderiam reinstalar os 51 componentes (docs oficiais, 2026-11-19).
- Upgrade futuro de versão, dist-tag `next` (`1.0.0-next.19`) e adoção de caret como nas demais devDependencies — fora deste ciclo; a pinagem `-E` é decisão registrada em `concept.md`.
- Status de CI ao vivo, merge, PRs, Forgejo e produção não são verificados nesta spec; `.github/workflows/ci.yml` roda `bun install` sem `--frozen-lockfile`, mas o lockfile é rastreado.
