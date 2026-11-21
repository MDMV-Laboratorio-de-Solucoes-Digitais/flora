# Nota de governança — 2026-11-21 (ciclo 101)

**Assunto:** login local ainda retorna 401 em produção embora a recuperação de senha do ciclo 100 tenha executado com sucesso (`applied=true`). Com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` já ativo, o hash Argon2id foi gravado no primeiro usuário ativo, mas `POST /api/v1/auth/password/login` resolve a conta pelo e-mail enviado no corpo e o e-mail real do usuário bootstrap é desconhecido. Decisão: **wildcard de login com opt-in** — aceitar `email='*'` na rota de login somente quando `FLORA_BOOTSTRAP__ADMIN_EMAIL` também for `*`.

**Âncora:** branch `feature/101-password-login-wildcard` @ `5890471be`; `.specify/feature.json` → `specs/101-password-login-wildcard`. Snapshot documental `2026-11-21T05:11:11Z`. Registro append-only; nenhuma linha histórica reescrita. Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.

## (a) Evidência — `applied=true` e login 401 por e-mail desconhecido

- O ciclo 100 introduziu o wildcard explícito em `apply_startup_password` (`flora-core/src/config/bootstrap.rs`): com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e a senha configurada, a rotina seleciona o primeiro usuário ativo e grava o hash Argon2id restrito a esse `user_id`. O chamador em `flora-api/src/state/app_state.rs` registra `tracing::info!(applied = applied, "Startup administrator password recovery")`.
- A medição em produção deste ciclo trouxe **`applied=true`**: a escrita do hash aconteceu. O par de variáveis temporárias está presente (`FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD`), sem valor de senha registrado nesta nota.
- Ainda assim, `POST /api/v1/auth/password/login` devolve **401**. A rota resolve a conta a partir do e-mail informado no corpo da requisição; como o e-mail real do usuário bootstrap é desconhecido, a busca não alcança o usuário cujo hash foi atualizado e a verificação de senha não pode suceder.
- O ponto de falha mudou de locus em relação ao ciclo 100: não é mais a recuperabilidade do hash no startup (`applied=false`), nem o guard de hash pré-existente (removido em 098), mas a **resolução da identidade no momento do login**. Corrigir a escrita do hash não foi suficiente sem um meio de endereçar a conta alvo.

## (b) Objetivo

Permitir o login local do administrador real sem conhecer o e-mail, com **opt-in explícito e temporário**, sem expor segredos, sem ampliar o gate de merge e sem afetar contas além do alvo selecionado.

## (c) Decisão registrada — wildcard de login com opt-in

1. **Opt-in no corpo da requisição.** `POST /api/v1/auth/password/login` passa a aceitar `email='*'` (exatamente `*`) **somente** quando `FLORA_BOOTSTRAP__ADMIN_EMAIL` também for `*` (exatamente `*`). O opt-in já está ativo em produção. A confirmação é feita no servidor contra a configuração carregada no startup; o cliente não pode habilitar o modo por conta própria.
2. **Resolução determinística do alvo.** Nesse modo, a rota não busca por e-mail: resolve o **primeiro usuário ativo** (`is_active = true`, `ORDER BY created_at ASC, id ASC`, sem filtro de role nem de sistema) e valida a senha fornecida contra o hash Argon2id gravado no ciclo 100. A sessão emitida é a mesma do login por e-mail; nenhuma senha ou hash aparece em log ou resposta.
3. **Sem opt-in, comportamento inalterado.** Com qualquer outro valor de `FLORA_BOOTSTRAP__ADMIN_EMAIL`, `email='*'` é rejeitado (401) e a busca por e-mail permanece como único caminho de login local. O fluxo OIDC authorization-code + PKCE/S256 continua primário e não relaxado; a ausência da env de recuperação também desliga o modo wildcard.
4. **Variáveis temporárias.** O par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` é credencial temporária e deve ser removido da app da API no Coolify (`9q3wuuwkur2kl27py0si785n`) **após** a confirmação do login, evitando que um reinício futuro volte a sobrescrever a senha ou reabilite o atalho. Nenhum valor de senha foi versionado ou registrado nesta nota.
5. **Fora do gate de merge (ADR-0003):** nenhum required check novo. A única autoridade de gate é `python3 scripts/ci/check_gate_integrity.py`, com **PASS** registrado para o alvo neste ciclo (não reexecutado por esta nota). Nenhuma alteração de `.specify/validations/`, `.specify/releases/`, workflow, gate, ADR, constituição, PR, merge, tag ou deploy por este documento.
6. **Rollback:** reapontar a app Coolify para a imagem/commit anterior e remover as variáveis temporárias de recuperação, sem force-push; revalidar `/health`, o fluxo OIDC e o login local.

## (d) Estado **[NV]** até deploy

**[NV] até deploy final:** aplicação da mudança no ambiente de produção, reinício/deploy da API, resultado do login com `email='*'` e a credencial de recuperação do administrador real, remoção das env temporárias após o sucesso, medição de `/health`/CORS em `https://flora-api.mdmv.qzz.io`, e CI/proteção/Coolify/Forgejo/aprovações ao vivo. Nenhum deploy foi executado por esta nota.

Verificações esperadas antes de declarar concluído:

1. `python3 scripts/ci/check_gate_integrity.py` → **PASS** (registrado no diagnóstico do alvo; reexecutar no checkout final) e `git diff --check` limpo.
2. Testes Rust cobrindo: `email='*'` **com** `ADMIN_EMAIL=*` → seleção do primeiro usuário ativo (`created_at ASC, id ASC`) e senha verificada; `email='*'` **sem** opt-in → 401, sem escrita; e-mail comum → caminho inalterado. Nenhuma senha/hash em log ou resposta.
3. Build Docker da API e medição local: login wildcard → sessão emitida e `GET /api/v1/tasks` → **200**; segundo fluxo OIDC inalterado quando configurado.
4. Após o deploy final, medir em navegador real o login no FQDN `https://flora.mdmv.qzz.io/`, a navegação autenticada a `/tasks` e `/health`/CORS da API; registrar data/hora, commit e resultado em nova nota append-only.

## Histórico e escopo

Documento novo e append-only. Apenas `STATE.md` e `PROGRESSO.md` receberam apêndices datados; nenhuma linha de ADRs, constituição, `specs/`, `.specify/validations/` ou `.specify/releases/` foi reescrita. Nenhuma credencial foi copiada ou exposta. Afirmações sobre estado remoto/Coolify/DNS/CI não remediadas nesta nota permanecem **[NV]** até reconsulta ao vivo e deploy final.
