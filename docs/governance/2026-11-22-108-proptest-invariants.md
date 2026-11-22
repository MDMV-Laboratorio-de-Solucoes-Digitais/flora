# 108 — proptest para invariantes de domínio

**Data**: 2026-11-22 · **Âncora**: `specs/108-proptest-invariants` · **Gate**: PASS (10 legs / 77 pisos)

Decisão F7 do assessment aplicada: três propriedades puras em `flora-core` (permissões, paginação, seletor wildcard). Dependência já existente; nenhum gate/required check/migration/serviço novo.

Verificação: `cargo test -p flora-core` → 55 passed; fmt/check exit 0; gate PASS. Rollback = novo commit de reversão dos três módulos de teste.

**[NV]**: cobertura adicional é evidência local; nenhuma mudança de produção.
