# Idea Research: Visão SO-PME

- **Slug**: 094-visao-so-pme
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: medium — a natureza não-normativa e o mapa registrado são bem documentados localmente, mas a visão é sinal único do SE e os sistemas/agents listados têm estado operacional não verificado (**[NV]**).

## Users & Demand

- A visão é atribuída ao SE/dono da MDMV (`@lfgranja`) e registrada como input literal do ciclo: "Flora como Sistema Operacional da PME/MDMV — nota de governança, sem mudança de gate". [source: `.specify/assessments/094-visao-so-pme/intake.md:16-19`; `specs/094-visao-so-pme/spec.md:9`] (confidence: high)
- O destinatário explícito no spec é o "futuro contribuidor", que recebe um mapa único de integrações, pessoas e agents para entender o escopo sem inferir dependências. [source: `specs/094-visao-so-pme/spec.md:28-40`; `intake.md:25`] (confidence: high)
- Não há registro de evento ou discussão que tenha precedido a formulação da visão; o próprio intake marca essa lacuna como pergunta aberta. A demanda é, portanto, uma afirmação isolada de direção, sem sinal de múltiplos usuários ou validação externa. [source: `intake.md:19,23`; `decision.md:3-13` | ASSUMPTION para a ausência de validação externa] (confidence: medium)

## Prior Art

- Notas de governança anteriores já registravam estado de produção e decisões append-only (ciclos 089–093); o ciclo 094 segue o mesmo formato de nota datada, sem criar obrigação nem autoridade. [source: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:1-11`; `.specify/memory/STATE.md:807-815`] (confidence: high)
- Imediatamente antes, os ciclos 091–093 registraram auditoria viva do CI, sincronização do lockfile npm e upgrade de segurança do SvelteKit — todos com a regra reafirmada de **não promover** checagem de advisories a required check; o gate segue `scripts/ci/check_gate_integrity.py` sob ADR-0003. [source: `STATE.md:814`; `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:28`] (confidence: high)
- Coolify e Forgejo já estavam implementados e verificados no alvo (app `9q3wuuwkur2kl27py0si785n`, `/health` 200; espelho Forgejo com refs sincronizados), o que deu base documental para duas linhas do mapa. [source: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:15-18`; `STATE.md:811-813`] (confidence: high)
- A ligação MDMV-Autonoma já aparecia em registros anteriores como condutora do deploy em Coolify, e Autonoma/dark-factory são referidos em notas de governança e memória como agentes de ligação da MDMV. [source: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:21`; `STATE.md:811`] (confidence: high)

## Market & Context

- A nota é explicitamente **NÃO-normativa**: não cria obrigação, não autoriza execução, não altera autoridade; nenhuma emenda constitucional, gate, workflow, migração ou código é criado ou modificado. [source: `decision.md:9-18`; `specs/094-visao-so-pme/spec.md:63-69`] (confidence: high)
- O mapa registrado cobre sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma), pessoas (sócios/funcionários/clientes) e agents MDMV (Autonoma, dark-factory, marketing, monetization, seller), com dogfooding na MDMV. [source: `decision.md:9-13`; `specs/094-visao-so-pme/spec.md:65-66`] (confidence: high)
- O estado registrado no alvo é heterogêneo: Coolify e Forgejo implementados/verificados; Grafana+VictoriaMetrics e Uptime Kuma deferidos; Autonoma/dark-factory como ligação documental; marketing/monetization/seller ausentes no alvo. [source: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:13-22`; `STATE.md:811`] (confidence: high)
- A evidência de produção registrada é datada: `/health` 200, deploy `finished 21:23:59Z` no commit `cee7b0cda`, refs Forgejo sincronizados e 0 advisories de runtime. A nota de 20:38Z que dizia "deploy em curso, health **[NV]** até confirmação" permanece como histórico, sem reescrita. [source: `decision.md:26-31`; `STATE.md:812`; `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:24-30`] (confidence: high para o registro; **[NV]** para estado remoto atual)
- A fronteira do diff é confinada aos 4 artefatos do ciclo, à nota de governança datada e a apêndices append-only em `STATE.md`/`PROGRESSO.md`. [source: `decision.md:20-24`; `specs/094-visao-so-pme/spec.md:81-83`] (confidence: high)

## Data & Constraints

- O registro é append-only e datado `2026-11-20`; `analyze` fechou com `critical=0`, `requirements_covered=5/5`, `sc=3/3`, `tasks=6/6`, `errors=[]`, e `git diff` confinado aos 4 artefatos. [source: `specs/094-visao-so-pme/analyze.md:7-16,24-28`; `specs/094-visao-so-pme/converge.md:9-20`] (confidence: high)
- Nenhuma automação, required check ou pipeline foi criada a partir da nota; qualquer CI futura depende de decisão e evidência próprias, fora do ciclo 094. [source: `specs/094-visao-so-pme/analyze.md:30-36`; `STATE.md:810`] (confidence: high)
- Zero PRs foram abertos ou mesclados no ciclo; nenhum merge, tag, deploy ou promoção foi realizado pela nota. [source: `specs/094-visao-so-pme/converge.md:14`; `specs/094-visao-so-pme/analyze.md:36`] (confidence: high)
- Cobertura idiomática real nos artefatos lidos: **pt-BR_en** — prosa em pt-BR; nomes de sistemas, agents e comandos em inglês, conforme o próprio mapa. [source: artefatos do ciclo e nota de governança lidos diretamente] (confidence: high)
- Integração viva dos sistemas, participação efetiva dos agents e estado futuro de CI/Coolify/Forgejo não foram verificados nesta pesquisa: **[NV]**. [source: `decision.md:33-35`; `specs/094-visao-so-pme/analyze.md:30-36`] (confidence: high)

## Evidence Against the Idea

- A visão nomeia Grafana+VictoriaMetrics e Uptime Kuma como pontos de integração, mas a própria nota de governança os classifica como **deferidos**, sem datasource, painel, exportador ou monitor materializado; o mapa descreve intenção, não capacidade instalada. [source: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:19-20`; `STATE.md:811`] (confidence: high)
- Três agents listados (marketing, monetization, seller) estão **ausentes no alvo**, com existência e estado fora do alvo **[NV]**; incluí-los no mapa pode sugerir operação que os artefatos não suportam. [source: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:22`; `STATE.md:811`] (confidence: high)
- A evidência de produção é fotografia datada de 2026-11-20 (commit `cee7b0cda`), enquanto o ciclo 095 registra mudança de FQDN e de apps em 2026-11-21; reutilizar os valores de 094 como estado corrente seria inválido sem reconsulta. [source: `decision.md:26-31`; `STATE.md:826-830`; `docs/PROGRESSO.md:292-297`] (confidence: high)
- "Ponto único de trabalho e integração" e "dogfooding na MDMV" são afirmações de direção sem plano, prazo, critério de inclusão de novos sistemas ou evidência de adoção nos artefatos — o intake registra cada uma como pergunta aberta. [source: `decision.md:9-13`; `intake.md:23-28` | ASSUMPTION quanto à ausência de adoção] (confidence: medium)
- Embora a nota seja declarada não-normativa, o ato de registrá-la com data e rastreabilidade pode ser lido por contribuidores futuros como prioridade implícita; o risco de "escopo normativo acidental" é citado como risco no plano, não como efeito observado. [source: `specs/094-visao-so-pme/plan.md:25-28`] (confidence: medium)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Qual evento ou discussão precedeu a formulação da visão SO-PME?]
- [NEEDS CLARIFICATION: Como a visão deve ser revisitada ou atualizada sem alterar o registro append-only?]
- [NEEDS CLARIFICATION: Quem são os destinatários pretendidos da nota de governança além dos contribuidores futuros citados em `spec.md:28-35`?]
- [NEEDS CLARIFICATION: A evidência de produção registrada em `decision.md:26-31` deve ser reconsultada antes de servir como referência, dado o estado remoto [NV]?]
- [NEEDS CLARIFICATION: Existe algum plano de dogfooding na MDMV com escopo ou prazo definidos?]
- [NEEDS CLARIFICATION: A inclusão de novos sistemas, pessoas ou agents no mapa segue algum critério além do registro documental?]

## Sources

- `.specify/assessments/094-visao-so-pme/intake.md:1-28` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/assessments/094-visao-so-pme/decision.md:3-36` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/094-visao-so-pme/spec.md:9-92`, `plan.md:1-38`, `tasks.md:3-8` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/094-visao-so-pme/analyze.md:1-40`, `converge.md:6-24` (system; policy: fonte primária local; lido 2026-11-25)
- `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:1-34` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/memory/STATE.md:807-815,826-830` e `docs/PROGRESSO.md:274-281,292-297` (system; policy: registro append-only local; lido 2026-11-25)
- `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` (host: forgejo.mdmv.qzz.io; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
- `https://flora.mdmv.qzz.io/health` (host: flora.mdmv.qzz.io; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
- `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora` (host: github.com; policy: allowlisted; não buscado nesta passagem — estado remoto **[NV]**)
