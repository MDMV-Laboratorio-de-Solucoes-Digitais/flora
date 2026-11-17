# Release: 047 — Release manifest lockstep

**Ciclo:** 047 · **Data:** 2026-11-17T13:52Z · **Baseline:** `feature/047-release-manifest-lockstep@04bf70bdd`
**Tipo:** correção de workflow; **nenhuma tag, release ou deploy executado.**

## Entregue

- `.github/workflows/release.yml`: contagem de manifests `8` → `6`, nome do step
  atualizado e commentário ligando o número aos membros do workspace.
- Lockstep de versão preservado (segunda cláusula `jq` intacta).
- Decisão 80/20, assess (5 artefatos), spec/clarify/plan/checklist/tasks/analyze,
  converge, validação R4 aprovada e nota de estado verificado em `OBJETIVO.md`/`ROADMAP.md`.

## SemVer

**Proposta:** patch — `0.1.0` → `0.1.1` (mudança de ferramenta de release, sem API
ou comportamento de runtime). Aplicável somente quando uma tag for autorizada;
**nenhuma tag foi criada nesta sessão**.

## Changelog (IDs)

- `FR-001` — asserção aceita os 6 manifests do workspace.
- `FR-002` — verificação de versão em lockstep mantida.
- `FR-003` — escopo restrito a `.github/workflows/release.yml`.
- `FR-004` — required checks intactos (`check_gate_integrity.py` PASS).
- `T001`–`T006` — todas concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; defeito registrado em `intake.md`
  (não foi aberto bug report porque não havia um preexistente).

## Evidência

`.specify/validations/047-release-manifest-lockstep/validate.md` ·
`specs/047-release-manifest-lockstep/converge.md` ·
`specs/047-release-manifest-lockstep/analyze.md`.

## Rollback

`git revert <commit-do-step>` na branch do ciclo; restaura `length == 8` e o nome
anterior do step. Nenhum efeito externo a desfazer: não houve tag, publicação,
deploy, mudança de branch protection nem merge.

## Não feito / NV

CI ao vivo, aprovações, merges, tag e recursos do Forgejo/GitHub: **[NV]**.
PR permanece para revisão deliberada de `.github/workflows/**`; merge depende de
check obrigatório verde observado e aprovador distinto.
