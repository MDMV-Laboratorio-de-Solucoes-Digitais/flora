# Assess — define: 112

**Data**: 2026-11-22

## Requisitos funcionais

- **FR-001**: todo botão ícone das rotas `(app)` expõe nome acessível via `aria-label` e chave pt-BR do dicionário (BUG-111-01).
- **FR-002**: botões desabilitados usam `bg-muted`/`text-muted-foreground` com contraste ≥4.5:1; `--muted-foreground` ajustado (BUG-111-02).
- **FR-003**: `app.html` declara `<title>` não vazio (BUG-111-03).
- **FR-004**: varredura axe permanece informativa, com resumo visível no log do CI; nenhuma regra desativada.

## Requisitos não funcionais

- Nenhum required check, gate floor, migration ou credencial alterado.
- Commits PT-BR, PR em inglês, roteiro e evidência antes do merge.
