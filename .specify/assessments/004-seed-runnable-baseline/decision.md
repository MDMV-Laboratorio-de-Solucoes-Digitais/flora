# Decision: 004 — Seed Runnable Baseline

**Date:** 2026-11-15  
**Verdict:** **go** (ciclo 004, escopo restrito e verificável)

## Problem validity
**Adequate.** O repositório possui implementação e specs substanciais, mas carece de baseline local reproduzível para dependências e de consolidação documental atualizada; isso impede verificação ponta a ponta confiável e aumenta drift entre artefatos e realidade.

## Evidence strength
**Adequate.** Baseado em inspeção de código/docs/specs/CI, verificações locais executadas em 2026-11-15, baseline do assessment 003 e pesquisa externa em fontes primárias/oficiais (ver `research.md` e `docs/research/2026-11-15-flora-seed-80-20-research.md`). Limitações/itens NV explicitados.

## Value vs inaction
**Strong (para o estágio atual).** Sem ambiente reproduzível e direção consolidada, correções incrementais permanecem difíceis de validar e priorizar; o ciclo reduz custo de iteração e risco de falsos “prontos”.

## Feasibility / appetite
**Adequate.** Entregáveis documentais e de enablement dev (compose/config/instruções/checklists) são de baixo a médio esforço, baixo risco e reversíveis, compatíveis com execução autônoma.

## Strategic fit / risk posture
**Adequate.** Alinhado à constituição (Seed/VPS-first, Postgres-first, rigor/verificação, debloat) e aos não-objetivos (sem paridade Macro, sem dependências core REA/Craft-Anything, sem RE/reuso incompatível). Risco principal: validação incompleta sem Docker/serviços/OIDC no ambiente — mitigado por documentação explícita, artefatos verificáveis e marcação NV.

## Chosen approach (80/20)
1. Consolidar **`OBJETIVO.md`** e **`ROADMAP.md`** com estado real, gaps, não-objetivos e próximos passos priorizados.
2. Registrar **pesquisa** e **assessment 004** (intake/research/decision) apontando baseline local reproduzível como próximo passo de maior alavancagem.
3. Criar/atualizar artefatos de suporte do ciclo (`specs/004-seed-runnable-baseline/*`, `docs/research/*`) e, quando viável sem autorização adicional, materiais dev de baixo risco (ex.: compose dev, instruções) com validação estática/documentada.

## In scope (this cycle)
- Documentação e artefatos de governança/pesquisa/decisão.
- Definição do baseline local Seed e passos de verificação.
- Arquivos de configuração/dev de baixo risco que não envolvam segredos, deploy de produção, merges/releases ou serviços pagos.

## Out of scope / requires explicit authorization
- Deploy/infra de produção, mudanças em proteções remotas, merges/releases.
- Manuseio de segredos/credenciais reais, provisionamento de serviços pagos/proprietários.
- Engenharia reversa não autorizada ou reuso incompatível com AGPL.
- Implementações amplas que não possam ser verificadas com evidência razoável neste ciclo.

## Acceptance / traceability
- Evidências: este diretório, `specs/004-seed-runnable-baseline/*`, `docs/research/2026-11-15-flora-seed-80-20-research.md`, `OBJETIVO.md`, `ROADMAP.md`.
- Próxima etapa recomendada: executar implementação do baseline dev (compose + instruções + smoke) como fatia vertical de enablement, registrando resultados em artefatos de validação do ciclo.
