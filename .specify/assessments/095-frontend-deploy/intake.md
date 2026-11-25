# Idea Intake: Deploy do frontend Flora

- **Slug**: 095-frontend-deploy
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/095-frontend-deploy/decision.md` (+ `specs/095-frontend-deploy/spec.md`)
- **Type**: fix

## Idea (as captured)

O FQDN `flora.mdmv.qzz.io` não serve o frontend: ele serve a API Rust. O `decision.md:12-34` registra a cadeia determinística — o workflow `docker-staging.yml` publica a imagem construída com `context: .` pelo `Dockerfile` da raiz, que compila apenas `-p flora-api` e contém somente `/usr/local/bin/flora-api`; por isso `/health`, `/metrics` e `/llms.txt` respondem como rotas da API, enquanto `/` e `/tasks` retornam 404. O artefato registra ainda que `apps/frontend/Dockerfile` existe mas nenhum workflow publica essa imagem, e que os defaults de `config.js` e `DEFAULT_API_BASE_URL` apontam para `http://localhost:3000`, com allowlist `FLORA_APP__CORS_ORIGINS` restrita a origens de loopback (`decision.md:24-31`). A decisão registrada classifica o ciclo como deploy do frontend em origem própria (`decision.md:36-39`), sem indício de falha de rede, DNS intermitente ou build quebrado (`decision.md:33-34`). O `specs/095-frontend-deploy/spec.md:183-187` assume o mesmo diagnóstico. Publicação no GHCR, deploy no Coolify, DNS/FQDN e respostas HTTP remotas permanecem **[NV]** (`decision.md:87-93`).

## Restated

Propõe-se separar a origem web da origem da API e publicar o frontend por um workflow manual, com configuração de runtime e allowlist CORS explícitas. A motivação registrada é o 404 nas rotas da aplicação no FQDN atual, explicado integralmente pelo conteúdo publicado.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Relato de 404 em `/` e `/tasks` no FQDN `flora.mdmv.qzz.io`, com rotas de API respondendo na mesma origem; diagnóstico determinístico registrado em `decision.md:10-34` e assumido em `specs/095-frontend-deploy/spec.md:183-187`.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Quais são as configurações atuais de DNS/FQDN e de deploy no Coolify, dado que não foram observadas neste snapshot ([NV])?]
- [NEEDS CLARIFICATION: Qual é a resposta HTTP real de `/`, `/tasks`, `/health`, `/metrics` e `/llms.txt` nas duas origens após a mudança ([NV])?]
- [NEEDS CLARIFICATION: Quem detém a autorização para disparar o workflow manual de publicação e para promover o deploy?]
- [NEEDS CLARIFICATION: Existem consumidores de `flora.mdmv.qzz.io` que dependam das rotas da API nessa origem?]
- [NEEDS CLARIFICATION: Qual estratégia de rollback se aplica se a separação de origens quebrar o login ou o websocket?]
- [NEEDS CLARIFICATION: O header CORS e o `config.js` entregues ao browser foram verificados em produção ou permanecem apenas como contrato especificado ([NV])?]
