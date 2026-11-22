# Analyze: 110 — piloto de mutation testing (READ-ONLY)

**Data**: 2026-11-22 · **Baseline**: `f12571cf4`

## Diagnóstico

- `cargo-mutants 27.1.0` já instalado; nenhuma instalação. `--file` precisa de caminho relativo à raiz do workspace (`flora-core/src/models/role.rs`).
- `/tmp` é tmpfs de 2 GB e a cópia do workspace estourava; `TMPDIR=/home/luis/.cache/flora-mutants-tmp` resolveu.
- O parser da ferramenta revelou `flora-core/src/services/workspace_service.rs:1` com `#!` em vez de `//!` (shebang acidental aceito pelo rustc); corrigido, sem outros casos no alvo.
- Piloto: 42 mutantes em `role.rs` → **39 caught, 1 missed, 2 unviable** (16 min). Mutante sobrevivente: `replace == with != in Role::add_permission`.
- Correção: teste `add_permission_is_idempotent` expôs **bug real de dedup** — `role.rs:53` comparava `Option<&str>` com `Option<&String>` (`Some(&perm_str)`), então a igualdade nunca era reconhecida e permissões repetidas eram inseridas. Corrigido para `Some(perm_str.as_str())`. Reexecução focalizada do mutante `==`→`!=`: 1/1 caught, exit 0.

## Checagem

`fr=3/3 sc=3/3 critical=0 go=True`. Gate inalterado (10 legs / 77 pisos).

## [NV]

Aplicação em todo o workspace e job de CI não autorizados; o piloto permanece informativo.
