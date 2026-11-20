# Decision: 094 — Visão SO-PME

**Verdict:** go (registro documental NÃO-normativo)

**Data:** 2026-11-20 · **Branch:** `feature/094-visao-so-pme` @ `fd25d8f8b`

## Decisão

Registrar, como **nota de governança NÃO-normativa**, a visão do SE de Flora
como "Sistema Operacional" da PME/MDMV: ponto único de trabalho e integração
entre sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma),
pessoas (sócios/funcionários/clientes) e agents MDMV (Autonoma, dark-factory,
marketing, monetization, seller), com dogfooding na MDMV.

O registro é documental e append-only: não cria obrigação, não autoriza
execução e não altera autoridade. Nenhuma emenda constitucional, gate,
workflow, migração ou código é criado ou modificado. O gate continua
`check_gate_integrity.py` sob ADR-0003.

## Delimitadores

- Apenas os 4 artefatos do ciclo 094: `decision.md`, `spec.md`, `plan.md`, `tasks.md`.
- Sem toque em `validate.md`, `release.md`, `STATE.md`, constituição, ADRs, CI ou deploy.
- PR independente; nenhum trabalho alheio tocado.

## Evidência de produção (registrada, não revalidada nesta sessão)

- `/health` 200.
- deploy `finished` 21:23:59Z, commit `cee7b0cda`.
- refs Forgejo sincronizados.
- advisories de runtime: 0.

## NV

Afirmações sobre estado remoto/CI além da evidência registrada: **[NV]**.
Nenhuma autorização é simulada.
