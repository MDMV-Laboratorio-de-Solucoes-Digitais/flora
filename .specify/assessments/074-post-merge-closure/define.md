# Define: fechamento pós-merge do ciclo 072

- **Slug**: 074-post-merge-closure · **Definido**: 2026-11-19T04:51Z

## Problema

Quem lê `.specify/memory/STATE.md` hoje encontra, como última palavra sobre o ciclo 072, a afirmação de que o merge permanece `[NV]`/não autorizado. O merge foi observado ao vivo 14 minutos depois. A memória do repo ficou, portanto, desatualizada justamente no evento final do ciclo.

## Afetados

- Agentes e maintainers que usam o STATE como âncora de estado.
- Futuras decisões 80/20, que podem reabrir o ciclo 072 como “em voo”.

## Metas

1. Registrar por apêndice datado o merge de #295 e #294, com merge commits, horários e checks observados.
2. Preservar integralmente a nota anterior de 04:36:59Z e todos os apêndices 071/072.
3. Manter `[NV]` apenas para o que não foi verificado (CI futuro, Forgejo, produção).

## Não-metas

Código, workflows, gates, pisos, ADRs, constituição, worktree 073, novos merges, renomeação de títulos.

## Métricas

- **SC-001** apêndice datado no STATE com `c16be5c3`, `04:50:23Z` e checks pass.
- **SC-002** diff restrito a `.specify/memory/STATE.md` + artefatos 074.
- **SC-003** prefixo byte-a-byte do STATE preservado; apenas acréscimos.
- **SC-004** gate autoritativo permanece PASS; nenhum `[NV]` convertido indevidamente.

## Baseline

Sem a nota, o estado “MERGED” só existe em mensagens de commit e na API do GitHub — não na memória versionada do projeto.
