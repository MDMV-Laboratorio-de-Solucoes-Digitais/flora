# Feature Specification: Oráculo dinâmico de contratos ignorados no gate de CI

**Feature Branch**: `feature/088-dynamic-gate-oracle`

**Created**: 2026-11-20

**Status**: Implemented (evidência local registrada em `tasks.md`)

**Input**: O guarda `scripts/ci/check_gate_integrity.py` é um espelho entre `FLOORS` e as legs `run_leg` do workflow; contratos `#[ignore]` esquecidos (086 RBAC, 087 bootstrap) passavam batido.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Contrato ignorado novo não pode passar despercebido (Priority: P1)

Como mantenedor do gate ADR-0003, quero que o guarda descubra os contratos
`#[ignore]` a partir do próprio código, para que um teste DB novo sem lega ou
sem piso registrado falhe o `rust-build` em vez de virar um terceiro
*confidence gap*.

**Why this priority**: é a causa raiz dos ciclos 086 e 087 — o espelho só
compara duas listas escritas à mão e não vê o workspace de testes.

**Independent Test**: adicionar temporariamente um contrato `#[ignore]` ou
alterar um piso em `FLOORS`; rodar o guarda e observar falha de contagem.
Remover a divergência e observar PASS `all 10 db contract floors intact
(58 total)`.

**Acceptance Scenarios**:

1. **Given** os 58 contratos `#[ignore]` atuais e 10 legs registradas,
   **When** o guarda roda, **Then** a contagem por leg vem da descoberta via
   `cargo test -p flora-tests -- --ignored --list` e o gate passa.
2. **Given** um contrato `#[ignore]` extra cujo nome casa com o filtro de uma
   leg, **When** o guarda roda, **Then** falha porque `count != floor`;
   **Given** um piso em `FLOORS` maior ou menor que a descoberta, falha da
   mesma forma.

---

### User Story 2 — Falha fechada e gate existente preservado (Priority: P2)

Como revisor, quero que a descoberta falhe fechado e não enfraqueca as
checagens já existentes, para que o oráculo não se torne ele próprio um gate
vacuo.

**Why this priority**: um guarda que pula a verificação quando o cargo falha
ou quando a lista está vazia reproduziria a armadilha do filtro vazio
(documentada em `rust.yml:83-85`).

**Independent Test**: forçar falha do cargo/lista vazia e conferir exit != 0;
conferir que `REQUIRED`, jobs, `--remote` e as mensagens antigas continuam
presentes no script.

**Acceptance Scenarios**:

1. **Given** `cargo ... --ignored --list` com código de saída != 0 ou stdout
   sem linhas `: test`, **When** o guarda roda, **Then** reporta falha e
   retorna 1.
2. **Given** uma leg com rótulo fora de `FLOORS`, piso divergente do mapa ou
   rótulo duplicado, **When** o guarda roda, **Then** falha com erro explícito
   e nenhuma checagem estática antiga é removida.

---

### User Story 3 — Escopo cirúrgico (Priority: P3)

Como dono do processo de merge, quero o diff limitado ao guarda e aos
artefatos do ciclo, para que nenhum required check, workflow ou documento de
estado seja alterado sem avaliação.

**Why this priority**: mantém ADR-0003 íntegro e o handoff auditável.

**Independent Test**: `git diff --name-only cbc885383`.

**Acceptance Scenarios**:

1. **Given** a branch 088 sobre `cbc885383`, **When** o diff é listado,
   **Then** aparecem apenas `scripts/ci/check_gate_integrity.py` e os 4
   artefatos 088; zero arquivos em `validate/`, `release/`, `STATE.md` ou no
   workflow.

---

### Edge Cases

- **Prefixos sobrepostos**: `contract::auth::db_` cobre submódulos; a
  associação usa `startswith`, então um nome pertence à leg cujo filtro é
  prefixo dele.
- **Lista vazia**: nenhuma linha terminada em `: test` no stdout é falha, não
  passivo — evita o "filtro casou zero testes → exit 0" de 037.
- **Warnings do cargo**: podem ir para `stderr`; só `stdout` é parseado, e o
  texto da `stderr` aparece apenas na mensagem de erro quando o processo falha.
- **Legs ausentes/duplicadas**: rótulo em `FLOORS` sem exatamente uma
  `run_leg` no workflow é falha.
- **Filtros `cargo test <filtro> -- --list`**: não confiáveis neste checkout;
  o oráculo nunca depende deles — lê a lista completa e filtra em Python.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O guarda DEVE descobrir os contratos ignorados executando
  `cargo test -p flora-tests -- --ignored --list` com `cwd=ROOT` via
  `subprocess.run(..., capture_output=True, text=True)`, consumindo somente
  `stdout`.
- **FR-002**: O guarda DEVE extrair de `.github/workflows/rust.yml` cada
  `run_leg <label> <filter> <floor>` com regex ancorada em linha e associar
  cada nome de teste terminado em `: test` à leg cujo `filter` seja prefixo
  do nome (`startswith`).
- **FR-003**: Para cada leg, o guarda DEVE exigir que `label` esteja em
  `FLOORS`, que o `floor` do workflow seja igual a `FLOORS[label]`, que haja
  exatamente uma `run_leg` por rótulo e que a quantidade de nomes ignorados
  casados pelo filtro seja exatamente igual ao piso.
- **FR-004**: O guarda DEVE falhar (exit != 0) se o cargo falhar, se a lista
  de ignorados estiver vazia, se não houver legs ou se qualquer contagem
  divergir; e DEVE imprimir `all N db contract floors intact (M total)`
  quando tudo passar.
- **FR-005**: As checagens estáticas existentes (`REQUIRED`, jobs,
  `continue-on-error`, path filters, `cargo build/test --workspace`) e o modo
  `--remote` DEVEM ser preservados; a implementação DEVE usar apenas stdlib e
  o diff DEVE ficar limitado a `scripts/ci/check_gate_integrity.py` e aos 4
  artefatos do ciclo 088, sem tocar workflow, `validate/`, `release/` ou
  `.specify/memory/STATE.md`.

### Key Entities

- **Oráculo dinâmico**: descoberta via `--ignored --list` usada como fonte de
  verdade para contar contratos por leg, em substituição ao espelho
  `FLOORS` ↔ `run_leg`.
- **Leg (`run_leg`)**: tripla `<label> <filter> <floor>` no workflow; `filter`
  é um prefixo de nomes de teste e `floor` é a contagem esperada.
- **FLOORS**: mapa determinístico no guarda (10 entradas, soma 58); continua
  sendo a declaração canônica dos pisos, agora conferida contra a descoberta.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `python3 scripts/ci/check_gate_integrity.py` retorna PASS com
  `all 10 db contract floors intact (58 total)`, sendo 58 a soma dos nomes
  `contract::…: test` realmente descobertos (13+4+8+8+5+6+4+3+3+4), não uma
  soma digitada no script.
- **SC-002**: divergência controlada — piso alterado em `FLOORS`, `run_leg`
  divergente ou contrato `#[ignore]` extra — faz o guarda retornar 1 com
  mensagem de piso/contagem; lista vazia ou cargo falhando também falha.
- **SC-003**: `git diff --name-only cbc885383` contém apenas
  `scripts/ci/check_gate_integrity.py` e os 4 artefatos 088; zero arquivos em
  `.github/workflows/`, `validate/`, `release/` ou envolvendo `STATE.md`.

## Assumptions

- O workspace de testes `flora-tests` é compilável no ambiente do job
  `rust-build`; `--ignored --list` apenas enumera testes e não exige Postgres.
- Os 58 contratos `#[ignore]` atuais são todos DB e pertencem a uma das 10
  legs; novos testes exigirão novo piso registrado (o oráculo força isso).
- Evidência de execução local e do gate é a registrada no handoff do ciclo;
  subagentes de documentação não reexecutam gates, testes ou formatters.
