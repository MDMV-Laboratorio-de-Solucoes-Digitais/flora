# Runbook — deploy Flora Workspace (GitHub → GHCR → Coolify → Cloudflare)

**Data:** 2026-11-23 · **Autoridade:** ADR-0003 — merge é determinístico, **deploy é manual e humano**. Nenhum workflow dispara em `push`, `pull_request` ou merge. A configuração de apps, segredos e DNS foi aplicada nesta data com as credenciais disponíveis; o resultado das validações ao vivo está na §7.

## 1. Matriz de ambientes

| Ambiente | Branch de origem | API FQDN | Web FQDN | Tags GHCR | Workflow GitHub Actions | Webhook secreto |
|---|---|---|---|---|---|---|
| Staging | `dev` | `https://flora-staging.mdmv.qzz.io` | `https://flora-staging-web.mdmv.qzz.io` | `flora:staging`, `flora-web:staging` + `:${{ github.sha }}` | `docker-staging.yml`, `docker-staging-web.yml` | `COOLIFY_STAGING_API_DEPLOY_WEBHOOK`, `COOLIFY_STAGING_WEB_DEPLOY_WEBHOOK` |
| Produção | `main` | `https://flora-api.mdmv.qzz.io` | `https://flora.mdmv.qzz.io` | `flora:production`, `flora-web:production` + `:${{ github.sha }}` | `docker-production-api.yml`, `docker-frontend.yml` | `COOLIFY_PRODUCTION_API_DEPLOY_WEBHOOK`, `COOLIFY_FRONTEND_DEPLOY_WEBHOOK` |

Regras do contrato versionado (validadas por `scripts/ci/check_deploy_workflows.py`):

- `workflow_dispatch` é o único gatilho; qualquer `push`/`pull_request`/`schedule` reprova o gate.
- Job guard: `refs/heads/dev` nos workflows de staging, `refs/heads/main` nos de produção. Dispatch desde a branch errada é ignorado pelo job.
- Cada workflow usa **um** webhook secreto; webhooks de staging e produção nunca são reutilizados.
- `cancel-in-progress: false` — um deploy em andamento nunca é cancelado.
- Se `deploy=true` (padrão) e o segredo do webhook não existe, o job falha antes do build: um run nunca termina verde sem ter deployado. Use `-f deploy=false` apenas para publicar a imagem sem acionar o Coolify.
- Nenhuma tag `:latest`; sempre tag de ambiente + SHA imutável.

Para disparar (ação humana, nunca automação):

```bash
gh workflow run 'Build & Deploy Flora API — Staging (GHCR -> Coolify)' --ref dev -f deploy=true
gh workflow run 'Build & Deploy Flora Web — Staging (GHCR -> Coolify)' --ref dev -f deploy=true
gh workflow run 'Build & Deploy Flora API — Production (GHCR -> Coolify)' --ref main -f deploy=true
gh workflow run 'Build & Deploy Flora Web — Production (GHCR -> Coolify)' --ref main -f deploy=true
```

## 2. Segredos do GitHub (nomes apenas)

Presentes em `MDMV-Laboratorio-de-Solucoes-Digitais/flora` após 2026-11-23: `COOLIFY_API_TOKEN`, `COOLIFY_FRONTEND_DEPLOY_WEBHOOK`, `COOLIFY_PRODUCTION_API_DEPLOY_WEBHOOK`, `COOLIFY_STAGING_API_DEPLOY_WEBHOOK`, `COOLIFY_STAGING_WEB_DEPLOY_WEBHOOK`, `FLORA_BRANCH_PROTECTION_TOKEN`, `FORGEJO_TOKEN`, `FORGEJO_USER`.

Os três novos webhooks foram provisionados como URLs de deploy do Coolify no formato `https://coolify.mdmv.qzz.io/api/v1/deploy?uuid=<uuid>&force=false`, uma por app. Nenhum valor é registrado neste repositório; a autorização do disparo continua sendo o Bearer `COOLIFY_API_TOKEN`.

## 3. Coolify — configuração por app

UUIDs (identificadores, não segredos): API produção `9q3wuuwkur2kl27py0si785n`, API staging `bkmc2irzasaopemcprvajiin`, web produção `uvykanlrkyp5dzrkktaj5bup`, web staging `vww7nxjbnrly3mjsdueyajjw`.

### Staging — API (`Flora - Staging`)

- Origem: Git `dev`; porta 3000. Healthcheck **habilitado**: `GET /health`, intervalo 10 s, timeout 5 s, 5 tentativas.
- Env agora presentes: `FLORA_APP__ENV=staging`, `FLORA_APP__BASE_URL=https://flora-staging.mdmv.qzz.io`, `FLORA_APP__CORS_ORIGINS=["https://flora-staging-web.mdmv.qzz.io"]`, `FLORA_APP__JWT_SECRET` (valor gerado no cofre local, não registrado; rotacionar se desejar), `DATABASE_URL` (staging).
- Zero state: `GET /api/v1/bootstrap/status` → `{"available":true}` e zero organizações não-sistema.

### Staging — Web (app criada `Flora Web - Staging`)

- Git `dev`, `base_directory=/apps/frontend`, `dockerfile_location=/Dockerfile`, porta 3000, auto-deploy desligado, healthcheck **habilitado** em `/config.js`.
- Env: `FLORA_API_BASE_URL=https://flora-staging.mdmv.qzz.io`.
- O primeiro build falhou por duplicação de caminho (`/apps/frontend/apps/frontend/Dockerfile`); corrigido para `/Dockerfile` relativo ao `base_directory` e redeploy disparado.

### Produção — API (`Flora`)

- Git `main`, healthcheck **habilitado** em `/health` (antes: desabilitado com path `/`).
- Env intacta: `FLORA_APP__ENV=production`, `FLORA_APP__BASE_URL=https://flora-api.mdmv.qzz.io`, `FLORA_APP__CORS_ORIGINS=["https://flora.mdmv.qzz.io"]`, JWT/DB de produção.
- Variáveis temporárias `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` **removidas** e API redeployada em 2026-11-23 (ADR-0007 fechado).

### Produção — Web (`Flora Web`)

- Git `main` (promovido de `dev` em 2026-11-23), healthcheck **habilitado** em `/config.js`; env `FLORA_API_BASE_URL=https://flora-api.mdmv.qzz.io`; app `running:healthy`. Deployment anterior preservado para rollback.

## 4. Cloudflare — DNS e CORS

Zona `mdmv.qzz.io`. Registro criado: `flora-staging-web.mdmv.qzz.io` é CNAME proxied para `flora-staging.mdmv.qzz.io`; resolução confirmada por DNS-over-HTTPS (`172.67.192.193`). O OCI `flora-staging-web.mdmv.qzz.io` aponta para a app web de staging; CORS é aplicado pela API Flora (`FLORA_APP__CORS_ORIGINS`), não pelo Cloudflare.

Manter `flora-api` e `flora-staging` em origens separadas da web; confirmar que o preflight retorna `access-control-allow-origin` exatamente igual à origem (sem `*`). Não copiar dados de produção para staging.

## 5. Verificação determinística

```bash
cd /home/luis/development/MDMV/projetos/flora
python3 scripts/ci/check_deploy_workflows.py
python3 scripts/ci/check_deploy_workflows.py --self-test
python3 scripts/ci/check_gate_integrity.py
python3 scripts/deploy/check_endpoints.py --self-test
FLORA_WEB_URL=https://flora.mdmv.qzz.io FLORA_API_URL=https://flora-api.mdmv.qzz.io \
  python3 scripts/deploy/check_endpoints.py
FLORA_WEB_URL=https://flora-staging-web.mdmv.qzz.io FLORA_API_URL=https://flora-staging.mdmv.qzz.io \
  python3 scripts/deploy/check_endpoints.py
```

## 6. Itens humanos — situação

| Item | Situação |
|---|---|
| Zero state do staging | **Concluído:** `bootstrap/status.available=true` após isolar o DB e redeployar a API. |
| Variáveis temporárias de recuperação em produção | **Concluído:** quatro entradas `FLORA_BOOTSTRAP__*` removidas e API redeployada. |
| Promoção `dev→main` da web de produção | **Concluído no Coolify:** app `Flora Web` em `main`, deploy `finished`, `running:healthy`. Nenhum merge git foi feito. |
| Decidir se CD continua manual ou passa a ser automático | **Em aberto:** mudança de política — este runbook mantém manual (ADR-0003). |
| Revisão do First Seed/roteiro completo no staging | **Em aberto pelo SE:** zero state confirmado; cenários do roteiro seguem execução humana. |

Rollback: reapontar o app no Coolify para o deployment anterior bem-sucedido, sem force-push e sem desativar gates.

## 7. Configuração executada em 2026-11-23

- Healthchecks habilitados em `Flora`, `Flora - Staging` e `Flora Web`. Nas duas apps web, após falha do healthcheck HTTP (imagem `node:22-alpine` sem `curl`; `wget` não conectava), host/porta explícitos `127.0.0.1:3000`, intervalo 15 s, timeout 5 s, 10 tentativas e start period 60 s.
- App `Flora Web - Staging` criada, com env de API e healthcheck; caminho do Dockerfile corrigido após falha do primeiro build.
- Env de staging definida no app da API (ENV/BASE_URL/CORS/JWT).
- Três segredos de webhook criados no GitHub; DNS `flora-staging-web` criado no Cloudflare.
- Deploys de staging (API e web) disparados via Coolify; status final e verificação de endpoints registrados abaixo.

**Status final:** deploys de produção e staging concluídos; zero state de staging atingido (`available:true`); produção com `available:false` (instalação consolidada, esperado). Todas as matrizes de endpoints estão verdes.

## 8. Resultado da verificação ao vivo (final, 2026-11-23)

| Ambiente | `check_endpoints.py` | `bootstrap/status` | Coolify |
|---|---|---|---|
| Produção | `ok=true` — 7 sondas 200, CORS `https://flora.mdmv.qzz.io` | `{"available":false}` | API e web `running:healthy`, ambas em `main` |
| Staging | `ok=true` — 7 sondas 200, CORS `https://flora-staging-web.mdmv.qzz.io` | `{"available":true}` | API e web `running`/`running:healthy`, ambas em `dev` |

## 9. Ações Tier 4 executadas nesta sessão (autorização explícita)

- **Isolamento de DB:** a API de staging apontava para o mesmo database de produção (`mdmv_core`). `DATABASE_URL` foi repontado para o database isolado `flora-staging-db` (`/flora_staging`); a entrada duplicada que apontava para produção foi removida. Após redeploy, `bootstrap/status` passou a `available:true`.
- **Produção:** as quatro variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` foram removidas da API e a API foi redeployada. A app `Flora Web` foi promovida de `dev` para `main` e redeployada; deployment anterior `qmqbepulw7bjwysru6hy9svb` mantido para rollback.
- **Rotação de segredo:** os dois `manual_webhook_secret_github` que apareceram em saída de ferramenta foram rotacionados nas apps de API; o GitHub não possui hooks configurados (`0`), então não houve reconciliação externa.
- **Ressalva:** o healthcheck da API de staging está habilitado, mas o container atual ainda não foi recriado após essa habilitação; o status do Coolify aparece `running:unknown` até o próximo deploy, embora `/health` responda 200.

Nenhum merge, force-push ou alteração de branch protection foi executado. Nenhum valor de segredo foi registrado em arquivo, chat ou log.
