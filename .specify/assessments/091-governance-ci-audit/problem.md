# Problem Definition: Coerência entre governança praticada e documentada

- **Slug**: 091-governance-ci-audit
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

Os ciclos 086–090 reforçaram o CI por via determinística (leg RBAC, leg bootstrap, oráculo do gate, espelho Forgejo advisory e bump de `rustls`), e a emenda de 2026-11-20 à ADR-0003, confirmada literalmente pelo SE, estabeleceu a regra vigente: CI/CD GREEN basta para merge autônomo em `dev/`; se o CI não inspirar confiança, o próximo 80/20 imediato é reforçá-lo — não escalar para aprovação humana (`decision.md:12-18`). Falta uma auditoria viva que confirme se a governança praticada coincide com a documentada em cinco pontos: required checks e proteção real de `dev`/`main`, emenda vigente aplicada sem aprovador humano, deploy fora do gate de merge, explicações auditáveis e omissão dos 4 scripts SDD da factory em relatórios (`decision.md:20-29`). O risco a evitar é que a auditoria se torne mutação de processo: promover scripts SDD ou advisories a required check, reintroduzir aprovação humana, publicar credenciais ou tocar `validate/`, `release/` e `STATE.md` (`decision.md:27-29`). A auditoria apoia-se nos registros do próprio ciclo como fonte do que pretende auditar, sem captura reproduzível da leitura `gh` (`research.md:43`); o estado ao vivo é **[NV]** (`decision.md:92-95`).

## Affected Users & Stakeholders

- **Users**: maintainer/SE que registrou a demanda; dono do processo/gate e auditor, papéis nomeados nas cinco User Stories (`research.md:13-15`). [ASSUMPTION] a audiência não é quantificada por telemetria ou tickets neste repo (`research.md:15`).
- **Stakeholders**: dono da ADR-0003 e da proteção de `dev`/`main`; maintainers; operador de deploy; responsável pela redação de relatórios de governança.

## Goals

- Verificar, de forma documental e não mutante, a coincidência entre a governança praticada e a registrada nos cinco pontos identificados.
- Produzir explicações auditáveis que separem explicitamente fatos verificados de itens diferidos **[NV]**, sem reescrever histórico nem reivindicar verde não observado (`decision.md:47-51`).
- Manter a regra de merge autônomo sob CI green e o gate autoritativo `check_gate_integrity.py` como única autoridade (`decision.md:52-58`).
- Manter os 4 scripts SDD da factory fora de relatórios como gate ou evidência de verificação — decisão 089 (`decision.md:56-58`).
- Preservar a fronteira de credenciais: nenhum valor copiado, gravado ou exibido (`research.md:30`).

## Non-Goals

- Criar required check novo, promover advisories ou scripts SDD a gate, ou reintroduzir aprovador humano (`decision.md:61-64`).
- Alterar gate, ADRs, constituição, código de aplicação, PRs, merge, tags ou produção.
- Tocar `validate/` e `release/` produzidos na fase final do próprio ciclo (`decision.md` 091, "Fora").
- Afirmar estado ao vivo de CI, proteção de branch, Coolify, Forgejo ou aprovações sem **[NV]** (`research.md:39`).

## Success Metrics

- Cinco compromissos rastreáveis em artefatos, com fatos verificados separados de itens **[NV]** (baseline: nenhuma auditoria viva registrada antes do ciclo 091 — `decision.md:20-29`).
- Zero required checks novos (baseline: 4 checks registrados — `frontend-build`, `rust-build`, `db-tests`, `dependency-review` — `research.md:36`).
- Zero scripts SDD apresentados como gate ou evidência (baseline: exclusão registrada na decisão 089 — `research.md:33`).
- Gate registrado como autoridade com resultado PASS (baseline: `all 10 db contract floors intact (58 total)` registrado; checkout com 11 pisos/87 — `research.md:38`).

## Cost of Inaction

Sem auditoria viva, uma divergência entre a proteção real e a ADR-0003 pode passar despercebida: o workflow diário de auditoria é advisory e apenas sinaliza falha, sem bloquear merges (`research.md:44`). Itens diferidos (Tauri/OTLP, leg RBAC, emendas pendentes) ficam sem acompanhamento, e explicações inconsistentes podem reintroduzir aprovação humana ou atribuir verde não observado, corroendo a base do merge autônomo.

## Open Questions

- [NEEDS CLARIFICATION: qual é o estado ao vivo dos required checks e da proteção de `dev`/`main` no momento da auditoria, além do registrado como **[NV]**?] (`intake.md:32`)
- [NEEDS CLARIFICATION: qual divergência concreta entre documentação e prática motivou a auditoria, se houver alguma?] (`intake.md:33`)
- [NEEDS CLARIFICATION: qual é o destino e a reavaliação das demais emendas pendentes registradas como escaladas **[NV]** a maintainer?] (`intake.md:34`)
- [NEEDS CLARIFICATION: qual é o estado atual do deploy em produção conduzido pela ligação MDMV-Autonoma, incluindo health e imagem de rollback?] (`intake.md:35`)
- [NEEDS CLARIFICATION: como os itens diferidos (Tauri/OTLP, leg RBAC) devem ser acompanhados até uma eventual reavaliação?] (`intake.md:36`)
- [NEEDS CLARIFICATION: qual audiência e periodicidade são esperadas para o relatório de auditoria viva?] (`intake.md:37`)
