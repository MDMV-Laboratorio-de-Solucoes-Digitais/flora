# Idea Research: Oráculo dinâmico de pisos no gate

- **Slug**: 088-dynamic-gate-oracle
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; nomes de testes, workflow e scripts em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. Números de linha citados são do checkout da worktree isolada (base `github/dev@e88dc8e4d`); estados remotos ao vivo não foram reconsultados e são marcados `[NV]`.

## Users & Demand

- O usuário é o mantenedor do gate ADR-0003, que precisa que um contrato `#[ignore]` novo sem lega falhe o `rust-build` em vez de se tornar um terceiro *confidence gap* [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:25-30; specs/088-dynamic-gate-oracle/spec.md:13-21] (confidence: high).
- A dor documentada é que `scripts/ci/check_gate_integrity.py` era um guarda-espelho: comparava `FLOORS` escrito à mão no script com as linhas `run_leg` escritas à mão em `rust.yml`; se as duas cópias continuassem iguais, o guarda passava mesmo com contrato `#[ignore]` fora de ambas [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:11-16; .specify/assessments/088-dynamic-gate-oracle/intake.md:10-16] (confidence: high).
- O custo já observado foram dois ciclos consecutivos de descoberta manual: 086 (três contratos de RBAC) e 087 (quatro de bootstrap) [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:18-24; .specify/memory/STATE.md:721-737] (confidence: high).
- A independência de fonte é o ponto central da demanda: os dois lados do espelho eram alimentados por humano, sem fonte de verdade externa [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:14-16] (confidence: high).
- O teste de independência proposto na spec é adicionar temporariamente um contrato `#[ignore]` ou alterar um piso e observar o guarda falhar, removendo a divergência depois [source: specs/088-dynamic-gate-oracle/spec.md:23-36] (confidence: high).

## Prior Art

- A armadilha do filtro vazio já estava documentada em `.github/workflows/rust.yml:87-89` (ciclo 037), e o helper `run_leg` contava `N passed` como defesa local [source: .github/workflows/rust.yml:87-108] (confidence: high).
- A lista confiável apontada pelo artefato é `cargo test -p flora-tests -- --ignored --list`, que enumera todos os testes ignorados; filtros `cargo test <filtro> -- --list` não funcionaram de forma confiável naquele checkout [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:26-31; .specify/assessments/088-dynamic-gate-oracle/intake.md:17-19] (confidence: high).
- A direção registrada preserva as checagens estáticas existentes (`REQUIRED`, jobs, `continue-on-error`, path filters, `cargo build/test --workspace`) e o modo `--remote`, usando apenas stdlib [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:55-58; scripts/ci/check_gate_integrity.py:118-122,179-195] (confidence: high).
- A formação do mapa `FLOORS` e da regex de legs vem dos ciclos anteriores: 086 acrescentou `rbac`, 087 acrescentou `bootstrap`; o oráculo reaproveita ambos em vez de criar novo check [source: .specify/memory/STATE.md:723-746; scripts/ci/check_gate_integrity.py:42-60] (confidence: high).
- A evidência local do ciclo (contagem de 58 nomes `contract::…: test`, gate PASS) foi registrada no handoff e não reexecutada por agentes de documentação [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:81-91] (confidence: high).

## Market & Context

- O gate autoritativo permanece o mesmo script; os ciclos 089/090 reforçaram explicitamente que nenhum script SDD ou checagem de advisories foi promovido a required check [source: .specify/memory/STATE.md:749-774] (confidence: high).
- O oráculo roda no step `rust-build` de `.github/workflows/rust.yml`, um job obrigatório sob ADR-0003, sem alterar o workflow [source: specs/088-dynamic-gate-oracle/plan.md:31-36; .specify/assessments/088-dynamic-gate-oracle/decision.md:71-79] (confidence: high).
- O escopo registrado é de um arquivo de código (`check_gate_integrity.py`) mais quatro artefatos do ciclo, sem tocar workflow, testes, `validate/`, `release/` ou `STATE.md` [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:71-79; specs/088-dynamic-gate-oracle/spec.md:146-148] (confidence: high).
- A linha de sucesso `all N db contract floors intact (M total)` é a interface observável consumida por registros e relatórios; mudanças no valor de N/M são visíveis fora do script [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:42; .specify/memory/STATE.md:745,770] (confidence: high).
- A implementação atual roda em Python 3 stdlib; nenhuma rede, segredo ou dependência nova foi introduzida no guarda [source: specs/088-dynamic-gate-oracle/plan.md:14-25; scripts/ci/check_gate_integrity.py:20,127-130] (confidence: high).

## Data & Constraints

- O oráculo está implementado no checkout: `subprocess.run(["cargo","test","-p","flora-tests","--","--ignored","--list"], cwd=ROOT, capture_output=True, text=True, check=False)`; apenas linhas de `stdout` terminadas em `: test` viram nomes de testes [source: scripts/ci/check_gate_integrity.py:124-142] (confidence: high).
- Extração das legs por `LEG_RE` ancorada em linha; validação de rótulo em `FLOORS`, igualdade do piso do workflow com `FLOORS[label]`, contagem por `startswith(filter_)` e exigência de uma única `run_leg` por rótulo [source: scripts/ci/check_gate_integrity.py:60,144-170] (confidence: high).
- Falha fechada em `rc != 0`, lista vazia, legs ausentes ou divergência de contagem/piso [source: scripts/ci/check_gate_integrity.py:131-146,161-173] (confidence: high).
- A mensagem de sucesso no checkout é `f"all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)"` — derivada do tamanho e da soma do mapa, não diretamente da quantidade de nomes descobertos [source: scripts/ci/check_gate_integrity.py:174-177] (confidence: high).
- O `FLOORS` atual tem 11 entradas somando 87; a mensagem corrente esperada é `all 11 db contract floors intact (87 total)`, enquanto o artefato 088 registrou `all 10 db contract floors intact (58 total)` [source: scripts/ci/check_gate_integrity.py:42-54,176; .specify/assessments/088-dynamic-gate-oracle/decision.md:50-53] (confidence: high).
- A descoberta depende da compilação do workspace de testes; `--ignored --list` apenas enumera, sem exigir Postgres, mas o artefato não mediu custo de tempo no CI [source: specs/088-dynamic-gate-oracle/spec.md:152-154; .specify/assessments/088-dynamic-gate-oracle/intake.md:43] (confidence: high).
- Modo `--remote` preservado: consulta `gh` para `dev`/`main` e compara contextos required ao vivo; resultado ao vivo no checkout: **[NV]** [source: scripts/ci/check_gate_integrity.py:179-195; .specify/memory/STATE.md:776] (confidence: high).

## Evidence Against the Idea

- A implementação usa `len(FLOORS)`/`sum(FLOORS.values())` na mensagem de sucesso, enquanto a decisão registrada dizia que a contagem seria "derivada da descoberta, não de constantes copiadas". O oráculo valida cada leg contra a descoberta, mas o total exibido ainda vem do mapa escrito à mão [source: scripts/ci/check_gate_integrity.py:174-177; .specify/assessments/088-dynamic-gate-oracle/decision.md:50-53] (confidence: high).
- A associação por `startswith` pode atribuir um mesmo teste a mais de um prefixo ou deixar órfãos testes cujo nome não casa com nenhuma leg; nenhuma checagem de "teste ignorado não coberto" aparece no script — a validação é feita leg a leg, não cobertura global [source: scripts/ci/check_gate_integrity.py:149-170; .specify/assessments/088-dynamic-gate-oracle/intake.md:40-41] (confidence: high).
- Falha fechada em cargo com `rc != 0` ou lista vazia pode transformar problemas de toolchain/compilação em falha do gate mesmo quando os pisos estão corretos; o custo/benefício não foi medido nesta avaliação [source: scripts/ci/check_gate_integrity.py:131-142; .specify/assessments/088-dynamic-gate-oracle/decision.md:49-53] (confidence: medium).
- Os números 10/58 do handoff estão defasados em relação ao checkout (11/87). Consumidores que comparam a mensagem exata podem reagir à mudança de total sem mudança de lógica [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:42; scripts/ci/check_gate_integrity.py:42-54,176] (confidence: high).
- A evidência de divergência controlada (piso alterado, lista vazia) foi registrada, não reproduzida aqui; gates, build e formatters não foram reexecutados [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:81-91; specs/088-dynamic-gate-oracle/spec.md:152-157] (confidence: high).
- A dependência de `cargo` dentro do guarda acrescenta compilação ao tempo do `rust-build`; a questão de custo ficou em aberto no intake e não há medição no artefato [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:43] (confidence: high).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: `cargo test -p flora-tests -- --ignored --list` é determinístico e seguro em todos os ambientes do job `rust-build`, inclusive sem Postgres?] [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:39] (confidence: high).
- [NEEDS CLARIFICATION: como tratar nomes de testes ignorados que não pertencem a nenhuma das legs ou que casam com mais de um prefixo de filtro?] [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:40; scripts/ci/check_gate_integrity.py:159-165] (confidence: high).
- [NEEDS CLARIFICATION: a associação por `startswith` é suficiente para submódulos futuros ou será preciso delimitar fronteiras entre prefixos?] [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:41] (confidence: high).
- [NEEDS CLARIFICATION: quais consumidores dependem da mensagem exata `all 10 db contract floors intact (58 total)` e como reagem a uma contagem derivada diferente (hoje 11/87)?] [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:42; scripts/ci/check_gate_integrity.py:176] (confidence: high).
- [NEEDS CLARIFICATION: a descoberta passa a depender da compilação do workspace de testes; quem garante que isso não acrescenta tempo ou custo ao gate?] [source: .specify/assessments/088-dynamic-gate-oracle/intake.md:43] (confidence: high).
- [NEEDS CLARIFICATION: a mensagem de sucesso deve passar a usar a contagem descoberta em vez de `len(FLOORS)`/`sum(FLOORS.values())`, ou a divergência registrada entre decisão e código é aceita?] [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:50-53; scripts/ci/check_gate_integrity.py:174-177] (confidence: high).
- [ASSUMPTION] Todos os testes ignorados descobertos pertencem a alguma das 11 legs; o script não verifica cobertura global. (confidence: low).

## Sources

- `system` | `.specify/assessments/088-dynamic-gate-oracle/intake.md:1-43` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/088-dynamic-gate-oracle/decision.md:1-91` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/088-dynamic-gate-oracle/spec.md:9-157` e `plan.md:14-60` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `scripts/ci/check_gate_integrity.py:42-60,118-195` (oráculo, `LEG_RE`, fail-closed, mensagem, `--remote`) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.github/workflows/rust.yml:87-121` (comment do ciclo 037 e legs `run_leg`) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:721-747,776` e `docs/PROGRESSO.md:213-221` (registro do ciclo 088 e auditoria posterior) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/086-rbac-ci-leg/decision.md:11-21` e `.specify/assessments/087-bootstrap-ci-leg/decision.md:21-27` (origem dos dois gaps) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: nenhum host buscado. Nenhuma URL externa é necessária para os fatos locais acima; nenhum redirect seguido, nenhuma credencial incluída.
