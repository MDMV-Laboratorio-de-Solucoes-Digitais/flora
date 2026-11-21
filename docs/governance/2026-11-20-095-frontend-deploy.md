# Nota de governança — 2026-11-20 (ciclo 095)

**Assunto:** 404 na raiz de `https://flora.mdmv.qzz.io/` — causa raiz (FQDN servia a API Rust, não o frontend SvelteKit), evidências e decisão de separar frontend e API.

**Âncora:** branch `feature/095-frontend-deploy` @ `29b34aefe`; `.specify/feature.json` → `specs/095-frontend-deploy`. Snapshot documental `2026-11-20T23:13:59Z`. Registro append-only; nenhuma linha histórica reescrita.

## (a) Sintoma verificado

- `GET https://flora.mdmv.qzz.io/` → **404** (corpo vazio); `/tasks` e `/login` também **404**.
- No checkout local, `/` e `/tasks` respondem **200** sob o dev server (`curl` com prazo estendido, medido nesta sessão) — o build do frontend contém as rotas; a falha é de publicação/roteamento no FQDN, não de código ausente.

## (b) Causa raiz: o FQDN servia a API

| Evidência | Local / medição | Leitura |
|---|---|---|
| Endpoints da API no FQDN | `/health` **200** `{"status":"ok","search":false,"messaging":false}`; `/metrics` **200** `flora_api_up 1`, `flora_api_info{service="flora-api"} 1`; `/llms.txt` **200** `# Flora Workspace (flora-api)` | São exatamente as rotas de `flora-api/src/routes/health.rs` e o registro em `flora-api/src/routes/mod.rs`: `/health*`, `/metrics`, `/llms.txt`, `/api/v1/*`, `/ws`. Não há `/`, `/tasks` ou `/login` nesse router — daí o 404. |
| Dockerfile da raiz | `Dockerfile:5` `RUN cargo build --release -p flora-api`; `:9` `COPY --from=builder /app/target/release/flora-api /usr/local/bin/flora-api`; `:11` `CMD ["/usr/local/bin/flora-api"]` | A imagem publicada contém somente a API; nenhum build npm/SvelteKit. |
| Workflow de deploy | `.github/workflows/docker-staging.yml:32` `uses: docker/build-push-action@v6`; `:34` `context: .`, sem `file:` | O build usa o `Dockerfile` da raiz. O `apps/frontend/Dockerfile` (build npm + entrypoint) existe, mas não é referenciado por nenhum workflow de deploy. |
| App Coolify (registro histórico, reconsultar ao vivo) | UUID `9q3wuuwkur2kl27py0si785n`, `build_pack=dockerfile`, `ports_exposes=3000`, FQDN `https://flora.mdmv.qzz.io`; deploy `finished_at 2026-11-20T21:23:59Z`, commit `cee7b0cda`; logs com `Compiling yaup`/`Compiling axum` e `INFO flora_api::state::app_state: Search service disabled by configuration` | Confirma que a imagem em produção é a Rust/axum. Dados de chamadas anteriores à Coolify; **[NV]** estado ativo atual. |
| Manifesto do frontend (red herring descartado) | `apps/frontend/build/server/manifest.js:33-34` `id:"/(auth)"`, `pattern: /^\/$/`; `:89-90` `id:"/(app)/tasks"`, `pattern: /^\/tasks\/?$/` (idêntico em `.svelte-kit/output/server/manifest.js`) | `/` e `/tasks` estão corretamente registrados no build; `/(auth)` é grupo interno, não URL. Isso isola a causa no deploy, não no roteamento da aplicação. |

## (c) Decisão registrada — separar frontend e API

1. Publicar uma imagem **`flora-web`** construída a partir de `apps/frontend/Dockerfile` (multi-stage: `npm ci` + `npm run build`; runtime `node build/index.js` via `apps/frontend/docker-entrypoint.sh`, `EXPOSE 3000`). Publicação por workflow manual (`workflow_dispatch`), sem disparo automático de deploy — o deploy permanece fora do gate de merge (ADR-0003).
2. O FQDN **`https://flora.mdmv.qzz.io`** passa a servir o frontend; a API migra para origem separada **`https://flora-api.mdmv.qzz.io`**, com DNS/FQDN e app próprios no Coolify.
3. O runtime config do frontend é gerado no iniciar do container: `apps/frontend/docker-entrypoint.sh` escreve `build/config.js` e `build/client/config.js` com `apiBaseUrl` lido de `FLORA_API_BASE_URL` (padrão local `http://localhost:3000`, nunca interpolado em shell). Definir `FLORA_API_BASE_URL=https://flora-api.mdmv.qzz.io` no app `flora-web`.
4. CORS da API deve incluir a origem do frontend: `FLORA_APP__CORS_ORIGINS` (campo `cors_origins` em `flora-core/src/config/app.rs:19`, validado em `:84-114`; `.env.example:35` mostra o formato de lista). A configuração de produção atual não foi lida — **[NV]**; sem essa inclusão, o browser bloqueará as chamadas entre origens.
5. Nenhuma reescrita de histórico, nenhum artifact `.specify/validations/` ou `.specify/releases/` tocado, nenhum workflow/gate/ADR/constituição/PR/merge/tag/deploy alterado por esta nota. Único gate segue `python3 scripts/ci/check_gate_integrity.py`.

## (d) Estado **[NV]** e próximos passos de validação

**[NV] até deploy final:** publicação da imagem `flora-web`, criação/configuração do app de API e do DNS `flora-api.mdmv.qzz.io`, valores de CORS em produção, resultado do deploy, `/` e `/tasks` no FQDN após a troca, e aprovações/CI/proteção ao vivo. Nenhum deploy foi executado por esta nota.

Validação a executar, nesta ordem, antes de declarar concluído:

1. Rodar o gate do alvo `python3 scripts/ci/check_gate_integrity.py` e `git diff --check` no checkout da feature; não continuar se vermelho.
2. Construir a imagem `flora-web` localmente a partir de `apps/frontend/Dockerfile` e executar o container com `FLORA_API_BASE_URL` de teste; confirmar `GET /` e `GET /tasks` → 200 e `GET /config.js` com o `apiBaseUrl` esperado.
3. Publicar a imagem via workflow manual e conferir tag/digest gerados; só então trocar o FQDN `flora.mdmv.qzz.io` para o app `flora-web`.
4. Criar/validar o app da API em `flora-api.mdmv.qzz.io`, com `FLORA_APP__CORS_ORIGINS` incluindo `https://flora.mdmv.qzz.io`; verificar preflight e chamada autentizada de browser.
5. Após o deploy final, medir novamente: `https://flora.mdmv.qzz.io/` e `/tasks` → 200, `https://flora-api.mdmv.qzz.io/health` → 200, e fluxo de login/tasks em navegador real. Registrar data/hora, commit e resultado; atualizar `STATE.md`/`PROGRESSO.md` com nova nota append-only. Rollback = reapontar o FQDN para a imagem anterior, sem force-push.

## Histórico e escopo

Documento novo e append-only. Nenhuma linha de `STATE.md`, `PROGRESSO.md`, ADRs, constituição, validations ou releases foi reescrita. Nenhuma credencial foi copiada ou exposta. Afirmações sobre estado remoto/Coolify/DNS/CI não remediadas nesta nota permanecem **[NV]** até reconsulta ao vivo.

## (e) Fechamento verificado — 2026-11-21T00:15:48Z

- PRs **#324**, **#325** e **#326** mesclados em `dev`; head local `aa1f01b8a`. O workflow `docker-frontend.yml` publica a imagem e aciona o webhook; a etapa temporária de visibilidade GHCR foi removida porque a API GitHub retorna 404 para pacotes de organização.
- A imagem GHCR permaneceu privada (`401` no pull do Coolify). O app `Flora Web` (UUID `uvykanlrkyp5dzrkktaj5bup`) foi reconfigurado para **buildar direto do Git**: repositório `MDMV-Laboratorio-de-Solucoes-Digitais/flora.git`, branch `dev`, `base_directory=/apps/frontend`, `dockerfile_location=/Dockerfile`, porta 3000, FQDN `https://flora.mdmv.qzz.io`, `FLORA_API_BASE_URL=https://flora-api.mdmv.qzz.io`. Deploy `kwzqvvvkeldq2zmyxa12fu3g` **finished**.
- API migrou para `https://flora-api.mdmv.qzz.io` com CORS `https://flora.mdmv.qzz.io`. Medição final: frontend `/`, `/tasks`, `/config.js` → **200**; API `/health`, `/llms.txt` → **200**; preflight CORS → **200**; página inicial renderizada em navegador real.
- **[NV] remanescente:** fluxo E2E autenticado e healthcheck do Coolify (`unknown`, sem health check configurado). Rollback = reapontar o FQDN para a origem/imagem anterior, sem force-push.
