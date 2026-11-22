# Define: 110 — piloto de mutation testing

**Problema**: pisos de testes garantem contagem, não sensibilidade; mutações em `Role::has_permission` podem sobreviver aos testes.

**Escopo**: piloto informativo em `flora-core/src/models/role.rs`, sem required check, sem alteração de gate; corrigir defeito de sintaxe encontrado pelo parser (`#!` → `//!`).

**Fora de escopo**: CI job, gate, migration, dependências de runtime, produção.
