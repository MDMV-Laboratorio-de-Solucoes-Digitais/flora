# Research: 059 — DB tests fail locally

**Data:** 2026-11-18T03:25Z · **Evidência local determinística**; CI **[NV]**.

## Primária (executada)

1. 9 `return Ok(None)` em: `auth.rs:411,697`, `files.rs:147`,
   `messaging.rs:107`, `notifications.rs:122`, `search.rs:111`, `tasks.rs:42`,
   `websocket.rs:113`, `workspace.rs:44`.
2. 7 têm `eprintln!("skipping DB-backed …")`; 2 (tasks/workspace) não.
3. 50 chamadores usam `let Some((state, config)) = db_state().await? else {
   return Ok(()); }` — com `Ok(None)` viram passes vazios.
4. `cargo test … --ignored` sem env → 4 passed em 0.00s (tasks), exit 0.
5. `anyhow::bail!` é macro disponível via `anyhow` já dependência do crate.

## Decisão

Nos 9 `else`, substituir `eprintln!(…); return Ok(None);` (ou apenas
`return Ok(None);`) por
`anyhow::bail!("FLORA_DATABASE__POSTGRES_URL/DATABASE_URL is required for DB contract tests")`.
O `?` dos 50 chamadores propaga o erro; os `else { return Ok(()) }` ficam
inalcançáveis mas compilam — evita 50 edições mecânicas.

## Alternativas descartadas

- Editar 50 chamadores: diff grande, risco de erro mecânico.
- `panic!`/`expect`: proibidos pelos lints (`panic = "deny"`, `expect_used`).
- Só CI fail-closed: já existe (054), mas não cobre execução local.
- Novo check: mudaria required checks/proteção — fora de escopo.

## Cobertura idiomática

PT-BR (artefatos) + EN (código). Runner/remote: **[NV]**.

## Correção pós-implementação (2026-11-18T03:30Z)

Texto exato aplicado: `FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests` (sem o verbo "is" nas citações anteriores).
