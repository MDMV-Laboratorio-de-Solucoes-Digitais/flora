# Converge: 110 — piloto de mutation testing

**Data**: 2026-11-22 · **Modo**: append-only · **Status: Converged**

- `flora-core/src/models/role.rs`: correção de `Some(&perm_str)` → `Some(perm_str.as_str())` — dedup de permissões agora funciona (bug real revelado pelo mutante sobrevivente), mais teste `add_permission_is_idempotent`.
- Piloto registrado: 42 mutantes, 39 caught, 1 missed (corrigido), 2 unviable; verificação focalizada 1/1 caught.
- Nenhum required check, gate, CI job ou dependência nova.

Fronteira preservada; sem credenciais.
