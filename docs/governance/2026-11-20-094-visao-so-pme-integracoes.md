# Nota de governança — 2026-11-20 (ciclo 094)

**Assunto:** visão do SE de Flora como "Sistema Operacional" das PME/MDMV, registrada como diretriz NÃO-normativa, e mapa de integrações e agents no alvo.

**Âncora:** branch `feature/094-visao-so-pme` @ `fd25d8f8b`; `.specify/feature.json` → `specs/094-visao-so-pme`. Registro append-only; nenhum validation/release existente tocado.

## (a) Natureza da diretriz

A visão do SE apresenta Flora como ponto único de trabalho e integração das PME/MDMV. Ela conecta sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma), pessoas (sócios, funcionários, clientes) e agents MDMV (Autonoma, dark-factory, marketing, monetization, seller), com dogfooding na MDMV.

Esta visão é registrada como nota de governança **NÃO-normativa**. Não houve emenda constitucional, novo gate, workflow, migration ou código. O gate do alvo continua sendo `python3 scripts/ci/check_gate_integrity.py` sob ADR-0003, sem required check novo.

## (b) Mapa de integrações e agents no alvo

| Integração / agente | Estado no alvo | Evidência |
|---|---|---|
| Coolify | Implementado e verificado | App `9q3wuuwkur2kl27py0si785n`, FQDN `https://flora.mdmv.qzz.io`; `/health` **200**; deploy **finished 21:23:59Z** no commit `cee7b0cda`. |
| Forgejo | Implementado e verificado | Espelho `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`, refs sincronizados; job `mirror` informativo, não required (ADR-0003). |
| Grafana + VictoriaMetrics | Deferido | Nenhum datasource, painel ou exportador materializado no alvo por esta nota. Estado ao vivo fora do alvo: **[NV]**. |
| Uptime Kuma | Deferido | Nenhum monitor materializado no alvo por esta nota. Estado ao vivo fora do alvo: **[NV]**. |
| Autonoma, dark-factory | Ligação documental | Referidos em notas de governança e memória como agentes de ligação da MDMV, inclusive o deploy em Coolify via MDMV-Autonoma. Nenhum código, endpoint ou integração materializada no alvo. |
| marketing, monetization, seller | Ausentes no alvo | Nenhum artefato ou ligação no alvo. Existência e estado fora do alvo: **[NV]**. |

## (c) Produção e advisories observados

- `/health` respondeu **200**.
- Deploy: **finished 21:23:59Z**, commit `cee7b0cda`. A nota de 20:38Z em `STATE.md` e `PROGRESSO.md` registrou "deploy em curso, health **[NV]** até confirmação" no momento da medição; permanece como histórico, sem linha reescrita.
- Advisories de runtime: **0**. Decisão dos ciclos 089, 091, 092 e 093 reafirmada: sem promoção de checagem de advisories a required check.

Deploy e espelho ficam fora do gate de merge (ADR-0003). Esta nota não executou deploy, merge, tag ou force-push; registra apenas o estado observado. **[NV]** estado futuro de CI, proteção de branch, Coolify e Forgejo: reconsultar antes de agir.

## Histórico e escopo

Documento append-only. Nenhuma linha de registros anteriores (STATE.md, PROGRESSO.md, ADRs, constituição, validations/releases) foi reescrita. Nenhum workflow, gate, migration, segredo, PR, merge, tag ou deploy foi alterado por esta nota; nenhuma credencial foi copiada ou exposta.
