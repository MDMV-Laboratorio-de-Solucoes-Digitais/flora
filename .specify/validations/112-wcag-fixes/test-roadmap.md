# Test roadmap — ciclo 112

**Data**: 2026-11-22 · **Baseline**: `ce3ce632e` · **Superfície**: UI real (SvelteKit) + CLI/HTTP

## Escopo

FR-001/002/003/004; bugs BUG-111-01/02/03. Nenhum required check alterado.

## Pré-condições

- Head do PR publicado; job `e2e-browser` (advisory) com Postgres/API/frontend provisionados pelo CI.
- Dicionário pt-BR compilado; nenhuma credencial de staging necessária (seed local do CI).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 10 legs / 77 pisos | CLI | sim |
| 2 | `bun run check` no worktree | 0 erros, 0 avisos | CLI | sim |
| 3 | `bunx prettier --check` nos arquivos alterados | exit 0 | CLI | sim |
| 4 | `gh pr checks <PR> --watch` no head exato | required checks verdes | CI | sim |
| 5 | `gh run view <run> --log-failed`/`--log` filtrando `WCAG 2.2 AA em` | duas linhas "sem achados" | CI/HTTP | sim |
| 6 | Inspeção visual de `/tasks` e `/channels` no preview local com sessão seed | botões com rótulo, contraste legível | UI (browser) | parcial — depende de credenciais de staging; usar CI |
| 7 | Code Review agêntico independente | achados advisory postados; blockers determinísticos corrigidos | PR | sim |

## Capturas

Se UI local disponível: `/tmp/flora-112-tasks.png` e `/tmp/flora-112-channels.png`, redigindo tokens. Na ausência de sessão segura, anexar log do job como evidência e marcar passo 6 **[NV]**.

## Rollback

Novo commit de reversão dos arquivos alterados; sem force-push.
