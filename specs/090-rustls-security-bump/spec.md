# Feature Specification: Bump de segurança rustls 0.23.43 → 0.23.46

**Feature Branch**: `feature/090-rustls-security-bump`

**Created**: 2026-11-20

**Status**: Implementado (evidência e escopo registrados em `tasks.md`)

**Input**: Bump patch de `rustls` no grafo resolvido, de `0.23.43` para
`0.23.46`, para fechar os 4 advisories `RUSTSEC-2026-0285`,
`RUSTSEC-2026-0346`, `RUSTSEC-2026-0348` e `RUSTSEC-2026-0349` sem alterar a
faixa declarada `rustls = "0.23"` em `Cargo.toml:189`. Âncora: branch
`feature/090-rustls-security-bump` @ `824ba00e9`;
`.specify/feature.json` → `specs/090-rustls-security-bump`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Fechar os 4 advisories com um patch de lockfile (Priority: P1)

Como mantenedor, quero que o `Cargo.lock` registre `rustls` `0.23.46` em vez
de `0.23.43`, para que as quatro advisories de segurança sejam fechadas sem
mudar a faixa de compatibilidade declarada no manifest.

**Why this priority**: é a finalidade do ciclo. `0.23.46` satisfaz a correção
de todas as advisories (duas medium e duas low) e `Cargo.toml:189` já admite
a faixa `0.23`, então nenhuma mudança de manifest é necessária.

**Independent Test**: ler a entrada `name = "rustls"` em `Cargo.lock:2748` e
confirmar `version = "0.23.46"` com nenhum `0.23.43` residual; confirmar que
`Cargo.toml:189` permanece `rustls = "0.23"`.

**Acceptance Scenarios**:

1. **Given** `Cargo.lock:2748` com `rustls` `0.23.43`, **When** o bump patch é
   aplicado, **Then** a entrada passa a `version = "0.23.46"` e nenhuma
   ocorrência de `0.23.43` permanece no lockfile.
2. **Given** o manifest na faixa `rustls = "0.23"`, **When** o lock é lido,
   **Then** `Cargo.toml:189` está inalterado e a versão resolvida `0.23.46`
   continua dentro da faixa declarada.

---

### User Story 2 — Alinhar a isenção cargo-vet à versão corrigida (Priority: P2)

Como auditor de supply chain, quero que a isenção `[exemptions.rustls]` aponte
para a mesma versão do lockfile, para que a exceção e o grafo resolvido não
divirjam e o critério `safe-to-deploy` continue explícito.

**Why this priority**: uma isenção desatualizada invalida a rastreabilidade
da auditoria; a isenção já estava em `0.23.46` enquanto o lock ficara em
`0.23.43` — alinhar os dois restaura a consistência verificável.

**Independent Test**: ler `supply-chain/config.toml:1115-1117` e confirmar
`version = "0.23.46"` com `criteria = "safe-to-deploy"` preservado.

**Acceptance Scenarios**:

1. **Given** a entrada `[[exemptions.rustls]]`, **When** a isenção é lida,
   **Then** `version = "0.23.46"` e `criteria = "safe-to-deploy"` estão
   presentes em `supply-chain/config.toml:1115-1117`.
2. **Given** a isenção e o lockfile, **When** ambas as versões são comparadas,
   **Then** são iguais (`0.23.46`) e nenhuma isenção aponta para `0.23.43`.

---

### User Story 3 — Preservar gate, testes e fronteira do diff (Priority: P3)

Como dono do processo de merge, quero que o bump não altere o gate
autoritativo nem amplie o diff, para que a verificação existente continue
sendo a única condição de merge e o registro auditável permaneça intacto.

**Why this priority**: promover verificações novas ou tocar
`validate/`/`release/`/`STATE.md` criaria dependência de infraestrutura e
quebraria o registro do ciclo; o bump é uma mudança de dependência, não de
processo.

**Independent Test**: rodar o gate e os testes registrados no handoff e
listar o diff da branch; conferir que só `Cargo.lock`,
`supply-chain/config.toml` e os 4 artefatos 090 aparecem.

**Acceptance Scenarios**:

1. **Given** a branch 090 sobre `824ba00e9`, **When** o gate
   `scripts/ci/check_gate_integrity.py` e `cargo test --workspace` são
   executados, **Then** o gate fecha PASS (10 legs / 58 contratos, exit 0) e
   os testes terminam com exit 0 e zero falhas.
2. **Given** o diff da branch, **When** os arquivos são listados, **Then**
   aparecem apenas `Cargo.lock`, `supply-chain/config.toml` e os 4 artefatos
   090; zero mudanças em manifest, workflows, ADRs, constituição,
   `validate/`, `release/` ou `STATE.md`.

---

### Edge Cases

- **Residual `0.23.43`**: qualquer ocorrência restante no lockfile invalida
  o bump; a checagem é literal, não por faixa.
- **Conflito de faixa**: se `Cargo.toml:189` deixar de ser `rustls = "0.23"`,
  o escopo é rompido — o manifest não pode ser usado como "correção".
- **Critério de isenção divergente**: perder `safe-to-deploy` ou apontar a
  isenção para `0.23.43` falha a auditoria mesmo com o lock correto.
- **Gate ou teste com falha**: qualquer exit != 0 invalida o fechamento do
  ciclo; não há reexecução por agentes de documentação.
- **Dependências transitivas alteradas**: o bump é patch dentro da faixa
  `0.23`; mudanças de API ou de outras dependências estão fora de escopo e
  devem ser tratadas em ciclo separado.
- **Estado ao vivo [NV]**: CI do PR/merge/produção, aprovações de maintainer
  e proteção de branch não foram reconsultados nesta avaliação documental.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O projeto DEVE manter `rustls = "0.23"` em `Cargo.toml:189`; a
  correção DEVE ser aplicada somente no grafo resolvido do `Cargo.lock`, sem
  alterar a faixa de compatibilidade declarada no manifest.
- **FR-002**: `Cargo.lock:2748` DEVE registrar `rustls` com
  `version = "0.23.46"` em vez de `0.23.43`, sem nenhuma ocorrência residual
  de `0.23.43`, fechando `RUSTSEC-2026-0285`, `RUSTSEC-2026-0346`,
  `RUSTSEC-2026-0348` e `RUSTSEC-2026-0349` com a versão corretiva comum.
- **FR-003**: A isenção `[[exemptions.rustls]]` em
  `supply-chain/config.toml:1115-1117` DEVE passar a
  `version = "0.23.46"`, preservando `criteria = "safe-to-deploy"`, de modo
  que a exceção e o lockfile apontem para a mesma versão auditada.
- **FR-004**: O gate autoritativo `scripts/ci/check_gate_integrity.py` DEVE
  permanecer PASS (oráculo dinâmico do ciclo 088: 10 legs / 58 contratos) e
  `cargo test --workspace` DEVE fechar com exit 0 e zero falhas após o bump.
- **FR-005**: `dependency-review` DEVE permanecer limpo e o diff de código
  DEVE ficar limitado a `Cargo.lock` + `supply-chain/config.toml`, com a
  documentação restrita aos 4 artefatos do ciclo 090; nenhum arquivo em
  `Cargo.toml`, `.github/workflows/`, ADRs, constituição, `validate/`,
  `release/` ou `.specify/memory/STATE.md` pode ser tocado, e nenhum
  PR/merge/tag/deploy/produção faz parte deste ciclo.

### Key Entities

- **Grafo resolvido (`Cargo.lock`)**: registro autoritativo da versão de
  `rustls` efetivamente compilada; aqui é o único ponto de correção.
- **Faixa declarada (`Cargo.toml`)**: requisito `rustls = "0.23"` em
  `Cargo.toml:189`; permanece inalterado porque já admite o patch.
- **Advisories**: `RUSTSEC-2026-0285` (medium, fix `0.23.45`),
  `RUSTSEC-2026-0346` (medium, fix `0.23.46`), `RUSTSEC-2026-0348` (low, fix
  `0.23.46`) e `RUSTSEC-2026-0349` (low, fix `0.23.46`).
- **Isenção cargo-vet**: entrada `[[exemptions.rustls]]` com `version` e
  `criteria = "safe-to-deploy"`, alinhada à versão do lockfile.
- **Gate autoritativo**: `check_gate_integrity.py` (10 legs / 58 contratos,
  ciclo 088) — continua sendo a condição de merge, sem novos required checks.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A leitura literal de `Cargo.lock:2748` retorna `rustls`
  `0.23.46` com zero ocorrências de `0.23.43`, e a leitura de
  `supply-chain/config.toml:1115-1117` retorna `version = "0.23.46"` com
  `criteria = "safe-to-deploy"` — as 4 advisories RUSTSEC listadas são
  cobertas pela versão corretiva e `Cargo.toml:189` segue `rustls = "0.23"`.
- **SC-002**: `python3 scripts/ci/check_gate_integrity.py` fecha **PASS**
  (`all 10 db contract floors intact (58 total)`, exit 0) e
  `cargo test --workspace` fecha com exit 0 e zero falhas após o bump.
- **SC-003**: `dependency-review` termina sem achados e o diff contém apenas
  `Cargo.lock`, `supply-chain/config.toml` e os 4 artefatos 090 — zero
  arquivos em manifest, workflows, ADRs, constituição, `validate/`,
  `release/` ou `STATE.md`. Estados ao vivo permanecem `[NV]` até reconsulta.

## Assumptions

- O `Cargo.lock` raiz é a fonte da verdade da versão compilada de `rustls`;
  o manifest expressa apenas a faixa de compatibilidade.
- `0.23.46` é compatível com a faixa `0.23` e corrige as quatro advisories;
  nenhuma mudança de API ou de outras dependências é necessária.
- A isenção cargo-vet com `criteria = "safe-to-deploy"` permanece a forma
  auditada de registrar a exceção para a versão corrigida.
- O gate do ciclo 088 (10 legs / 58 contratos) continua autoritativo; nenhum
  required check novo é adicionado.
- Evidência de execução e de fronteira do diff é a registrada em `tasks.md`;
  subagentes de documentação não reexecutam gates, testes ou formatters e
  não acessam PR/merge/produção — marcado `[NV]` onde aplicável.
