# Release: 096 — Autenticação local (fallback de senha) e login pt-BR

**Ciclo**: 096 · **Data**: 2026-11-20 · **Branch**: `feature/096-auth-login-ptbr` @ `1910d1c7d` · **Baseline**: `1910d1c7d`
**Tipo**: correção de autenticação — credencial local Argon2id, bootstrap e recuperação por env, endpoint de login por senha com rate limiting e tela de login pt-BR. **Nenhuma alteração de gate, required check, constituição, ADR, schema além da migration de credencial ou fluxo OIDC existente.** O login da conta bootstrap em produção segue **[NV]** até o deploy final.

## Entregue

- Migration acrescentando `users.password_hash` e `users.password_updated_at`, sem remover `oidc_subject` nem alterar contas OIDC existentes, idempotente e reversível (FR-001, SC-003).
- `PasswordService` com Argon2id `m=19456, t=2, p=1`, hash em PHC, verificação com dummy hash e resposta `401` genérica para senha inválida e conta inexistente, sem enumeração (FR-002, SC-002).
- `POST /bootstrap` passa a exigir `admin_password` e grava o hash na mesma transação atômica de organização/usuário/roles/sessão; nenhuma credencial em log ou resposta (FR-003, SC-001, SC-003).
- `POST /api/v1/auth/password/login` emite token/sessão no mesmo formato do fluxo OIDC, com rate limiting e `429` uniforme (FR-004, FR-006, SC-001, SC-002).
- Recuperação no startup por `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD`, aplicada apenas quando `password_hash IS NULL`, sem sobrescrever credencial já presente (FR-005, SC-001, SC-003).
- Frontend com "Sign In" local em pt-BR enviando e-mail/senha ao novo endpoint, mantendo o tratamento de `429` e a transição para `/tasks`, e `app.html` com `lang="pt-BR"` e todas as strings da tela traduzidas (FR-007, FR-008, SC-002, SC-003).
- Verificação documental: dependências pinadas, migration, serviço de senha, bootstrap, endpoint, recuperação e frontend com testes **esperado PASS** do handoff; `check_gate_integrity.py` **PASS** (`exit=0`). Detalhe literal em `.specify/validations/096-auth-login-ptbr/validate.md` (R4 **APROVADO**), com **[NV]** para o login em produção até o deploy final (SC-001, SC-002, SC-003; T005, T007, T009, T011, T013, T014).

## SemVer

**Proposta:** MINOR `0.5.0` → `0.6.0`. Motivo: adiciona uma capacidade de autenticação ao produto — credencial local com Argon2id, um novo endpoint de login, bootstrap e recuperação por env, rate limiting e interface pt-BR — sem remover o fluxo OIDC existente. Não é patch: há nova migration, novo contrato de endpoint e mudança observável na tela de login. Não é major: nenhuma API pública existente, contrato de dados, schema anterior ou migration prévia é quebrado; o fluxo OIDC e as sessões atuais permanecem válidos. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T / bug)

- `FR-001` migration de `users.password_hash` e `users.password_updated_at`, idempotente e reversível, preservando contas OIDC (T002, T003, T014).
- `FR-002` Argon2id `m=19456, t=2, p=1` em PHC, dummy hash e `401` genérico sem enumeração de conta (T001, T004, T005).
- `FR-003` `POST /bootstrap` exige `admin_password` e grava o hash na transação atômica, sem credencial em log/resposta (T006, T007).
- `FR-004` `POST /api/v1/auth/password/login` emite token/sessão equivalentes ao fluxo OIDC (T008, T009).
- `FR-005` recuperação por `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` apenas com `password_hash IS NULL` (T010, T011).
- `FR-006` rate limiting no login local com `429` uniforme (T008, T009, T014).
- `FR-007` "Sign In" local em pt-BR enviando e-mail/senha ao endpoint e preservando a transição para `/tasks` (T012, T013).
- `FR-008` `app.html` com `lang="pt-BR"` e strings da tela de login traduzidas, inclusive a falha genérica (T013, T014).
- `T001` dependências `argon2`/`password-hash` · `T002` migration · `T003` model/repository · `T004` PasswordService · `T005` testes PasswordService · `T006` bootstrap · `T007` testes bootstrap · `T008` endpoint de senha + rate limit · `T009` testes auth · `T010` config/startup recovery · `T011` testes recovery · `T012` frontend login pt-BR · `T013` app.html + testes frontend · `T014` lockfile/supply-chain/gates.
- **Bug**: nenhum ID de `.specify/bugs/` é acionado neste ciclo. A falha relatada — a conta `luis.granja@mdmv.com.br` criada via bootstrap não consegue entrar, porque o login só iniciava OIDC sem IdP disponível e não havia fallback de senha — é tratada por FR-001..FR-008 / T001..T014 e registrada na decisão do ciclo; nenhum bug externo é aberto ou reclassificado por este release.

## Evidência

`.specify/validations/096-auth-login-ptbr/validate.md` (R4 **APROVADO**; evidência esperada do handoff: deps/migration/PasswordService/bootstrap/endpoint/recovery/frontend com testes PASS, gate PASS e login em produção **[NV]** até o deploy final) · `specs/096-auth-login-ptbr/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=8/8`, `tasks=14/14`) · `specs/096-auth-login-ptbr/converge.md` (**Converged**, sem novas tarefas) · baseline `1910d1c7d`. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar o estado anterior de autenticação — **reapontar o app/web** e **reverter a migration e a config se necessário**, sem force-push e sem reescrita de histórico:

1. no Coolify, reapontar o app `Flora Web` (FQDN `https://flora.mdmv.qzz.io`) para a imagem/build anterior ao ciclo 096 e executar o redeploy; AGUARDAR o deploy terminar antes de declarar a reversão concluída. A tela volta a exibir o fluxo OIDC em inglês — comportamento anterior esperado, não regressão do rollback;
2. reverter a config de recuperação: remover `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` do ambiente se tiverem sido injetados para este ciclo, preservando as demais variáveis; nenhuma senha deve ser reutilizada ou registrada no processo;
3. reverter a migration se necessário (down de `users.password_hash` e `users.password_updated_at`), depois de garantir que nenhuma credencial local precisa ser preservada — a reversão elimina os hashes; as contas e sessões OIDC existentes permanecem intactas;
4. se as alterações ainda estiverem não commitadas, descartar apenas os arquivos do ciclo 096; se commitadas, usar novo commit de reversão ou `git revert` — nunca `--force`. Restaurar os arquivos de implementação ao estado pré-096 (deps, migration, model/repository, PasswordService, bootstrap, rotas de auth, config de startup e frontend de login); os documentos do ciclo (decision/spec/plan/tasks/analyze/converge, R4 e este release) são registros e podem permanecer como histórico;
5. **não reverter** gate, `check_gate_integrity.py`, required checks, constituição, ADR-0003, fluxo OIDC, PRs, merge ou tags — nenhum deles foi alterado por este ciclo;
6. após o redeploy e a reversão, repetir `python3 scripts/ci/check_gate_integrity.py` (esperado permanecer **PASS**, `exit=0`) e conferir que o login OIDC anterior torna a responder conforme a configuração restaurada. Com a config OIDC original sem IdP, a conta bootstrap voltará a não conseguir entrar — esse é o estado anterior conhecido que este ciclo veio corrigir.

## [NV]

- **Login em produção**: a autenticação de `luis.granja@mdmv.com.br` não foi executada; depende da migration aplicada e das variáveis de recuperação no ambiente de destino e permanece **[NV]** até o deploy final.
- **CI/PR/produção**: nenhum PR, merge, tag, aprovação, proteção de branch ou ambiente de produção verificado; `cargo test`, `npm run check`, build e gate PASS esperados não substituem CI remota para promoção.
- **Credenciais**: nenhuma senha real foi fornecida, lida, registrada ou transmitida; ausência de credencial em log/resposta é evidência esperada dos testes (SC-003), não observação ao vivo.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
