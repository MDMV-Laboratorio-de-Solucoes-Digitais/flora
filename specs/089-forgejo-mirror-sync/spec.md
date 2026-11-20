# Feature Specification: Espelho Forgejo com push fast-forward e job advisory

**Feature Branch**: `feature/089-forgejo-mirror-sync`

**Created**: 2026-11-20

**Status**: Implementado (evidência e escopo registrados em `tasks.md`)

**Input**: Espelhar `dev`, `main` e tags `v*` para
`https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` sem reescrever
histórico e sem criar required check (ADR-0003). Âncora: branch
`feature/089-forgejo-mirror-sync` @ `15203ee78`; `origin/dev` = `f12a328c` é
ancestral de `dev` (fast-forward).

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Publicar `dev`/`main` no Forgejo por fast-forward (Priority: P1)

Como mantenedor, quero que um push em `dev` seja espelhado automaticamente
para o Forgejo somente quando o commit local descende do tip remoto, para que
o espelho nunca tenha seu histórico reescrito.

**Why this priority**: é a finalidade do ciclo; `origin/dev` = `f12a328c` já é
ancestral de `dev`, então a condição de FF é satisfeita e verificável.

**Independent Test**: com `forgejo/dev` apontando para `f12a328c`, disparar
push em `dev`; o job atualiza o remote-tracking, confirma a ancestralidade com
`git merge-base --is-ancestor` e faz `git push` sem force.

**Acceptance Scenarios**:

1. **Given** `forgejo/dev` = `f12a328c` e `HEAD` descendente dele,
   **When** o push em `dev` dispara o job, **Then** o guarda de
   ancestralidade passa e o push para `refs/heads/dev` termina com exit 0.
2. **Given** `forgejo/dev` com um commit que não é ancestral de `HEAD`
   (divergência), **When** o job roda, **Then** `merge-base --is-ancestor`
   falha, o job termina antes do push e nenhum force/deleção é tentado.

---

### User Story 2 — Credenciais isoladas e falha fechada (Priority: P2)

Como operador de infraestrutura, quero que `FORGEJO_USER` e `FORGEJO_TOKEN`
sejam lidos de secrets com falha fechada, para que um secret ausente nunca
vire um "sucesso" silencioso nem vaze para logs.

**Why this priority**: autenticação quebrada ou vazamento de token é risco
operacional e de segurança; o job não pode depender de credenciais embutidas.

**Independent Test**: rodar o step sem um dos secrets → erro imediato e exit
!= 0; com secrets válidos, conferir que a URL do remoto não contém credenciais
e que nenhum valor de token aparece na saída.

**Acceptance Scenarios**:

1. **Given** `FORGEJO_USER` ou `FORGEJO_TOKEN` vazio, **When** o step inicia,
   **Then** ele emite erro de secret obrigatório e retorna 1 sem tocar o
   remoto.
2. **Given** ambos os secrets presentes, **When** a autenticação é
   configurada, **Then** o token vai para `http.<host>/.extraHeader` via
   Basic, é descartado com `unset` e permanece fora da URL, de argumentos e de
   logs.

---

### User Story 3 — Escopo advisory e diff cirúrgico (Priority: P3)

Como dono do processo de merge, quero o espelho fora do conjunto de required
checks e com diff limitado ao workflow e aos artefatos do ciclo, para que
ADR-0003 e o gate autoritativo permaneçam intactos.

**Why this priority**: promover o espelho a gate criaria uma nova dependência
de infraestrutura externa no merge autônomo; alterar `validate/`, `release/`
ou `STATE.md` quebraria o registro auditável.

**Independent Test**: inspecionar gatilhos e nome do job no workflow e listar
o diff da branch; conferir que não há mudança em proteção de branch, checks
obrigatórios, `validate/`, `release/` ou `STATE.md`.

**Acceptance Scenarios**:

1. **Given** o workflow do espelho, **When** seus gatilhos são lidos, **Then**
   só há `push` em `dev`, `main` e tags `v*`; o job `mirror` não aparece no
   conjunto de required checks (ADR-0003).
2. **Given** a branch 089 sobre `15203ee78`, **When** o diff é listado,
   **Then** aparecem apenas `.github/workflows/mirror-to-forgejo.yml` e os 4
   artefatos 089; zero arquivos em `validate/`, `release/` ou envolvendo
   `STATE.md`.

---

### Edge Cases

- **Divergência do espelho**: remote à frente ou com histórico distinto falha
  o guarda FF; nunca é feito `--force`, `--mirror` ou deleção de refs.
- **Branch remota ausente/infetchável**: o `fetch` das branches `dev`/`main` é
  tolerado, mas a ancestralidade falha fechado se o ref de destino não existir.
- **Corrida entre verificação e push**: `concurrency` por `github.ref` com
  `cancel-in-progress: false` serializa os pushes; um push rejeitado pelo
  Forgejo encerra o job com erro, sem retentativa forçada.
- **Refs fora do escopo**: branches diferentes de `dev`/`main` e tags sem
  prefixo `v*` são recusados em defesa em profundidade, mesmo com gatilho
  restritivo.
- **Estado ao vivo [NV]**: condição real do remoto Forgejo, do job e da
  proteção de branch não foi reconsultada nesta avaliação de documentação.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O job DEVE ler `FORGEJO_USER` e `FORGEJO_TOKEN` de `secrets.*`
  para o `env` do step, falhar fechado se qualquer um estiver vazio e
  autenticar via `http.<host>/.extraHeader` com Basic sem embutir credenciais
  na URL do remoto, em argumentos ou em logs; o valor DEVE ser descartado
  após a configuração.
- **FR-002**: O job DEVE espelhar somente pushes em `dev`, `main` e tags `v*`
  para `FORGEJO_REMOTE=https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`
  (URL não secreta), com checkout de histórico completo
  (`fetch-depth: 0`, `persist-credentials: false`), `permissions: {}` e
  serialização por ref (`cancel-in-progress: false`).
- **FR-003**: Antes de cada push de branch, o job DEVE atualizar
  `refs/remotes/forgejo/dev` e `refs/remotes/forgejo/main` e exigir
  `git merge-base --is-ancestor forgejo/<branch> HEAD`; o envio DEVE ser
  `git push forgejo HEAD:refs/heads/<branch>`, sem `--force`, sem `--mirror` e
  sem deleção de refs. Falha do guarda, divergência ou push rejeitado DEVEM
  encerrar o job sem tentativa de reescrita. Premissa verificada:
  `origin/dev` = `f12a328c` é ancestral de `dev`.
- **FR-004**: O job DEVE ser informativo/advisory sob ADR-0003: gatilho
  somente `push`, nome `mirror` fora do conjunto de required checks e nenhuma
  alteração em proteção de branch, `REQUIRED`, `ALLOWED_JOB_IF` ou jobs de CI
  existentes. O gate autoritativo permanece
  `scripts/ci/check_gate_integrity.py` (10 legs / 58 contratos, ciclo 088).
- **FR-005**: A mudança DEVE ficar limitada a
  `.github/workflows/mirror-to-forgejo.yml` e aos 4 artefatos mínimos do
  ciclo 089 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`); nenhum arquivo
  em `validate/`, `release/` ou `.specify/memory/STATE.md` pode ser tocado, e
  nenhum segredo pode ser registrado nos artefatos.

### Key Entities

- **Espelho (mirror)**: cópia one-way do histórico local no Forgejo; nunca
  bidirecional, nunca com `--mirror` ou force-push.
- **Guarda de ancestralidade (fast-forward)**: `git merge-base --is-ancestor
  forgejo/<branch> HEAD` — única condição que autoriza o push de branch.
- **Secrets de acesso**: `FORGEJO_USER` e `FORGEJO_TOKEN`, transportados via
  `env`/`extraHeader`, jamais interpolados no corpo do script ou na URL.
- **Job advisory**: `mirror` no workflow `mirror-to-forgejo.yml`, fora dos
  required checks de ADR-0003.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Com `origin/dev` = `f12a328c` ancestral de `dev`, um push em
  `dev` executa o job, o guarda FF passa e o push chega a
  `refs/heads/dev` sem force; a saída não contém nenhum valor de secret.
- **SC-002**: Secret ausente, ref divergente, branch remota indisponível ou
  push rejeitado produzem exit != 0 e nenhum push/force/deleção; estados ao
  vivo permanecem marcados `[NV]` até reconsulta.
- **SC-003**: Gatilhos do job são apenas `push` em `dev`/`main`/`v*` e o nome
  `mirror` não está entre os required checks (ADR-0003); o diff contém apenas
  o workflow e os 4 artefatos 089, com zero arquivos em `validate/`,
  `release/` ou `STATE.md`.

## Assumptions

- **Fast-forward obrigatório**: o histórico local é a fonte da verdade do
  espelho; qualquer divergência é tratada como falha, não como conflito a
  resolver automaticamente.
- **Secrets**: `FORGEJO_USER` e `FORGEJO_TOKEN` estão provisionados no
  repositório; nenhum valor foi copiado, gravado ou exposto neste ciclo.
- **ADR-0003**: o job de espelho não é required check; CI green continua
  sendo suficiente para merge autônomo em `dev`.
- Tags `v*` são espelhadas; demais refs são recusadas. Não há deploy,
  migração, mudança de gate ou alteração de documentação de estado neste ciclo.
- Evidência de execução e de escopo do diff é a registrada em `tasks.md`; subagentes de
  documentação não reexecutam gates, testes ou formatters.
