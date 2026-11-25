# Idea Research: Bump de segurança rustls

- **Slug**: 090-rustls-security-bump
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: medium
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; advisories, nomes de crates e comandos em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. Números de linha citados são do checkout da worktree isolada (base `github/dev@e88dc8e4d`); o estado ao vivo de PR/CI/produção é **[NV]**. O host `rustsec.org` não está na allowlist — nenhum fetch foi feito e isso é registrado em `Sources`.

## Users & Demand

- O demandante é o mantenedor/compliance que precisa fechar quatro advisories RUSTSEC em `rustls` no `Cargo.lock` raiz, sem alterar faixa declarada, API, gate ou processo de merge [source: .specify/assessments/090-rustls-security-bump/intake.md:8-21; .specify/assessments/090-rustls-security-bump/decision.md:10-30] (confidence: high).
- Os quatro alertas nomeados são `RUSTSEC-2026-0285` (medium, correção `0.23.45`), `RUSTSEC-2026-0346` (medium, `0.23.46`), `RUSTSEC-2026-0348` (low, `0.23.46`) e `RUSTSEC-2026-0349` (low, `0.23.46`) [source: .specify/assessments/090-rustls-security-bump/decision.md:16-21; docs/research/2026-11-16-028-advisory-job-plan.md:40] (confidence: high).
- A divergência motivadora era `0.23.43` no grafo resolvido contra `0.23.46` já registrado na isenção cargo-vet [source: .specify/assessments/090-rustls-security-bump/intake.md:14-18; .specify/assessments/090-rustls-security-bump/decision.md:23-26] (confidence: high).
- A spec traduz a demanda em leitura determinística de `Cargo.lock`, da isenção e do manifesto, com gate e testes preservados [source: specs/090-rustls-security-bump/spec.md:18-39,43-62,66-90] (confidence: high).
- A decisão 089/091 de não promover checagem de advisories a required check é contexto da demanda: a correção é de dependência, não de gate [source: .specify/memory/STATE.md:749-754,786-788] (confidence: high).

## Prior Art

- O plano de job advisory (`docs/research/2026-11-16-028-advisory-job-plan.md:40`) é a fonte local canônica dos quatro alertas e das versões corretivas, obtida via API do GitHub naquele ciclo [source: docs/research/2026-11-16-028-advisory-job-plan.md:40] (confidence: high).
- A atualização foi aplicada com `cargo update -p rustls --precise 0.23.46 --offline`, registrada no `STATE.md` do ciclo [source: .specify/memory/STATE.md:768] (confidence: high).
- Padrão anterior de auditoria: `cargo deny check bans licenses sources` e `cargo vet --locked --no-minimize-exemptions`, ambos informativos, com resultado registrado no handoff [source: .specify/memory/STATE.md:771-772; .github/workflows/rust.yml:124-129] (confidence: high).
- O `dependency-review` já é um dos quatro required checks; em ciclos posteriores ele bloqueou um PR por advisories no frontend, mostrando que a política local é corrigir versões, não relaxar o gate [source: .specify/memory/STATE.md:786-790,770] (confidence: high).
- O padrão de diff cirúrgico dos ciclos 086–089 foi reaproveitado: lockfile + isenção + quatro artefatos do ciclo [source: .specify/assessments/090-rustls-security-bump/decision.md:53-57; specs/090-rustls-security-bump/plan.md:40-53] (confidence: high).

## Market & Context

- `rustls` é dependência TLS comum no ecossistema Rust; quatro advisories com correções em `0.23.45`/`0.23.46` permitem uma única versão corretiva compatível com a faixa `0.23` [source: .specify/assessments/090-rustls-security-bump/decision.md:32-41; specs/090-rustls-security-bump/spec.md:24-26] (confidence: medium).
- A política de supply-chain do repo registra isenções com `criteria = "safe-to-deploy"` em `supply-chain/config.toml`, e o gate `check_gate_integrity.py` permanece como única autoridade de merge [source: supply-chain/config.toml:1119-1121; .specify/memory/STATE.md:749-754] (confidence: high).
- Nenhum `cargo deny advisories` foi promovido a required check; a decisão 089 é explícita [source: .specify/memory/STATE.md:754,770] (confidence: high).
- O gate autoritativo registrado no artefato era 10 legs/58 contratos (ciclo 088); o checkout atual tem 11 entradas/87 [source: .specify/assessments/090-rustls-security-bump/decision.md:49-52; scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- PR, merge, aprovações de maintainer e produção foram marcados `[NV]` no handoff e não foram reconsultados [source: .specify/assessments/090-rustls-security-bump/decision.md:79-87; .specify/memory/STATE.md:774] (confidence: high).

## Data & Constraints

- No checkout, `Cargo.lock:2787-2789` registra `name = "rustls"`, `version = "0.23.46"`; nenhuma ocorrência de `0.23.43` permanece no lockfile [source: Cargo.lock:2787-2789; varredura literal de `0.23.43` em `Cargo.lock` em 2026-11-24] (confidence: high).
- `Cargo.toml:189` permanece `rustls = "0.23"` (e `:190` `tokio-rustls = "0.26"`), ou seja, a faixa declarada não foi alterada [source: Cargo.toml:189-190] (confidence: high).
- `supply-chain/config.toml:1119-1121` registra `[[exemptions.rustls]]` com `version = "0.23.46"` e `criteria = "safe-to-deploy"`, alinhado ao lock [source: supply-chain/config.toml:1119-1121] (confidence: high).
- O número de linha citado no artefato para o lock (`:2748`) é do snapshot do ciclo; no checkout a entrada está em `:2787` [source: .specify/assessments/090-rustls-security-bump/decision.md:23; Cargo.lock:2787-2789] (confidence: high).
- A verificação registrada no `STATE.md` inclui gate PASS (`all 10 db contract floors intact (58 total)`, exit 0), `cargo test --workspace` com zero falhas, `cargo deny check bans licenses sources` com `bans ok, licenses ok, sources ok` e `cargo vet --locked --no-minimize-exemptions` com `Vetting Succeeded (424 exempted)` [source: .specify/memory/STATE.md:770-773] (confidence: high).
- O `cargo update` normalizou uma aresta de `getrandom` em `tempfile` sem trocar versões — efeito colateral registrado, ainda dentro do escopo de duas linhas de código [source: .specify/memory/STATE.md:768] (confidence: high).
- O rollout do ciclo 090 registrou R7 `0.4.4→0.4.5` com rollback restaurando lock e isenção; nada disso foi reexecutado nesta avaliação [source: .specify/memory/STATE.md:773] (confidence: high).

## Evidence Against the Idea

- As severidades e versões corretivas vêm de uma linha do plano local de research, que por sua vez as atribui à API do GitHub; o banco RustSec (`rustsec.org`) não foi consultado porque o host não é allowlisted. A equivalência "0.23.46 corrige as quatro" não foi verificada de forma independente nesta avaliação [source: docs/research/2026-11-16-028-advisory-job-plan.md:40; .specify/assessments/090-rustls-security-bump/intake.md:37] (confidence: medium).
- A referência `Cargo.lock:2748` no artefato está defasada (entrada atual em `:2787`); citá-la sem ressalva pode levar a leitura da dependência errada [source: .specify/assessments/090-rustls-security-bump/decision.md:23; Cargo.lock:2787-2789] (confidence: high).
- `cargo update` alterou uma aresta de `getrandom` em `tempfile`, de modo que a afirmação "somente `rustls` mudou" não é literalmente exata no grafo resolvido [source: .specify/memory/STATE.md:768] (confidence: high).
- `cargo vet` reportou `424 exempted`; um número alto de isenções é consistente com a auditoria local, mas não prova por si que o critério `safe-to-deploy` da entrada `rustls` foi reavaliado para a nova versão [source: .specify/memory/STATE.md:772; supply-chain/config.toml:1119-1121] (confidence: medium).
- As contagens de gate (10/58) e o resultado limpo de `dependency-review` são registros de 2026-11-20, não medições desta avaliação; o checkout corrente tem 11/87 [source: .specify/memory/STATE.md:770-773; scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- Não há evidência local de que advisories adicionais (fora os quatro canônicos) tenham sido recontados após o bump; o `STATE.md` menciona um "bônus 0347" sem detalhar verificação [source: .specify/memory/STATE.md:769] (confidence: medium).
- [NV] Estado ao vivo de CI do PR, merge, aprovações e produção impede afirmar que o bump chegou a algum ambiente publicado [source: .specify/assessments/090-rustls-security-bump/decision.md:84-87] (confidence: high).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: em qual fonte ou ferramenta foi verificado que `0.23.46` corrige efetivamente as quatro advisories, e quem valida essa equivalência?] [source: .specify/assessments/090-rustls-security-bump/intake.md:37; docs/research/2026-11-16-028-advisory-job-plan.md:40] (confidence: high).
- [NEEDS CLARIFICATION: [NV] qual é o estado do CI do PR, das aprovações de maintainer, da proteção de branch e do ambiente de produção, não reconsultados neste ciclo?] [source: .specify/assessments/090-rustls-security-bump/intake.md:38; .specify/assessments/090-rustls-security-bump/decision.md:84-87] (confidence: high).
- [NEEDS CLARIFICATION: o bump altera dependências transitivas de `rustls` ou qualquer outro pacote do grafo, mesmo sem mudança de API, dado que a aresta de `getrandom` em `tempfile` foi normalizada?] [source: .specify/assessments/090-rustls-security-bump/intake.md:39; .specify/memory/STATE.md:768] (confidence: high).
- [NEEDS CLARIFICATION: se `dependency-review` ou o gate acusarem achados após o bump, quem decide o descarte ou o seguimento?] [source: .specify/assessments/090-rustls-security-bump/intake.md:40] (confidence: high).
- [NEEDS CLARIFICATION: a isenção cargo-vet com `criteria = "safe-to-deploy"` continua adequada para a versão corrigida ou exige reavaliação do critério?] [source: .specify/assessments/090-rustls-security-bump/intake.md:41; supply-chain/config.toml:1119-1121] (confidence: high).
- [NEEDS CLARIFICATION: como atualizar referências de linha defasadas (`Cargo.lock:2748` → `:2787`) sem reescrever o registro histórico do ciclo?] [source: .specify/assessments/090-rustls-security-bump/decision.md:23; Cargo.lock:2787] (confidence: high).
- [ASSUMPTION] A faixa `0.23` do manifesto continua satisfazendo todas as dependências que usam `rustls`; nenhuma resolução alternativa foi observada no lock. (confidence: low).

## Sources

- `system` | `.specify/assessments/090-rustls-security-bump/intake.md:1-41` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/090-rustls-security-bump/decision.md:1-87` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/090-rustls-security-bump/spec.md:9-178` e `plan.md:15-58` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `Cargo.lock:2787-2789` (entrada `rustls = 0.23.46`; varredura literal sem `0.23.43`) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `Cargo.toml:189-190` e `supply-chain/config.toml:1119-1121` (faixa `0.23`, isenção `safe-to-deploy`) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `docs/research/2026-11-16-028-advisory-job-plan.md:40` (tabela com os 4 alertas canônicos) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:765-774,786-790` e `docs/PROGRESSO.md:237-243` (registro do ciclo 090 e decisões posteriores) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `scripts/ci/check_gate_integrity.py:42-54` (contagens atuais do gate) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- URL `https://rustsec.org/advisories/RUSTSEC-2026-0285` (e equivalentes para 0346/0348/0349) | host `rustsec.org` | `[UNVERIFIED — fetch skipped: host not allowlisted]` em 2026-11-24 | policy: auto-refused: host not allowlisted | nenhum redirect seguido, nenhuma credencial usada.
- `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora` | host `github.com` | policy: allowlisted, **não consultado** — estado de PR/CI/aprovações marcado `[NV]`; nenhum token ou chamada à API em 2026-11-24.
