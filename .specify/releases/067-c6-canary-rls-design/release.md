# Release: 067 — C6 canário RLS

**Ciclo:** 067 · **Data:** 2026-11-18T15:59Z · **Baseline:** `github/dev@9d5f6656f`
**Tipo:** documentação de desenho; **nenhuma tag, release, deploy ou produção.**

## Entregue

- `docs/research/2026-11-18-067-c6-canary-rls-design.md`: estado verificado,
  pré-condição ADR-0006(a) (descrita, não implementada), 5 fases de canário,
  SQL de referência fail-closed, matriz (a)–(d) com `[NV]`, rollback em 6 passos
  sem dado destrutivo e gate de decisão para promoção (SE).
- Refresh append-only de `docs/PROGRESSO.md` (contagens por `find`, 8 pisos/51,
  PR #285 observado ao vivo) e `docs/research/index.md` (seção 2026-11-18 +
  dívida de indexação retroativa registrada).
- Apêndices datados em `.specify/memory/STATE.md` (decisão 80/20 e execução).
- Artefatos completos do ciclo (assess 5, specs 7, validate, release).

## SemVer

**Proposta:** patch `0.2.0` → `0.2.1` (documentação, sem mudança de API externa;
propostas 0.1.x e a minor 0.2.0 do ciclo 066 permanecem não aplicadas).
Nenhuma tag criada.

## Changelog (IDs)

- `FR-001/002` — estado verificado e fases com critérios de entrada/saída documentados.
- `FR-003/004` — SQL RESTRICTIVE+FORCE fail-closed; `users`/`organizations` fora do canário operacional.
- `FR-005` — rollback em 6 passos, sem `DELETE`/`UPDATE`/`TRUNCATE`, com verificação de retorno de linhas.
- `FR-006` — itens (a)–(d) do ADR-0006 e rounds de backup marcados `[NV]`.
- `FR-007` — docs atualizadas por `find`, append-only, remoto marcado `[NV]` quando aplicável.
- `SC-001`–`SC-004` — gates locais colados; CI observada no Bloco 7.
- `T001`–`T009` — executados/conduzidos neste ciclo; evidência em `validate.md`.
- Bug: nenhum em `.specify/bugs/`; item veio da tabela de escalonamento do SE.

## Evidência

`.specify/validations/067-c6-canary-rls-design/validate.md` ·
`specs/067-c6-canary-rls-design/converge.md` ·
`specs/067-c6-canary-rls-design/analyze.md` ·
verificador isolado PASS.

## Rollback

`git revert` dos commits `docs(research): desenha canario RLS com rollback` e
`docs(067): registra artefatos e estado verificado`. Não há mudança de runtime,
logo o rollback não requer rebuild, deploy, restauração de DB ou troca de
credenciais. Nenhuma policy/FORCE foi aplicada.
