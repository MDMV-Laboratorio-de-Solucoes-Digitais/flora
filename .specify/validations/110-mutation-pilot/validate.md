# Validação R4 — ciclo 110 (piloto de mutation testing)

**Data**: 2026-11-22 · **Veredito: APROVADO**

| Comando | Resultado observado |
|---|---|
| `cargo test -p flora-core` | 56 passed, exit 0 |
| `cargo mutants ... --re 'replace == with != in Role::add_permission'` | 1 mutant tested, **1 caught**, exit 0 |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy -p flora-core --all-targets -- -D warnings` | exit 0 |
| `check_gate_integrity.py` | PASS, 10 legs / 77 pisos, exit 0 |

Piloto completo: 42 mutantes, 39 caught, 1 missed (corrigido), 2 unviable. O mutante sobrevivente revelou bug real de dedup em `Role::add_permission`, corrigido e verificado.

**[NV]**: aplicação em todo o workspace e job de CI não autorizados.

## Conclusão

**R4 APROVADO.** Segue para R7 release.
