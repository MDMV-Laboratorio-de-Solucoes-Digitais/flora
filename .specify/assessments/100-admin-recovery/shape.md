# Shape: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Data:** 2026-11-21 · **Branch**: `feature/100-admin-recovery@9cf6775a7bd46395648ca3491536535373e0de6b`

## Forma escolhida

Estender o mecanismo de 098 com um **único token de opt-in**: `ADMIN_EMAIL=*`.
O `*` é inválido como e-mail, então sua presença é uma decisão explícita do
operador, não um acidente de digitação. Com o token ativo, a recuperação escolhe
o primeiro usuário ativo por ordem determinística e sobrescreve o hash de forma
idempotente; sem o token, nada muda em relação a 098 (busca exata, aviso e
`applied = false` se o e-mail não existir). Nenhuma chave de configuração nova é
introduzida.

1. **Causa e rastro (T001)**: fixar em intake/research a cadeia — organizações
   existem (`status.available = false`), env com e-mail inexistente,
   `find_by_email` → `None` → aviso e `applied = false`, sem escrita.
2. **Opt-in (T002 / FR-001)**: `validate()` aceita o token literal `*` após
   `trim()` como valor especial de `admin_email`, apenas quando a senha também
   está presente; todo o resto continua exigindo e-mail válido e senha 15..1024.
   `is_configured()` e a redaction permanecem.
3. **Seleção (T003 / FR-002)**: com `*`, consulta `SELECT id FROM users WHERE
   is_active = true ORDER BY created_at ASC, id ASC LIMIT 1`. Sem usuário ativo:
   aviso e `Ok(false)`. Fora do opt-in: busca exata inalterada, sem fallback.
4. **Escrita (T004 / FR-003)**: UPDATE incondicional de `password_hash` e
   `password_updated_at` (`bootstrap.rs:120-128`), com hash via `PasswordService`
   antes da escrita; `rows_affected() > 0` → `applied = true`. Reaplicações com a
   mesma senha são seguras e estáveis.
5. **Observabilidade (T005 / FR-004)**: `app_state.rs:93-99` mantém `applied` em
   log; `bootstrap.rs:112-116,130` registra e-mail inexistente / nenhum usuário
   ativo / conta sobrescrita via `user_id`, sem senha, hash ou e-mail real.
6. **Testes (T006 / FR-005)**: contratos de bootstrap e auth — opt-in com
   seleção e login 200; e-mail exato inexistente e typo sem fallback;
   nenhum usuário ativo; senha inválida; 401 genérico preservado; nenhum segredo
   em log/resposta.
7. **Env operacional (T007 / FR-005)**: `.env.example:32-39` documenta `*` como
   opt-in explícito, o critério "primeiro usuário ativo" e a remoção das
   variáveis após o login, sem valores reais.
8. **Regressão e fecho (T008)**: gates do alvo PASS no handoff, rastreabilidade
   T001..T008 em decision/spec/plan/tasks e verificação da fronteira proibida
   (`validate/`, `release/`, `STATE.md`, `PROGRESSO*`).

## Riscos e mitigação

- **Conta errada selecionada**: mitigada pelo opt-in explícito (token `*`, que
  não é um e-mail válido), pela ordem determinística (usuário ativo mais antigo)
  e pela documentação em `.env.example`; o operador remove as variáveis após o
  login. Se houver vários usuários ativos, o critério é o `created_at` mais
  antigo — comportamento deliberado e documentado do opt-in.
- **Fallback acidental em typo**: um typo é string diferente de `*` e segue a
  busca exata; `find_by_email` → `None` → aviso e `applied = false`, sem
  sobrescrita (FR-001/FR-002/SC-002).
- **Vazamento de credencial**: senha com `skip_serializing` e `Debug` redactado,
  validação e hash antes da escrita; nenhuma mensagem inclui senha, hash ou o
  e-mail real da conta (FR-004/SC-003).
- **Divergência de contrato de login**: rota, `verify_or_dummy`, 401 genérico,
  JWT e rate limit não mudam — apenas o hash armazenado passa a corresponder à
  senha do env (SC-001).
- **Regressão do bootstrap/OIDC**: `POST /bootstrap`, advisory lock, emissão de
  sessão e allowlist OIDC permanecem intocados (FR-005/SC-003).
- **Produção real**: nenhuma escrita ou login ao vivo é executado neste ciclo;
  o resultado depende do deploy e do DB reais **[NV]**.

## Entrega

Diff restrito a `flora-core/src/config/bootstrap.rs` (token `*` e seleção),
`flora-tests` (contratos/unit tests), `.env.example` e artefatos do ciclo 100.
`flora-api/src/state/app_state.rs` mantém o log de `applied` já existente.
Verificação local cobre T001–T008 e os gates do alvo (PASS conforme handoff
consolidado). DB de produção, login ao vivo, Deploy/Coolify, IdP externo e
allowlist OIDC permanecem **[NV]**.
