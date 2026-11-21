# 104 — Identidade real da sessão na navegação

**Data**: 2026-11-21 · **Branch**: `feature/104-session-identity` @ `23ff4bc809`
**Âncora**: `.specify/feature.json` → `specs/104-session-identity`
**Gate do alvo**: `python3 scripts/ci/check_gate_integrity.py` — **PASS** (`exit=0`, 10 legs / 77 pisos)
**Fonte factory (somente leitura)**: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`

## Sinal e decisão

A navegação exibia os literais fixos `Nome do usuário` e `usuario@flora.app`, mesmo com sessão autenticada. O JWT de sessão já contém o claim `email` (`flora-core/src/utils/jwt.rs:25`) e `authState.token` é reativo. Decisão: decodificar o e-mail no cliente **somente para exibição**, com fail-closed, mantendo os literais do dicionário como fallback. Nenhuma autorização passa a depender do valor decodificado; o backend continua validando assinatura e claim.

Fora de escopo: backend, gate, required checks, ADR, constituição, schema, flags, novas chaves i18n e o chat (`Usuário <id>`), que depende de lookup de usuários.

## Verificação observada

- Vitest projeto `server`: 14 arquivos / 49 testes (7 novos casos de `emailFromToken`).
- `bun run check`: 0 erros/0 warnings; prettier/eslint ok nos arquivos alterados.
- Navegador real: `qa-admin` / `qa-admin@flora.test` no rodapé da navegação.
- Gate do alvo PASS, 10 legs / 77 pisos.

Rollback: novo commit de reversão dos três arquivos frontend, sem force-push; nenhuma migration ou variável a desfazer.

**[NV]**: staging real, CI remota, PR/merge e deploy. Nenhum segredo registrado. Dívida: autoria das mensagens no chat ainda mostra ID do usuário.
