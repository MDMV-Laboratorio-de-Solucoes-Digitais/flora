# Idea Research: Deploy do frontend Flora

- **Slug**: 095-frontend-deploy
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high — a cadeia causal do 404 é verificável por inspeção direta de Dockerfile/workflows no worktree, e o desfecho remoto foi registrado em append-only em `STATE.md`/`PROGRESSO.md`; hosts remotos não foram consultados nesta pesquisa (**[NV]**).

## Users & Demand

- O defeito relatado é 404 em `/` e `/tasks` no FQDN `flora.mdmv.qzz.io`, enquanto rotas de API respondem na mesma origem; a demanda é do maintainer/SE (`@lfgranja`), registrada no intake e na decisão do ciclo. [source: `.specify/assessments/095-frontend-deploy/intake.md:8-19`; `decision.md:10-13`] (confidence: high)
- A medição registrada em 2026-11-20T23:13:59Z: `GET https://flora.mdmv.qzz.io/` → 404 com corpo vazio, `/tasks` e `/login` → 404; no checkout local, `/` e `/tasks` → 200 sob o dev server. A divergência isolou a falha na publicação/roteamento do FQDN, não nas rotas do build. [source: `STATE.md:820`; `docs/governance/2026-11-20-095-frontend-deploy.md:7-10`] (confidence: high para o registro; **[NV]** para medição remota atual)
- O usuário final citado no spec precisa abrir a aplicação no FQDN e navegar para `/tasks` sem receber 404 da API Rust. [source: `specs/095-frontend-deploy/spec.md:13-24`] (confidence: high)

## Prior Art

- O workflow `docker-staging.yml` publica a imagem construída com `context: .` pelo `Dockerfile` da raiz, que compila somente `-p flora-api`; por isso `/health`, `/metrics` e `/llms.txt` respondem no FQDN. [source: `.github/workflows/docker-staging.yml:46-54`; `Dockerfile:1-11`; `STATE.md:821`] (confidence: high — leitura direta nesta pesquisa)
- `apps/frontend/Dockerfile` já existia (`node:22-alpine`, build `adapter-node`, `EXPOSE 3000`), mas nenhum workflow o publicava; o frontend não era servido em nenhum FQDN. [source: `apps/frontend/Dockerfile:1-17`; `decision.md:24-26`] (confidence: high)
- O manifesto do build registrava corretamente `/` e `/tasks` (`apps/frontend/build/server/manifest.js:33-34,89-90`), e `/(auth)` é grupo interno — hipótese de rota ausente descartada antes da decisão. [source: `STATE.md:821`; `docs/governance/2026-11-20-095-frontend-deploy.md:20`] (confidence: high)
- A publicação manual já era a regra para deploy (ADR-0003, decisão 037 citada no spec), e o ciclo 093 deixara registrado que app de produção e DNS estavam prontos, com deploy dependente da promoção de `main`. [source: `specs/095-frontend-deploy/spec.md:86-98`; `STATE.md:797,801-805`] (confidence: high)

## Market & Context

- A decisão registrada classifica a causa como determinística: o conteúdo publicado pelo workflow/Dockerfile explica integralmente o 404, sem indício de falha de rede, DNS intermitente ou build quebrado. [source: `decision.md:33-34`; `specs/095-frontend-deploy/spec.md:183-187`] (confidence: high)
- A correção separa duas origens: `flora.mdmv.qzz.io` para o web (`/`, `/tasks`) e `flora-api.mdmv.qzz.io` para a API (`/health`, `/metrics`, `/llms.txt`), com publicação da imagem `flora-web` por `workflow_dispatch` e tags `:production`/`:${{ github.sha }}`. [source: `decision.md:38-59`; `specs/095-frontend-deploy/spec.md:127-152`] (confidence: high)
- A configuração de runtime e a allowlist CORS são tratadas como parte do mesmo contrato: `config.js` gerado de `FLORA_API_BASE_URL` sobrescreve `globalThis.__FLORA_CONFIG__.apiBaseUrl`, e `FLORA_APP__CORS_ORIGINS` deve incluir a origem web exata, sem `*`. [source: `decision.md:50-56`; `apps/frontend/docker-entrypoint.sh:18-27`; `apps/frontend/static/config.js:1-7`] (confidence: high — leitura direta)
- O deploy permanece fora do gate de merge (ADR-0003) e nenhum trigger automático de push/PR/merge é introduzido; a fronteira do diff exclui `validate/`, `release/` e `.specify/memory/STATE.md`. [source: `decision.md:57-59`; `specs/095-frontend-deploy/plan.md:49-53`] (confidence: high)

## Data & Constraints

- Leitura direta confirma a cadeia: `Dockerfile:5` `cargo build --release -p flora-api`, `:9` cópia de `/usr/local/bin/flora-api`, `:10-11` `EXPOSE 3000`/`CMD`; `.github/workflows/docker-staging.yml:49` `context: .` sem `file:`. [source: `Dockerfile:1-11`; `.github/workflows/docker-staging.yml:46-54`] (confidence: high)
- O entrypoint do frontend renderiza `build/config.js` e `build/client/config.js` com `apiBaseUrl` lido de `process.env.FLORA_API_BASE_URL` (fallback `http://localhost:3000`), remove variantes comprimidas obsoletas e corrige metadados da tabela de assets antes de importar `build/index.js` — sem interpolação em shell e sem gravar em camada de imagem. [source: `apps/frontend/docker-entrypoint.sh:1-43`] (confidence: high)
- O estado final registrado em append-only: frontend `/`, `/tasks` e `/config.js` → 200 em `flora.mdmv.qzz.io`; API `/health` e `/llms.txt` → 200 em `flora-api.mdmv.qzz.io`; preflight CORS → 200; deploy Coolify `kwzqvvvkeldq2zmyxa12fu3g` `finished`; PRs #324, #325 e #326 mesclados em `dev`. [source: `STATE.md:826-830`; `docs/PROGRESSO.md:292-297`; `docs/governance/2026-11-20-095-frontend-deploy.md:46-51`] (confidence: high para o registro; remoto não reconsultado nesta pesquisa)
- Cobertura idiomática real nos artefatos lidos: **pt-BR_en** — prosa em pt-BR; caminhos, comandos, nomes de imagem e trechos de log em inglês. [source: artefatos do ciclo e arquivos de deploy lidos diretamente] (confidence: high)
- `analyze` fechou com `critical=0`, `requirements_covered=6/6`, `sc=3/3`, `tasks=8/8`, `errors=[]`; o resultado do deploy remoto/Coolify e medições ao vivo são **[NV]** nos artefatos do ciclo. [source: `specs/095-frontend-deploy/analyze.md:9-18,33-39`; `decision.md:87-93`] (confidence: high)
- Permanecem **[NV]** no fechamento registrado: fluxo E2E autenticado e healthcheck do Coolify (sem health check configurado). [source: `docs/governance/2026-11-20-095-frontend-deploy.md:51`; `docs/PROGRESSO.md:297`] (confidence: high)

## Evidence Against the Idea

- O plano registrado previa publicar `flora-web` no GHCR e apontar o FQDN para essa imagem; o fechamento registra que o GHCR permaneceu privado (401 no pull do Coolify) e que o app `Flora Web` passou a buildar direto do Git (branch `dev`, `base_directory=/apps/frontend`). O objetivo de servir o frontend foi atingido por via diferente da especificada em FR-002. [source: `specs/095-frontend-deploy/spec.md:132-136`; `STATE.md:829`; `docs/governance/2026-11-20-095-frontend-deploy.md:48-49`] (confidence: high)
- A decisão descreve o runtime como `CMD node build/index.js`, enquanto o arquivo presente usa `ENTRYPOINT ["docker-entrypoint.sh"]` e o entrypoint faz o `import` de `build/index.js` no mesmo processo. Há descompasso entre a descrição do artefato e o arquivo efetivo. [source: `decision.md:69-71`; `apps/frontend/Dockerfile:14-17`; `apps/frontend/docker-entrypoint.sh:6-11,42`] (confidence: high)
- A tese de "falha de publicação, não de código" depende da observação local de `/` e `/tasks` → 200 sob dev server e do manifesto do build; nenhuma dessas medições foi reexecutada nesta pesquisa, e o diagnóstico é assumido dos artefatos do ciclo. [source: `docs/governance/2026-11-20-095-frontend-deploy.md:7-10,20`; `specs/095-frontend-deploy/spec.md:185-187`] (confidence: medium)
- Consumidores existentes de `flora.mdmv.qzz.io` que dependessem das rotas da API não foram identificados; a reatribuição do FQDN poderia quebrar integrações não documentadas, e o intake registra essa incógnita. [source: `intake.md:26` | ASSUMPTION] (confidence: low)
- A estratégia de rollback é registrada como "reapontar o FQDN para a imagem anterior", mas o fechamento mostra que o build passou a ser feito pelo Coolify direto do Git; reapontar para uma imagem anterior do GHCR pode não ser aplicável sem reconfiguração do app. [source: `STATE.md:823,829`; `docs/governance/2026-11-20-095-frontend-deploy.md:40,49`] (confidence: medium)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Quais são as configurações atuais de DNS/FQDN e de deploy no Coolify, dado que não foram observadas neste snapshot ([NV])?]
- [NEEDS CLARIFICATION: Qual é a resposta HTTP real de `/`, `/tasks`, `/health`, `/metrics` e `/llms.txt` nas duas origens após a mudança ([NV])?]
- [NEEDS CLARIFICATION: Quem detém a autorização para disparar o workflow manual de publicação e para promover o deploy?]
- [NEEDS CLARIFICATION: Existem consumidores de `flora.mdmv.qzz.io` que dependam das rotas da API nessa origem?]
- [NEEDS CLARIFICATION: Qual estratégia de rollback se aplica se a separação de origens quebrar o login ou o websocket?]
- [NEEDS CLARIFICATION: O header CORS e o `config.js` entregues ao browser foram verificados em produção ou permanecem apenas como contrato especificado ([NV])?]
- [NEEDS CLARIFICATION: Como o desvio entre a publicação prevista no GHCR (FR-002) e o build direto do Git no Coolify deve ser refletido nos artefatos do ciclo?]

## Sources

- `.specify/assessments/095-frontend-deploy/intake.md:1-28` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/assessments/095-frontend-deploy/decision.md:10-34,38-77,87-106` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/095-frontend-deploy/spec.md:13-194`, `plan.md:9-72`, `tasks.md:12-60` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/095-frontend-deploy/analyze.md:1-41`, `converge.md:6-24` (system; policy: fonte primária local; lido 2026-11-25)
- `docs/governance/2026-11-20-095-frontend-deploy.md:7-51` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/memory/STATE.md:817-830` e `docs/PROGRESSO.md:283-297` (system; policy: registro append-only local; lido 2026-11-25)
- `Dockerfile:1-11`, `.github/workflows/docker-staging.yml:46-54`, `apps/frontend/Dockerfile:1-17`, `apps/frontend/docker-entrypoint.sh:1-43`, `apps/frontend/static/config.js:1-7` (system; policy: fonte primária local, leitura direta; lido 2026-11-25)
- `https://flora.mdmv.qzz.io/` (host: flora.mdmv.qzz.io; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
- `https://flora-api.mdmv.qzz.io/health` (host: flora-api.mdmv.qzz.io; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
- `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora` (host: github.com; policy: allowlisted; não buscado nesta passagem — CI/produção ao vivo **[NV]**)
