# Release: 098 — Recuperação determinística da senha do administrador bootstrap via env

**Ciclo**: 098 · **Data**: 2026-11-21 · **Branch**: `feature/098-password-recovery` @ `76f936e24` · **Baseline**: `76f936e24`
**Tipo**: correção de recuperação de credencial — reutilização do par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD`, sobrescrita do hash a cada startup via `set_password_hash`, evento `Startup administrator password recovery` no target `flora_api` e contratos dos quatro estados. **Nenhuma alteração de gate, required check, constituição, ADR, schema, PR/merge/tag, `STATE.md` ou `PROGRESSO*`.** O login da conta `luis.granja@mdmv.com.br` em produção segue **[NV]** até o deploy final.

## Entregue

- Reutilização do par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD`, já presente e correto em produção — **nenhuma chave nova é criada**. `validate()`, o mapeamento Figment (`config/mod.rs:43-64`), `skip_serializing` e o `Debug` com `[REDACTED]` permanecem (FR-002, SC-002).
- `apply_startup_password` passa a sobrescrever o hash **a cada startup** quando o par de env existe, inclusive com hash pré-existente: o guard `password_hash IS NULL` é substituído por `set_password_hash` (UPDATE incondicional por `id`, com `password_updated_at`), usando o mesmo `PasswordService` (Argon2id) (FR-003, SC-001).
- O resultado deixa de ser descartado em `app_state.rs:93`: o evento `Startup administrator password recovery` é emitido no target `flora_api`, não suprimido pelo filtro de produção, sem senha, hash ou dados extras (FR-001, SC-002).
- Contratos `flora-tests`: quatro estados de startup (sem env / e-mail ausente / hash nulo / hash presente), nenhuma senha em log/resposta e sobrescrita com hash pré-existente, preservando `db_startup_password_applies_only_to_users_without_hash` (`:340-380`); login 200 com a senha recuperada e 401 genérico inalterado para senha errada, sem hash e conta inativa (`auth.rs` contratos `:1131,1156,1169`), com rate limit `PasswordLogin` preservado (FR-004, SC-001, SC-002).
- `.env.example` mantém as variáveis de recuperação documentadas sem valor de senha real; nenhuma credencial entrou em repositório, log ou artefato (FR-002, FR-005, SC-002).
- Fronteira verificada: gates do alvo PASS no handoff, nenhum toque em validate/release pré-existentes, `STATE.md`, `PROGRESSO*`, OIDC/allowlist, `POST /bootstrap`, advisory lock ou rate limit; o 401 em produção permanece **[NV]** até o deploy final (FR-005, SC-003; T001..T008).

## SemVer

**Proposta:** PATCH `0.7.0` → `0.7.1`. Motivo: correção de comportamento de uma capacidade existente — a recuperação da senha do administrador via env, introduzida no ciclo 096, passa a sobrescrever o hash no startup e a relatar o evento; nenhuma API pública, variável de ambiente, migration, coluna ou contrato de resposta é acrescentado. Não é MINOR: não há novo endpoint, nova rota, nova chave de configuração nem mudança observável na interface. Não é major: nenhuma API existente, contrato de dados, schema ou rota é quebrado — `POST /api/v1/auth/password/login`, OIDC, `POST /bootstrap`, advisory lock, rate limit e as sessões atuais permanecem válidos. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T / bug)

- `FR-001` recuperação observável: evento `Startup administrator password recovery` no target `flora_api`, substituindo o valor descartado em `app_state.rs:93`, sem senha (T004, T008).
- `FR-002` reutilização de `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD`, já configurados em produção, sem nova chave; validação, Figment e redação preservados (T002, T007).
- `FR-003` sobrescrita do hash a cada startup quando o par de env existe, via `set_password_hash`, inclusive com hash pré-existente (T003, T005).
- `FR-004` contratos dos quatro estados, nenhuma senha em log/resposta, login 200 com a senha recuperada e 401 genérico preservado (T005, T006).
- `FR-005` fronteira preservada: gates do alvo PASS, sem toque em validate/release/STATE/PROGRESSO, OIDC/`POST /bootstrap`/lock/rate limit inalterados e env documentada sem segredo (T001, T007, T008).
- `T001` causa raiz documentada com `file:line` · `T002` contrato de config existente · `T003` sobrescrita no startup · `T004` evento/outcome observável em `flora_api` · `T005` contratos bootstrap · `T006` contratos auth · `T007` `.env.example` e redaction · `T008` fecho de gates, rastreabilidade e fronteira.
- **Bug**: nenhum ID de `.specify/bugs/` é acionado neste ciclo. A falha relatada — login de `luis.granja@mdmv.com.br` com 401 em produção apesar do endpoint e da env corretos — é tratada por FR-001..FR-005 / T001..T008 e registrada em `docs/governance/2026-11-21-098-password-recovery.md`; nenhum bug externo é aberto ou reclassificado por este release.

## Evidência

`.specify/validations/098-password-recovery/validate.md` (R4 **APROVADO**; gate do alvo `check_gate_integrity.py` **PASS** no handoff, demais verificações esperadas do handoff e **[NV]** para login em produção/DB real/CI/deploy até o deploy final) · `specs/098-password-recovery/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3`, `tasks=8/8`) · `specs/098-password-recovery/converge.md` (**Converged**, sem novas tarefas) · `.specify/assessments/098-password-recovery/{intake,research,define,shape}.md` · `docs/governance/2026-11-21-098-password-recovery.md` · baseline `76f936e24`. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar o comportamento anterior de recuperação — **reverter o código e reapontar o app/web**, sem force-push e sem reescrita de histórico:

1. reverter o código para o estado pré-098 em `flora-core` (config/bootstrap, config/mod, repositories/user_repository, traits/repository, services/password_service se necessário), `flora-api` (state/app_state) e `flora-tests` (contratos bootstrap/auth), além de `.env.example`. Se as alterações ainda estiverem não commitadas, descartar apenas os arquivos do ciclo 098; se commitadas, usar novo commit de reversão ou `git revert` — nunca `--force`. Os documentos do ciclo (intake/research/define/shape, governança, spec/plan/tasks, analyze/converge, R4 e este release) são registros e podem permanecer como histórico;
2. reapontar o app `Flora Web` (FQDN `https://flora.mdmv.qzz.io`) e a API (FQDN `https://flora-api.mdmv.qzz.io`, app Coolify `9q3wuuwkur2kl27py0si785n`) para as imagens/builds anteriores ao ciclo 098 e executar o redeploy; **AGUARDAR** o deploy terminar antes de declarar a reversão concluída. Com o código anterior, o login volta a exibir o 401 original — comportamento anterior esperado, não regressão do rollback;
3. o par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` é **temporário** e, conforme `.env.example:32-39`, deve ser removido (com restart) após a recuperação bem-sucedida, para que o segredo não seja reaplicado: nesse caso, remover ambas e reiniciar a API, preservando as demais variáveis (`FLORA_APP__BASE_URL`, `FLORA_OIDC__ALLOWED_REDIRECT_URIS` e as 8 pré-existentes). Se a reversão ocorrer antes da recuperação, mantê-las como estavam — elas já configuravam a produção antes do ciclo 098 e este release não alterou sua configuração. Nenhuma senha deve ser exibida, reutilizada ou registrada no processo;
4. **não reverter** gate, `check_gate_integrity.py`, required checks, constituição, ADR-0003, OIDC/allowlist, `POST /bootstrap`, advisory lock, rate limit, PRs, merge ou tags — nenhum deles foi alterado por este ciclo. Não há migration para desfazer: o hash gravado pelo startup permanece no DB e continuará sendo a credencial ativa; se for desejado voltar ao hash anterior, isso deve ser feito fora deste ciclo, com processo próprio e sem registrar segredos;
5. após o redeploy e a reversão, reexecutar `python3 scripts/ci/check_gate_integrity.py` (esperado permanecer **PASS**, `exit=0`) e conferir `/`, `/config.js` e `/health` → 200. Com o código original, o guard `password_hash IS NULL` volta a ignorar a conta pré-existente e o 401 retorna — esse é o estado conhecido que este ciclo veio corrigir.

## [NV]

- **Login em produção**: o 401 medido e o efeito da sobrescrita no DB real não foram revalidados nesta passagem; o desfecho depende do deploy e da escrita no DB de produção com o par de env já configurado, e permanece **[NV]** até o deploy final (SC-001).
- **Logs em produção**: o filtro que suprime `flora_core` impede medir o evento antigo; o evento `Startup administrator password recovery` no target `flora_api` foi projetado para ser observável, mas sua medição em produção permanece **[NV]** até o deploy (SC-002).
- **CI/PR/produção**: nenhum PR, merge, tag, aprovação, proteção de branch ou ambiente de produção verificado; gate PASS do handoff não substitui CI remota para promoção (SC-003).
- **Credenciais**: nenhuma senha real foi fornecida, lida, registrada ou transmitida; ausência de credencial em log/resposta/artefato é evidência esperada dos contratos e da redação, não observação ao vivo.
- **IdP/allowlist OIDC e Coolify/Forgejo/DNS**: publicação de imagem, resultado do deploy e medição em navegador real do fluxo de login permanecem **[NV]** até o deploy final.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4, no analyze/converge e no handoff do main agent.
