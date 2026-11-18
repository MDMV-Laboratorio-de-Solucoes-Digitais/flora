# Converge: 063 — Validate OSV run

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T04:50Z — entrada 1

- Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6: `(2×5)/(2×2)=2.5`.
  R1 `go (scoped)`; analyze PASS.
- Implementação: disparo de `osv-scan.yml` via `workflow_dispatch --ref dev`
  (gatilho declarado no workflow, sem merge/force-push/deleção). Run
  `44239662140`: `status=completed`, `conclusion=success`,
  `createdAt=04:46:43Z`, `updatedAt=04:47:13Z`. Etapas checkout/setup/
  reporter/upload artefato/upload code-scanning concluídas. Nenhum código,
  workflow, ADR ou gate alterado.
- Resultado registrado em `STATE.md` (27+/0−): fecha o **[NV]** de 052 e o
  apêndice de 04:29Z, preservando ambos como histórico; ressalva explícita de
  que `fail-on-vuln: false` torna `success` ≠ "zero vulnerabilidades".
- Verificador isolado: PASS 6/6 (diff, campos do run, ressalva, gatilho/pin,
  nenhum segredo copiado, nenhuma injeção).
- Gates locais (04:49Z): guardião PASS, fmt/clippy/deny/build/test exit 0;
  frontend check 0/0, lint ok, 42/42, build ok.
- Factory fonte `b5bc7265d7fe6efaa316c54a54eed1c3ce1e71bc`; nada transplantado.
