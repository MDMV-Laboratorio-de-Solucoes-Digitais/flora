# 043 — Manifesto agent-readable sem drift de tenancy

**Data:** 2026-11-17 · **Tipo:** correção de documentação viva + 1 teste ·
**Metodologia:** hub `mdmv-dark-factory` aplicado ao Flora; gates determinísticos como
autoridade (constituição V).

## 1. Decisão 80/20

Follow-up D5 da auditoria do 042: o `/llms.txt` (manifesto agent-readable, superfície
que LLMs e agentes consomem) anunciava 8 endpoints como "requires auth + `X-Organization-ID`" —
mas **nenhuma rota lê esse header** (ciclo 024 / ADR-0001: a org vem da sessão verificada
via `require_org()`). Isso ensina a agentes um mecanismo de auth que não existe e é
spoofável. Corrigido no manifesto vivo e no comentário do roteador.

| Candidato | Impacto | Confiança | Esforço | Veredito |
|---|---|---|---|---|
| D5 — manifesto/rotas sem drift de header | Médio-alto | Alta (3 auditorias) | Muito baixo | **FEITO** |
| D3 — normalizar permissões do seed | Médio | Alta | Médio | Não (dados canônicos, autonomia NÃO) |
| RLS/role least-privilege | Alto | Baixa | Alto | Não (autorização SE) |

## 2. Evidência (c043-01)

- `health.rs:69-76`: 8 linhas com `(requires auth + X-Organization-ID)` — enganoso.
- `mod.rs:30`: comentário "require auth + X-Organization-ID" — obsoleto.
- `cors.rs:34`: `x-organization-id` aparece **só** na allowlist CORS (compatibilidade,
  não auth) — confirmado por grep: nenhum handler lê o header.
- Org real: `require_org()` validada contra a linha de sessão no Postgres (ciclo 024).
- Premissas falsas descartadas: toolchain já 1.98.1 nos pinos; e2e vivos; `/metrics`
  stub documentado com precisão.

## 3. Resultado

Manifesto agora descreve "verified session Bearer token" e explicita que a org vem da
sessão, não de header. Novo teste `test_llms_txt_does_not_advertise_header_as_auth`
garante a ausência. Gates: fmt/clippy/deny/test verdes (29 flora_api lib vs 28 antes).
Diff restrito a `health.rs` + `mod.rs`. Nenhum comportamento de runtime alterado.
