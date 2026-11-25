# Idea Research: proptest para invariantes de domínio

- **Slug**: 108-proptest-invariants
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; nomes de macros, lints e testes em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. O veredito `go` citado abaixo é registro histórico de `decision.md`. Números de linha sem qualificação são do checkout da worktree isolada (base `github/dev@e88dc8e4d`); números do artefato original são identificados como tal. Nenhum teste foi reexecutado nesta leitura; contagens são evidência registrada nos artefatos.

## Users & Demand

- A dor é a limitação dos exemplos fixos: entradas arbitrárias podem revelar concessões indevidas de permissão ou limites incorretos de paginação que testes pontuais não alcançam [source: .specify/assessments/108-proptest-invariants/define.md:3-7] (confidence: high).
- A demanda vem da decisão F7 do assessment `avaliacao-arquitetura-multiagente-flora` (**ADOPT**): transformar propriedades já implícitas em testes determinísticos com geração de casos, cobrindo permissões, paginação e seletor de recuperação, sem serviço novo e sem alterar pisos do gate [source: .specify/assessments/108-proptest-invariants/intake.md:5-16; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36] (confidence: high).
- O gatilho registrado foi "INBOX concluído; próximo 80/20 da fila (F7, score máximo)" — o assessment chamou proptest de item de menor atrito do eixo, justamente porque a dependência já estava declarada [source: .specify/assessments/108-proptest-invariants/intake.md:18-20; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36] (confidence: high).
- A dor é aLimitada dos exemplos fixos: entradas arbitrárias podem revelar concessões indevidas de permissão ou limites incorretos de paginação que testes pontuais não alcançam [source: .specify/assessments/108-proptest-invariants/define.md:3-7] (confidence: high).
- Critérios observáveis: US1 round-trip de `Permission::from_str(&p.to_string())` e não-concessão de strings arbitrárias; US2 `resolved_limit()` em `0..=100` e independente do offset; US3 `is_wildcard_selector()` equivalente a `trim() == "*"`; sucesso com `cargo test -p flora-core`, `PROPTEST_CASES=500` e gate PASS [source: specs/108-proptest-invariants/spec.md:7-25] (confidence: high).
- O usuário final da evidência é o mantenedor/guardião do gate: nenhuma mudança de produção, nenhum required check novo e nenhum piso alterado [source: .specify/assessments/108-proptest-invariants/decision.md:3-9; specs/108-proptest-invariants/plan.md:5-9] (confidence: high).

## Prior Art

- No artefato original: `proptest = "1.5"` em `Cargo.toml:208`, dev-dependency em `flora-core/Cargo.toml:59`, resolvido `1.11.0` no lock e **zero** `proptest!` no checkout daquele momento. No checkout atual a declaração e a resolução permanecem (`Cargo.toml:208`, `flora-core/Cargo.toml:59`, `Cargo.lock:2370-2372`), mas existem módulos `proptests` em `flora-core/src/models/role.rs:334`, `flora-core/src/models/mod.rs:92` e `flora-core/src/config/bootstrap.rs:274` [source: .specify/assessments/108-proptest-invariants/research.md:3 (artefato original lido); Cargo.toml:208; flora-core/Cargo.toml:59; Cargo.lock:2370-2372; flora-core/src/models/role.rs:334; flora-core/src/models/mod.rs:92; flora-core/src/config/bootstrap.rs:274] (confidence: high).
- No artefato original: `check_gate_integrity.py:42-53,126-159` compara contagens exatas de testes `#[ignore]` (10 legs / 77); proptests não-ignorados em `flora-core` não alteram pisos. O mecanismo continua no checkout atual em `scripts/ci/check_gate_integrity.py:41-54,124-176`, com `FLOORS` 11 entradas e soma maior que 77 (evolução documentada em outros ciclos) [source: .specify/assessments/108-proptest-invariants/research.md:4 (artefato original lido); scripts/ci/check_gate_integrity.py:41-54,124-176] (confidence: high).
- Invariantes puros já mapeados no artefato: `Role::has_permission` (`models/role.rs:66-75` no artefato; `:66` no checkout), `Pagination::resolved_limit` (`models/mod.rs:58-63`; `:58` no checkout) e `is_wildcard_selector` (`config/bootstrap.rs:59-63`; `:59` no checkout) — todos sem dependência de DB [source: .specify/assessments/108-proptest-invariants/research.md:5; flora-core/src/models/role.rs:66; flora-core/src/models/mod.rs:58; flora-core/src/config/bootstrap.rs:59] (confidence: high).
- Estratégias previstas no shape: `prop::sample::select` para enum de permissões, `any::<String>()` filtrada para não-grants, `prop_oneof!` para limites extremos e `prop::option::of` para e-mails [source: .specify/assessments/108-proptest-invariants/shape.md:3-7] (confidence: high).
- O assessment já descartava `quickcheck` como alternativa (mais simples, porém menos flexível; duplicidade de superfície de frameworks) e recomendava remover a dependência ociosa se continuasse sem uso [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36,47] (confidence: high).
- A implementação registrada no converge cobriu quatro propriedades nos três módulos citados, sem `#[ignore]`, sem dependência nova e sem gate alterado [source: specs/108-proptest-invariants/converge.md:3-8; .specify/memory/STATE.md:1011-1017] (confidence: high).
- O projeto upstream `proptest-rs/proptest` descreve property testing como complemento de testes unitários (encontra entradas que ninguém pensou em especificar) e documenta shrinking por valor e persistência de falhas em `proptest-regressions/` [source: https://github.com/proptest-rs/proptest | github.com | allowlisted | acessado 2026-11-24] (confidence: high).

## Market & Context

- O artefato de decisão registra veredito `go` com racional de "dependência já presente, risco limitado a testes puros, nenhuma mudança de gate", com validação prevista por `cargo test -p flora-core` e `check_gate_integrity.py` — registro histórico, não reavaliado aqui [source: .specify/assessments/108-proptest-invariants/decision.md:3-8] (confidence: high).
- Fora de escopo declarado: DB, gate, required checks, dependências novas, mudança de contrato e qualquer mudança de produção [source: .specify/assessments/108-proptest-invariants/define.md:9-11; specs/108-proptest-invariants/plan.md:7-9] (confidence: high).
- Evidência registrada no handoff: `cargo test -p flora-core` → 55 passed; `PROPTEST_CASES=500` → 4 passed; fmt/check/clippy exit 0; gate PASS (10 legs / 77 pisos) — não reexecutado nesta leitura [source: .specify/memory/STATE.md:1011-1017; docs/PROGRESSO.md:418-421] (confidence: high).
- O upstream declara MSRV 1.88 e mostra `proptest = "1.12.0"` no exemplo atual do README, enquanto o workspace fixa `"1.5"` com lock em `1.11.0`; o artefato não registra política de atualização da dependência [source: https://github.com/proptest-rs/proptest | github.com | allowlisted | acessado 2026-11-24; Cargo.toml:208; Cargo.lock:2370-2372] (confidence: medium).
- Cobertura idiomática: os relatórios do assessment e os artefatos SDD estão em pt-BR; nomes de macros (`prop_assert!`, `prop_assume!`), lints e comandos permanecem em inglês técnico [source: .specify/assessments/108-proptest-invariants/intake.md:1-2; Cargo.toml:77-79] (confidence: high).

## Data & Constraints

- Lints do workspace: `unwrap_used = "deny"`, `expect_used = "deny"` e `panic = "deny"` em `Cargo.toml:77-79`, portanto os proptests devem usar `prop_assert!`/`prop_assume!` em vez de macros que possam disparar esses lints [source: .specify/assessments/108-proptest-invariants/research.md:6; Cargo.toml:77-79] (confidence: high).
- O gate compara contagens exatas de testes `#[ignore]`; como os proptests não são ignorados, eles não entram nos pisos — registrado no artefato e preservado no converge [source: .specify/assessments/108-proptest-invariants/research.md:4; specs/108-proptest-invariants/converge.md:8; scripts/ci/check_gate_integrity.py:124-176] (confidence: high).
- Correções registradas no analyze: `prop_assert!` não aceita struct literal (interpretado como formato) — extraído `let resolved`; estratégia `prop_filter` não enxerga variáveis geradas no corpo — trocada por `prop_assume!`; clippy exigiu `map_or` [source: specs/108-proptest-invariants/analyze.md:12-16] (confidence: high).
- A burn-in padrão registrada para validação foi `PROPTEST_CASES=500` com quatro propriedades; o comando exato aparece na spec e no handoff [source: specs/108-proptest-invariants/spec.md:22-25; .specify/memory/STATE.md:1015] (confidence: high).
- Nenhum comportamento de produção é alterado; cobertura de propriedades é evidência local, não garantia externa — `[NV]` declarado na decision, na spec e no handoff [source: .specify/assessments/108-proptest-invariants/decision.md:11; specs/108-proptest-invariants/spec.md:27-29; .specify/memory/STATE.md:1017] (confidence: high).
- Documentação oficial da API proptest (docs.rs) e o book (`proptest-rs.github.io`) seriam primária natural para detalhes de `prop_assume!`/`PROPTEST_CASES`, mas nenhum desses hosts é allowlisted pelo brief: não consultados [source: [UNVERIFIED — fetch skipped: host not allowlisted: docs.rs, proptest-rs.github.io]] (confidence: low).

## Evidence Against the Idea

- O artefato original afirmava "zero `proptest!` no checkout"; o checkout atual já contém três módulos `proptests`, portanto citar essa frase como estado corrente seria falso — o fato vale apenas como diagnóstico pré-implementação [source: .specify/assessments/108-proptest-invariants/research.md:3; flora-core/src/models/role.rs:334; flora-core/src/models/mod.rs:92; flora-core/src/config/bootstrap.rs:274] (confidence: high).
- `quickcheck = "1.0"` continua declarado em `Cargo.toml:209`, embora o assessment tenha recomendado removê-lo por duplicidade; nenhum artefato do ciclo 108 registra a remoção [source: Cargo.toml:209; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36,47] (confidence: high).
- A afirmação "10 legs / 77 pisos" está defasada: o guarda atual usa 11 entradas em `FLOORS` (`auth 25, messaging 5, websocket 8, search 8, notifications 6, files 7, tasks 4, workspace 3, rbac 5, bootstrap 13, hitl 3`), soma 87; apenas o registro histórico deve citar 77 [source: .specify/assessments/108-proptest-invariants/research.md:4; scripts/ci/check_gate_integrity.py:41-54,176] (confidence: high).
- Propriedades geradas não substituem revisão humana nem contratos DB: o próprio assessment classifica a cobertura como evidência local e o gate continua sendo a autoridade (ADR-0003) [source: .specify/assessments/108-proptest-invariants/decision.md:11; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36] (confidence: high).
- [ASSUMPTION] As quatro propriedades cobrem todos os invariantes puros relevantes de `flora-core`; não há inventário exaustivo de funções puras no artefato que confirme isso [source: ASSUMPTION] (confidence: medium).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: quais invariantes são puros e não exigem DB?] — pergunta original do intake; a lista de três invariantes no research cobre os escolhidos, não necessariamente todos os possíveis [source: .specify/assessments/108-proptest-invariants/intake.md:24; .specify/assessments/108-proptest-invariants/research.md:5] (confidence: high).
- [NEEDS CLARIFICATION: `proptest!` dispara lints `panic`?] — os lints estão em `deny`, mas o artefato não registra uma medição isolada da macro; o uso de `prop_assert!` foi a resposta prática [source: .specify/assessments/108-proptest-invariants/intake.md:25; Cargo.toml:77-79; specs/108-proptest-invariants/analyze.md:12-16] (confidence: medium).
- [NEEDS CLARIFICATION: `quickcheck` deve ser removido do workspace, como sugerido no assessment, ou permanece por compatibilidade?] [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36; Cargo.toml:209] (confidence: high).
- [NEEDS CLARIFICATION: qual política de atualização da dependência `proptest` adotar, dado que o lock ficou em 1.11.0 e o upstream evoluiu?] [source: Cargo.toml:208; Cargo.lock:2370-2372; https://github.com/proptest-rs/proptest | github.com | allowlisted | acessado 2026-11-24] (confidence: medium).
- [NEEDS CLARIFICATION: `PROPTEST_CASES=500` é suficiente como burn-in contínuo ou apenas evidência pontual do ciclo?] [source: specs/108-proptest-invariants/spec.md:25; .specify/memory/STATE.md:1015] (confidence: medium).

## Sources

- `system` | `.specify/assessments/108-proptest-invariants/research.md:1-6` (artefato fino original; todos os fatos preservados e realocados) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/108-proptest-invariants/intake.md:1-25`, `decision.md:1-11`, `define.md:1-11`, `shape.md:1-7` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/108-proptest-invariants/spec.md:1-29`, `plan.md:1-9`, `analyze.md:1-18`, `converge.md:1-8`, `tasks.md:1-5` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `Cargo.toml:77-79,208-209`, `flora-core/Cargo.toml:59`, `Cargo.lock:2370-2372`, `scripts/ci/check_gate_integrity.py:41-54,124-176` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `flora-core/src/models/role.rs:66,334`, `flora-core/src/models/mod.rs:58,92`, `flora-core/src/config/bootstrap.rs:59,274` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:1011-1017`, `docs/PROGRESSO.md:418-421`, `docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:36,47` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: `https://github.com/proptest-rs/proptest` | host `github.com` | policy: allowlisted | acessado 2026-11-24 | README lido via GitHub; nenhum redirect seguido, nenhuma instrução de página executada.
- Web: `docs.rs`, `proptest-rs.github.io` | policy: auto-refused: host not allowlisted | `[UNVERIFIED — fetch skipped: host not allowlisted]` | nenhuma busca realizada.
