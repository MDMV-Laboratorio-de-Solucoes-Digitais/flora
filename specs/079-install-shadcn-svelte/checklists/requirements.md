# Requirements Quality Checklist — 079

**Feature**: [spec.md](../spec.md) · **Slug**: 079-install-shadcn-svelte
**Created**: 2026-11-19T20:58Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T20:58Z, contra `define.md`/`decision.md` (acessados 2026-11-19). A execução da instalação e dos gates permanece `[NV]`.

## Completude

- [x] FR-001 define a declaração exata de `"shadcn-svelte": "1.8.1"` (sem `^` e sem `~`) em `devDependencies` de `apps/frontend/package.json`, gerada por `bun add -D -E shadcn-svelte@1.8.1` executada em `apps/frontend`; `latest = 1.8.1` no registry npm/bun em 2026-11-19, licença MIT, bin `shadcn-svelte`.
- [x] FR-002 define o registro da resolução em `apps/frontend/bun.lock` (entrada do pacote + 4 deps transitivas `cn`, `postcss`, `commander`, `node-fetch-native`), com lockfile rastreado e consistente com `package.json`.
- [x] FR-003 define a preservação byte-idêntica de `$schema`, `style`, `tailwind` e `aliases` de `apps/frontend/components.json:1-15`, de `src/routes/layout.css`, dos 51 `.svelte` de `src/lib/components/ui/` e de `src/lib/utils.ts`, com proibição explícita de `init`, `add` e `apply` (docs oficiais: `init` sobrescreve o CSS global, recria `components.json` e reinstala componentes).
- [x] FR-004 define a manutenção da formatação de `package.json` conforme `prettier.config.js:5-8` (tabs, sem trailing commas; aspas duplas obrigatórias em JSON) e proíbe segredos, `.tokens/`, mudanças em gate/scripts/workflows e mutação de DB/rede fora do registry.
- [x] FR-005 limita o diff rastreado a `apps/frontend/package.json` + `apps/frontend/bun.lock` (além dos 5 artefatos 079) e mantém `node_modules/` ignorado e local.

## Clareza e consistência

- [x] SC-001 é verificável: `bun pm ls shadcn-svelte` reporta `shadcn-svelte@1.8.1` no workspace `apps/frontend`; `grep` em `package.json`/`bun.lock` confirma a versão exata, sem caret. `[NV]` — depende da instalação.
- [x] SC-002 é verificável: `bun run shadcn-svelte --help` (em `apps/frontend`) lista `init`, `add`, `apply` e `registry build`; bin resolvido localmente, sem `bun x`. `[NV]` — depende da instalação.
- [x] SC-003 é verificável: `bun install --frozen-lockfile --dry-run` em `apps/frontend` termina sem diff de lockfile; `bun run lint` (inclui `prettier --check .`) e `bun run check` sem regressão de formatação/tipo. `[NV]` — depende da instalação. `bun run test:unit` e `bun run build` ficam no plano de verificação do orchestrator, não como portaria SDD.
- [x] SC-004 é verificável: `git diff --stat` mostra só `apps/frontend/package.json` (1 linha adicionada) e `apps/frontend/bun.lock`, mais os 5 artefatos 079; gate do alvo PASS (`python3 scripts/ci/check_gate_integrity.py`); zero mutação de DB/segredos; `components.json` e os 51 componentes byte-idênticos ao base `ea5cd2069`.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004, FR-005 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7); a rastreabilidade por T001..T005 em `plan.md` e `tasks.md` usa cada ID nominalmente.
- [x] Evidência determinística (registry npm/bun com URL e data de acesso 2026-11-19, docs oficiais com URL e data, estado local: `package.json`/`bun.lock` sem `shadcn`, 51 componentes + `cn` + `components.json` corrigido em 078) distinguida de `[NV]` para a execução da instalação e dos comandos de verificação (`bun pm ls shadcn-svelte`, `bun run shadcn-svelte --help`, `bun install --frozen-lockfile --dry-run`, gates e CI).

## Edge cases

- [x] E se `shadcn-svelte@1.8.1` já estiver declarado? FR-001/FR-002 são idempotentes: `bun add -D -E` mantém a versão exata e SC-001/SC-003 já satisfeitos; nenhuma duplicação de chave.
- [x] E se `bun add` gravar `^1.8.1` ou `~1.8.1`? A flag `-E` exige exatidão; SC-001 falha se aparecer caret/til — corrigir antes de aceitar, sem ampliar o diff.
- [x] E se `bun add` reformatar `package.json` além da linha nova? FR-004 exige tabs, sem trailing commas e aspas duplas; `bun run lint` (`prettier --check .`) e `git diff --name-only` detectam qualquer desvio; se houver reordenação de chaves, registrar `[NV]` e reavaliar antes de aceitar.
- [x] E se o CLI executar `init`/`add`/`apply` por acidente? FR-003 proíbe: docs oficiais confirmam sobrescrita de CSS, `components.json` e componentes; a reversão dos 2 paths não restauraria esses artefatos — registrado como risco bloqueante, não como efeito colateral aceitável.
- [x] E se a instalação exigir rede fora do registry ou tocar DB/segredos/`.tokens/`? Fora de escopo por FR-004/FR-005; nenhum comando de DB é executado e a rede é usada apenas pelo registry para resolver o pacote; qualquer outro efeito invalida SC-004.
- [x] E se `node_modules/` aparecer no diff? Permanece ignorado e local por FR-005; `git diff --name-only` não deve listá-lo.
- [x] E se os gates de verificação não puderem ser executados agora? Registram-se como `[NV]` em `spec.md`/`clarify.md`, sem bloquear o `go (scoped)`; a decisão vale pelo registry + docs + estado local, e a validação fica para a implementação.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` (2026-11-19T20:57Z) antes da escrita dos artefatos, com E=(3×5)/(2×1)=7,50 e opção A (`bun add -D -E shadcn-svelte@1.8.1`) eleita.
- [x] R3 zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 078→079 documentada via `--dry-run`, sem bloqueio); os `[NV]` não são perguntas bloqueantes.
- [x] R4 e R7 respeitados na rastreabilidade: FR/SC individuais em `spec.md`, `plan.md` e `tasks.md`, sem intervalos, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: `components.json`, `layout.css`, os 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, scripts/CI/gate, ADRs, constituição, STATE, `specs/079-*` (exceto artefatos), DB/rede fora do registry, segredos, `.tokens/`, fechamento de issues, leg RBAC, a tag `next` e `init`/`add`/`apply`.
