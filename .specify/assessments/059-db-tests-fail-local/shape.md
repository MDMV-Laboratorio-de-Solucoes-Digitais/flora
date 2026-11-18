# Shape: 059 — DB tests fail locally

## Forma escolhida

Nos 9 blocos `else` dos helpers `db_state()`, substituir o retorno `Ok(None)`
(e o `eprintln!` de "skipping" quando presente) por
`anyhow::bail!("FLORA_DATABASE__POSTGRES_URL/DATABASE_URL is required for DB contract tests")`.
A assinatura `Result<Option<...>>` permanece; os 50 `else { return Ok(()) }`
ficam inalcançáveis em tempo de execução.

## Alternativas descartadas

- Mudar assinatura para `Result<(AppState, Config)>`: exigiria 50 edições.
- `panic!`/`expect`: proibidos por lints.
- Remover os `else` inalcançáveis: cosmético, aumentaria diff sem benefício.
- Editar CI: já fail-closed; não resolve o falso positivo local.

## Risco e reversão

Risco baixo/médio: `cargo test --workspace` (sem `--ignored`) não executa
esses testes; a suíte DB do CI continua com env válida. Reversão: `git revert`
dos commits por arquivo. Nenhuma credencial envolvida.

## Verificação

Sem env → exit != 0; com DB isolado → 50 passed; `--list` → 50; gates locais.

## Correção pós-implementação (2026-11-18T03:30Z)

Texto exato aplicado: `FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests` (sem o verbo "is" nas citações anteriores).
