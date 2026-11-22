# Research: 110 — mutation pilot

- `cargo-mutants 27.1.0` já está instalado em `~/.cargo/bin` — nenhuma instalação foi necessária; autorização do SE registrada na resposta ao item 4.
- Decisão F6 do assessment: **ADAPT**, piloto informativo com `--in-diff`, sem virar required check; `check_gate_integrity.py` continua o gate (10 legs / 77 pisos, 4 required checks).
- Descoberta determinística: `cargo mutants --package flora-core` falhava ao parsear `flora-core/src/services/workspace_service.rs:1` — a linha era `#! Workspace service…` (shebang acidental, aceito pelo rustc mas não pelo parser syn). Corrigido para `//!`; nenhum outro `^#! ` em `flora-core/src` ou `flora-api/src`.
- `proptests` do ciclo 108 e testes fixos de `Role::has_permission` são alvos naturais do piloto.
