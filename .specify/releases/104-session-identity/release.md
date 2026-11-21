# Release: 104 — identidade real da sessão na navegação

**Ciclo**: 104 · **Data**: 2026-11-21 · **Branch**: `feature/104-session-identity` @ `23ff4bc809`
**Baseline**: `23ff4bc809`

## Entregue

Rodapé da navegação exibe o e-mail real da sessão (e a parte local como nome) em vez dos literais fixos. Helper puro `emailFromToken` com fail-closed e 7 testes unitários; nenhuma mudança de backend, gate, required check, ADR, schema, flag ou nova chave i18n.

## SemVer

**Proposta:** PATCH `0.7.5` → `0.7.6`. Ajuste de exibição em capacidade existente, com fallback preservado. Não é MINOR/MAJOR: nenhuma API, rota, claim, configuração ou contrato novo.

## Changelog (FR / T)

- `FR-001` helper `emailFromToken` base64url/UTF-8, fail-closed, apenas para exibição (T002).
- `FR-002` `$derived` no layout com claim `email` do token (T003).
- `FR-003` nome derivado da parte local e fallback nas chaves existentes do dicionário (T004).
- `FR-004` fronteira preservada: nenhum backend/gate/i18n novo (T005).
- `FR-005` rastreabilidade e verificação em navegador real (T006–T008).
- **Dívida**: autoria no chat ainda exibe ID (`ChatLayout.svelte`); exige lookup de usuários, fora deste ciclo.

## Evidência

`.specify/validations/104-session-identity/validate.md` (**R4 APROVADO**) · `specs/104-session-identity/analyze.md` (5/5 FR, 3/3 SC, 8/8 tarefas) · `specs/104-session-identity/converge.md` (**Converged**) · assessoria do ciclo · `docs/governance/2026-11-21-104-session-identity.md` · gate do alvo PASS.

## Rollback

Reverter os três arquivos frontend (`src/lib/utils/jwt.ts`, `tests/unit/jwt.test.ts`, `src/routes/(app)/+layout.svelte`) em **novo commit de reversão**, sem force-push. Nenhuma migration, variável ou dado persistente a desfazer.

## [NV]

- Staging real, CI remota, branch protection, PR/merge e deploy: **[NV]** até verificação ao vivo.
