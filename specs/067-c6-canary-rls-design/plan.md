# Implementation Plan: 067 — C6 canário RLS

**Data:** 2026-11-18T15:52Z · **Baseline:** `github/dev@9d5f6656f`

## Abordagem

1. Escrever `docs/research/2026-11-18-067-c6-canary-rls-design.md` com estado
   verificado, pré-condição (wrapper), fases de canário, SQL de referência,
   verificação (ADR-0006 a–d) e rollback — sem aplicar nada.
2. Atualizar `docs/PROGRESSO.md` e `docs/research/index.md` com notas datadas e
   append-only, contagens por `find`, e 8 pisos / 51 total.
3. Registrar apêndices em `STATE.md` (decisão 80/20 já feita antes do BLOCO 6;
   fechamento após o merge).
4. Rodar gates locais: gate determinístico, fmt/clippy/deny/build/test (workspace),
   frontend check/lint/test/build, e verificador isolado do documento.
5. PR para `dev`, CI verde observado ao vivo, merge deliberado, limpeza da worktree.

## Riscos

Nulo em runtime: nenhum código/workflow/migration/ADR é alterado. Risco de
documentação (afirmações remotos) mitigado com **[NV]** e evidência local.

## Verificação

`grep` de ausências (policies/FORCE no runtime), gate, cargo/bun, verificador
isolado lendo ADR-0005/0006 + documento, e CI ao vivo.

## Fora do plano

Código, workflows, migrações, ADRs, constituição, produção, credenciais, tags,
auto-merge, force-push.
