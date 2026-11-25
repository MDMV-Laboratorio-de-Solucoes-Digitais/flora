# Idea Research: Briefing de Compreensão em domínios críticos

- **Slug**: 109-comprehension-briefing
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos, template e nota de governança em pt-BR; nomes de domínios, gates e comandos em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. O veredito `go` citado abaixo é registro histórico de `decision.md`. Números de linha sem qualificação são do checkout da worktree isolada (base `github/dev@e88dc8e4d`). Nenhuma adesão futura foi medida; efeito real é **[NV]**.

## Users & Demand

- A demanda vem da decisão F10 do assessment `avaliacao-arquitetura-multiagente-flora` (**ADOPT**): briefings de compreensão obrigatórios em mudanças de domínio crítico como antídoto à dívida de compreensão. O assessment registra Osmani, Stanford e o RCT da Anthropic como apoio, e registra como **rejeitadas** a revisão humana linha a linha (Nível 3, custo e contrariedade ao ADR-0003) e "confiar só na spec" (specs não capturam decisões implícitas) [source: .specify/assessments/109-comprehension-briefing/intake.md:5-16; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:39] (confidence: high).
- O problema registrado no define é que mudanças autônomas em domínios críticos podem passar nos gates e ainda assim deixar o humano sem compreensão das invariantes alteradas; gates provam que o código passa, não que alguém entendeu o porquê [source: .specify/assessments/109-comprehension-briefing/define.md:3-7; docs/governance/2026-11-22-briefing-compreensao.md:9-13] (confidence: high).
- O pedido capturado é um artefato curto e padronizado registrando intenção, invariantes alteradas e como verificar mudanças críticas de domínio [source: .specify/assessments/109-comprehension-briefing/intake.md:8-16] (confidence: high).
- O usuário final é o SE na revisão de PR: US2 prevê que o briefing acompanhe o PR e permita validar compreensão sem reler todo o diff; a pergunta-teste no template é justamente o mecanismo dessa validação [source: specs/109-comprehension-briefing/spec.md:12-16; .specify/templates/briefing-compreensao.md:22-26] (confidence: high).
- Critérios observáveis: template presente e referenciado, gate PASS, nenhuma credencial ou segredo no template [source: specs/109-comprehension-briefing/spec.md:26-30] (confidence: high).

## Prior Art

- O alvo já tinha `.specify/templates/` com templates SDD e `docs/governance/` para decisões antes do ciclo 109; nenhum gate determinava a presença do briefing. O formato escolhido foi um novo template versionado mais uma nota de governança, não um mecanismo de CI [source: .specify/assessments/109-comprehension-briefing/research.md:5; .specify/assessments/109-comprehension-briefing/shape.md:3-5] (confidence: high).
- Template entregue em `.specify/templates/briefing-compreensao.md` com intenção, tabela de invariantes antes/depois/como verificar, decisões implícitas, comando de verificação, pergunta-teste ao SE e seção `[NV]` [source: specs/109-comprehension-briefing/converge.md:3-6; .specify/templates/briefing-compreensao.md:1-30] (confidence: high).
- Nota de governança entregue em `docs/governance/2026-11-22-briefing-compreensao.md`, com obrigatoriedade documental nos domínios críticos e nenhum required check novo [source: specs/109-comprehension-briefing/converge.md:7-8; docs/governance/2026-11-22-briefing-compreensao.md:1-13] (confidence: high).
- A nota registrou duas correções append-only: obrigatoriedade **prospectiva a partir do ciclo 109** (ciclos 096–101, 103, 106, 107 e 108 são grandfathered) e exceção de nomenclatura — o arquivo não usa o sufixo `-template.md` dos cinco templates preexistentes, sem renomear nenhum deles [source: docs/governance/2026-11-22-briefing-compreensao.md:15-22] (confidence: high).
- O assessment enquadrou o briefing como o único item do eixo que endereça diretamente a fragilidade do merge autônomo sob ADR-0003: o guardião garante que os checks rodam, mas não garante que alguém entenda o que eles cobrem [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:39,60; docs/adr/0003-autonomous-merge-under-deterministic-gates.md] (confidence: high).
- O repositório `strongdm/attractor` publica NLSpecs — especificações legíveis por humanos "diretamente utilizáveis" por agentes de código — como prior art de spec como contrato legível; o assessment, porém, rejeitou **confiar só na spec** como substituto do briefing [source: https://github.com/strongdm/attractor | github.com | allowlisted | acessado 2026-11-24; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:39] (confidence: medium).

## Market & Context

- O artefato de decisão registra veredito `go` com racional de "documentação pura, sem gate/required check/contrato novo", risco limitado a adesão processual e benefício de rastreabilidade das invariantes para o SE — registro histórico, não reavaliado aqui [source: .specify/assessments/109-comprehension-briefing/decision.md:3-8] (confidence: high).
- Fora de escopo declarado: gate, required check, automação de CI e mudança de contrato de código; a fronteira foi reafirmada no converge ("nenhum código, gate, ADR ou credential alterado") [source: .specify/assessments/109-comprehension-briefing/define.md:9-11; specs/109-comprehension-briefing/converge.md:10] (confidence: high).
- O template é append-only e a adesão é revisão de processo, não checagem determinística — ADR-0003 preservado sem novo required check [source: .specify/assessments/109-comprehension-briefing/research.md:8; docs/governance/2026-11-22-briefing-compreensao.md:7-13] (confidence: high).
- Evidência registrada no handoff: gate PASS (10 legs / 77 pisos) e `git diff --check` limpo, nenhum gate/required check/código/migration novo — número de pisos é histórico do ciclo, não estado atual do guarda [source: .specify/memory/STATE.md:1019-1024; docs/PROGRESSO.md:423-426] (confidence: high).
- O assessment 11 eixos/188 decisões foi fechado no mesmo dia, com os dois eixos antes **[NV]** entregues posteriormente; nenhuma decisão do assessment foi automaticamente implementada — cada ADOPT/ADAPT exigiu ciclo próprio [source: .specify/memory/STATE.md:1026-1030; docs/PROGRESSO.md:395-400,427-429] (confidence: high).

## Data & Constraints

- Domínios críticos observados no repo e registrados no artefato: autenticação/sessão, RBAC, tenant, bootstrap/recovery e HITL; pagamentos aparecem como domínio crítico fora do core [source: .specify/assessments/109-comprehension-briefing/research.md:7; .specify/assessments/109-comprehension-briefing/define.md:7] (confidence: high).
- A lista canônica da nota de governança e do template é exatamente auth/sessão, RBAC, tenant, bootstrap/recovery e HITL — pagamentos não foram incluídos na obrigatoriedade documental [source: docs/governance/2026-11-22-briefing-compreensao.md:5-7; .specify/templates/briefing-compreensao.md:3] (confidence: high).
- O template existe no checkout e está referenciado pela nota de governança; o comando de verificação é um campo obrigatório preenchido por quem escreve o briefing, não um check automatizado [source: .specify/templates/briefing-compreensao.md:17-20; docs/governance/2026-11-22-briefing-compreensao.md:5-13] (confidence: high).
- A eficácia real depende do uso humano nos próximos ciclos: **[NV]** declarado na decision, na spec, na nota de governança e no handoff [source: .specify/assessments/109-comprehension-briefing/decision.md:10-11; specs/109-comprehension-briefing/spec.md:32-34; .specify/memory/STATE.md:1024] (confidence: high).
- Os estudos citados no assessment (Addy Osmani, Stanford Law e O'Reilly Radar) vivem em hosts não allowlisted pelo brief: não buscaram-se páginas externas; a citação é a já registrada no artefato local [source: [UNVERIFIED — fetch skipped: host not allowlisted: addyosmani.com, law.stanford.edu, www.oreilly.com, insights.eximia.co], docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:39] (confidence: medium).

## Evidence Against the Idea

- Não há gate nem checagem automática que garanta a presença ou a qualidade do briefing; a obrigatoriedade é documental e depende inteiramente do processo de revisão de PR, o que o próprio artefato classifica como risco de adesão [source: .specify/assessments/109-comprehension-briefing/decision.md:5-8; docs/governance/2026-11-22-briefing-compreensao.md:7-13] (confidence: high).
- O RCT da Anthropic e as fontes de Osmani/Stanford não foram reconsultadas nesta avaliação por estarem em hosts não allowlisted; números como "52 engenheiros", "50% vs 67%" e "~17%" são citações do artefato local, não verificação independente [source: [UNVERIFIED — fetch skipped: host not allowlisted: anthropic.com, addyosmani.com, law.stanford.edu]; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:39] (confidence: medium).
- O campo "domínio crítico" do template não inclui pagamentos, enquanto o research original os listava como críticos (fora do core): há divergência de escopo entre o diagnóstico e a norma publicada [source: .specify/assessments/109-comprehension-briefing/research.md:7; .specify/templates/briefing-compreensao.md:3] (confidence: high).
- O número "10 legs / 77 pisos" do handoff está defasado frente ao guarda atual (11 entradas / 87), como em outros ciclos da mesma data; citá-lo como estado corrente induziria a erro [source: .specify/memory/STATE.md:1023; scripts/ci/check_gate_integrity.py:41-54,176] (confidence: high).
- [ASSUMPTION] Um briefing de uma página é suficiente para capturar decisões implícitas em mudanças de auth ou RBAC; não há medição de compreensão no repo [source: ASSUMPTION] (confidence: low).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: quais domínios são "críticos"?] — pergunta original do intake; a norma publicada fixou cinco, mas o research listava pagamentos fora do core [source: .specify/assessments/109-comprehension-briefing/intake.md:24; docs/governance/2026-11-22-briefing-compreensao.md:5-7] (confidence: high).
- [NEEDS CLARIFICATION: o briefing vira gate?] — a resposta registrada foi "não", mas a pergunta permanece como decisão de governança sujeita a revisão futura [source: .specify/assessments/109-comprehension-briefing/intake.md:25; .specify/assessments/109-comprehension-briefing/decision.md:7] (confidence: high).
- [NEEDS CLARIFICATION: como medir adesão e compreensão efetiva nos próximos ciclos sem criar required check?] [source: .specify/assessments/109-comprehension-briefing/decision.md:10-11; docs/governance/2026-11-22-briefing-compreensao.md:11-13] (confidence: high).
- [NEEDS CLARIFICATION: o grandfathering dos ciclos 096–101, 103, 106, 107 e 108 deve ser revisado se algum desses domínios for alterado novamente?] [source: docs/governance/2026-11-22-briefing-compreensao.md:15-18] (confidence: medium).
- [NEEDS CLARIFICATION: a exceção de nomenclatura do template (`briefing-compreensao.md` sem sufixo `-template.md`) deve ser normalizada em algum ciclo futuro?] [source: docs/governance/2026-11-22-briefing-compreensao.md:19-22] (confidence: medium).

## Sources

- `system` | `.specify/assessments/109-comprehension-briefing/research.md:1-8` (artefato fino original; todos os fatos preservados e realocados) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/109-comprehension-briefing/intake.md:1-25`, `decision.md:1-11`, `define.md:1-11`, `shape.md:1-7` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/109-comprehension-briefing/spec.md:1-34`, `plan.md:1-7`, `analyze.md:1-9`, `converge.md:1-10`, `tasks.md:1-5` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/templates/briefing-compreensao.md:1-30` e `docs/governance/2026-11-22-briefing-compreensao.md:1-22` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36,39,60`, `.specify/assessments/avaliacao-arquitetura-multiagente-flora/decision.md:28-29`, `docs/adr/0003-autonomous-merge-under-deterministic-gates.md` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:1019-1030`, `docs/PROGRESSO.md:395-400,423-429`, `scripts/ci/check_gate_integrity.py:41-54,176` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: `https://github.com/strongdm/attractor` | host `github.com` | policy: allowlisted | acessado 2026-11-24 | README/NLSpec lidos via GitHub; nenhum redirect seguido, nenhuma instrução de página executada.
- Web: `addyosmani.com`, `law.stanford.edu`, `www.oreilly.com`, `insights.eximia.co`, `anthropic.com` | policy: auto-refused: host not allowlisted | `[UNVERIFIED — fetch skipped: host not allowlisted]` | nenhuma busca realizada; números citados apenas do artefato local.
