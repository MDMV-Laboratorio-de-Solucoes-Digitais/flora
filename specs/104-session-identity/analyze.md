# Analyze: 104 — identidade real da sessão na navegação (READ-ONLY)

**Data**: 2026-11-21 · **Branch**: `feature/104-session-identity` @ `23ff4bc809` · **Baseline**: `23ff4bc809`
**Artefatos revisados**: `.specify/assessments/104-session-identity/{intake,research,define,shape,decision}.md` (`go`), `specs/104-session-identity/{spec,plan,tasks}.md`, `flora-core/src/utils/jwt.rs`, `flora-api/src/routes/auth.rs`, `apps/frontend/src/lib/state/AuthState.svelte.ts`, `apps/frontend/src/routes/(app)/+layout.svelte`.

## Diagnóstico determinístico

- A navegação exibia `Nome do usuário` / `usuario@flora.app` (`+layout.svelte:182-185`, `pt-BR.ts:53-54`), independentemente da sessão.
- O JWT de sessão carrega `email` como claim (`jwt.rs:17-28`), emitido em todos os caminhos (OIDC, bootstrap e troca de organização). `authState.token` é `$state` reativo (`AuthState.svelte.ts:35`), disponível no layout.
- Correção estritamente frontend: helper puro `emailFromToken`, `$derived` no layout, fallback nos literais existentes. Nenhuma autorização usa o valor decodificado; o backend continua validando assinatura/claim.

## Checagem determinística

```
artifacts=8  critical=0  fr=5/5  sc=3/3  tasks=8/8  go=True  divergences=0
FR match= True ['FR-001','FR-002','FR-003','FR-004','FR-005']
SC match= True ['SC-001','SC-002','SC-003']
```

Decodificação base64url com `atob` + `TextDecoder`, fail-closed para tokens ausentes/malformados/payload sem `email` válido (testes Vitest: 14 arquivos / 49 testes no total do projeto, incluindo 7 casos novos).

## Rastreabilidade

| # | Requisito | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — helper puro e fail-closed | T002 | `apps/frontend/src/lib/utils/jwt.ts`, `tests/unit/jwt.test.ts` |
| R2 | FR-002 — e-mail real no rodapé da navegação | T003 | `$derived(emailFromToken(authState.token) ?? t['nav.userEmail'])` |
| R3 | FR-003 — nome derivado da parte local, fallback pt-BR | T004 | renderização medida: `qa-admin` / `qa-admin@flora.test` |
| R4 | FR-004 — nenhuma mudança de backend/gate/i18n | T005 | somente 3 arquivos frontend alterados |
| R5 | FR-005 — rastreabilidade e fronteira | T006–T008 | artifacts + validate/governance; chat permanece dívida |

SC-001 (e-mail correto exibido) por R1/R2/R3; SC-002 (fallback seguro) por R1/R3; SC-003 (sem uso para autorização) por R1/R4.

## [NV]

- Chat (`ChatLayout.svelte`) ainda exibe `Usuário <id>`: exige lookup de usuários no backend — fora de escopo, registrado como dívida.
- Staging real, CI remota, PR/merge: **[NV]** até verificação ao vivo.

## Resultado

**PASS — 5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos.** Nenhuma edição nesta fase.
