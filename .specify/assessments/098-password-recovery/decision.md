# Decision: 098 — recuperação da senha do administrador bootstrap via env

- **Slug**: 098-password-recovery
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/098-password-recovery` @ `76f936e245f70a7bd2735887e7ff8bc5e0c5c06f`
- **Âncora**: `.specify/feature.json` → `specs/098-password-recovery`
- **Evidência**: investigação local determinística + relato de produção (096)
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

Relato de produção: `POST /api/v1/auth/password/login` com
`luis.granja@mdmv.com.br` retorna **401**, embora
`FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` estejam
presentes e corretos no ambiente, e os logs não mostrassem sinal da recuperação
(filtro suprimia `flora_core`). A cadeia explica os três fatos sem falha
intermitente:

1. `flora-api/src/routes/bootstrap.rs:294-295` — o `POST /bootstrap` de 096 já
   exige `admin_password` e grava o hash na transação atômica. A conta de
   produção, criada por esse caminho, **já possui `password_hash` não nulo**.
2. A recuperação anterior parava em `password_hash.is_some()` e no SQL
   `WHERE password_hash IS NULL` (`flora-core/src/repositories/user_repository.rs:93-110`)
   — por design, ignorava exatamente a conta que precisava de recuperação.
3. `flora-api/src/state/app_state.rs` descartava o retorno (`let _applied`) e o
   skip não logava: nenhum rastro da recuperação nos logs, compatível com o
   relato.
4. `flora-api/src/routes/auth.rs:451-460` — `verify_or_dummy()` devolve o mesmo
   `Error::InvalidCredentials` (401 genérico) para senha errada, usuário sem
   hash e conta inativa. Comparando a senha do env com o hash antigo, o 401 é o
   resultado esperado do código.

Não há indício de problema de rede, DNS, migração ausente ou valor de env
incorreto. As variáveis estão corretas; o que faltava era a recuperação agir
sobre uma conta com hash pré-existente.

## Decisão

Registrar o ciclo 098 como **recuperação temporária da senha do administrador
pelas próprias variáveis de env já configuradas**, com cinco compromissos
rastreáveis (FR-001..FR-005), três critérios (SC-001..SC-003) e oito tarefas
(T001..T008, todas concluídas):

1. **Observabilidade (FR-001 / T001, T004)**: `app_state.rs:96-99` registra
   `applied` com "Startup administrator password recovery";
   `bootstrap.rs:111-115,130-132` registra e-mail ausente e sobrescrita, sem
   segredo.
2. **Chave (FR-002 / T002, T007)**: `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
   `FLORA_BOOTSTRAP__ADMIN_PASSWORD` — já presentes em produção — são as
   credenciais temporárias de recuperação; nenhuma chave nova. Redaction,
   validação 15..1024 e documentação de remoção após o uso
   (`.env.example:32-39`).
3. **Sobrescrita (FR-003 / T003)**: `apply_startup_password()` usa
   `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`
   (`bootstrap.rs:120-128`), incondicional — sobrescreve um hash existente a
   cada startup enquanto as variáveis estiverem definidas.
4. **Contratos (FR-004 / T005–T006)**: sem env, e-mail ausente, hash
   pré-existente sobrescrito, sem hash, nenhum segredo em log/resposta; login
   200 com a senha recuperada e 401 genérico preservado.
5. **Fronteira (FR-005 / T008)**: gates do alvo PASS (handoff), nenhum toque em
   `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`; OIDC, `POST /bootstrap`,
   advisory lock e rate limit inalterados.

## Evidência de entrada

- Baseline `76f936e245f70a7bd2735887e7ff8bc5e0c5c06f`, branch
  `feature/098-password-recovery`.
- Relato de produção (401 e env correta) e ausência de sinal nos logs, com o
  filtro `flora_core` suprimido.
- `flora-core/src/config/bootstrap.rs:1-7,21-42,57-81,96-134` — credenciais
  temporárias, config sem chave adicional, validação sem segredo e UPDATE
  incondicional do hash.
- `flora-api/src/state/app_state.rs:96-99` — `applied` logado no startup.
- `flora-api/src/routes/auth.rs:83,443-492` e
  `flora-api/src/middleware/rate_limit.rs:150,195,238-244` — rota alcançável,
  401 genérico e classe `PasswordLogin` inalterada.
- `.env.example:32-39` — instrução de remover as variáveis após a recuperação.
- `flora-tests/src/contract/bootstrap.rs` e `auth.rs:1039-1169` — suítes
  existentes a ajustar à nova semântica, não a substituir.

## Racional

**go.** O 401 é consequência direta de quatro pontos verificáveis: conta com
hash gravado no bootstrap, recuperação que recusava sobrescrever, retorno
descartado sem log e login que comparava com o hash antigo. A correção é isolada:
as mesmas duas variáveis já configuradas passam a sobrescrever o hash a cada
startup, com o resultado logado sem segredo e orientação operacional de removê-las
após o uso. Nenhum gate, contrato de login, JWT, rate limit, lock ou
infraestrutura é alterado. Todos os FR/SC são verificáveis por contratos e
inspeção locais; o diff é reversível e a fronteira é preservada. O custo aceito
é a reaplicação da senha temporária enquanto as variáveis permanecerem definidas
— mitigada pela documentação e pela remoção das variáveis após a recuperação.

## [NV]

- Estado real do DB de produção (hash da conta, valor efetivo das variáveis) e o
  login ao vivo: **[NV]**. Nenhuma credencial foi lida e nenhuma requisição de
  login/bootstrap foi executada contra produção nesta investigação.
- Filtro de logging real, Deploy/Coolify, IdP externo e allowlist OIDC: **[NV]** —
  independentes deste ciclo; o veredito autoriza apenas a recuperação local.
- Correção não executada contra produção neste artefato; o gate do alvo está
  registrado como PASS no handoff consolidado e este documento não reexecutou
  gates, build, testes ou formatters.

## Fora de âmbito

Mudanças em `analyze/`, `converge/`, `validate/`, `release/`,
`.specify/memory/STATE.md` ou `PROGRESSO*`; alteração de required checks,
workflows, ADRs ou constituição; mudança no contrato do `POST /bootstrap`, no
advisory lock, no formato do 401, no JWT ou no rate limit; IDP externo,
allowlist OIDC, nova variável de configuração, interface de "esqueci minha
senha" ou API pública de troca de senha. Nenhuma senha real é fornecida ou
registrada neste ciclo.

## Handoff

`/speckit.specify` — especificar a recuperação como requisitos verificáveis
(FR-001..FR-005 / SC-001..SC-003) e tarefas T001..T008, com as variáveis
`ADMIN_EMAIL`/`ADMIN_PASSWORD` já configuradas como chave temporária,
sobrescrita consistente do hash, resultado observável sem segredo, contratos de
startup e login e fronteira preservada.
