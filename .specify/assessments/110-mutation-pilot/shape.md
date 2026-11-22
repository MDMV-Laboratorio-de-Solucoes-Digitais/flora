# Shape: 110 — piloto de mutation testing

- Ferramenta já instalada (`cargo-mutants 27.1.0`); nenhuma instalação nova.
- Escopo: um arquivo (`flora-core/src/models/role.rs`), 42 mutantes listados; execução informativa, sem gate.
- `TMPDIR` precisa apontar para filesystem com espaço: `/tmp` é tmpfs de 2 GB e a cópia do workspace estoura.
- Correção colateral: doc comment `#!` → `//!` em `workspace_service.rs` para o parser syn.
