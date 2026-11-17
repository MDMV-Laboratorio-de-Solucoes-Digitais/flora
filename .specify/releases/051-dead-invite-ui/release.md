# Release: 051 — Dead invite UI

**Ciclo:** 051 · **Data:** 2026-11-17T17:35Z · **Baseline:** `github/dev@b3bfda505`
**Tipo:** frontend; **nenhuma tag, release ou deploy executado.**

## Entregue

- Remoção de `UserManager.svelte` e do import/uso em `settings/+page.svelte`.
- Marcação de não implementado em `specs/002-.../spec.md:34` (texto preservado).
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`.

## SemVer

**Proposta:** patch `0.1.0` → `0.1.5` (remoção de superfície morta, sem API externa;
considera propostas 0.1.1–0.1.4 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — settings sem formulário de convite.
- `FR-002` — zero referências a `users/invite` no frontend.
- `FR-003` — drift da spec 002 marcado, histórico preservado.
- `FR-004` — demais componentes intactos.
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum em aberto; C4 veio do ranking 80/20 (`intake.md`).

## Evidência

`.specify/validations/051-dead-invite-ui/validate.md` ·
`specs/051-dead-invite-ui/converge.md` ·
`specs/051-dead-invite-ui/analyze.md`.

## Rollback

`git revert` dos commits do ciclo restaura o componente e o layout anterior e
desfaz a marcação em spec 002. Nenhum efeito externo a desfazer.

## Não feito / NV

Convites, e-mail, token e RBAC: fora de escopo (dependem de decisão de dono).
CI/PR/aprovações ao vivo: **[NV]**. Sem merge/auto-merge/force-push.
