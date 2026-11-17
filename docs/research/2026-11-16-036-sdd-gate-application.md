# 2026-11-16-036 — Aplicação dos gates determinísticos SDD/produção a Flora

**Tipo:** research (somente leitura; nenhum arquivo de código/governança alterado).
**Escopo:** `mdmv-dark-factory` (`scripts/ci/check_sdd_completeness.py`,
`scripts/ci/check_production_gate.py`, `docs/governance/*.policy.json`,
`.specify/memory/constitution.md`) aplicado a este repositório Flora.
**Data:** 2026-11-16.

## Conclusão (resposta direta)

Os gates **não podem ser executados contra Flora "as-is"**. Ambos são
**hub-specific** por construção:

| Gate | Mecanismo de acoplamento ao hub | Status para Flora |
|---|---|---|
| `check_production_gate.py` | `ROOT = Path(__file__).resolve().parents[2]` fixo no hub; lê `docs/governance/production-gate-hub.policy.json` (policy `"mdmv-hub-production-gate"`); executa 22 scripts que só existem no hub (`hub_docs_gate.py`, holdout, telemetry, runner, sandbox, `scripts/policy/check_policy.py`, etc.); verifica `.forgejo/workflows/hub-docs-gate.yml`; `--release <slug>` lê `.specify/validations/<slug>/validate.md` e `.specify/releases/<slug>/release.md` **no ROOT do hub**. Não tem `--root`. | **Inaplicável.** Invocá-lo a partir de Flora ainda verifica o hub — não produce resultado sobre Flora. Não existe invocação válida para Flora. |
| `check_sdd_completeness.py` | Tem `--root`/`--baseline`/`--json`, então roda tecnicamente, mas a semântica é hub-specific: espera `.specify/{assessments,validations,releases}/<slug>/` **sem prefixo numérico** (convenção do hub) e baseline `docs/governance/sdd-completeness-baseline.json`. Flora usa diretórios **com prefixo** (`010-compose-uuidv7-smoke-frontend-verify`) e não tem `docs/governance/`. | **Executável, mas FAIL estrutural.** Requer renomeação de diretórios + baseline própria + artefatos faltantes para passar. |

A constituição do hub (v1.1.0, gates R1/R3/R4/R5/R7, portarias
`00→01→…→26`) **não é** a constituição de Flora (v1.3.0, princípios I–VI,
TDD, commits atômicos, CI 100% via GitHub Actions). Portanto as portarias não
são automaticamente herdadas; adoção exige mapeamento local/ADR.

## Evidência 1 — produção: acoplamento rígido ao hub

`check_production_gate.py` (linhas 6, 15, 22, 28–31):

```python
ROOT = Path(__file__).resolve().parents[2]          # sempre mdmv-dark-factory
policy = ... (ROOT/"docs/governance/production-gate-hub.policy.json")
wf = ROOT/".forgejo/workflows/hub-docs-gate.yml"
for spec in policy["required_scripts_pass"]:        # 22 comandos hub-only
    cmd = ["python3"] + spec.split()
```

Flora não possui `scripts/ci/`, `docs/governance/`, `.forgejo/`,
`scripts/holdout/`, `scripts/telemetry/`, `scripts/runner/`, `scripts/sandbox/`
nem `scripts/policy/`. Seus gates reais são os do `AGENTS.md` / CI:
`cargo fmt --all -- --check` → `cargo clippy --workspace --all-targets -- -D warnings`
→ `cargo deny check bans licenses sources` → `cargo build` → `cargo test`
(`.github/workflows/rust.yml`), além de `bun run check/lint/test:unit/build` em
`apps/frontend` (`.github/workflows/ci.yml`).

Policy `production-gate-hub.policy.json` (schema `043.2.0`) declara
`human_approval_required: false` e `required_deterministic_checks:
["hub-docs-gate / hub-docs-gate (pull_request)"]` — status context do hub,
inexistente em Flora. `release_gate_requirements` (linhas 40–45) é a parte
**reutilizável como checklist manual** (ver abaixo).

`no-human-review-eligibility.policy.json` tem `scope: mdmv-dark-factory-hub-docs-only`
e `status: advisory-draft`; `nv` registra que branch protection do hub ainda
exige approvals=1. Ou seja: nem o hub habilitou auto-merge sem humano — não há
base para aplicar isso a Flora.

## Evidência 2 — SDD completeness: rodável, mas incompatível

Inventário Flora: 34 diretórios `specs/[0-9][0-9][0-9]-*`; `.specify/`
contém `assessments/`, `validations/`, `releases/`, `bugs/`, `memory/`,
`scripts/`, `templates/`, `workflows/`. Não há `docs/governance/`.

O gate calcula `slug = name.split("-", 1)[1]` (linha 26) e procura esse slug
em conjuntos construídos a partir dos nomes dos diretórios (linhas 36–38).
No hub os diretórios são `autonomous-execution-harness/` (sem número); em
Flora são `010-compose-uuidv7-smoke-frontend-verify/` (com número). Resultado:
mesmo quando a evidência existe, o gate não a encontra.

Invocação executada (somente leitura):

```bash
python3 /home/luis/development/MDMV/projetos/mdmv-dark-factory/scripts/ci/check_sdd_completeness.py \
  --root /home/luis/development/MDMV/projetos/Arquivados/flora \
  --baseline /tmp/nonexistent-flora-baseline.json
```

Saída real (abreviada, 34 linhas `REGRESSIONS`; exit 1):

```text
sdd-completeness: FAIL (spec_dirs=34 gaps=34 baselined=0)
REGRESSIONS (missing artifacts not in baseline):
  - 001-flora-seed-v0-1: assessment, release, validate
  - 002-svelte-frontend-rewrite: assessment, release, validate
  ...
  - 021-unassign-notification-dedup-search-hardening: assessment, checklists, release, validate
  - 025-seed-fase0-verification: assessment, checklists, plan.md, release, validate
  ...
  - 034-origin-boundary: assessment, checklists, plan.md, release, validate
```

Notas sobre a invocação:

- Sem `--baseline`, o default é o baseline **do hub** (caminho absoluto
  calculado no import). Nesse caso, além das 34 regressões, aparecem entradas
  `STALE_BASELINE` para features do hub (`001-dark-factory-meta-loop`, etc.).
- `--json` produces `{"schema":"047.1.0","ok":false,...}` com `regressions` e
  `stale_baseline`.
- Há gaps **reais** misturados com falsos negativos: specs 025–034 não têm
  `plan.md` nem `checklists/*.md` (verificável em `specs/`); specs 001–020
  têm `spec.md`/`plan.md`/`tasks.md`/`checklists/`, mas a evidência de
  assess/validate/release está em diretórios com prefixo numérico, que o
  script não reconhece.

Para " PASS" seu uso em Flora exigiria: (a) criar
`docs/governance/sdd-completeness-baseline.json` específico de Flora,
(b) adotar a convenção de nomes sem prefixo em `.specify/{assessments,validations,releases}/`
(ou adaptar o script — fora do escopo desta research), e (c) preencher
`plan.md`/`checklists/` das specs 025–034.

## Checklist manual para o ciclo de bugs de Flora

Derivado de (i) `REQ_SPEC_FILES`/`REQ_DERIVED` do gate SDD, (ii)
`release_gate_requirements` da production policy, (iii) gates R1/R3/R4/R5/R7
da constituição do hub, (iv) `SATELLITE-ADOPTION-CHECKLIST.md`, (v) prática
já existente em `.specify/bugs/`.

### A. Portaria do bug (R5 — paralelo, rastreável, desconectado)

- [ ] Bug reportado/registrado em `.specify/bugs/<slug>/` (Flora já usa esse padrão).
- [ ] `assessment.md` com: slug, data, fonte/URL, `Verdict` (valid/invalid),
      `Severity`,_report verbatim_, sintoma, causa-raiz e escopo.
- [ ] `fix.md` com: slug, data, link para `assessment.md`, `Status: applied`,
      resumo, tabela de arquivos alterados, diff highlights, testes
      adicionados/atualizados.
- [ ] `test.md` com: slug, data, links para assessment/fix, `Result`
      (pass/partial/fail), tabela de verificações (comando, resultado, nota)
      e trechos de saída.
- [ ] Flag explícita de que o fluxo é paralelo ao spec principal; não
      inventar vínculo FR/T se o bug não tiver spec pai.
- [ ] Se `Result: partial` (caso do bug `forgejo-rust-ci-rust-toolchain-action-not-found`),
      listar cada check `not-run` como pendência com responsável/gatilho
      (re-executar CI no runner Forgejo e observar CI do GitHub).

### B. Artefatos SDD quando o bug toca uma spec numerada (gate SDD)

- [ ] `specs/<NNN>-*/spec.md` presente e atualizado com o comportamento
      corrigido.
- [ ] `specs/<NNN>-*/plan.md` presente.
- [ ] `specs/<NNN>-*/tasks.md` presente, com ID de tarefa único para a correção.
- [ ] `specs/<NNN>-*/checklists/*.md` com pelo menos um checklist executado.
- [ ] Evidência de assess (`.specify/assessments/<slug>/`),
      validate (`.specify/validations/<slug>/validate.md`) e
      release (`.specify/releases/<slug>/release.md`) — **usar a convenção
      de nomes do hub (sem prefixo numérico) se o gate vier a ser adotado**.
- [ ] Gaps históricos conhecidos registrados em baseline própria de Flora,
      nunca silenciados por `continue-on-error` ou `#[allow]`.

### C. Release / produção (production policy)

- [ ] `validation_evidence_exists` — validate aprovado **antes** de release (R4).
- [ ] `release_record_exists` — release.md com SemVer (R7).
- [ ] `no_unresolved_blocking_escalation_for_scope` — nenhuma escalação
      bloqueante aberta no escopo.
- [ ] `deterministic_checks_green` — em Flora: `cargo fmt --check`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo deny check bans licenses sources`, `cargo build`, `cargo test`
      e, para frontend, `bun run check`, `bun run lint`,
      `bun run test:unit -- --run --project server`, `bun run build`.
- [ ] `decision.md` com veredito `go` se houver nova spec (R1).
- [ ] No máximo 3 esclarecimentos, um por vez (R3).
- [ ] Changelog vinculando IDs FR/T/bug, plano de rollback e comunicação (R7).
- [ ] Converge append-only: não reescrever histórico de clarify/converge.

### D. Adoção como satélite (se o objetivo for integração ao hub)

- [ ] Identidade: `target_repo` (flora-workspace), host (`forgejo.mdmv.qzz.io`/
      GitHub), owner, branch default; `hub_sha` do dark-factory avaliado.
- [ ] `satellite-manifest.json` baseado no template do hub, com
      `adopted_policies` mínimas (022/023/024/025/026/028/031) e overrides
      marcados `local-only`/`proposta-pendente` com racional.
- [ ] CI com gates determinísticos bloqueando merge + required status checks
      e branch protection (hoje: GitHub Actions; Forgejo é mirror).
- [ ] Runner self-hosted seguro (labels, sem segredos em logs).
- [ ] Telemetria 035.1.0 (emit/validate) e retorno ao hub, se exigido.
- [ ] Secrets hygiene, least privilege, pins de supply chain, sem auto-merge
      cego.
- [ ] Drift periódico contra `hub-policy-manifest.json` → SE decision request
      ou ADR local.
- [ ] **Atenção:** `GUARDRAILS-HUB-ONLY.md` registra que propagação,
      enforcement em satélites e drift ainda são "parciais" e que a regra de
      escala é não escalar para 3+ satélites sem evidência operacional de G2/G3.
      Flora é repositório **arquivado** — adotar o ciclo como checklist manual
      local é coerente; integrá-lo como satélite ativo exigiria decisão explícita.

## Comandos de referência (hub — não Flora)

```bash
# no root de mdmv-dark-factory
python3 scripts/ci/check_sdd_completeness.py
# → sdd-completeness: PASS (spec_dirs=<N> gaps=<G> baselined=<B>)

python3 scripts/ci/check_production_gate.py --check
# → JSON schema 043.2.0: {"ok": true|false, "errors": [], "warnings": [], "results": [...]}

python3 scripts/ci/check_production_gate.py --release <slug>
# → acrescenta checagem de .specify/validations/<slug>/validate.md e
#   .specify/releases/<slug>/release.md (no hub)
```

## Lacunas / MISSING

- Não existe baseline SDD de Flora, nem mapeamento ADR entre a constituição
  v1.3.0 de Flora e as portarias R1–R7 do hub.
- Não foi verificado se os 22 scripts exigidos pela production policy do hub
  estão verdes no próprio hub neste checkout (fora do escopo; a policy
  registra `nv` sobre workflow em base/dev e proteção de branch).
- O bug `forgejo-rust-ci-rust-toolchain-action-not-found` tem `test.md` com
  `Result: partial` e checks `not-run` — depende de execução remota no Forgejo
  para fechar.
