# Nota de governança — 2026-11-20 (ciclo 093)

**Assunto:** bloqueio do PR #318 por advisories do `@sveltejs/kit@2.70.3`, upgrade para a linha `3.1.1`/adapter `6`/svelte `5.57.1` e prontidão de produção.

**Âncora:** branch `feature/093-sveltekit-security-upgrade` @ `b426e4f81`; `.specify/feature.json` → `specs/093-sveltekit-security-upgrade`. Registro append-only; nenhum validation/release existente tocado.

## (a) PR #318 bloqueado por 4 advisories do kit 2.70.3

PR **#318** — `release: promove dev para main (producao)`, `dev` → `main`, `OPEN`/`BLOCKED`, observado ao vivo via `gh` em 2026-11-20T19:56Z. O check obrigatório `dependency-review` falhou (`fail-on-severity: moderate`, modo "PR diff only") no run `44806804503` (`https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/44806804503/job/136532834045`). Log de falha lido via `gh run view 44806804503 --log-failed`:

| Advisory | Severidade | Finding em `apps/frontend/package-lock.json » @sveltejs/kit@2.70.3` |
|---|---|---|
| `GHSA-3r8g-jcqq-3p5j` | **high** | DoS ao processar form data de remote functions `form` |
| `GHSA-mw6w-2fmv-pvw3` | moderate | `adapter-node` entra em `URIError` não tratado quando `reroute` reescreve pathnames malformados |
| `GHSA-rqp5-r3wg-vg35` | moderate | Import de módulo protocol-relative na resolução de rota SSR permite execução de script cross-origin |
| `GHSA-6cf5-gc4q-fm3x` | moderate | Funções privadas `query.batch` podem colidir, produzindo combinações inesperadas de parâmetro/função |

São **4 advisories (1 high + 3 moderate)**, todos na mesma versão do kit. O diff do PR também introduzia `@sveltejs/adapter-node@5.5.7`, `@sveltejs/vite-plugin-svelte@7.3.0` e `svelte@5.56.10`.

Demais checks observados em `gh pr checks 318`: `db-tests`, `frontend-build`, `rust-build`, `e2e-browser`, `supply-chain-vet` e `mirror` `pass`; havia também 1 `e2e-browser` `pending` e 1 `dependency-review` `skipping` em runs anteriores. Nenhum check foi reexecutado, redefinido ou silenciado por esta nota. **[NV]** estado vivo posterior do PR.

## (b) Correção: upgrade para kit 3.1.1 / adapter 6.0.5 / svelte 5.57.1

Versões corretas identificadas para o ciclo: `@sveltejs/kit` **3.1.1**, `@sveltejs/adapter-node` **6.0.5**, `svelte` **5.57.1**. O kit 3.1.1 declara pares `vite ^8.0.12`, `svelte ^5.57.1`, `typescript ^6` e `vite-plugin-svelte ^7`; o adapter-node 6.0.5 declara par `@sveltejs/kit ^3`. A compatibilidade dos pares é pré-requisito do upgrade — não basta trocar apenas a versão do kit.

Snapshot local neste checkout: `apps/frontend/package.json` já declara `kit 3.1.1`, `adapter-node 6.0.5`, `svelte 5.57.1`, `vite ^8.0.16` e `vite-plugin-svelte ^7.1.2`; `bun.lock` e `package-lock.json` estão em sincronização. `apps/frontend/bun.lock` permanece o lockfile canônico do CI (decisão 092). **[NV]** resultado do `dependency-review` após a materialização do bump e a execução dos gates.

## (c) Produção: app e DNS prontos; deploy depende de `main` promovido

App de produção e DNS estão prontos. O deploy **não** foi executado: depende da promoção de `dev` para `main` (PR #318). Promoção e deploy ficam fora do gate de merge (ADR-0003) e não foram realizados por esta nota. **[NV]** estado ao vivo de Coolify, FQDN e revisão: reconsultar antes de qualquer promoção ou rollback.

## (d) Advisories não são promovidos a gate

Decisão dos ciclos 089, 091 e 092 reafirmada: **sem promoção** de checagem de advisories a required check. `dependency-review` já integra os 4 required checks da ADR-0003; a correção do bloqueio é substituir as versões vulneráveis pelas versões corrigidas, não flexibilizar, desativar ou substituir o gate. Nenhum check obrigatório novo foi criado e nenhum script SDD da factory foi promovido a gate ou incluído em relatórios (decisões 089/091). Única autoridade de gate: `python3 scripts/ci/check_gate_integrity.py`.

## Histórico e escopo

Documento append-only. Nenhuma linha de registros anteriores (STATE.md, PROGRESSO.md, ADRs, constituição, validations/releases) foi reescrita. Nenhum workflow, gate, migration, segredo, PR, merge, tag ou deploy foi alterado por esta nota; nenhuma credencial foi copiada ou exposta. **[NV]** estado ao vivo de CI, proteção de branch, deploy, Forgejo e aprovações de maintainer: reconsultar antes de agir.
