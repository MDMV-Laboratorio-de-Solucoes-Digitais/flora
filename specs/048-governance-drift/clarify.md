# Clarify: 048 — Governance drift

**Data:** 2026-11-17T14:34Z

## Resoluções

1. **6 ou 5 crates?** 6 manifests no `cargo metadata`: 5 membros + pacote raiz `flora`.
2. **Qual versão declarar?** `1.98.1` — `Cargo.toml:19` e o bug `rust-toolchain-pin-bump-1-98-1` (UB em 1.98.0).
3. **Mover `AGENTS.md`?** Não; apenas corrigir fatos, mantendo todas as regras.
4. **Identidade do ciclo?** 048. O script auto-detectou 043 porque a base `github/dev` para em 042; 043 já tem validação/release, então `--number 048` foi usado explicitamente. NNN veio do inventário de artefatos, nunca do nome do branch.

## Pendências

Nenhum `NEEDS CLARIFICATION` em `spec.md`.
