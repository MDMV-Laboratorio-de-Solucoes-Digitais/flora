# Decision: 047 — Release manifest lockstep

**Verdict:** go (scoped)

## Decisão

Aprovar a correção da contagem de manifests em `.github/workflows/release.yml` de 8
para 6, preservando a asserção de versão em lockstep. A identidade do ciclo é 047
porque 043–046 já estão ocupados por validations/releases, embora `specs/` pare em 042.

## Delimitadores

- Uma linha funcional alterada + commentário; nenhum outro arquivo de código.
- Nenhum tag/release/deploy/push/auto-merge; merge permanece decisão de máquina sob
  4 required checks verdes (ADR-0003).
- Documentação (`OBJETIVO.md`, `ROADMAP.md`) atualizada em nota append-only separada.

## NV

Execução do workflow em GitHub Actions e proteção de branch: **[NV]**.
