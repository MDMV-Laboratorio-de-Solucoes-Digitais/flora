# Decision: 108 — proptest

**Data**: 2026-11-22 · **Veredito: go**

Dependência já presente, risco limitado a testes puros, nenhuma mudança de gate. Implementar as três propriedades e validar com `cargo test -p flora-core` e `check_gate_integrity.py`.

**[NV]**: nenhum comportamento de produção é alterado; cobertura de propriedades é evidência local, não garantia externa.
