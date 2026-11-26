# ADR-0003 — Push, PR e merge como decisões de máquina sob gates determinísticos

- **Status:** Accepted
- **Data:** 2026-11-16
- **Autoridade:** diretriz do proprietário do repositório (2026-11-16) + Constituição da Dark Factory, Princípios I e V (determinismo primeiro; gates determinísticos são autoritativos, saída de LLM é informativa)
- **Supera:** `OBJETIVO.md:94` e `docs/research/2026-11-16-036-authorization-matrix.md` onde exigem autorização humana explícita para merge

## Contexto

Até o ciclo 036, o repositório tratava merge em `dev`/`main` como decisão humana explícita. Na prática isso não protegia nada: `dev` não tinha *required status checks*, `enforce_admins` era `false` e pushes diretos eram rotina. Ao mesmo tempo, dois gates podiam ficar verdes sem executar nada — `cargo test` no pacote raiz (corrigido no ciclo 036) e os seis legs `db-tests` com filtros que casam zero testes (libtest retorna 0). O workflow `docker-staging.yml` falhava em 100% dos runs por usar `secrets.*` em `if:`.

O proprietário determinou que push, PR e merge **nunca** devem ser decisões humanas; a segurança deve vir de gates determinísticos bons o suficiente.

## Decisão

1. **Merge é decisão de máquina.** O agente executa push, abre PR e mescla quando — e somente quando — todos os checks obrigatórios estão verdes com evidência de que testes realmente rodaram. Não há etapa de aprovação humana no caminho de merge.
2. **Gate de verificação reforçado (ciclo 037):**
   - nomes de job únicos (`frontend-build`, `rust-build`) para que required checks sejam inequívocos;
   - cada leg `db-tests` soma os contadores `N passed` do libtest e exige um piso (13/4/8/8/5/6/4/3 — total 51), falhando se o filtro casar zero testes ou se `cargo` falhar (`pipefail`);
   - `timeout-minutes` e `concurrency` por job;
   - `docker-staging.yml` corrigido (`secrets.*` movido para `env`); o gatilho de push foi reativado apenas em `dev` pela decisão SWE de 2026-11-25 (emenda abaixo) — deploy continua fora do gate de merge e sob contrato determinístico separado.
3. **Proteção de branch:** `dev` e `main` passam a exigir os checks `frontend-build`, `rust-build`, `db-tests` e `dependency-review`, com *strict* (branch atualizada) e `enforce_admins`. Sem required reviews — isso reintroduziria decisão humana.
4. **Jobs informativos** (`supply-chain-vet`, `e2e-browser`, `osv-scan`) permanecem `continue-on-error`/advisory e **nunca** entram no conjunto de required checks.

## Consequências

- Positivo: nenhuma alteração entra em `dev`/`main` sem compilação, clippy estrito, checagem de supply-chain e suíte de testes com contagem mínima verificada.
- Risco residido: GitHub trata `skipped` como sucesso em required check. Portanto nenhum job obrigatório pode ter `if:` ou `continue-on-error`. `dependency-review` roda apenas em PR — aceitável porque merges ocorrem via PR.
- Risco operacional: renomear um job muda o nome do check e pode travar todos os PRs. Após qualquer renomeação, os required checks devem ser reselecionados a partir de um run recente na branch base.
- Deploy (GHCR + Coolify) fica explicitamente fora dos required checks desta ADR: é automático por push/merge em `dev` (staging) ou `main` (produção), com `workflow_dispatch` como backup manual; o contrato é validado por `scripts/ci/check_deploy_workflows.py` (emenda 2026-11-25).
- O ciclo 037 é o primeiro executado sob esta política; o merge só é realizado após verificação real dos checks no PR.

## Verificação

Evidência em `.specify/bugs/rust-toolchain-pin-bump-1-98-1/test.md` e no log do PR: contagens de testes por leg, nomes de check no rollup e configuração de proteção via `gh api`. Itens não executados são marcados `NV`, nunca reivindicados como verdes.

## Emenda 2026-11-20 — confirmação do SE

Decisão do proprietário (SE) em 2026-11-20, registrada literalmente. Esta emenda prevalece sobre qualquer cláusula anterior que exija aprovador distinto ou revisão deliberada como condição de merge.

- **Regra antiga (substituída):** "Merge requer check obrigatório verde observado, aprovador distinto e revisão deliberada..."
- **Regra nova (vigente):** "O CI/CD GREEN deve nos fornecer confiança suficiente para fazer merge para dev/ de forma autônoma sem necessidade de aprovação. Caso o CI/CD não inspire confiança, o próximo 80/20 IMEDIATO é reforçar o CI/CD até garantir confiança para conduzir o merge para dev/ sem necessidade de aprovação."

A regra nova prevalece sobre qualquer cláusula anterior de aprovador distinto/revisão deliberada — incluindo a decisão "sem merge" registrada em `.specify/memory/STATE.md` na revalidação de `2026-11-20T09:34:46Z` e exigências equivalentes em `docs/PROGRESSO.md` na mesma data. Esses registros permanecem como histórico, mas deixam de ser autoritativos para decisões de merge.

Reforço operacional: **CI/CD GREEN é suficiente** para conduzir o merge para `dev/` de forma autônoma, sem aprovação. Se o CI/CD **não inspirar confiança**, o próximo 80/20 **IMEDIATO** é reforçar o CI/CD até que ele inspire confiança suficiente para conduzir o merge para `dev/` sem necessidade de aprovação — não escalar para aprovação humana.

- **Status inalterado:** Accepted. A Decisão e as Consequências acima permanecem; apenas a condição de aprovador/revisão é superada por esta emenda.

## Emenda 2026-11-25 — deploy automático por branch (decisão SWE)

- Decisão SWE: todo push/merge em `dev` publica os artefatos de staging (API + web) e todo push/merge em `main` publica os de produção (API + web); `workflow_dispatch` permanece como acionamento manual de contingência.
- Implementação: `on.push.branches` contém exatamente `dev` em `docker-staging.yml`/`docker-staging-web.yml` e exatamente `main` em `docker-frontend.yml`/`docker-production-api.yml`. O guard de job `github.ref` e o fail-closed do webhook continuam obrigatórios; as etapas de webhook usam `github.event_name == 'push' || inputs.deploy`.
- Nenhum required check, piso, ratchet ou tripwire foi enfraquecido: o deploy segue fora do gate de merge e `scripts/ci/check_deploy_workflows.py` (com `--self-test`) rejeita trigger sem branch, branch errada, cross-env ref, webhook reaproveitado, `:latest`, `cancel-in-progress: true`, `continue-on-error` e etapa fail-closed ausente.
- **[NV]**: estado ao vivo do GitHub Actions, segredos Coolify e resultado do deploy não foram observados nesta mudança; nenhum deploy foi disparado.
