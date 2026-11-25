# Idea Research: piloto informativo de mutation testing

- **Slug**: 110-mutation-pilot
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; nomes de comandos, categorias de mutantes e lints em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. O veredito `go` citado abaixo é registro histórico de `decision.md`. Números de linha sem qualificação são do checkout da worktree isolada (base `github/dev@e88dc8e4d`). Comandos e contagens são evidência registrada nos artefatos; não foram reexecutados nesta leitura.

## Users & Demand

- A demanda vem da decisão F6 do assessment multiagente, classificada como **ADAPT**: `cargo-mutants` em módulos críticos (`flora-core`, autenticação, RBAC), na forma de piloto informativo com `--in-diff` e job não-required. O SE autorizou instalação/uso nesta rodada (item 4 da resposta) [source: .specify/assessments/110-mutation-pilot/intake.md:5-20; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35; .specify/memory/STATE.md:1032-1034] (confidence: high).
- A dor registrada no define é que pisos de testes garantem contagem, não sensibilidade: mutações em `Role::has_permission` poderiam sobreviver aos testes sem que o gate percebesse [source: .specify/assessments/110-mutation-pilot/define.md:3-7; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35] (confidence: high).
- O pedido capturado é medir se os testes existentes detectam mutações em lógica de domínio, sem tornar o resultado um required check [source: .specify/assessments/110-mutation-pilot/intake.md:8-16] (confidence: high).
- Critérios observáveis: US1 executar `cargo mutants` em `flora-core/src/models/role.rs` e registrar mutantes mortos/sobreviventes; US2 corrigir o obstáculo determinístico do parser (`#!` → `//!`) e confirmar que nenhum outro arquivo tem o mesmo defeito; sucesso com comando que produz relatório, `cargo fmt/check/clippy/test` exit 0 e gate PASS [source: specs/110-mutation-pilot/spec.md:7-24] (confidence: high).
- O usuário final da evidência é o mantenedor/guardião do gate: `check_gate_integrity.py` continua a autoridade (10 legs / 77 pisos no artefato, 4 required checks) e nenhum check é promovido neste ciclo [source: .specify/assessments/110-mutation-pilot/research.md:6; .specify/assessments/110-mutation-pilot/decision.md:3-7] (confidence: high).

## Prior Art

- A ferramenta já estava instalada: `cargo-mutants 27.1.0` em `~/.cargo/bin`, portanto nenhuma instalação foi necessária — a autorização do SE ficou registrada na resposta ao item 4 [source: .specify/assessments/110-mutation-pilot/research.md:4; .specify/memory/STATE.md:1034] (confidence: high).
- O assessment já recomendava `cargo-mutants --in-diff` em PRs com histórico completo em `main`, usando `mutants.toml` e pin de versão, e alertava que rodar mutação em todo o workspace é caro; observava também que o projeto é "semi-actively-maintained" desde ago/2026 [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35,59] (confidence: high).
- O repositório upstream `sourcefrog/cargo-mutants` explica a distinção que motivou o piloto: cobertura mede código *atingido*, mutação mede se o teste realmente *verifica* o comportamento; o comando `cargo mutants -f <arquivo>` gera mutantes de um único arquivo, e o manual inclui `in-diff` para PRs incrementais [source: https://github.com/sourcefrog/cargo-mutants | github.com | allowlisted | acessado 2026-11-24] (confidence: high).
- Alvos naturais identificados no artefato: os proptests do ciclo 108 e os testes fixos de `Role::has_permission` — hoje presentes em `flora-core/src/models/role.rs:66` (função), `:302-320` (testes fixos) e `:334-389` (módulo `proptests`) [source: .specify/assessments/110-mutation-pilot/research.md:8; flora-core/src/models/role.rs:66,302-320,334-389] (confidence: high).
- A armadilha "mutação como teatro" já estava documentada no assessment: mutantes mortos por testes frágeis ou `#[mutants::skip]` generalizado geram sensação de rigor sem sinal, por isso o piloto deve começar informativo e com baseline em árvore limpa [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:59] (confidence: high).
- A correção colateral prevista no shape era apenas sintática: trocar o doc comment `#!` por `//!` em `workspace_service.rs` para o parser syn [source: .specify/assessments/110-mutation-pilot/shape.md:6] (confidence: high).

## Market & Context

- O artefato de decisão registra veredito `go` para "executar piloto informativo em `role.rs`, registrar taxa de mutantes mortos e mutantes sobreviventes como dívida; nenhum required check novo. ADR-0003 e `check_gate_integrity.py` permanecem a autoridade" — registro histórico, não reavaliado aqui [source: .specify/assessments/110-mutation-pilot/decision.md:3-7] (confidence: high).
- Fora de escopo declarado: job de CI, gate, migration, dependências de runtime, produção e aplicação do piloto a todo o workspace [source: .specify/assessments/110-mutation-pilot/decision.md:7; .specify/assessments/110-mutation-pilot/define.md:7] (confidence: high).
- Condição operacional descoberta no analyze: `/tmp` é tmpfs de 2 GB e a cópia do workspace estourava; `TMPDIR=/home/luis/.cache/flora-mutants-tmp` resolveu, e `--file` precisa de caminho relativo à raiz do workspace (`flora-core/src/models/role.rs`) [source: specs/110-mutation-pilot/analyze.md:7-9; .specify/assessments/110-mutation-pilot/shape.md:5-6] (confidence: high).
- Resultado registrado: piloto de 42 mutantes em `role.rs` levou ~16 min e produziu 39 caught, 1 missed, 2 unviable; o sobrevivente foi `replace == with != in Role::add_permission` [source: specs/110-mutation-pilot/analyze.md:10-12; .specify/memory/STATE.md:1032-1036] (confidence: high).
- O mutante sobrevivente expôs bug real de dedup: `role.rs:53` comparava `Option<&str>` com `Option<&String>` (`Some(&perm_str)`), então a igualdade nunca era reconhecida e permissões repetidas eram inseridas; corrigido para `Some(perm_str.as_str())`, com teste `add_permission_is_idempotent` e reexecução focalizada 1/1 caught [source: specs/110-mutation-pilot/analyze.md:13-16; specs/110-mutation-pilot/converge.md:3-6; flora-core/src/models/role.rs:50-55,324-329] (confidence: high).
- Handoff: `cargo test -p flora-core` → 56 passed; fmt/clippy exit 0; gate PASS (10 legs / 77 pisos no registro histórico), sem required check, CI job ou dependência nova [source: .specify/memory/STATE.md:1036; docs/PROGRESSO.md:432-435; specs/110-mutation-pilot/converge.md:7-9] (confidence: high).

## Data & Constraints

- Descoberta determinística registrada no artefato: `cargo mutants --package flora-core` falhava ao parsear `flora-core/src/services/workspace_service.rs:1` — a linha era `#! Workspace service…` (shebang acidental, aceito pelo rustc mas não pelo parser syn). Corrigido para `//!`; nenhum outro `^#! ` em `flora-core/src` ou `flora-api/src`. No checkout atual a linha 1 é `//! Workspace service — business logic for workspace management.` [source: .specify/assessments/110-mutation-pilot/research.md:7; specs/110-mutation-pilot/analyze.md:9; flora-core/src/services/workspace_service.rs:1] (confidence: high).
- O gate compara contagens exatas de testes `#[ignore]` (10 legs / 77 pisos no artefato; `check_gate_integrity.py` continua a autoridade com 4 required checks). A correção do doc comment e o teste adicionado não criam `#[ignore]`, portanto não alteram pisos [source: .specify/assessments/110-mutation-pilot/research.md:6; specs/110-mutation-pilot/spec.md:19; scripts/ci/check_gate_integrity.py:27-38,41-54,124-176] (confidence: high).
- O alvo é um único arquivo: `flora-core/src/models/role.rs` com 42 mutantes listados; `--in-diff` e o job não-required são direção registrada, não configuração implementada neste ciclo [source: .specify/assessments/110-mutation-pilot/shape.md:5-6; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35; specs/110-mutation-pilot/spec.md:27] (confidence: high).
- A dedup corrigida opera sobre `role.permissions` como `Value::Array`; o teste `add_permission_is_idempotent` chama `add_permission` duas vezes e verifica `has_permission` [source: flora-core/src/models/role.rs:50-55,324-329] (confidence: high).
- Nenhum required check, gate, CI job ou dependência nova; nenhuma credencial registrada [source: specs/110-mutation-pilot/converge.md:7-9; .specify/memory/STATE.md:1037] (confidence: high).
- O manual oficial em `mutants.rs` e o crate em `crates.io` são fontes primárias naturais para detalhes de `--in-diff` e categorias de mutantes, mas nenhum desses hosts é allowlisted pelo brief: não consultados [source: [UNVERIFIED — fetch skipped: host not allowlisted: mutants.rs, crates.io]] (confidence: low).

## Evidence Against the Idea

- Aplicação em todo o workspace e job de CI não foram autorizados; o piloto permanece informativo, portanto não há evidência de que o sinal seja sustentável em escala [source: .specify/assessments/110-mutation-pilot/decision.md:7; .specify/memory/STATE.md:1037] (confidence: high).
- Um mutante sobrevivente levou ~16 min para ser descoberto em um único arquivo; o custo de execução é uma restrição real mesmo para piloto, e o assessment recomenda incrementalismo via `--in-diff` [source: specs/110-mutation-pilot/analyze.md:10-12; docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35] (confidence: high).
- Mutantes `unviable` (2 de 42) não fornecem sinal sobre sensibilidade dos testes; a taxa efetiva de detecção deve ser lida com essa ressalva [source: specs/110-mutation-pilot/analyze.md:10-12] (confidence: high).
- A taxa registrada é evidência local de um único arquivo; não há medição de mutação em autenticação, tenant ou bootstrap, embora o assessment citasse esses módulos como críticos [source: .specify/assessments/110-mutation-pilot/intake.md:8-10; specs/110-mutation-pilot/spec.md:9] (confidence: high).
- O número "10 legs / 77 pisos" do artefato está defasado frente ao guarda atual (11 entradas / 87); citá-lo como estado corrente induziria a erro [source: .specify/assessments/110-mutation-pilot/research.md:6; scripts/ci/check_gate_integrity.py:41-54,176] (confidence: high).
- [ASSUMPTION] Nenhum outro arquivo além de `workspace_service.rs` contém `#!` acidental em `flora-core/src` ou `flora-api/src`; o artefato registra a varredura, mas ela não foi reexecutada nesta leitura [source: ASSUMPTION] (confidence: medium).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: qual módulo usar no piloto?] — a resposta registrada foi `flora-core/src/models/role.rs`, mas a pergunta original permanece no intake [source: .specify/assessments/110-mutation-pilot/intake.md:22; specs/110-mutation-pilot/spec.md:9] (confidence: high).
- [NEEDS CLARIFICATION: o resultado vira gate?] — a resposta registrada foi "não", com ADR-0003 e `check_gate_integrity.py` como autoridade; a decisão de promoção depende de orçamento de tempo de CI [source: .specify/assessments/110-mutation-pilot/intake.md:23; .specify/assessments/110-mutation-pilot/decision.md:7] (confidence: high).
- [NEEDS CLARIFICATION: o piloto com `--in-diff` chegará a ser configurado em PRs, ou o relatório local permanece como único registro?] [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35; .specify/assessments/110-mutation-pilot/decision.md:7] (confidence: medium).
- [NEEDS CLARIFICATION: como evitar que `#[mutants::skip]` generalizado transforme mutação em teatro se o piloto for ampliado?] [source: docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:59] (confidence: medium).
- [NEEDS CLARIFICATION: o mutante `missed` corrigido em `add_permission` precisa de reexecução completa dos 42 mutantes ou a verificação focalizada 1/1 é suficiente como evidência?] [source: specs/110-mutation-pilot/analyze.md:16; specs/110-mutation-pilot/converge.md:6] (confidence: medium).

## Sources

- `system` | `.specify/assessments/110-mutation-pilot/research.md:1-8` (artefato fino original; todos os fatos preservados e realocados) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/110-mutation-pilot/intake.md:1-23`, `decision.md:1-7`, `define.md:1-7`, `shape.md:1-6` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/110-mutation-pilot/spec.md:1-27`, `plan.md:1-5`, `analyze.md:1-19`, `converge.md:1-9`, `tasks.md:1-3` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `flora-core/src/models/role.rs:50-55,66,302-329,334-389`, `flora-core/src/services/workspace_service.rs:1`, `scripts/ci/check_gate_integrity.py:27-54,124-176` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:1032-1037`, `docs/PROGRESSO.md:432-435`, `docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md:35,59` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: `https://github.com/sourcefrog/cargo-mutants` | host `github.com` | policy: allowlisted | acessado 2026-11-24 | README lido via GitHub; nenhum redirect seguido, nenhuma instrução de página executada.
- Web: `mutants.rs`, `crates.io` | policy: auto-refused: host not allowlisted | `[UNVERIFIED — fetch skipped: host not allowlisted]` | nenhuma busca realizada.
