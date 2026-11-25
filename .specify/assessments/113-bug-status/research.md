# Idea Research: fechamento documental de bugs stale

- **Slug**: 113-bug-status
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos, bugs e specs em pt-BR; nomes de rotas, testes e gates em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. O veredito `go` citado abaixo é registro histórico de `decision.md`. Números de linha sem qualificação são do checkout da worktree isolada (base `github/dev@e88dc8e4d`). Nenhum comando foi reexecutado nesta leitura; contagens são evidência registrada nos artefatos.

## Users & Demand

- A demanda é documental: três registros em `.specify/bugs/` descrevem status como proposto/planejado embora código e testes já estejam mesclados — D15 (HITL receipt port), OIDC redirect fallback e WCAG-111. Isso infla a dívida aparente e prejudica o próximo 80/20 [source: .specify/assessments/113-bug-status/intake.md:5-14] (confidence: high).
- O pedido é registrar status verificado com evidência determinística, preservando histórico (append-only) e marcando **[NV]** o que depende de ambiente remoto [source: .specify/assessments/113-bug-status/intake.md:11; .specify/assessments/113-bug-status/shape.md:3-9] (confidence: high).
- O usuário final é o mantenedor que prioriza o próximo ciclo: a spec define que o inventário de bugs deve refletir o que já foi corrigido e verificado, para priorizar com evidência em vez de dívida stale [source: specs/113-bug-status/spec.md:5-7] (confidence: high).
- Critérios observáveis: D15 → `cargo test -p flora-api hitl_receipt` com 2 passed anotado em `fix.md`/`test.md`; OIDC → rota presente e `cargo test -p flora-api password` com 1 passed em novos `fix.md`/`test.md`; WCAG-111 → PR #370 mesclado e axe "sem achados" anotado em `assessment.md`; nenhum código/gate/required check alterado [source: specs/113-bug-status/spec.md:9-17] (confidence: high).
- A escolha do ciclo foi registrada no critério 80/20: fechamento de bugs documentais stale `3 × 5 × 5 × 5 = 375/625`, contra 144/625 de automação de staging efêmero; deploy em `main` exige autorização SE 023 e ficou fora de escopo [source: .specify/memory/STATE.md:1070-1076] (confidence: high).

## Prior Art

- D15: `flora-core/src/models/hitl_receipt.rs`, `flora-core/src/traits/hitl_receipt.rs` e `flora-api/src/adapters/hitl_receipt.rs` estão presentes; `specs/106-hitl-receipt-port/analyze.md` registra o round-trip com 2 passed (`cargo test -p flora-api hitl_receipt`), HMAC canônico e vetor dourado do validador Python [source: .specify/assessments/113-bug-status/research.md:5; specs/106-hitl-receipt-port/analyze.md:1-20; .specify/memory/STATE.md:981-987] (confidence: high).
- OIDC: a rota `POST /api/v1/auth/password/login` existe — o artefato a cita em `flora-api/src/routes/mod.rs` (no checkout atual, o comentário está em `mod.rs:32` e o registro da rota em `flora-api/src/routes/auth.rs:83`) — e o fallback foi implementado nos ciclos 096–101 [source: .specify/assessments/113-bug-status/research.md:6; flora-api/src/routes/mod.rs:32; flora-api/src/routes/auth.rs:83; .specify/memory/STATE.md:884-899] (confidence: high).
- WCAG-111: PR #370 mesclado em `dev` com o commit `544a075ca` registrado como âncora do ciclo 113; axe "sem achados" em `/tasks` e `/channels` no run 45048150000 [source: .specify/assessments/113-bug-status/research.md:9; .specify/bugs/wcag-axe-2026-11-22/assessment.md:21; .specify/bugs/wcag-axe-2026-11-22/test.md:14; .specify/memory/STATE.md:1080] (confidence: high).
- O PR #370 registra as correções BUG-111-01/02/03 (nomes acessíveis em botões ícone, contraste de desabilitados 5,84:1 no tema claro, `<title>` não vazio), nenhuma regra axe desabilitada e o resumo final "sem achados" em `/tasks` e `/channels` após re-revisão independente [source: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/370 | github.com | allowlisted | acessado 2026-11-24] (confidence: high).
- A abordagem já usada em ciclos anteriores era append-only nos artefatos de bug, com tabela `bug → evidência → status → [NV]`, sem reescrever linhas antigas; quando o arquivo não existia (OIDC), ele era criado como registro de fechamento [source: .specify/assessments/113-bug-status/shape.md:3-9; .specify/assessments/113-bug-status/decision.md:13] (confidence: high).
- O upstream axe-core documenta que a automação encontra, em média, 57% dos problemas WCAG e que elementos `incomplete` exigem revisão manual — contexto para a alegação "axe sem achados" [source: https://github.com/dequelabs/axe-core | github.com | allowlisted | acessado 2026-11-24] (confidence: high).

## Market & Context

- O artefato de decisão registra veredito `go` com score 375/625 e a ressalva de que R5 é aplicável apenas como fechamento documental — os bugs já têm fix/test executados em ciclos anteriores. Registro histórico, não reavaliado aqui [source: .specify/assessments/113-bug-status/decision.md:3-11] (confidence: high).
- Restrições declaradas: preservar histórico, marcar **[NV]** para produção e não tocar required checks; fora de escopo: mudança de código, deploy e credenciais [source: .specify/assessments/113-bug-status/decision.md:13; .specify/assessments/113-bug-status/intake.md:15] (confidence: high).
- Evidência registrada no handoff: gate PASS (10 legs / 77 pisos no ciclo), nenhum código/gate/required check alterado; PRs #369 e #371 permaneciam na fila SWE sem merge [source: .specify/memory/STATE.md:1076-1083; docs/PROGRESSO.md:434-438] (confidence: high).
- O contexto imediato eram os ciclos 111/112 (axe no e2e e correção das três violações reais), com BUG-111-01/02/03 registrados em `.specify/bugs/wcag-axe-2026-11-22/`; o ciclo 113 apenas fechou documentalmente o que já havia sido corrigido [source: .specify/memory/STATE.md:1046-1068; .specify/bugs/wcag-axe-2026-11-22/assessment.md:19-21] (confidence: high).
- SemVer PATCH para documentação; nenhuma credencial e nenhum deploy na frente do fechamento [source: .specify/assessments/113-bug-status/define.md:11-13] (confidence: high).

## Data & Constraints

- Cobertura idiomática declarada no artefato original: pt-BR; os nomes de rotas, comandos e testes permanecem em inglês técnico [source: .specify/assessments/113-bug-status/research.md:3] (confidence: high).
- FR-001 a FR-004 rastreiam cada bug a uma evidência determinística e exigem que nenhuma regra/gate/check seja alterado; o checklist registra 4/4 rastreados, 0 clarificações e histórico preservado [source: .specify/assessments/113-bug-status/define.md:5-11; specs/113-bug-status/checklist.md:1-8] (confidence: high).
- Os quatro artefatos de bug envolvidos existem no checkout: `d15-hitl-receipt-contract-unilateral/`, `oidc-redirect-uri-not-allowed/` e `wcag-axe-2026-11-22/` (mais `d11-wildcard-unbounded/`, de outro ciclo); as notas de fechamento foram anexadas em `assessment.md`/`fix.md`/`test.md` [source: .specify/assessments/113-bug-status/research.md:5-9; .specify/bugs/d15-hitl-receipt-contract-unilateral/; .specify/bugs/oidc-redirect-uri-not-allowed/; .specify/bugs/wcag-axe-2026-11-22/assessment.md:21] (confidence: high).
- O gate do alvo é `check_gate_integrity.py`, com 4 required checks por ADR-0003; o número "10 legs / 77 pisos" é o do artefato do ciclo (o guarda atual tem 11 entradas / 87) [source: .specify/assessments/113-bug-status/research.md:6; .specify/memory/STATE.md:1082; scripts/ci/check_gate_integrity.py:27-54,176; docs/adr/0003-autonomous-merge-under-deterministic-gates.md] (confidence: high).
- **[NV]** declarado no artefato original e reiterado em decision/spec/converge: comportamento remoto em produção, IdP externo e staging real [source: .specify/assessments/113-bug-status/research.md:11-13; .specify/assessments/113-bug-status/decision.md:13; specs/113-bug-status/spec.md:19-21; specs/113-bug-status/converge.md:9] (confidence: high).
- Especificações W3C (WCAG 2.2) e a especificação OIDC são fontes primárias naturais para os critérios citados, mas `w3.org` e `openid.net` não são allowlisted pelo brief: não consultadas [source: [UNVERIFIED — fetch skipped: host not allowlisted: w3.org, openid.net]] (confidence: low).

## Evidence Against the Idea

- Fechar um bug documentalmente não prova que ele esteja resolvido em produção; o próprio shape identifica esse risco e o mitiga com **[NV]** explícito e evidência local determinística, sem afirmação remota não observada [source: .specify/assessments/113-bug-status/shape.md:11; .specify/assessments/113-bug-status/research.md:11-13] (confidence: high).
- "Axe sem achados" não equivale a conformidade WCAG 2.2 AA: o upstream axe-core estima cobertura média de 57% e classifica resultados incertos como `incomplete`, exigindo revisão humana; o próprio assessment já advertira que "AA automatizado" é promessa falsa [source: https://github.com/dequelabs/axe-core | github.com | allowlisted | acessado 2026-11-24; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:38,62] (confidence: high).
- O fallback OIDC foi medido localmente nos ciclos 096–101; comportamento com IdP externo real permanece **[NV]**, portanto o registro de fechamento é documental e não uma garantia de autenticação federada [source: .specify/assessments/113-bug-status/research.md:6; .specify/memory/STATE.md:890-899; specs/113-bug-status/spec.md:19-21] (confidence: high).
- O round-trip HITL com a Autônoma ao vivo, transporte HTTP e credenciais reais estavam **[NV]** já no ciclo 106; o fechamento do ciclo 113 cita os 2 testes locais, não uma integração remota verificada [source: .specify/memory/STATE.md:985-987; specs/106-hitl-receipt-port/analyze.md:19-22] (confidence: high).
- O número "10 legs / 77 pisos" citado no artefato está defasado frente ao guarda atual (11 entradas / 87); citá-lo como estado corrente induziria a erro [source: .specify/assessments/113-bug-status/research.md:6; scripts/ci/check_gate_integrity.py:41-54,176] (confidence: high).
- [ASSUMPTION] O commit `544a075ca` citado no artefato como âncora do PR #370 corresponda ao head mesclado em `dev`; no checkout ele aparece como âncora da branch do ciclo 113, e o PR foi verificado separadamente via GitHub [source: ASSUMPTION] (confidence: medium).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: o status em produção dos três bugs pode ser confirmado com ambiente e credenciais do SE?] — produção, IdP externo e staging real continuam **[NV]** [source: .specify/assessments/113-bug-status/research.md:11-13; .specify/assessments/113-bug-status/decision.md:13] (confidence: high).
- [NEEDS CLARIFICATION: a revisão manual de critérios WCAG não automatizáveis foi registrada em algum artefato, além do resumo axe "sem achados"?] [source: https://github.com/dequelabs/axe-core | github.com | allowlisted | acessado 2026-11-24; .specify/bugs/wcag-axe-2026-11-22/assessment.md:21] (confidence: medium).
- [NEEDS CLARIFICATION: o commit `544a075ca` deve ser registrado como head do PR #370 ou como âncora da branch do ciclo 113?] [source: .specify/assessments/113-bug-status/research.md:9; .specify/memory/STATE.md:1080] (confidence: high).
- [NEEDS CLARIFICATION: os PRs #369 e #371 na fila SWE foram posteriormente promovidos, e isso altera o inventário de dívida que motivou o ciclo?] [source: .specify/memory/STATE.md:1076,1097] (confidence: medium).
- [NEEDS CLARIFICATION: como reconciliar o "10 legs / 77 pisos" histórico com o guarda atual sem reescrever notas append-only?] [source: .specify/memory/STATE.md:1082; scripts/ci/check_gate_integrity.py:41-54] (confidence: high).

## Sources

- `system` | `.specify/assessments/113-bug-status/research.md:1-13` (artefato fino original; todos os fatos preservados e realocados) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/113-bug-status/intake.md:1-15`, `decision.md:1-13`, `define.md:1-13`, `shape.md:1-11` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/113-bug-status/spec.md:1-21`, `plan.md:1-12`, `analyze.md:1-12`, `converge.md:1-9`, `tasks.md:1-7`, `checklist.md:1-8` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/bugs/d15-hitl-receipt-contract-unilateral/`, `.specify/bugs/oidc-redirect-uri-not-allowed/`, `.specify/bugs/wcag-axe-2026-11-22/assessment.md:19-21,test.md:14` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/106-hitl-receipt-port/analyze.md:1-22`, `flora-core/src/models/hitl_receipt.rs`, `flora-core/src/traits/hitl_receipt.rs`, `flora-api/src/adapters/hitl_receipt.rs`, `flora-api/src/routes/mod.rs:32`, `flora-api/src/routes/auth.rs:83` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:884-899,981-987,1046-1083,1097`, `docs/PROGRESSO.md:434-438`, `scripts/ci/check_gate_integrity.py:27-54,176`, `docs/adr/0003-autonomous-merge-under-deterministic-gates.md`, `docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:38,62` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/370` | host `github.com` | policy: allowlisted | acessado 2026-11-24 | PR mesclado lido via GitHub; nenhum redirect seguido, nenhuma credencial copiada.
- Web: `https://github.com/dequelabs/axe-core` | host `github.com` | policy: allowlisted | acessado 2026-11-24 | README lido via GitHub; nenhuma instrução de página executada.
- Web: `w3.org`, `openid.net` | policy: auto-refused: host not allowlisted | `[UNVERIFIED — fetch skipped: host not allowlisted]` | nenhuma busca realizada.
