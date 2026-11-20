# Tasks: Bump de segurança rustls 0.23.43 → 0.23.46

**Input**: Design documents from `/specs/090-rustls-security-bump/`

**Prerequisites**: plan.md, spec.md, `.specify/assessments/090-rustls-security-bump/decision.md`

**Tests**: incluídos porque a spec exige gate, testes e dependency-review
limpo após o bump (FR-004, FR-005, SC-002, SC-003).

**Status**: T001–T006 concluídas — evidência registrada abaixo.

## Format: `[ID] [P?] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência)

---

## Phase 1: Setup & Diagnosis

- [X] **T001** Confirmar âncora do ciclo 090 e as 4 advisories.
  **Evidência**: `.specify/feature.json` =
  `{"feature_directory":"specs/090-rustls-security-bump"}`; branch
  `feature/090-rustls-security-bump` @ `824ba00e9`. Os 4 alertas canônicos
  (`docs/research/2026-11-16-028-advisory-job-plan.md:40`):
  `RUSTSEC-2026-0285` (medium, fix `0.23.45`), `RUSTSEC-2026-0346` (medium,
  fix `0.23.46`), `RUSTSEC-2026-0348` (low, fix `0.23.46`),
  `RUSTSEC-2026-0349` (low, fix `0.23.46`), publicados em 2026-11-02 /
  2026-11-15. Decisão: **go**, bump patch (FR-001).

- [X] **T002** [P] Diagnosticar a divergência lock ↔ isenção.
  **Evidência**: baseline `Cargo.lock:2748` registrava `rustls` `0.23.43`,
  abaixo das versões corretivas, enquanto
  `supply-chain/config.toml:1115-1117` já apontava `0.23.46`;
  `Cargo.toml:189` = `rustls = "0.23"` inalterado. O gate autoritativo
  continua `scripts/ci/check_gate_integrity.py` (10 legs / 58 contratos,
  ciclo 088), sem required check novo.

---

## Phase 2: Implementation

- [X] **T003** Aplicar o bump patch no lockfile.
  **Evidência**: `Cargo.lock:2748` passa a registrar `name = "rustls"` com
  `version = "0.23.46"`; nenhum `0.23.43` residual. `Cargo.toml:189`
  permanece `rustls = "0.23"` — faixa de compatibilidade preservada, sem
  mudança de manifest (FR-001/FR-002, SC-001).

- [X] **T004** [P] Alinhar a isenção cargo-vet à versão corrigida.
  **Evidência**: `[[exemptions.rustls]]` em
  `supply-chain/config.toml:1115-1117` com `version = "0.23.46"` e
  `criteria = "safe-to-deploy"` preservado. Lockfile e isenção apontam para a
  mesma versão auditada (FR-003, SC-001).

- [X] **T005** [P] Verificar gate, testes, dependency-review e fronteira do diff.
  **Evidência**: `python3 scripts/ci/check_gate_integrity.py` → **PASS**,
  `all 10 db contract floors intact (58 total)`, exit 0;
  `cargo test --workspace` → exit 0, zero falhas; `dependency-review` limpo,
  sem achados. Diff de código restrito a `Cargo.lock` +
  `supply-chain/config.toml`; manifest, workflows, ADRs, constituição,
  `validate/`, `release/` e `STATE.md` intocados (FR-004/FR-005, SC-002/SC-003).

---

## Phase 3: Verification & Documentation

- [X] **T006** Fechar os 4 artefatos 090 e a rastreabilidade.
  **Evidência**: SC-001 — leitura literal de `Cargo.lock:2748` = `0.23.46`
  (zero `0.23.43`) e de `supply-chain/config.toml:1115-1117` =
  `0.23.46` / `safe-to-deploy`, cobrindo as 4 advisories; SC-002 — gate PASS
  (10 legs / 58 contratos, exit 0) e testes exit 0; SC-003 — dependency-review
  limpo e diff confinado a 2 arquivos de código + 4 artefatos, zero arquivos
  em `validate/`, `release/` ou `STATE.md`. Artefatos: `decision.md`
  (verdict go, 2026-11-20, 4 advisories, bump patch), `spec.md`
  (FR-001..FR-005, SC-001..SC-003), `plan.md`, `tasks.md`. `analyze.md`
  registrou `critical=0`, `requirements_covered=6/6`, `fr=5/5`, `sc=1/1`,
  `tasks=6/6`, `errors=[]`; `converge.md` registrou equivalência
  spec/implementação. **[NV]** CI do PR/merge/produção, aprovações de
  maintainer e proteção de branch — reconsultar antes de qualquer
  merge/promoção; subagentes de documentação não reexecutaram gates, testes
  ou formatters.

## Dependencies

- T002 bloqueia T003/T004/T005 (diagnóstico da divergência antes da edição).
- T003 e T004 tocam arquivos distintos e podem rodar em paralelo.
- T005 depende de T003 + T004; T006 depende de T003 + T004 + T005.
- Nenhuma tarefa depende de Postgres, rede ao vivo, PR, merge, tag, deploy ou
  ambiente de produção.

## Implementation Strategy

MVP = T001–T005 (um patch de lockfile + uma linha de isenção). T006 fecha
verificação e documentação. O diff permanece cirúrgico: 2 arquivos de código
+ 4 artefatos; o bump não cria required check, não reescreve o manifest e
não altera gate, processo ou estado do ciclo.
