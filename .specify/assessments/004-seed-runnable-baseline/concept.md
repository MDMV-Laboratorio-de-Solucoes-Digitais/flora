# Concept: Baseline local reproduzível do Seed

- **Slug**: 004-seed-runnable-baseline
- **Created**: 2026-11-25
- **Recommended option**: Baseline local reproduzível do Seed

## Options

### Option A — Smoke manual com NV explícito

- **Sketch**: Mantém o que já existe — `dev-seed.sql`, `test-flora.sh` e as verificações determinísticas locais — e apenas documenta o percurso com uma lista honesta do que depende de ambiente externo. Um desenvolvedor sabe onde está e o que não pode afirmar, mas ainda reconstrói o setup à sua maneira.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar para reduzir confusão imediata, sem configuração nova. Não resolve reprodutibilidade: o que é "pronto" continua dependendo de quem executa; integrações com Postgres/Valkey/Meilisearch/OIDC seguem sem evidência.
- **Rabbit holes**: Documentar tanto o NV que o caminho passe a parecer definitivo; transformar o script manual em pseudo-harness sem execução observada; confundir "check passou" com "Seed sobe".

### Option B — Baseline local reproduzível do Seed

- **Sketch**: Um perfil dev mínimo que qualquer pessoa consegue repetir a partir de um checkout limpo: subir as dependências essenciais, aplicar o seed e percorrer um smoke acordado, com direção (objetivo/roadmap) alinhada ao estado observável. Nada de segredos reais, deploy de produção ou serviços pagos; o que não roda no ambiente é marcado como NV.
- **Appetite**: medium
- **Trade-offs**: Destrava verificação ponta a ponta e onboarding, vira base para as fatias verticais seguintes e reduz custo de iteração e "falso pronto". Não entrega feature de usuário final por si só; o ganho depende de definir o que conta como smoke mínimo e deDocker/serviços estarem disponíveis. Appetite médio com incerteza: integração runtime não foi exercitada neste ambiente.
- **Rabbit holes**: Incluir mock OIDC/Zitadel "por completude" e inflar o perfil dev; tentar reproduzir fidelidade de produção; reescrever objetivo/roadmap como promessa de features em vez de espelho do estado real; perseguir health checks e quotas antes de ter o caminho básico.

### Option C — Ambiente completo de produção/CI remota (comprar)

- **Sketch**: Provisionar infraestrutura e automação amplas, com integração contínua remota e todos os serviços em configuração de produção, buscando fidelidade máxima desde o início.
- **Appetite**: large
- **Trade-offs**: Alta fidelidade eventual, mas exige autorizações, infra, segredos e serviços fora do escopo autônomo; atrasa justamente o que o ciclo precisa destravar.
- **Rabbit holes**: Migrar infraestrutura antes de provar o fluxo central; depender de credenciais para validar algo que deveria ser reproduzível localmente; confundir governança remota com baseline.

## Recommendation

Option B. Os goals do `problem.md` apontam para enablement de verificação: explicitar o estado real em objetivo/roadmap, caracterizar a ausência de ambiente reproduzível como pré-requisito bloqueador, definir o smoke mínimo e reduzir custo de iteração sem segredos nem deploy. B é a única opção que atende a esses quatro pontos com baixo risco e reversibilidade; a decisão do ciclo 004 já a aponta como maior alavancagem. Onde Docker/serviços/OIDC não estão disponíveis, a evidência deve permanecer NV.

## Out of Scope (for the recommended option)

- Solução detalhada de infraestrutura, compose, configuração ou lista de tarefas do ciclo.
- Features de usuário final como meta; o foco é tornar a verificação possível.
- Afirmar que Postgres/Valkey/Meilisearch/OIDC funcionam sem execução observada.
- Deploy/infra de produção, proteções remotas, merges/releases, credenciais reais, serviços pagos/proprietários, RE não autorizada ou reuso incompatível com AGPL.

## Assumptions to Validate

- Docker e os serviços locais estão disponíveis para executar o smoke integrado; caso contrário, o resultado é NV.
- Um perfil dev com infra essencial é suficiente como smoke mínimo aceito, sem incluir mock OIDC/Zitadel.
- Existe critério acordado para considerar o Seed "localmente verificável" e para alinhar objetivo/roadmap ao estado observável.
- Verificações parciais já registradas (check/fmt e testes de biblioteca) podem ser reutilizadas como evidência, sem substituir integração runtime.
