# Problem Definition: Inventário de bugs desatualizado

- **Slug**: 113-bug-status
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

Três registros em `.specify/bugs/` descrevem status como proposto/planejado embora código e testes já estejam mesclados: D15 (HITL receipt port), OIDC redirect fallback e WCAG-111. Isso infla a dívida aparente e prejudica a priorização do próximo 80/20 (intake.md:5-7; research.md:12). A evidência primária é local e determinística: D15 tem model, trait e adapter presentes e 2 passed em `cargo test -p flora-api hitl_receipt` (research.md:20); OIDC tem a rota `POST /api/v1/auth/password/login` registrada em `flora-api/src/routes/auth.rs:83` e fallback implementado nos ciclos 096–101 (research.md:21); WCAG-111 tem PR #370 mesclado em `dev` (consulta a github.com permitida, acessada em 2026-11-24) e axe "sem achados" em `/tasks` e `/channels` no run 45048150000 (research.md:22-23).

O pedido é registrar status verificado com evidência determinística, preservando o histórico em append-only e marcando **[NV]** o que depende de ambiente remoto (intake.md:9-11; shape.md:3-9). A escolha do ciclo foi registrada no critério 80/20: `3 × 5 × 5 × 5 = 375/625`, contra 144/625 de automação de staging efêmero; deploy em `main` exige autorização SE 023 e ficou fora de escopo (research.md:16). Comportamento em produção, IdP externo e staging real permanece **[NV]** (decision.md:13; research.md:41). Além disso, "axe sem achados" não equivale a conformidade WCAG 2.2 AA: o upstream axe-core estima cobertura média de 57% e classifica resultados incertos como `incomplete` (research.md:47).

## Affected Users & Stakeholders

- **Users**: o mantenedor que prioriza o próximo ciclo e precisa de um inventário de bugs que reflita o que já foi corrigido e verificado, em vez de dívida stale (intake.md:5-14; spec citada em research.md:14).
- **Stakeholders**: os revisores dos artefatos de bug em `.specify/bugs/`, que exigem histórico append-only; o guardião dos required checks e do gate `check_gate_integrity.py`, que não deve ver regra alterada; o SE, cuja autorização SE 023 seria necessária para qualquer deploy (decision.md:13; research.md:16,40).

## Goals

- Rastrear cada um dos três bugs a uma evidência determinística em tabela `bug → evidência → status → [NV]`, sem reescrever linhas antigas (shape.md:7; define.md:5-10).
- Registrar o fechamento de D15 com 2 testes locais do ciclo 106 e o de OIDC com a rota e o teste de fallback dos ciclos 096–101, criando `fix.md`/`test.md` onde o arquivo não existe (define.md:7-8; research.md:20-21).
- Registrar o fechamento de WCAG-111 com PR #370 e o log axe final, preservando o assessment original (define.md:9; research.md:22-23).
- Manter código, gate, required checks e regras inalterados, com SemVer PATCH apenas para documentação (define.md:11-13; decision.md:13).
- Tornar a priorização do próximo 80/20 baseada em status verificado e explicitamente limitado por **[NV]** (intake.md:7; research.md:14).

## Non-Goals

- Não mudar código, fazer deploy, usar credenciais ou tocar required checks (intake.md:15; decision.md:13).
- Não tratar fechamento documental como prova de resolução em produção nem afirmar comportamento remoto não observado: **[NV]** (shape.md:11; research.md:41,46).
- Não reescrever notas append-only nem reconstruir o histórico dos três artefatos de bug (shape.md:7; research.md:24).
- Não promover os PRs #369 e #371 da fila SWE neste ciclo — seu destino é pergunta em aberto (research.md:31,58).

## Success Metrics

- Bugs com status rastreado a evidência local: alvo 3/3; baseline: 0 fechamentos documentais, com os registros ainda em proposto/planejado (intake.md:5-7; checklist 4/4 em research.md:38).
- Testes citados como evidência: D15 2 passed (`cargo test -p flora-api hitl_receipt`), OIDC 1 passed (`cargo test -p flora-api password`); baseline: status sem evidência anexada (research.md:15,20-21).
- Varredura axe registrada: alvo 1 log "sem achados" (run 45048150000) anexado ao assessment WCAG; baseline: 3 violações reportadas no ciclo 111 (research.md:22-23; 112-wcag-fixes/intake.md:5-8).
- Regras/gates/required checks alterados: alvo 0; baseline: 4 required checks por ADR-0003 e guarda com 11 entradas / 87 (o "10 legs / 77 pisos" do ciclo é histórico) (research.md:40).
- Itens com **[NV]** explícito para produção: alvo 3; baseline: unknown — nenhuma verificação remota registrada (research.md:41,46).

## Cost of Inaction

O inventário continua inflado por bugs já corrigidos, desviando o próximo 80/20 para trabalho redundante — o próprio critério do ciclo contrasta 375/625 de fechamento documental com 144/625 de automação de staging (intake.md:7; research.md:16). Sem o registro determinístico, revisores não conseguem ligar status a código e testes mesclados, e a dívida aparente cresce a cada ciclo (research.md:12-15). Fechar sem **[NV]**, por outro lado, trocaria dívida stale por afirmação remota não observada sobre produção, IdP externo ou conformidade AA — risco explicitamente identificado no shape e no upstream axe-core (shape.md:11; research.md:46-49).

## Open Questions

- [NEEDS CLARIFICATION: o status em produção dos três bugs pode ser confirmado com ambiente e credenciais do SE?] — produção, IdP externo e staging seguem **[NV]** (research.md:55).
- [NEEDS CLARIFICATION: a revisão manual de critérios WCAG não automatizáveis foi registrada em algum artefato além do resumo axe "sem achados"?] (research.md:56).
- [NEEDS CLARIFICATION: `544a075ca` deve ser registrado como head do PR #370 ou como âncora da branch do ciclo 113?] (research.md:57).
- [NEEDS CLARIFICATION: os PRs #369 e #371 foram posteriormente promovidos, e isso altera o inventário de dívida que motivou o ciclo?] (research.md:58,31).
- [NEEDS CLARIFICATION: como reconciliar "10 legs / 77 pisos" com o guarda atual sem reescrever notas append-only?] (research.md:59).
