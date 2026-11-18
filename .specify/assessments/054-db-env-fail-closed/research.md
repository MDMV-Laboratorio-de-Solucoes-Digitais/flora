# Research: 054 — DB env fail-closed

**Data:** 2026-11-18T00:29Z · **Evidência local determinística**; CI remota **[NV]**.

## Primária (executada)

1. `grep -rn 'return Ok(None);' flora-tests/src/contract/*.rs` → 9 ocorrências
   em 8 arquivos; `auth.rs` tem dois `db_state()` (`:403`, `:689`), os demais um.
2. `grep -rn 'return Ok(());' flora-tests/src/contract/*.rs` → 50 ocorrências,
   uma por teste `#[ignore]`. Cada uma é precedida de
   `let Some((state, config)) = db_state().await? else { return Ok(()); }`.
3. `rust.yml:89` já usa `set -euo pipefail`; `run_leg` (`:90-104`) soma `N
   passed` e compara ao piso — não verifica env.
4. `rust.yml:64-66` define as duas env no nível do job `db-tests`.
5. `check_gate_integrity.py` não contém `POSTGRES_URL` nem `DATABASE_URL`:
   remover as linhas de env deixaria o guardião PASS.

## Decisão (verificador isolado)

Correção em **2 arquivos**, sem editar os 50 testes nem renomear checks:

- **Runtime** — em `rust.yml`, dentro de `run_leg`, antes do `cargo test`:
  `: "${FLORA_DATABASE__POSTGRES_URL:?…}"` e `: "${DATABASE_URL:?…}"`.
  Com `set -euo pipefail`, uma variável ausente causa erro de expansão fatal,
  o step falha e os 8 legs ficam vermelhos em vez de verdes-vacuos.
- **Guardião** — em `check_gate_integrity.py`, quando `check == "db-tests"`,
  exigir que o bloco do job contenha as duas chaves de env. Remove-las no futuro
  deixa o gate RED antes do merge.

## Alternativas descartadas

- Editar 9 `db_state()` para `anyhow::bail!`: corrige também o falso positivo
  local, mas toca 8 arquivos de teste e deixa 50 `else { return Ok(()) }`
  inalcançáveis — diff maior sem ganho para a decisão de merge.
- Editar 50 chamadores: risco mecânico alto, sem benefício extra.
- `panic!`/`expect`: proibidos pelos lints (`panic = "deny"`, `expect_used`).
- Novo required check: mudaria a proteção de branch (4 contextos).

## Cobertura idiomática

PT-BR (artefatos do alvo) + EN (código/workflow). **[NV]** para CI ao vivo.
