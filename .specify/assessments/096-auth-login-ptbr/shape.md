# Shape: 096 — login local pt-BR para conta bootstrap

**Data:** 2026-11-21 · **Branch**: `feature/096-auth-login-ptbr@1910d1c7d`

## Forma escolhida

Um fallback local que não desfaz o OIDC: ele coexiste com o fluxo atual e
passa a ser o caminho utilizável quando não há IdP.

1. **Dados (T002–T003)**: migration SQL adiciona duas colunas nullable em
   `users`; `User` e `user_repository` ganham apenas o necessário para ler e
   gravar o hash, com consulta por e-mail normalizado.
2. **Criptografia (T004–T005)**: `PasswordService` encapsula Argon2id
   (`m=19456`, `t=2`, `p=1`) e PHC; `hash()` e `verify()` recebem o segredo,
   retornam booleano e limpam buffers; a verificação sempre executa um hash
   (real ou dummy) antes de responder 401 genérico.
3. **Escrita (T006–T007)**: `BootstrapRequest` ganha `admin_password`; o hash é
   calculado e gravado dentro da transação existente, antes do commit, com a
   validação de complexidade já disponível em
   `flora-core/src/utils/validation.rs:59-105`.
4. **Leitura (T008–T009)**: nova rota
   `POST /api/v1/auth/password/login` usa o mesmo par de
   `session_token`/`expires_in` do callback OIDC e reutiliza a classe de rate
   limit do middleware; a resposta de erro é idêntica para qualquer falha.
5. **Recuperação (T010–T011)**: no startup, se `FLORA_BOOTSTRAP__ADMIN_EMAIL`
   e `ADMIN_PASSWORD` estiverem presentes, localizar a conta e preencher o hash
   somente quando `password_hash IS NULL`. Nada é logado, nada é sobrescrito.
6. **Interface (T012–T013)**: formulário "Entrar" em pt-BR na tela atual,
   chamando a nova rota e preservando o botão SSO; `app.html` passa a
   `lang="pt-BR"`.
7. **Fecho (T014)**: `Cargo.lock`/supply-chain e gates do alvo; rastreabilidade
   T001..T014 em decision/spec/plan/tasks.

## Riscos e mitigação

- **Enumeração de usuário**: mitigada por dummy hash + 401 idêntico + mesmo
  tempo de resposta lógico (FR-002/SC-002); testes cobrem os três casos de
  falha.
- **Credencial em log/resposta**: senha nunca é serializada; `Debug` do payload
  evita o campo ou o mantém ofuscado; nenhum `tracing` com o valor (SC-003).
- **Escrita parcial no bootstrap**: hash gerado e validado antes do `INSERT`,
  tudo na transação existente; falha aborta sem criar organização (FR-003).
- **Sobrescrita por env**: condição `password_hash IS NULL` e checagem
  idempotente; um hash já definido nunca é substituído (FR-005/SC-003).
- **Compatibilidade OIDC**: colunas nullable e nenhuma alteração nas rotas
  `/auth/login` ou `/auth/callback`; contas com `oidc_subject` seguem intactas.
- **Supply chain**: novas crates de password hashing entram com versão pinada e
  lockfile atualizado (T014).

## Entrega

Diff restrito a Rust/migrations/config/frontend/lockfile e artefatos do ciclo.
Verificação local cobre T005, T007, T009, T011, T013 e os gates do alvo (PASS
conforme handoff consolidado). IdP externo, deploy remoto e observações em
produção permanecem **[NV]**.
