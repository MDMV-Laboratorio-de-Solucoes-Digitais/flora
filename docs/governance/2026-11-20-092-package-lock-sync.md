# Nota de governança — 2026-11-20 (ciclo 092)

**Assunto:** sincronização do `package-lock.json` do frontend e triagem dos advisories de dependência.

**Âncora:** branch `feature/092-npm-lock-sync` @ `bf00caf5c`; `.specify/feature.json` → `specs/092-npm-lock-sync`. Registro append-only; nenhum validation/release existente tocado.

## (a) Lockfile canônico e sincronização do derivado

`apps/frontend/bun.lock` continua sendo o lockfile canônico do CI. `apps/frontend/package-lock.json` é artefato derivado: foi sincronizado com o manifesto sem alterar `apps/frontend/package.json`.

Comando contratual, executado em `apps/frontend`:

```
npm install --package-lock-only --ignore-scripts
```

Evidência neste checkout: `git diff -- apps/frontend/package-lock.json` → 1 arquivo, `+116/−0`. A linha da root `devDependencies` passou a incluir `"shadcn-svelte": "1.8.1"`, antes ausente. O restante são entradas transitivas e opcionais do mesmo grafo, como `@emnapi/*` em `@tailwindcss/oxide-wasm32-wasi`, `cn`, `commander`, `node-fetch-native` e a entrada `node_modules/shadcn-svelte`. `git diff -- apps/frontend/package.json` → vazio. Nenhuma versão declarada no manifesto foi alterada e nenhum script de instalação foi executado.

## (b) Advisories: triagem sem promoção a gate

O registro do ciclo 091 fixou a baseline em **25 alerts do Dependabot (7 high)** para `apps/frontend/package-lock.json`. São fontes distintas: alerts do Dependabot e advisories do registro npm lidos do lockfile, sem comparação estrita entre eles.
Contagem ao vivo após a regeneração: `npm audit --package-lock-only --json` em `apps/frontend` → **21 advisories (5 high, 15 moderate, 1 low, 0 critical, 0 info)**, `auditReportVersion 2`. `metadata.dependencies`: 65 prod, 298 dev, 83 optional, 362 total. O comando é somente leitura: a SHA-256 do lock permaneceu `68bd731c7e84b6e81e91a2d00b3a05c13500014fa7388a08d8b7e28a17790a28`; exit code 1 é o comportamento normal do `audit` quando há advisories. A variação numérica em relação ao baseline (25 → 21, 7 high → 5 high) é registrada apenas como referência entre fontes diferentes.

Decisão reafirmada dos ciclos 089 e 091: **não promover** checagem de advisories a required check. Nenhum check obrigatório novo foi criado; o conjunto de gates permanece o da ADR-0003.

## (c) Promoção dev→main e deploy de produção

Promoção e deploy ficam fora do gate de merge (ADR-0003) e não foram executados por esta nota. **[NV]** estado ao vivo de PR, CI, proteção de branch e Coolify: reconsultar antes de qualquer merge ou promoção. Conforme o registro do ciclo 091, apenas a app `Flora-Staging` existe no Coolify; não há app de produção, FQDN ou revisão de produção confirmados. Não promover Staging a produção sem FQDN e revisão verificados ao vivo.

## Histórico e escopo

Documento append-only. Nenhuma linha de registros anteriores (STATE.md, PROGRESSO.md, ADRs, constituição, validations/releases) foi reescrita. Nenhum código, workflow, gate, migration, segredo, PR, merge, tag ou deploy foi alterado por esta nota; nenhuma credencial foi copiada ou exposta. **[NV]** estado ao vivo de CI, proteção de branch, deploy, Forgejo e aprovações de maintainer: reconsultar antes de agir.
