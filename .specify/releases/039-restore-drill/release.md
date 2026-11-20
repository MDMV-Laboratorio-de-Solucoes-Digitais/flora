# Release: 039 — Restore drill (NV-1 do Seed backup)

**Ciclo:** 039 · **Data:** 2026-11-17 · **Baseline:** `github/dev@c177caa46` · **Tipo:** evidência
(local, sem código, sem deploy, sem tag).

## Entregue

- Restore drill executado de ponta a ponta com o runbook `032` (B1–B9 todos PASS).
- NV-1 do Seed backup (sem restore executado) **fechado localmente**.
- Evidência redacted e rastreável em `.specify/validations/039-restore-drill/validate.md`;
  artefatos brutos em `$HOME/.local/state/flora/restore-drill/` (fora do repo).

## Decisão 80/20 que originou o ciclo

Ver `docs/research/2026-11-17-039-restore-drill-80-20.md`. Resumo: com código, gates e
toolchain já estabilizados (ciclos 035–038), o gargalo remanescente da tese "VPS barata"
não é implementação, é **ausência de evidência executada de restore e de limites**. O
maior retorno por menor esforço/risco foi converter planos (030/031/032) em evidência,
sem tocar em comportamento de tenancy, ADRs ou required checks.

## O que NÃO foi feito (autonomia preservada)

- Nenhum deploy, tag, release, segredo ou serviço pago.
- Nenhuma mudança de required checks/protections/CODEOWNERS.
- Nenhuma alteração de ADR ou constituição.
- Nenhum load test/OOM, nenhum dump de cluster de outro agente, nenhuma porta 3000/5432/5433
  tocada. Containers `floraaudit012-*` e `tirateima_fc-db-1` intactos.

## Follow-ups (fora do escopo deste ciclo)

1. Variante dois clusters (NV-5) e backup off-host (NV-2) — exigem destino/credenciais (SE).
2. Converter NV-3/loaded footprint em evidência com gerador off-box em 2 vCPU/4 GB (SE/host).
3. Corrigir drift documental identificado: `ROADMAP.md` marca 038 "em preparação" e 035
   "candidato" Embora 035–038 estejam mergeados em `dev@c177caa46`; `OBJETIVO.md` cita 33
   specs (há 34). Feito em commit separado de docs, não neste release.
