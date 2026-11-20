# Release: 069 — corrigir nota falsa de dívida no índice de research

**Ciclo:** 069 · **Data:** 2026-11-19T02:44Z · **Baseline:** `github/dev@c8d4fb2e3`
**Tipo:** documentação append-only. **Nenhuma tag, release, deploy ou produção.**

## Entregue

- `docs/research/index.md` — apêndice datado corrigindo a afirmação de dívida 047+: verificação determinística 137 arquivos / 137 links / `missing=[]`, comando reprodutível e distinção entre cobertura e atribuição de ciclo nos títulos. Notas antigas preservadas.
- Artefatos completos: assess (5), spec (8), validate (R4), release (R7) e apêndices em `STATE.md` (decisão 80/20, execução e fechamento pós-merge).

## SemVer

**Proposta:** patch `0.2.1` → `0.2.2` (documentação, sem mudança de API externa). Propostas anteriores permanecem não aplicadas. **Nenhuma tag criada.**

## Changelog (IDs)

- `FR-001` — nota corretiva com evidência e comando reprodutível em `docs/research/index.md`.
- `FR-002` — append-only: 307 inserções, 0 deleções; nenhuma nota antiga reescrita.
- `FR-003` — rastreabilidade em `STATE.md` e 14 commits de um arquivo por commit.
- `SC-001` — verificação 137/137 repetida antes e depois da edição.
- `SC-002` — gates do alvo verdes (local + `--remote`, Rust e frontend), saídas em `validate.md`.
- `SC-003` — diff restrito aos paths declarados; zero mudança em código/workflows/gates/ADRs/constituição/produção.
- `T001`–`T005` — executados; `T005` (PR/merge/higiene) conduzido no BLOCO 7.
- Bug: nenhum em `.specify/bugs/`; origem é decisão autônoma 80/20 em `STATE.md`.

## Evidência

`.specify/validations/069-research-index-truth/validate.md` ·
`specs/069-research-index-truth/converge.md` ·
`specs/069-research-index-truth/analyze.md` ·
verificador isolado `VEREDITO: PASS`.

## Rollback

Reverter os commits do PR em `dev/` com um novo commit de reversão (sem force-push e sem reescrita de histórico). Mudança é puramente documental; sem dados, schema, required checks, deploy ou estado de produção a restaurar.
