# Decision: Wildcard de recuperação com papel e consumo único

- **Slug**: 107-wildcard-one-shot
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | MAJOR D11 autorizado pelo SE: `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` não filtra papel e não se desliga após o primeiro login, deixando janela de escalada indefinida (intake.md:10; problem.md:9). |
| Evidence strength | strong | Papéis built-in `Admin`/`Owner`, FK `roles.organization_id == memberships.organization_id`, `Config` imutável em `AppState.config` e ausência de enumeração evidenciados localmente em research.md:28,35-41; staging/produção e restart multiréplica são [NV]. |
| Value vs. inaction | strong | Sem a correção, o seletor segue elegível após login e restart enquanto o gate permanece PASS, e o próximo item autorizado do INBOX fica bloqueado (problem.md:42; intake.md:19). |
| Feasibility / appetite | adequate | Appetite medium: estado persistente, seleção compartilhada entre startup e login e migration aditiva; concorrência sob latência de replicação e multiréplica não observados (concept.md:11-12). |
| Strategic fit | strong | Correção autorizada pelo SE, determinística e verificável com os gates existentes, sem required check novo (decision.md anterior:11). |
| Risk posture | adequate | Risco concentrado na migration aditiva e no helper SQL compartilhado; restauração de backup anterior ao consumo e expiração do ADR sem remoção das variáveis limitam a garantia (problem.md:11). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: o scorecard sustenta a correção pela validade do problema D11, pela evidência local determinística e pelo alinhamento com a autorização do SE e com os gates existentes. A opção escolhida reduz a janela a um consumo verificável sem criar required check; as alternativas em memória, em `organizations.settings` e em `users` permanecem rejeitadas (decision.md anterior:7; concept.md:15-25). Nenhuma contradição nos artefatos justificaria mudança de veredito: a incerteza restante é operacional, não de direção. Staging, produção, credenciais reais e restart multiréplica permanecem **[NV]**; nenhuma credencial real foi versionada (decision.md anterior:15).

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; as perguntas em aberto são acompanhamento, não bloqueio do handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Restringir o opt-in wildcard de recuperação a papel administrativo built-in e torná-lo one-shot com estado persistente e ADR de expiração.
- **Chosen approach**: Opção A — seletor restrito a `Admin`/`Owner` built-in fora de organização de sistema, marcador persistente de consumo único com 401 genérico (`invalid_credentials`) após o uso, e ADR 0007 com expiração em `2026-11-29T23:59:59Z` condicionada à remoção das variáveis; rejeitadas as opções em memória, em `organizations.settings` e em `users`.
- **In scope / out of scope**: Inclui estado aditivo, helper SQL compartilhado, falha fechada no login e ADR; exclui gate/required checks, novos `#[ignore]`, formato de sessão, rota de login, credenciais reais, deploy, reescrita append-only e qualquer afirmação sobre staging/produção sem observação **[NV]**.
- **Success metrics**: no máximo 1 sessão emitida sob tentativas concorrentes; 0 logins com o seletor após o consumo; apenas `Admin`/`Owner` selecionáveis; 0 enumeração de e-mail/papel em log ou resposta; pisos inalterados (`bootstrap 13`, `auth 25` no guarda atual).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: a expiração do ADR deve ser 7 dias?]; [NEEDS CLARIFICATION: o consumo ocorre antes ou depois da emissão da sessão?]; [NEEDS CLARIFICATION: restart/replica é coberto pelo marcador persistente?]; [NEEDS CLARIFICATION: tentativas concorrentes produzem no máximo uma sessão sob latência de replicação?]; [NEEDS CLARIFICATION: como conciliar `auth 23` citado no registro antigo com o piso atual `auth 25` sem reescrever notas append-only?].
