# Nota de governança — 2026-11-21 (ciclo 100)

**Assunto:** recuperação da senha do administrador real em produção sem conhecer o e-mail — bootstrap `available=false` (organizações existem), variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` presentes e corretas, mas `apply_startup_password` retornou `applied=false` porque o e-mail configurado não corresponde a nenhum usuário no banco. Decisão: **wildcard explícito** (`FLORA_BOOTSTRAP__ADMIN_EMAIL=*`) para selecionar o administrador real sem depender do e-mail configurado.

**Âncora:** branch `feature/100-admin-recovery` @ `9cf6775a7`; `.specify/feature.json` → `specs/100-admin-recovery`. Snapshot documental `2026-11-21T04:19:25Z`. Registro append-only; nenhuma linha histórica reescrita. Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.

## (a) Evidência — `applied=false` com e-mail ausente

- O ciclo 098 (commit `d703c075a`, PR #332) removeu o guard `password_hash.is_some()` em `apply_startup_password` e passou a sobrescrever o hash via `UPDATE users … WHERE id = $1`, desde que o e-mail configurado correspondesse a um usuário existente. A hipótese registrada na nota 098 era de hash pré-existente com valor divergente; a medição deste ciclo refina o diagnóstico.
- Em produção, o bootstrap reporta **`available=false`** — já existem organizações não-sistema, de modo que o caminho de primeira configuração (ciclos 085/097) não se aplica. O par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` foi confirmado presente e correto (conferido sem exibição de segredo, nota 098), e o login local ainda falha.
- O chamador em `flora-api/src/state/app_state.rs:93-99` registra `tracing::info!(applied = applied, "Startup administrator password recovery")`. A medição em produção trouxe **`applied=false`**. Imediatamente antes, `apply_startup_password` (`flora-core/src/config/bootstrap.rs:96-133`) emitiu o warn `FLORA_BOOTSTRAP__ADMIN_EMAIL does not match an existing user; startup password skipped` (`:112-114`) e retornou `Ok(false)` (`:115`).
- Causa determinística: `find_by_email(&email)` (`:111`) não encontrou usuário para o e-mail configurado. Não é mais o guard de hash pré-existente (removido em 098), nem erro de `Config::load()`/`validate()` (a env é lida e validada). O e-mail configurado simplesmente não existe no banco; o e-mail do administrador real é desconhecido.

## (b) Objetivo

Recuperar a credencial do administrador real sem saber o e-mail, com **opt-in explícito**, sem expor segredos, sem ampliar o gate de merge e sem afetar contas além do alvo selecionado.

## (c) Decisão registrada — wildcard explícito

1. **Opt-in por e-mail curinga.** Com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` (exatamente `*`) e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` configurada, `apply_startup_password` troca a busca por e-mail pela seleção **wildcard**: primeiro usuário ativo (`is_active = true`) ordenado por `created_at ASC`, `LIMIT 1`, sem filtro de role nem de sistema. A atualização continua restrita a um único `user_id`, gravando hash Argon2id e `password_updated_at`, sem log de senha ou hash.
2. **Sem opt-in, comportamento inalterado.** Com qualquer outro valor de `FLORA_BOOTSTRAP__ADMIN_EMAIL`, a busca por e-mail permanece; se não houver correspondência, a função retorna `Ok(false)` sem escrita e o fluxo OIDC segue primário e não relaxado. A ausência da env de senha igualmente desliga a recuperação.
3. **Variáveis temporárias.** O par de recuperação é credencial temporária: deve ser removido da app da API no Coolify (`9q3wuuwkur2kl27py0si785n`) **após** a confirmação do login, evitando que um reinício futuro volte a sobrescrever a senha do administrador. Nenhum valor de senha é versionado ou registrado nesta nota.
4. **Fora do gate de merge (ADR-0003):** nenhum required check novo. A única autoridade de gate é `python3 scripts/ci/check_gate_integrity.py`, com **PASS** registrado para o alvo neste ciclo (não reexecutado por esta nota). Nenhuma alteração de `.specify/validations/`, `.specify/releases/`, workflow, gate, ADR, constituição, PR, merge, tag ou deploy por este documento.
5. **Rollback:** reapontar a app Coolify para a imagem/commit anterior e remover as variáveis temporárias de recuperação, sem force-push; revalidar `/health`, o fluxo OIDC e o login local.

## (d) Estado **[NV]** até deploy

**[NV] até deploy final:** aplicação da mudança no ambiente de produção, reinício/deploy da API, resultado do login com a credencial de recuperação do administrador real, remoção das env temporárias após o sucesso, medição de `/health`/CORS em `https://flora-api.mdmv.qzz.io`, e CI/proteção/Coolify/Forgejo/aprovações ao vivo. Nenhum deploy foi executado por esta nota.

Verificações esperadas antes de declarar concluído:

1. `python3 scripts/ci/check_gate_integrity.py` → **PASS** (registrado no diagnóstico do alvo; reexecutar no checkout final) e `git diff --check` limpo.
2. Testes Rust cobrindo: sem env (desligado), e-mail ausente sem opt-in (`Ok(false)`, sem escrita), **e-mail ausente com `ADMIN_EMAIL=*` (seleção wildcard do primeiro usuário ativo e overwrite)**, e e-mail presente (caminho restrito inalterado). Nenhuma senha/hash em log ou resposta.
3. Build Docker da API e medição local do startup com a env curinga: login local → sessão emitida e `GET /api/v1/tasks` → **200**; segundo fluxo OIDC inalterado quando configurado.
4. Após o deploy final, medir em navegador real o login no FQDN `https://flora.mdmv.qzz.io/`, a navegação autenticada a `/tasks` e `/health`/CORS da API; registrar data/hora, commit e resultado em nova nota append-only.

## Histórico e escopo

Documento novo e append-only. Apenas `STATE.md` e `PROGRESSO.md` receberam apêndices datados; nenhuma linha de ADRs, constituição, `specs/`, `.specify/validations/` ou `.specify/releases/` foi reescrita. Nenhuma credencial foi copiada ou exposta. Afirmações sobre estado remoto/Coolify/DNS/CI não remediadas nesta nota permanecem **[NV]** até reconsulta ao vivo e deploy final.
