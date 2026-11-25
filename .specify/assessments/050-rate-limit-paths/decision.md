# Decision: Corrigir casamento de paths do rate limiter (050)

- **Slug**: 050-rate-limit-paths
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Teste vermelho executado: `/api/v1/auth/login`, `/api/v1/auth/refresh` e `/api/v1/users/invite` classificados como `GENERAL`; limites 5/60, 20/60 e 10/3600 nunca disparam (intake.md:13-21). |
| Evidence strength | strong | Leitura direta de `rate_limit.rs:155-170`, `routes/mod.rs:25-29`, `routes/auth.rs:72,75` e `UserManager.svelte:18`, com teste vermelho registrado (research.md:5-13). |
| Value vs. inaction | strong | Inação mantém força bruta em login/refresh e spam de convite sem limites específicos, com testes que cobrem só o contador e permitem regressão silenciosa (problem.md:Cost of Inaction). |
| Feasibility / appetite | strong | Appetite small: uma regra pura `classify_endpoint` e 4 testes sem servidor HTTP, sem alterar limites, janelas, estado ou mensagens (concept.md:Option A). |
| Strategic fit | adequate | Reforça a proteção existente sem tocar rotas, autenticação, CORS, constituição ou ADRs; escopo deliberadamente estreito (define.md:Non-Goals). |
| Risk posture | strong | Diff confinado a `middleware/rate_limit.rs` (código + testes), limites/janelas/resposta preservados; comportamento em produção e runner/CI ao vivo **[NV]** (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard é favorável em todas as dimensões: o defeito é reproduzido por teste vermelho, a evidência é leitura direta do código e a opção recomendada é pequena e reversível. O escopo permanece "scoped": nenhuma rota, autoria, CORS, constituição, ADR, workflow ou limite é alterado, e nenhum bug report em aberto exige fluxo bug paralelo (decision.md anterior:Delimitadores). As incertezas sobre formas legadas de path, estado distribuído do limitador e runner/CI ao vivo são carregadas como **[NV]**/**[NEEDS CLARIFICATION:]**, não resolvidas aqui.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; nenhuma pergunta bloqueia a especificação.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: o rate limiter compara paths sem o prefixo `/api/v1` montado pelo router, então login, refresh e invite caem no limite genérico (100/min) em vez dos limites específicos.
- **Chosen approach**: Option A — classificação isolada e testada: extrair `classify_endpoint(path: &str) -> RateLimitClass` com sufixos reais e legados (`/auth/login`, `/auth/refresh`, `/users/invite`, `/invites`, `/invitations`), usada pelo middleware e exercitada por testes unitários sem servidor (concept.md:Recommendation; research.md:Correção planejada).
- **In scope / out of scope**: in — `flora-api/src/middleware/rate_limit.rs` (código + testes); out — limites, janelas, estado do limitador, mensagens de resposta, rotas, autoria, CORS, constituição, ADRs, workflows, C4/C5/C6 e proteção de branch. Nenhum bug report em aberto; nenhum fluxo bug paralelo necessário.
- **Success metrics**: 4 casos classificados corretamente (login, refresh, invite com variantes legadas, general); limites 5/60, 20/60 e 10/3600 aplicados de fato; `cargo test -p flora-api` verde com os novos testes; sem regressão nos gates Rust/frontend e `check_gate_integrity.py`; comportamento em produção **[NV]**.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: formas legadas `/invites`/`/invitations` ainda são usadas por algum cliente?]; [NEEDS CLARIFICATION: os sufixos cobrem todas as variantes que precisam de limite específico?]; [NEEDS CLARIFICATION: estado distribuído do limitador altera a eficácia em produção?] **[NV]**; [NEEDS CLARIFICATION: runner/CI ao vivo será observado em qual PR?] **[NV]**.
