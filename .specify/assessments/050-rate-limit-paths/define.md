# Define: 050 — Rate limit paths

## Problema

O casamento de paths no rate limiter usa rotas sem o prefixo `/api/v1`, então os
limites específicos de login/refresh/invite nunca são aplicados — proteção ineficaz
contra força bruta e spam de convite.

## Requisitos funcionais

- **FR-001**: Classificador puro DEVE reconhecer `/api/v1/auth/login` e variantes legadas.
- **FR-002**: DEVE reconhecer `/api/v1/auth/refresh`.
- **FR-003**: DEVE reconhecer `/api/v1/users/invite` (e formas legadas `/invites`, `/invitations`).
- **FR-004**: Demais paths DEVEM permanecer no limite genérico.
- **FR-005**: Testes unitários DEVEM cobrir os 4 casos sem depender de servidor HTTP.

## Não-objetivos

- Não alterar limites, janelas, estado do limitador ou mensagens de resposta.
- Não mexer em rotas, auth, CORS, constituição, ADRs ou workflows.

## Critérios de sucesso

- `cargo test -p flora-api` verde com os novos testes de classificação.
- Nenhuma regressão nos gates Rust/frontend e `check_gate_integrity.py`.
