# Problem Definition: Caminho dev/smoke quebrado por incompatibilidade `uuidv7`

- **Slug**: 010-compose-uuidv7-smoke-frontend-verify
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md, problem.md (anterior)

## Problem Statement

**Documented dev/smoke path quebrado em fresh install:** `docker-compose.yml` usava `postgres:16-alpine`, mas a migração inicial (`flora-core/src/migrations/sql/20240818000001_init_schema.sql`) define `DEFAULT uuidv7()` em múltiplas tabelas (PKs); `uuidv7()` é nativo do PostgreSQL 18, e em PG16 a migração falha no fresh install, conforme evidenciado no ciclo 009. Essa incompatibilidade impede que um checkout limpo suba e conclua o caminho dev documentado. A imutabilidade do `sqlx::migrate!` (checksums/histórico) torna sensível qualquer alteração em migração já aplicada; como a init falha antes das demais, uma migration compat adicional não resolve o fresh install sem bootstrap pré-migrate ou mudança de baseline.

**Evidência de smoke integrada incompleta no caminho documentado:** o ciclo 009 comprovou o fluxo `select org → workspace → task` com rc=0 em stack alternativa (Postgres 18 em porta alternativa, Valkey/Meilisearch em portas distintas, seed + bootstrap de sessão + `test-flora.sh`), mas faltava reconciliar compose/docs e evidenciar o caminho compatível. O CI `db-tests` já utilizava `postgres:18`, reforçando o descolamento entre o que é exercitado e o que é documentado.

**Ambiente local com restrição:** porta host `5432` ocupada externamente impede o bind literal padrão (NV ambiental), exigindo caminho equivalente documentado/parametrizado; o probe do ciclo 010 também encontrou `.env` ausente, portas 6379/7700/3000 livres, nenhum container/volume da Flora e imagem `postgres:18-alpine` disponível localmente. Volumes antigos de PG16 exigem reset consciente em ambiente descartável.

**Verificação frontend:** a configuração do projeto de testes `server` (Vitest em `apps/frontend/vite.config.ts`) não cobria explicitamente `tests/**` conforme esperado para execução determinística alinhada à CI (`bun install`, `check`, `lint`, `test:unit -- --run --project server`, `build`); os checks precisavam ser executados e registrados para o slice auth→org→workspace→task entregue no ciclo 009. Alguns testes podem estar defasados após esse ciclo, e o alinhamento fino de tipos/erros/contratos do slice é incremental. O problema central é a impossibilidade de comprovar, de forma reproduzível e alinhada à documentação, que o Seed sobe e que o fluxo núcleo funciona — agravada pela falta de evidência determinística no frontend.

## Affected Users & Stakeholders

- **Users**: desenvolvedores que seguem `docs/dev-smoke.md` a partir de um checkout limpo e encontram falha de migração; operadores de VPS de PME que dependem do compose para instalar o Seed; usuários finais cujo fluxo auth→org→workspace→task só foi demonstrado em configuração artesanal.
- **Stakeholders**: operador do repositório e mantenedores de CI/QA, que precisam de gates e documentação confiáveis; mantenedores de frontend, que precisam de execução determinística dos checks; comunidade de contribuidores, que replica o caminho documentado.

## Goals

- Caracterizar a incompatibilidade entre a versão de Postgres do compose/documentação e o requisito `uuidv7()` da migração inicial como causa da falha em fresh install.
- Explicitar a necessidade de evidência do smoke no caminho documentado, distinguindo-a da execução em stack alternativa já observada.
- Reconhecer a restrição de porta e a ausência de `.env`/volumes compatíveis como condições ambientais que exigem parametrização e NV transparente.
- Delimitar a lacuna de cobertura e de execução dos checks determinísticos do frontend para o slice já implementado.
- Preservar a restrição de não alterar migrações `sqlx` aplicadas como limite a ser respeitado por qualquer tratamento futuro do problema.

## Non-Goals

- Não decidir neste artefato a versão/alvo de Postgres, editar migrações, implementar polyfill/extensão `uuidv7` para PG16 ou redefinir baseline de `sqlx`.
- Não especificar alterações de `docker-compose.yml`, `docs/dev-smoke.md`, configuração do Vitest ou código frontend.
- Não implementar OIDC/JWKS e2e real, session authority no extrator, paridade/UX ampla de frontend ou CI remoto além do PR.
- Não afirmar que o smoke na porta padrão, a compatibilidade com volumes PG16 ou os gates de frontend foram executados com sucesso sem evidência (NV).

## Success Metrics

- Fresh install pelo caminho documentado conclui migração e startup (baseline: falha — PG16 sem `uuidv7()` nativo).
- Smoke `select org → workspace → task` executado pelo caminho documentado com rc e logs registrados (baseline: NV no caminho documentado; rc=0 apenas em stack alternativa PG18 no ciclo 009).
- Compatibilidade de imagem/volume de Postgres entre compose, docs e CI (baseline: divergente — compose PG16, CI `db-tests` PG18; volumes PG16 exigem reset).
- Cobertura da configuração de testes frontend sobre `tests/**` no projeto `server` (baseline: parcial — diretório não explicitamente incluído; possíveis testes defasados pós-009).
- Checks determinísticos do frontend (`check`, `lint`, `test:unit -- --run --project server`, `build`) executados e registrados (baseline: NV/parcial — dependente de toolchain/`bun`).

## Cost of Inaction

Enquanto o compose documentado usar PG16, cada novo operador reproduz a mesma falha de migração e o Smoke só "funciona" nas máquinas de quem conhece o workaround. A evidência ficou restrita a uma stack alternativa, então mudanças futuras em migração, compose ou fluxo de auth podem quebrar o caminho oficial sem ninguém notar até a produção. No frontend, a ausência de cobertura explícita e de execução registrada deixa o slice auth→org→workspace→task sem sinal determinístico, permitindo que erros de contrato e testes defasados se acumulem. O custo é perda de reprodutibilidade, confiança e velocidade de onboarding — exatamente o diferencial prometido para VPS barata.

## Open Questions

- [NEEDS CLARIFICATION: qual é a versão de Postgres alvo e qual tratamento é aceitável para `uuidv7()` sem alterar migrações já aplicadas (alinhar compose/docs ao PG18 ou polyfill/extensão com bootstrap pré-migrate)?]
- [NEEDS CLARIFICATION: como evidenciar o smoke quando a porta `5432` está ocupada por processo externo não descartável, e qual desvio dev ser registrado como NV ambiental?]
- [NEEDS CLARIFICATION: como lidar com volumes PG16 existentes e com a nota de chave do Meilisearch no bootstrap documentado, sem afetar bancos não descartáveis?]
- [NEEDS CLARIFICATION: qual toolchain/`bun` está disponível para executar check/lint/test:unit/build do frontend e qual falha mínima é considerada em escopo?]
