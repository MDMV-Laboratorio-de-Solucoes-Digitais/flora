# Intake: 052 — OSV scan ref pin

**Data:** 2026-11-17T23:38Z · **Âncora:** `github/dev@01d822205` · branch `feature/052-osv-scan-ref-pin`

## Sinal

`.github/workflows/osv-scan.yml:14` referencia o workflow reutilizável
`google/osv-scanner-action/.github/workflows/osv-scanner-reusable.yml@v2`.
`git ls-remote` ao vivo não encontra `refs/heads/v2` nem `refs/tags/v2` no action
(only `v2.2.3` … `v2.7.1`), portanto o `@v2` não resolve para nenhum ref.
O scan semanal advisory não pode iniciar. Não é check obrigatório (ADR-0003:23),
logo não bloqueia merge — o defeito é invisível ao gate autoritativo.

## Evidência (verificada ao vivo)

- `git ls-remote https://github.com/google/osv-scanner-action.git 'refs/heads/v2' 'refs/tags/v2'` → vazio, exit 0.
- `git ls-remote --tags … 'refs/tags/v2*'` → `v2.7.1` em `32d3cd482bdadfb38b186c0ad1b031902d55d932`.
- `docs oficiais` (`google.github.io/osv-scanner/github-action/`) → snippet recomenda `@v2.7.1`.
- Workflow `v2.7.1` expõe inputs `scan-args` e `fail-on-vuln` (compatíveis com os atuais).
- Histórico local: STATE.md 051 nomeou "validar tag `osv-scan@v2` (6.0, [NV])" como próximo 80/20.

## 80/20

`(I×C)/(E×R)` = (3×5)/(1×1) = **15.0** — à frente de C25b (2.0), V1 (1.25) e N3 (1.0).
C5 exige decisão de dono; C6 exige autorização SE — ambos não autônomos.

## Fronteiras

Escopo: uma linha do workflow + artefatos do ciclo + notas datadas de estado.
Fora: gate autoritativo, ADR-0003, outros workflows, RBAC/RLS, merge, deploy.
Nenhum bug report em aberto em `.specify/bugs/`. Factory lida como fonte em
`d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7`; nada materializado no alvo.
