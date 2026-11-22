# 110 — Piloto informativo de mutation testing

**Data**: 2026-11-22 · **Âncora**: `specs/110-mutation-pilot` · **Gate**: PASS (10 legs / 77 pisos)

Decisão F6 aplicada: `cargo-mutants 27.1.0` já instalado, sem instalação nova e sem job de CI. Piloto em `flora-core/src/models/role.rs`: 42 mutantes → 39 mortos, 1 sobrevivente, 2 inviáveis. O sobrevivente (`==` → `!=` em `Role::add_permission`) foi morto pelo novo teste `add_permission_is_idempotent`; reexecução focalizada: 1/1 caught.

Efeito colateral útil: correção de `#!` → `//!` em `workspace_service.rs:1`, defeito que o rustc tolera como shebang mas que impedia o parser da ferramenta.

Rollback: novo commit de reversão dos dois arquivos, sem force-push. **[NV]**: aplicação em todo o workspace e uso em CI (não autorizados).
