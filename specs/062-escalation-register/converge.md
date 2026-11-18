# Converge: 062 — Escalation register

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T04:31Z — entrada 1

- Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6: `(2×5)/(2×1)=5.0`.
  R1 `go (scoped)`; analyze PASS.
- Implementação: apêndice de 25 linhas em `.specify/memory/STATE.md`
  (25+/0−): tabela canônica com 4 itens escalados (ADR-0003/0006, N3, C5, C6),
  cada um com evidência, bloqueio, ação necessária e status **[NV]**; e
  parágrafo OSV com estado ao vivo via `gh api` (`active`, id 420259367,
  criado 2026-11-16, **0 runs**, cron `37 14 * * 1`, pin `@v2.7.1` ainda sem
  execução validadora — **[NV]**).
- Nenhum item bloqueado foi executado: nenhum ADR, migração, `rbac.rs`,
  workflow OSV, constituição, código ou gate alterado.
- Verificador isolado: PASS 6/6 — diff insertions-only, 4 linhas completas,
  6 fontes citadas conferidas, parágrafo OSV íntegro, itens bloqueados intactos,
  nenhuma injeção.
- Gates locais (04:29Z): guardião PASS, fmt/clippy/deny/build/test exit 0;
  frontend check 0/0, lint ok, 42/42, build ok.
- Factory fonte `4a5de2ce218fa253a06de313c8008aea78aab351`; nada transplantado.
