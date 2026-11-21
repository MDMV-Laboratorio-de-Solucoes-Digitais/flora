# Release: 099 — Interface 100% pt-BR via dicionário fixo

**Ciclo**: 099 · **Data**: 2026-11-21 · **Branch**: `feature/099-ptbr-ui` @ `7183a33fc` · **Baseline**: `7183a33fc`
**Tipo**: tradução integral da interface — dicionário fixo pt-BR (`src/lib/i18n/pt-BR.ts` + `index.ts`), preservação de `lang="pt-BR"` em `app.html`, cobertura de rotas `(auth)`/`(app)` e componentes de domínio, mapeamento de erros do backend e atualização de seletores de teste. **Nenhuma alteração de gate, required check, constituição, ADR, contratos de API, valores de status, schema, PR/merge/tag, `STATE.md` ou `PROGRESSO*`.** A renderização em produção permanece **[NV]** até o deploy final.

## Entregue

- `apps/frontend/src/lib/i18n/pt-BR.ts` centraliza todas as strings visíveis em um único objeto `t` achatado (chaves pontuadas), reexportado por `index.ts`. Nenhuma dependência de i18n, nenhum catálogo por idioma, nenhum seletor de idioma e nenhuma detecção de locale: uma única língua alvo, pt-BR (FR-001, SC-001, SC-002).
- `apps/frontend/src/app.html:2` permanece `<html lang="pt-BR">`, coerente com o texto visível (FR-002, SC-001).
- Rotas `(auth)` — login, primeira configuração/bootstrap e callback — passam a consumir `t.*`, inclusive estados de erro e carregamento; "Entrar", "E-mail", "Senha" e "Autenticando…" saem do dicionário, com `OIDC`/`SSO` preservados (FR-003, SC-001, SC-002).
- Layouts global e `(app)` e a navegação traduzidos: sessão ("Acesso revogado", "Sessão expirada", "Sair agora"), canais, DMs, tarefas, conta, configurações e painel `AI` (FR-004, SC-001, SC-002).
- Páginas `(app)` traduzidas: `channels`, `tasks`, `select-org`, `settings`, `dms` — títulos, placeholders, botões e mensagens de estado/vazio, sem alterar os valores canônicos `TODO`/`IN_PROGRESS`/`DONE` do contrato com a API (FR-005, SC-001).
- Componentes compartilhados de domínio traduzidos: `ChatLayout`, `SearchBox`, `FileUpload`, `FilePreview`, `ThreadReply`, `KanbanBoard`, `TaskFilter`, `OrganizationManager`, `WorkspaceCreate`, incluindo `aria-label`/`title` e singular/plural; termos técnicos da allowlist preservados (FR-006, SC-001, SC-002).
- Erros do backend mapeados no frontend: `oidc_error`, `invalid_credentials`, `rate_limited`, `conflict`, `validation`, com fallback genérico, cobrindo o `redirect_uri is not allowed` reportado pelo usuário. `flora-api/src/routes/auth.rs` e `flora-core/src/error.rs` intocados; endpoints, OIDC, sessões, `config.js` e CORS preservados (FR-007, SC-001, SC-002).
- Testes e seletores de texto atualizados nos `*.spec.ts` para as novas strings pt-BR (FR-008, SC-003).
- Fronteira verificada: gate do alvo PASS no handoff, nenhum toque em validate/release pré-existentes, `STATE.md`, `PROGRESSO*`, contratos de API, valores de status, OIDC/allowlist, `config.js` ou CORS; a renderização em produção segue **[NV]** até o deploy final (FR-008, SC-003; T001..T010).

## SemVer

**Proposta:** MINOR `0.7.1` → `0.8.0`. Motivo: entrega uma capacidade nova e observável em toda a interface — a localização integral pt-BR — com a introdução do módulo `src/lib/i18n/`, do mapeamento de erros do backend e da atualização dos seletores de teste; há mudança visível ao usuário em praticamente todas as telas e um novo ponto de manutenção das strings. Não é PATCH: não se limita a corrigir uma literal isolada — há novo módulo, nova cobertura de comportamento e alteração de seletores. Não é major: nenhuma API pública, contrato de dados, schema, migration, rota, valor de status (`TODO`/`IN_PROGRESS`/`DONE`), fluxo OIDC ou sessão existente é removido ou quebrado; `config.js`, CORS e os endpoints permanecem válidos. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T / bug)

- `FR-001` dicionário pt-BR único (`t` achatado em `src/lib/i18n/pt-BR.ts`, reexportado por `index.ts`), sem runtime de i18n (T001, T002, T010).
- `FR-002` `app.html:2` mantido como `<html lang="pt-BR">` (T003, T010).
- `FR-003` rotas `(auth)`: login, primeira configuração e callback em pt-BR (T004).
- `FR-004` layouts global e `(app)` e navegação em pt-BR, com `AI` preservado (T005).
- `FR-005` páginas `(app)` — channels, tasks, select-org, settings, dms — em pt-BR, com status da API intactos (T006).
- `FR-006` componentes compartilhados de domínio em pt-BR, allowlist de termos técnicos preservada (T007).
- `FR-007` mapeamento de `oidc_error`/`invalid_credentials`/`rate_limited`/`conflict`/`validation` para mensagens pt-BR com fallback, sem tocar no backend (T008).
- `FR-008` testes/selectors ajustados às novas strings; gate do alvo PASS e fronteira preservada (T009, T010).
- `T001` inventário/causa (≈140 strings em ≈15 arquivos, sem i18n) · `T002` dicionário · `T003` `lang` · `T004` `(auth)` · `T005` layouts/navegação · `T006` páginas `(app)` · `T007` componentes · `T008` erros backend · `T009` testes/selectors · `T010` fecho de lint/testes/gate/fronteira.
- **Bug**: nenhum ID de `.specify/bugs/` é acionado neste ciclo. O texto em inglês — inclusive a mensagem OIDC reportada pelo usuário — é tratado por FR-001..FR-008 / T001..T010 e registrado em `docs/governance/2026-11-21-099-ptbr-ui.md`; nenhum bug externo é aberto ou reclassificado por este release.

## Evidência

`.specify/validations/099-ptbr-ui/validate.md` (R4 **APROVADO**; gate do alvo `check_gate_integrity.py` **PASS** no handoff, `git diff --check` exit=0, `npm --prefix apps/frontend run check` 0 errors, `npm run lint` sem issues, vitest/Playwright verdes, varredura de literais limpa, `lang="pt-BR"` confirmado e **[NV]** para renderização em produção/CI/deploy até o deploy final) · `specs/099-ptbr-ui/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=8/8`, `fr=8/8`, `sc=3/3`, `tasks=10/10`) · `specs/099-ptbr-ui/converge.md` (**Converged**, sem novas tarefas) · `.specify/assessments/099-ptbr-ui/{intake,research,define,shape,decision}.md` · `docs/governance/2026-11-21-099-ptbr-ui.md` · baseline `7183a33fc`. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar a interface anterior ao ciclo 099 — **reverter o código e reapontar o app web**, sem force-push e sem reescrita de histórico:

1. reverter o código para o estado pré-099 em `apps/frontend/src/**`: remover o módulo `src/lib/i18n/`, devolver os literais anteriores nas rotas `(auth)`/`(app)` e nos componentes de domínio, restabelecer os seletores de texto originais em `*.spec.ts` e manter `app.html:2` como `lang="pt-BR"` (o atributo já era correto no baseline e **não** deve ser revertido para `en`). Se as alterações ainda estiverem não commitadas, descartar apenas os arquivos do ciclo 099; se commitadas, usar novo commit de reversão ou `git revert` — nunca `--force`. Os documentos do ciclo (governança, intake/research/define/shape/decision, spec/plan/tasks, analyze/converge, R4 e este release) são registros e podem permanecer como histórico;
2. reapontar o app web `Flora Web` (FQDN `https://flora.mdmv.qzz.io`, app Coolify `uvykanlrkyp5dzrkktaj5bup`) para a imagem/build anterior ao ciclo 099 e executar o redeploy; a API permanece em `https://flora-api.mdmv.qzz.io` (app Coolify `9q3wuuwkur2kl27py0si785n`) — este ciclo não alterou a API, logo ela **não** precisa de reapontamento. **AGUARDAR** o deploy terminar antes de declarar a reversão concluída. Com o código anterior, parte da interface volta a exibir texto em inglês e o erro OIDC volta à mensagem original — comportamento anterior esperado, não regressão do rollback;
3. **não reverter** gate, `check_gate_integrity.py`, required checks, constituição, ADR-0003, contratos de API, valores de status `TODO`/`IN_PROGRESS`/`DONE`, OIDC/allowlist, `config.js`, CORS, PRs, merge ou tags — nenhum deles foi alterado por este ciclo. Não há migration para desfazer. Nenhum segredo, credencial ou dado de produção deve ser exibido, reutilizado ou registrado no processo;
4. após o redeploy e a reversão, reexecutar `python3 scripts/ci/check_gate_integrity.py` (esperado permanecer **PASS**, `exit=0`) e conferir `/`, `/config.js` e `/health` → 200. O texto em inglês retornado é o estado conhecido que este ciclo veio corrigir.

## [NV]

- **Renderização em produção**: a conferência visual de 100% pt-BR em cada rota e o fluxo de erro OIDC traduzido não foram medidos nesta passagem; dependem do deploy e permanecem **[NV]** até o deploy final (SC-001, SC-002).
- **CI/PR/produção**: nenhum PR, merge, tag, aprovação, proteção de branch ou ambiente de produção verificado; gate PASS do handoff não substitui CI remota para promoção (SC-003).
- **Coolify/Forgejo/DNS**: publicação de imagem, reapontamento do app web, resultado do deploy e medição em navegador real permanecem **[NV]** até o deploy final.
- **Textos na origem backend**: a tradução dos códigos de erro é implementada no frontend; a tradução das mensagens em `flora-api`/`flora-core` está fora deste ciclo e permanece **[NV]** como decisão de escopo (FR-007).
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4, no analyze/converge e no handoff do main agent.
