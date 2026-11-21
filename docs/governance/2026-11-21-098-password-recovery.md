# Nota de governança — 2026-11-21 (ciclo 098)

**Assunto:** recuperação da credencial local de `luis.granja@mdmv.com.br` em produção — endpoint `POST /api/v1/auth/password/login` existe e as variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes e corretas, mas o login retorna **401**; decisão de **sobrescrever** o hash no startup quando a env de recuperação estiver configurada.

**Âncora:** branch `feature/098-password-recovery` @ `76f936e24`; `.specify/feature.json` → `specs/098-password-recovery`. Snapshot documental `2026-11-21T02:52:54Z`. Registro append-only; nenhuma linha histórica reescrita. Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.

## (a) Evidência do 401 em produção

- O ciclo 096 publicou o fallback local de senha. Após a promoção e o deploy da API (`https://flora-api.mdmv.qzz.io`, app Coolify `9q3wuuwkur2kl27py0si785n`), a tentativa de login com `luis.granja@mdmv.com.br` pela credencial local continuou retornando **HTTP 401** — medido em produção, conforme registro do estado ao vivo. A rota `POST /api/v1/auth/password/login` **existe** e está registrada em `flora-api/src/routes/auth.rs:83` (`.route("/password/login", post(password_login))`) e o handler em `auth.rs:443`; não é defeito de rota ausente.
- As quatro variáveis injetadas na app da API pelo agente de ligação Coolify (`CoolifyLoginEnvAgent`) foram confirmadas por leitura subsequente via API: `FLORA_BOOTSTRAP__ADMIN_EMAIL`, `FLORA_BOOTSTRAP__ADMIN_PASSWORD`, `FLORA_APP__BASE_URL` e `FLORA_OIDC__ALLOWED_REDIRECT_URIS`. O par de recuperação foi conferido quanto ao valor esperado **sem exibição de segredo**; e-mail e senha da env estão corretos. Nenhuma credencial foi copiada para o repositório, logs, artefatos ou para esta nota. As 8 variáveis pré-existentes permaneceram inalteradas; nenhuma credencial de banco ou JWT foi exposta.
- Os logs do deploy da API não mostraram sinal da recuperação, mas o filtro de logging em produção **suprime `flora_core`**. A ausência do evento `Startup administrator password applied` no log, portanto, **não** é evidência conclusiva de que a função não rodou; a causa é determinística no código (item b).

## (b) Causa determinística — guard `password_hash IS NULL`

- No snapshot inicial deste ciclo, `flora-core/src/config/bootstrap.rs` (`apply_startup_password`) continha:

```rust
if user.password_hash.is_some() {
    return Ok(false);
}
```

e gravava via `set_password_hash_if_absent(user.id, &hash, Utc::now())`. Ou seja, mesmo com `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` válidos e o e-mail encontrado, a recuperação era **ignorada silenciosamente** quando a conta já possuía hash. A conta `luis.granja@mdmv.com.br` é pré-existente (criada pelo bootstrap do ciclo 085/096, FR-012), de modo que o caminho plausível e determinístico é o hash já estar preenchido com valor diferente da senha de recuperação — resultando no **401** medido, com `validate()` aprovando a config e nenhum erro de startup.
- O chamador `flora-api/src/state/app_state.rs:93` já invocava `apply_startup_password(&db_pool, &config.bootstrap, PasswordService::shared())` e `Config::load()` já desserializava o par `FLORA_BOOTSTRAP__*` (confirmado localmente, sem imprimir a senha). Portanto, não havia defeito de configuração, de chamada ou de rota: o bloqueio estava no guard de idempotência do helper de repositório.
- A validação de senha da env (`BootstrapConfig::validate()`) e o `Debug` com `[REDACTED]` permanecem; a hipótese não envolve credencial errada no ambiente.

## (c) Decisão registrada — overwrite controlado no startup

1. **Sobrescrever o hash quando a env de recuperação estiver configurada.** Com `FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` presentes, válidos e o e-mail correspondendo a um usuário existente, `apply_startup_password` deve gravar o novo hash Argon2id **mesmo que já exista um hash**, substituindo a credencial antiga. A atualização é restrita ao `user_id` resolvido pelo e-mail configurado (`UPDATE users SET password_hash = …, password_updated_at = … WHERE id = …`), não a todas as contas.
2. **Sem env, sem mudança.** Se qualquer uma das duas variáveis estiver ausente, a função retorna `false` sem escrita: o fallback local continua desligado e o fluxo OIDC permanece primário e não relaxado. A senha e o hash resultante nunca são registrados em log; apenas um evento de nível `info` com `user_id` e a indicação de overwrite.
3. **Variáveis temporárias.** `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` são credenciais de recuperação: devem ser removidas da app da API Coolify **após** a confirmação do login, evitando que uma reinicialização futura volte a sobrescrever a credencial do administrador. Nenhum valor de senha é versionado ou registrado nesta nota.
4. **Fora do gate de merge (ADR-0003):** nenhum required check novo. A única autoridade de gate é `python3 scripts/ci/check_gate_integrity.py`, com **PASS** registrado para o alvo neste ciclo (não reexecutado por esta nota). Nenhuma alteração de `.specify/validations/`, `.specify/releases/`, workflow, gate, ADR, constituição, PR, merge, tag ou deploy por este documento.
5. **Rollback:** reapontar o app Coolify para a imagem/commit anterior e remover as variáveis temporárias de recuperação, sem force-push; revalidar `/health`, o fluxo OIDC e o login local.

## (d) Estado **[NV]** até deploy

**[NV] até deploy final:** aplicação da mudança no ambiente de produção, reinício/deploy da API, resultado do login com `luis.granja@mdmv.com.br` e a senha de recuperação, remoção das env temporárias após o sucesso, medição de `/health`/CORS em `https://flora-api.mdmv.qzz.io`, e CI/proteção/Coolify/Forgejo/aprovações ao vivo. Nenhum deploy foi executado por esta nota.

Verificações esperadas antes de declarar concluído:

1. `python3 scripts/ci/check_gate_integrity.py` → **PASS** (registrado no diagnóstico do alvo; reexecutar no checkout final) e `git diff --check` limpo.
2. Testes Rust cobrindo os quatro estados: sem env, env com hash nulo, **env com hash pré-existente (overwrite)** e env incompleta; `password_hash` gravado e senha ausente em log/resposta.
3. Build Docker da API e medição local do startup com a env configurada: login local → sessão emitida e `GET /api/v1/tasks` → **200**; segundo fluxo OIDC inalterado quando configurado.
4. Após o deploy final, medir em navegador real o login com a credencial de recuperação em `https://flora.mdmv.qzz.io/`, a navegação autenticada a `/tasks` e `/health`/CORS da API; registrar data/hora, commit e resultado em nova nota append-only.

## Histórico e escopo

Documento novo e append-only. Apenas `STATE.md` e `PROGRESSO.md` receberam apêndices datados; nenhuma linha de ADRs, constituição, `specs/`, `.specify/validations/` ou `.specify/releases/` foi reescrita. Nenhuma credencial foi copiada ou exposta. Afirmações sobre estado remoto/Coolify/DNS/CI não remediadas nesta nota permanecem **[NV]** até reconsulta ao vivo e deploy final.
