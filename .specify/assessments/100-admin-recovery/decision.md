# Decision: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

- **Slug**: 100-admin-recovery
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/100-admin-recovery` @ `9cf6775a7bd46395648ca3491536535373e0de6b`
- **Âncora**: `.specify/feature.json` → `specs/100-admin-recovery`
- **Evidência**: investigação local determinística + relato de produção (após 098)
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

Relato de produção após o ciclo 098: `GET /api/v1/bootstrap/status` informa
`available = false` (organizações existem),
`FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes
e corretos, mas o startup registra `applied = false` e o aviso de que o e-mail
não corresponde a usuário. A cadeia explica os fatos sem falha intermitente:

1. `flora-core/src/config/bootstrap.rs:102-107` — ambas as variáveis estão
   presentes, logo a recuperação não é ignorada por configuração incompleta.
2. `bootstrap.rs:108-111` — o e-mail é normalizado (`trim().to_lowercase()`) e
   buscado **exatamente** com `repo.find_by_email()`.
3. `bootstrap.rs:112-116` — o e-mail configurado não existe em `users`: aviso
   "does not match an existing user" e `Ok(false)` sem UPDATE. É o ramo
   observado.
4. `flora-api/src/routes/bootstrap.rs` / contrato
   `flora-tests/src/contract/bootstrap.rs:441-483` — com organizações
   pré-existentes, `POST /bootstrap` retorna conflito; não há como recriar o
   administrador.
5. `flora-api/src/state/app_state.rs:93-99` — `applied` é logado; o valor
   `false` é observável e confirma o diagnóstico.
6. `flora-api/src/routes/auth.rs:451-460` — o login permanece com 401 genérico;
   a correção atua apenas no hash armazenado.

Não há indício de problema de rede, DNS, migração ausente ou valor de env
incorreto. O e-mail configurado não é o e-mail do administrador real, que é
desconhecido de quem opera.

## Decisão

Registrar o ciclo 100 como **recuperação do administrador real por opt-in
explícito**, com cinco compromissos rastreáveis (FR-001..FR-005), três critérios
(SC-001..SC-003) e oito tarefas (T001..T008, todas concluídas):

1. **Wildcard explícito (FR-001 / T002)**: o token literal `*` em
   `FLORA_BOOTSTRAP__ADMIN_EMAIL` — aceito por `BootstrapConfig::validate()`
   (`bootstrap.rs:57-81`) somente com a senha presente — é o único valor que
   ativa a seleção; nenhuma chave nova. Todo outro valor continua exigindo
   e-mail válido (FR-001).
2. **Seleção (FR-002 / T003)**: com `*`, `apply_startup_password()` escolhe o
   primeiro usuário ativo (`is_active = true`, `ORDER BY created_at ASC, id ASC
   LIMIT 1`). Fora do opt-in, a busca por e-mail permanece exata e, sem
   correspondência, resulta em aviso e `applied = false` — **sem fallback** em
   typo.
3. **Sobrescrita idempotente (FR-003 / T004)**:
   `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`
   (`bootstrap.rs:120-128`), incondicional quanto ao hash pré-existente, com
   senha validada e hasheada via `PasswordService` antes da escrita;
   reaplicações são estáveis e sem efeitos colaterais.
4. **Log sem segredo (FR-004 / T005)**: `app_state.rs:93-99` registra `applied`;
   `bootstrap.rs:112-116,130` registra o motivo do skip e o `user_id`
   sobrescrito, sem senha, hash ou e-mail real. Redaction e `skip_serializing`
   permanecem.
5. **Testes e env (FR-005 / T006–T008)**: contratos e testes unitários cobrem
   opt-in, seleção, typo sem fallback, nenhum usuário ativo, senha inválida e
   ausência de segredo; `.env.example:32-39` documenta o token e a remoção das
   variáveis após o login. Gates do alvo PASS (handoff), fronteira preservada.

## Evidência de entrada

- Baseline `9cf6775a7bd46395648ca3491536535373e0de6b`, branch
  `feature/100-admin-recovery`.
- Relato de produção: `status.available = false`, env presente, `applied = false`
  e aviso de e-mail inexistente.
- `flora-core/src/config/bootstrap.rs:57-81,96-132` — validação (que hoje
  rejeita `*`), busca exata, aviso, hash e UPDATE incondicional.
- `flora-core/src/traits/repository.rs:16-45` — sem método de "primeiro usuário
  ativo"; a seleção é uma consulta pontual no caminho de recuperação.
- `flora-api/src/state/app_state.rs:93-99` — `applied` logado no startup.
- `flora-tests/src/contract/bootstrap.rs:340-407,441-483` e
  `flora-tests/src/contract/auth.rs:1104-1173` — suítes existentes a ajustar,
  não a substituir.
- `.env.example:32-39` — documentação das variáveis temporárias.

## Racional

**go.** O bloqueio é consequência direta de quatro pontos verificáveis:
organizações existem (bootstrap fechado), o env aponta para um e-mail
inexistente, a busca é exata e o retorno é logado como `false`. A extensão é
isolada: um token que não é e-mail válido (`*`) ativa explicitamente a seleção
do primeiro usuário ativo e a sobrescrita idempotente do hash; qualquer outro
valor — inclusive um typo — não altera o comportamento de 098. Nenhum gate,
contrato de login, JWT, rate limit, lock ou infraestrutura é alterado. Todos os
FR/SC são verificáveis por contratos e inspeção locais; o diff é reversível e a
fronteira é preservada. O custo aceito é a possibilidade de selecionar o usuário
ativo mais antigo quando houver mais de um — mitigada pelo opt-in explícito,
pela ordem determinística e pela orientação de remover as variáveis após o
login.

## [NV]

- Estado real do DB de produção (e-mail do administrador, `is_active`,
  `created_at`, valor efetivo das variáveis) e login ao vivo: **[NV]**. Nenhuma
  credencial foi lida e nenhuma requisição de login/bootstrap foi executada
  contra produção nesta investigação.
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
(FR-001..FR-005 / SC-001..SC-003) e tarefas T001..T008, com o wildcard `*` como
opt-in explícito, seleção determinística do primeiro usuário ativo, sobrescrita
idempotente, nenhum fallback em typo, resultado observável sem segredo, testes e
documentação de env e fronteira preservada.
