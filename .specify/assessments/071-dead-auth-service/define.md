# Define: 071 — remover stub morto AuthService

## Problema

Um serviço que sempre falha occupa um campo público em `AppState`, é construído em 7 sites de teste e implementa uma trait sem outro uso. Isso aumenta o número de conceitos que um leitor (humano ou agente) precisa manter e sugere uma capacidade de autenticação que não existe — a real está em outro lugar.

## Requisitos funcionais

- **FR-001**: excluir `flora-core/src/services/auth_service.rs` e remover `mod`/`pub use` de `services/mod.rs`.
- **FR-002**: remover campo, import, inicialização e struct literal de `AppState` em `flora-api/src/state/app_state.rs`.
- **FR-003**: remover o campo dos 7 construtores de teste (`extractors/auth.rs` + 6 contratos), mantendo os demais serviços intactos.
- **FR-004**: preservar `Error::ServiceUnavailable` e a trait `AuthProvider`/`UserInfo` (extensão futura documentada).
- **FR-005**: rastrear em `STATE.md` e artefatos; um arquivo por commit.

## Critérios de sucesso

- **SC-001**: `grep -rn 'AuthService\|auth_service' --include='*.rs'` → **0 ocorrências**.
- **SC-002**: gates do alvo verdes (autoritativo local/`--remote`, fmt/clippy/deny/build/test, frontend); nenhum piso/job alterado.
- **SC-003**: diff restrito aos 11 arquivos declarados + artefatos; zero mudança em comportamento, workflows, migrations, ADRs ou constituição.

## Fora de escopo

Remover `AuthProvider`/`UserInfo` ou `ServiceUnavailable`; mudar handlers de auth; tocar em CI/gates; emenda da constituição.
