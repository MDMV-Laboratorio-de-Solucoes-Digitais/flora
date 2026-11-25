# Problem Definition: Fluxo núcleo ainda não verificável de forma integrada

- **Slug**: 009-db-smoke-frontend-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md

## Problem Statement

O ciclo 008 entregou testes CI-safe para `select-organization` e um caminho de dev smoke documentado, mas deixou três lacunas interconnected. Primeiro, os casos que dependem de DB (sucesso com rotação/revogação, 403 não-membro, 401 sessão ausente/inativa/mismatch) não estavam automatizados: a CI não fornecia Postgres e o fluxo exigia sessão por `jti` e membership no banco. Segundo, o smoke `docs/dev-smoke.md` não havia sido executado no caminho documentado neste ambiente: a porta host `5432` estava ocupada e o `docker-compose.yml` usava `postgres:16-alpine`, cuja migração inicial falha por ausência de `uuidv7()` (função nativa no PostgreSQL 18); o ciclo 009 só conseguiu evidenciar o fluxo `select org → workspace → task` (rc=0) em stack alternativa com portas não conflitantes e Postgres 18. Terceiro, o contrato frontend↔backend seguia desalinhado: o login não persistia `state`, o callback não validava/enviava `state` nem tratava `select-organization`/rotação de token, `AuthState` não guardava org/workspace e as telas de workspaces/tasks tinham payloads e listagem assignee-scoped divergentes.

O problema é a verificabilidade incompleta do fluxo nuclear em condições próximas às de uso real, somada à falta de coerência mínima entre as pontas. Existia evidência de que o backend funcionava em stack efêmera, mas ela não bastava para afirmar reprodutibilidade pelo caminho documentado, para proteger casos DB na CI ou para permitir que um usuário percorresse o fluxo pela interface. Research.md ainda registra NV sobre a execução do job `db-tests` em runners remotos e sobre os checks determinísticos completos do frontend, dependentes de toolchain/`bun`.

## Affected Users & Stakeholders

- **Users**: usuários finais que tentam completar login → seleção de org → workspace → tarefa pela UI do Seed; desenvolvedores que precisam reproduzir o fluxo localmente e confiar no que a CI informa.
- **Stakeholders**: operador do repositório e mantenedores de CI/QA, que precisam de gates determinísticos e evidência de runtime; mantenedores de frontend, que precisam de contrato coerente; operadores de VPS, que herdam qualquer incompatibilidade de versão de Postgres no caminho de instalação.

## Goals

- Caracterizar a ausência de cobertura DB automatizada para `select-organization` como risco no ponto de autoridade da sessão.
- Explicitar a distância entre "smoke passou em stack alternativa" e "caminho documentado reproduzível" como problema de evidência, não apenas de configuração.
- Delimitar a incompatibilidade observada entre Postgres 16 do compose e `uuidv7()` da migração como obstáculo à reprodutibilidade.
- Tornar visível a divergência de contrato no caminho feliz do frontend como parte do mesmo fluxo não verificável.
- Separar o que foi realmente observado (rc=0 em stack alternativa, decisões implementadas) do que permanece NV (CI remota, toolchain frontend, compose documentado).

## Non-Goals

- Não prescrever neste artefato a versão final de Postgres, o desenho do job de CI, a estratégia de migração ou a implementação das telas de frontend.
- Não tratar a compatibilidade de `uuidv7` em Postgres 16/18 como resolva implicitamente sem evidência no caminho documentado.
- Não afirmar que o job `db-tests` passou em CI remoto ou que check/lint/build/test do frontend foram integralmente executados sem registro (NV).
- Não incluir OIDC/JWKS e2e real, session authority no extrator, paridade frontend ampla ou realtime/files/search avançados.

## Success Metrics

- Casos DB-backed automatizados e executáveis para `select-organization` (baseline: 0 — ciclo 008 cobriu apenas casos CI-safe).
- Smoke executado pelo caminho documentado com resultado registrado (baseline: NV; evidência rc=0 obtida em stack alternativa PG18 com portas 15432/16379/17700).
- Compatibilidade fresh install do compose com a migração inicial (baseline: falha em `postgres:16-alpine` por `uuidv7()` inexistente; PG18 funcional).
- Etapas do caminho feliz coerentes entre frontend e API (baseline: divergentes — `state`, seleção de org, rotação de token, workspace/task).
- Verificação determinística do frontend disponível (baseline: NV parcial — toolchain/`bun` não garantida no ambiente).

## Cost of Inaction

Sem cobertura DB, a autoridade da sessão pode regredir sem sinal até a produção; sem smoke pelo caminho documentado, novos operadores esbarram na mesma falha de migração e o suporte se torna ad hoc. A divergência de contrato obriga o frontend a compensar comportamentos do backend com suposições, tornando qualquer mudança de auth uma quebra potencial. O resultado é um Seed que demonstra valor apenas em condições artesanais — insuficiente para a promessa de alternativa self-hosted reproduzível em VPS barata.

## Open Questions

- [NEEDS CLARIFICATION: qual é a versão/alvo de Postgres suportada e como tratar a função `uuidv7()` em fresh install sem alterar migrações já aplicadas?]
- [NEEDS CLARIFICATION: como evidenciar que o job `db-tests` passa no CI remoto do PR, dado que isso não é observável localmente?]
- [NEEDS CLARIFICATION: qual toolchain frontend está disponível para executar check/lint/build/test e qual é o escopo mínimo aceitável de alinhamento de contrato?]
