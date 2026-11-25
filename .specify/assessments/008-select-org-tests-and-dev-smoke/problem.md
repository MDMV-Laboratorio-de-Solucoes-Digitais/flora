# Problem Definition: Seleção de org sem testes nem smoke reproduzível

- **Slug**: 008-select-org-tests-and-dev-smoke
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md

## Problem Statement

O ponto crítico entregue no ciclo 007 — `POST /api/v1/auth/select-organization`, que valida sessão por `jti`, membership e revoga/recria a sessão com `jti` rotacionado — não tinha testes dedicados. Ao mesmo tempo, não existia um caminho de smoke reproduzível em dev que não dependesse de OIDC real: o `dev-seed.sql` fornecia organização (`11111111-...`), usuária Alice (`22222222-...`), memberships, workspaces e tasks, mas nenhuma sessão; para exercitar o fluxo era preciso bootstrapar sessão/JWT com o mesmo segredo local. A CI (`.github/workflows/rust.yml`) executava `cargo build` e `cargo test` sem Postgres/Valkey/Meilisearch, de modo que casos que dependem de DB não podiam rodar lá. Assim, o comportamento mais sensível do fluxo de autenticação — org nil rejeitado, não-membro, sessão ausente/inativa, sucesso com rotação — ficou sem proteção automatizada e sem forma previsível de reprodução local.

Havia ainda uma nuance de contrato que podia gerar falsa leitura de resultado: `GET /api/v1/tasks` é filtrado pelo assignee do usuário autenticado, e uma task criada pela API não define assignee por padrão, então a criação pode suceder sem a task aparecer na listagem posterior. Research.md também registra que o frontend não persiste `state`, não chama `select-organization`, não substitui o token rotacionado e mantém UI/tipos de workspaces e tasks desalinhados. O problema é a ausência de evidência determinística e de reprodutibilidade em torno da seleção de org — não a falta de mais funcionalidade na API, já implementada.

## Affected Users & Stakeholders

- **Users**: desenvolvedores e engenheiros de CI que precisam verificar o fluxo `select org → workspace → task` sem depender de provedor OIDC externo; usuários finais do Seed, que dependem de seleção de org correta e segura para acessar suas tarefas.
- **Stakeholders**: operador do repositório e mantenedores de CI, responsáveis por gates confiáveis; responsáveis por QA/governança, que precisam distinguir "teste verde" de "caminho realmente exercitado"; mantenedores de frontend, que herdam o contrato divergente.

## Goals

- Caracterizar a falta de testes dedicados para `select-organization` como risco de regressão em um ponto de segurança e tenancy.
- Explicitar a ausência de bootstrap de sessão/token no seed como causa da não reprodutibilidade do smoke sem OIDC real.
- Delimitar a limitação da CI atual (sem Postgres) como motivo pelo qual casos DB-backed não são automaticamente verificados.
- Tornar visível a nuance da listagem assignee-scoped para evitar interpretação errônea de resultados de smoke.
- Separar o que é viável verificar localmente e em CI do que depende de infraestrutura adicional e permanece NV.

## Non-Goals

- Não detalhar neste artefato a implementação dos testes, fixtures, scripts de bootstrap ou estrutura de documentação do smoke.
- Não exigir OIDC real/JWKS completo, alinhamento frontend amplo, deploy, serviços pagos ou segredos reais como pré-condição para tratar o problema.
- Não afirmar que testes DB-backed ou execução integrada do smoke foram rodados sem evidência observável (NV).
- Não alterar o comportamento de runtime da API como objetivo deste definition.

## Success Metrics

- Casos de teste dedicados cobrindo `select-organization` (baseline: 0 — endpoint sem testes próprios conforme intake.md).
- Cenários do fluxo executáveis na CI atual sem serviços externos (baseline: apenas casos que não tocam DB; casos DB-backed NV em CI segundo research.md).
- Etapas necessárias para reproduzir o smoke local sem OIDC (baseline: não documentadas/bootstráveis no início do ciclo; seed sem sessões).
- Taxa de leitura incorreta da listagem de tasks após criação (baseline: nuance observada — task criada sem assignee não aparece em `GET /tasks` assignee-scoped).
- Gates determinísticos locais disponíveis (baseline: check/fmt e testes de biblioteca ok em ciclos anteriores; DB integrado NV).

## Cost of Inaction

Uma mudança futura em permissão, revogação ou rotação de `jti` passaria despercebida até um incidente de acesso entre organizações. Sem bootstrap documentado, cada desenvolvedor reconstrói o ambiente à sua maneira, e um "smoke passou" pode significar coisas diferentes em máquinas diferentes. A CI verde, sem cobertura dos casos DB, transmite confiança infundada. A nuance da listagem assignee-scoped ainda pode levar a conclusões erradas sobre falha na criação de tarefas, desperdiçando tempo de investigação.

## Open Questions

- [NEEDS CLARIFICATION: a cobertura DB-backed deve entrar na CI com serviço Postgres dedicado ou como testes `#[ignore]`/env-gated com job separado?]
- [NEEDS CLARIFICATION: qual forma de bootstrap de sessão/JWT é aceitável para o smoke sem usar segredos ou credenciais reais?]
- [NEEDS CLARIFICATION: o alinhamento mínimo de frontend entra no escopo deste ciclo ou fica explicitamente deferido para o próximo?]
