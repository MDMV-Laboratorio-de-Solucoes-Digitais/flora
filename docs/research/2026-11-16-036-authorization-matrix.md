# 036 — Matriz de autorização: correções de CI, PR/merge, docker-staging e toolchain

**Data:** 2026-11-16. **Tipo:** research-only (nenhuma edição de código/CI/deploy).
**Pergunta:** fixar bug do gate de testes do CI e abrir/merger PR exigem autorização explícita? E corrigir `docker-staging.yml` (webhook de deploy) ou fazer bump do toolchain Rust?
**Método:** leitura de fontes normativas (dark-factory hub) + guardrails do Flora (`OBJETIVO.md`/`ROADMAP.md`/bug flow). Citações com `arquivo:linha`.

> **SUPERSEDADO PARCIALMENTE EM 2026-11-16 (ciclo 037):** a conclusão deste documento de que merge em branch protegida exige autorização humana foi revogada pela diretriz do proprietário e registrada em `docs/adr/0003-autonomous-merge-under-deterministic-gates.md`. Push, PR e merge passaram a ser decisão de máquina sob four required checks (frontend-build, rust-build, db-tests, dependency-review), strict + enforce_admins + auto-merge, sem required reviews. As seções abaixo permanecem como registro histórico do raciocínio pré-037; **não são mais normativas para merge**. Continuam válidas as ressalvas sobre deploy (GHCR/Coolify) e release/tag.

## 1) Fontes normativas (autoridade)

**Hub `mdmv-dark-factory`:**
- `AGENTS.md:47` — política 023: decidir autonomamente; escalar ao SE **só** com autorização explícita ou incapacidade (registrar em 024).
- `AGENTS.md:19,45` — gate verde local ≠ aprovação remota; **nunca** tratar gate verde como licença para merge; sem auto-merge cego.
- `AGENTS.md:35` — Authority split (018/025): gates determinísticos são autoritativos; saída de LLM é informativa; não decide merge/release/promotion.
- `.specify/memory/constitution.md:28-30` (Princípio V) — ordem SDD obrigatória; release exige `validate` aprovado.
- `specs/023-.../spec.md:34` (FR-001) — autonomia por padrão; **não pedir confirmação** para ações reversíveis/de baixo risco já no escopo.
- `specs/023-.../spec.md:35` (FR-002) — **autorização explícita obrigatória antes de**: merge em branch protegido / release externo / deploy; mudar protections/admin/rulesets/segredos/credenciais/permissões; custo externo; ações irreversíveis/destrutivas; dados CONFIDENTIAL; comunicação externa oficial; mudança de objetivo/escopo; **bypass de gates/policies**.
- `specs/023-.../spec.md:44-48` (matriz) — autônomo: pesquisa, specs/docs, **criação de branch/PR**, testes locais, análise de impacto, refactor reversível. Autorização explícita: **merge/deploy**, protections, novos tokens, custo, dados sensíveis, comunicação oficial, mudança de escopo, bypass → **escalar antes**.
- `specs/023-.../spec.md:57` — "Merge de PR #22 pode exigir autorização explícita/política de branch — escalar se bloqueado".
- `specs/023-.../contracts/escalation-contract.md:5` — escalar somente se `autorizacao_explicita_requerida` OU `incapacidade_execucao_autonoma`; `:27-29` proíbe escalar por conveniência e agir além da autorização.
- `specs/024-.../registry/escalations.yaml:79-81` (ESC-004) — "política 023 exige autorização explícita para merge"; `:389-396` (ESC-20261114-001) — required check vermelho, "merge humano exigido", "autorização do SE para leitura de logs e/ou re-dispatch", "não automatizar o diagnóstico sem autorização (… não 'consertar o gate' silenciando a suíte)".
- `specs/025-.../spec.md` (FR-002/FR-005/FR-006) — padrão rigoroso; PRs preferenciais para mudanças protegidas; auto-merge/deploy **só** após FR-002 integralmente satisfeito.
- `specs/018-.../contracts/guardrails.md:9` e `spec.md:60` (FR-003) — mudanças de fábrica/gates/policies/config operacional só via **PR versionado + gates determinísticos + aprovação humana**; sem auto-merge cego; proibida auto-modificação de paths protegidos (gates, harness, políticas, contratos) sem isso.
- `.specify/assessments/politica-propostas-padrao-repos-alvo/converge-govAB.md:34` — **aprovação obrigatória** para mudanças em `workflows/protections/CODEOWNERS/toolchain/lockfile`; atualização de pins por PR.
- `specs/022-.../spec.md:31` (FR-005) — auto-merge condicionado a CI+review rigorosos; regras para `workflows/CODEOWNERS/toolchain/lockfiles`.

**Flora (guardrails):**
- `OBJETIVO.md:94` — "Sem declarar 'verde' sem evidência; **sem deploy/produção, merges/releases, segredos ou serviços pagos sem autorização explícita**."
- `OBJETIVO.md:112` — tag/publicação de release exigem autorização explícita.
- `ROADMAP.md:106` — "Fora de escopo até decisão explícita: … Deploy/produção, merges/releases, segredos ou serviços pagos sem autorização."
- `.specify/memory/constitution.md:40` — "Merge to main requires 100% CI pipeline approval. Pull Requests opened via `gh cli`."

## 2) Matriz de autorização

| Ação | Classe 023 | Autorização **explícita** do SE? | Aprovação humana de PR / gates? | Base |
|---|---|---|---|---|
| **Decidir + implementar o fix do gate de testes** (`rust.yml:35` `cargo test` → `cargo test --workspace`) | Autônoma (P1, insistência sanitizada) | **NÃO** — é o 80/20 sancionado "fazer agora"; risco nenhum; não é deploy/merge/segredo/escopo/bypass | Sim, porque `rust.yml` é path protegido: PR versionado + gates determinísticos + aprovação humana | 035-síntese `:9,:43`; OBJETIVO `:86`; ROADMAP `:65`; bug assessment `:40-48`; 023 FR-001; 018 guardrails §6; converge-govAB `:34` |
| **Abrir PR** (em geral, inclusive com o fix acima) | Autônoma | **NÃO** — criação de branch/PR é reversível/de baixo risco; constituição manda abrir via `gh cli` | (o PR é o mecanismo de aprovação) | 023 matriz `:46`; 025 FR-005; Flora constitution `:40` |
| **Mergear PR em `dev`/`main` (protegido)** | Autorização explícita | **SIM** — "merge em branch protegido" é lista nominal do FR-002; gate verde não basta; sem auto-merge cego | Sim (review humano + checks rigorosos) | 023 FR-002 `:35`, matriz `:47`, `:57`; AGENTS `:19,45`; OBJETIVO `:94`; ROADMAP `:106`; 024 ESC-004; 025 FR-006 |
| **Corrigir `docker-staging.yml` (webhook de deploy, `secrets` em `if:`)** | Autorização explícita (deploy) | **SIM — não executar sem permissão.** Correção pode publicar no GHCR e disparar webhook Coolify; guardrail de deploy/produção | Sim | 035-síntese `:74` ("somente com autorização … não execute sem permissão"); OBJETIVO `:84-86`; ROADMAP `:66`; bug assessment `:47` ("fora da fronteira de autorização"); 023 FR-002 deploy; OBJETIVO `:94` |
| **Bump do toolchain Rust (`1.98.0` → `1.98.1`/`1.100.0`)** | Decisão **separada**, não "bloqueada por autorização" | **Não classificado como exigência de autorização explícita do SE** (não é deploy/merge/segredo/escopo/bypass); mas exige decisão própria, não pode ser colado no bug do gate | Sim — toolchain/workflow é path protegido: PR + gates + **aprovação obrigatória**; merge do PR exige autorização explícita; se o bump chegar a deploy, vale o guardrail de deploy | bug assessment `:48,:57`; 035-síntese `:72`; ROADMAP `:67`; stack-035 `:27,:53,:69`; converge-govAB `:34`; 018 guardrails §6; 023 FR-002 (merge/deploy) |

## 3) Distinção que evita erro de interpretação

1. **"Fazer" ≠ "publicar".** 023 separa a decisão/autonomia (FR-001) da autorização para merge/deploy (FR-002). O fix do gate é autônomo para decidir, implementar e abrir PR; o **merge** desse PR em branch protegido continua exigindo autorização explícita + aprovação humana. Um gate verde (inclusive o corrigido) nunca é licença para merge (`AGENTS.md:19,45`).
2. **Paths protegidos não são "proibidos".** `rust.yml` e o pin do toolchain são `workflows/...`/`toolchain` → mudança permitida, mas só via PR versionado + gates determinísticos + aprovação humana (`018 guardrails §6`, `converge-govAB.md:34`); isso é **aprovação de review**, distinta da **autorização explícita do SE** para merge/deploy.
3. **Deploy é a linha dura.** `docker-staging.yml` é o único dos itens marcado literalmente como "bloqueado por autorização"/"somente com autorização"/"não execute sem permissão" — porque a correção em si dispara publicação GHCR + webhook Coolify. Escalonar tipado (`tipo: autorizacao`) antes de tocar nele.
4. **Toolchain tem justificativa de correção, não de novidade** (UB de trait-object em 1.98.0 corrigido em 1.98.1; estável 1.100.0). Isso o torna defensável como tarefa autônoma separada, mas não o isenta do review de path protegido nem da autorização de merge/deploy. O bug fix anterior de toolchain manteve o pin em `1.98.0` (`fix.md:10`) — o bump de versão não foi feito lá.
5. **Não "consertar o gate silenciando a suíte"** (ESC-20261114-001): qualquer diagnóstico/correção do required check exige autorização de leitura de logs/re-dispatch; a correção legitima é a que aumenta a cobertura (0 → 104 testes), não a que desliga o check.

## 4) Resposta direta

- **Fix do gate de testes + abertura de PR:** NÃO exigem autorização explícita (autônomos, sancionados como "ação imediata"). **O merge do PR exige SIM** autorização explícita + aprovação humana por ser branch protegido/path de workflow.
- **`docker-staging.yml` (deploy webhook):** SIM, exige autorização explícita; não executar sem permissão; escalar em 024.
- **Bump do toolchain:** não é item de autorização explícita do SE, mas é **decisão separada** com aprovação obrigatória de path protegido; merge (e eventual deploy) seguem exigindo autorização explícita.

**NV / limites:** requisitos de required checks/protections do host Forgejo/GitHub não foram verificados ao vivo (mesma ressalva de `AGENTS.md:19` e `024 ESC-004`); a contagem de 104 testes é a deste checkout (`035-síntese:82`). Esta nota não altera código, CI, PRs nem deploy.
