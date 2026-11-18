# Checklist: 067 — C6 canário RLS

> Checklist de **qualidade de requisitos**, não de implementação. `[x]` significa
> "requisito formulado e evidenciado", nunca "feature pronta".

## Requisitos

- [x] FR-001 — estado verificado com `file:line` e distinção ENABLE vs enforcement.
- [x] FR-002 — fases de canário com entrada/saída, restritas a não-produção.
- [x] FR-003 — policies RESTRICTIVE + FORCE fail-closed para 9 tabelas; sem fallback.
- [x] FR-004 — ressalvas de `users`/`organizations`; `BYPASSRLS` proibido.
- [x] FR-005 — rollback sem dado destrutivo + verificação de retorno de linhas.
- [x] FR-006 — itens (a)–(d) do ADR-0006 listados como **[NV]**.
- [x] FR-007 — refresh de docs com contagens por `find`, append-only, `[NV]` remoto.

## Critérios

- [x] SC-001 — gate PASS + CI verde antes do merge (evidência no Bloco 7).
- [x] SC-002 — escopo documental; zero código/workflow/migration/ADR/constituição.
- [x] SC-003 — documento com fases, SQL, rollback, `[NV]` e ADR-0005/0006.
- [x] SC-004 — contagens conferem com `find`; histórico preservado.

## Qualidade

- [x] R1 `decision.md` com `go` antes do BLOCO 6.
- [x] R2/R3 — sem esclarecimentos pendentes; `clarify.md` PASS.
- [x] IDs FR/SC/T únicos e rastreáveis em `validate.md`/`release.md`.
- [x] Nenhuma afirmação remota sem `[NV]`; nenhuma credencial copiada.
- [x] Factory lida, nunca modificada; HEAD `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`.

## Revisão do checklist (obrigatória, realizada)

- [x] Revisado após a escrita: nenhum `[x]` foi concedido por "praticamente feito";
      cada item apoia-se em artefato escrito ou comando executado neste ciclo.
