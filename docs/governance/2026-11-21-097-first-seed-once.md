# Nota de governança — 2026-11-21 (ciclo 097)

**Assunto:** o link "É uma instalação nova? Fazer primeira configuração" ("Is this a fresh install? Start First Seed Journey") aparece incondicionalmente na tela de login, embora a garantia de bootstrap único exista somente no backend, dentro de `POST /api/v1/bootstrap`. Evidências locais, causa raiz, decisão de condicionar a exibição ao estado real da instalação, gates esperados e estado **[NV]** até deploy.

**Âncora:** branch `feature/097-first-seed-once` @ `696849934`; `.specify/feature.json` → `specs/097-first-seed-once`. Snapshot documental `2026-11-21T02:12:45Z`. Registro append-only; nenhuma linha histórica reescrita. Gate do alvo `python3 scripts/ci/check_gate_integrity.py` **PASS** no diagnóstico (reexecutar no checkout final).

## (a) Sintoma verificado

- **Local:** em `apps/frontend/src/routes/(auth)/+page.svelte:238-243`, o bloco do rodapé da tela de login exibe "É uma instalação nova?" e o botão "Fazer primeira configuração" dentro do branch `{:else}` do modo normal. Não há nenhuma consulta ao estado da instalação: o link é renderizado **sempre**, independentemente de o workspace já ter sido configurado.
- **Consequência:** em instalação já configurada, o usuário é levado ao fluxo de primeira configuração (`isSetup = true` via `toggleSetup()`, `:174-177`) e, ao submeter `handleBootstrap()` (`:123-172`), recebe **409** — mapeado para a mensagem "Este workspace já foi configurado." em `:140-143`. O frontend oferece uma ação que o backend necessariamente rejeita.
- **Tradução do relato:** "Is this a fresh install? Start First Seed Journey" corresponde ao par de strings pt-BR acima; o defeito não é de idioma, mas de exibição incondicional.

## (b) Causa raiz: estado de "instalação vazia" existe só no backend

| Evidência | Local | Leitura |
|---|---|---|
| Link incondicional na UI | `apps/frontend/src/routes/(auth)/+page.svelte:238-243`, branch `{:else}`; `toggleSetup()` em `:174-177` | A visibilidade do fluxo de primeira configuração não depende de nenhum estado consultado; a pergunta "É uma instalação nova?" é feita a todo visitante. |
| Guardião de unicidade no backend | `flora-api/src/routes/bootstrap.rs:28-29` (`const BOOTSTRAP_LOCK_KEY`), `:238-244` (`POST /bootstrap` documentado com 409 quando já inicializado); `advisory transaction lock` + contagem de organizações não-sistema na mesma transação | A garantia "uma vez por instalação" é correta e atômica no servidor — é a última linha de defesa. O problema é a UI não a conhecer antes de oferecer a ação. |
| Nenhum sinal exposto ao frontend | Não há endpoint/flag que informe "zero organizações não-sistema"; `create_bootstrap_router()` (`bootstrap.rs:57-60`) registra somente `POST /bootstrap` | Sem esse estado, o cliente não pode decidir se exibe ou oculta o link; daí o comportamento incondicional. |
| Especificação ainda não materializada | `specs/097-first-seed-once/spec.md` é template `Draft` (placeholders de User Stories/FRs) no snapshot | A intenção do ciclo 097 ainda não foi formalizada em requisitos; esta nota registra o diagnóstico e a decisão que a spec/implementação devem seguir. |

## (c) Decisão registrada

1. **Condicionar a exibição ao estado real:** o link "Fazer primeira configuração" só deve aparecer quando o backend indicar instalação vazia (zero organizações não-sistema). Em instalação configurada, a tela de login segue apenas com os fluxos de Entrar/SSO.
2. **Manter o guardião 409:** a checagem atômica de `POST /bootstrap` (advisory lock + contagem) permanece como defesa final — a correção de UI não a substitui nem a relaxa; chamadas diretas e concorrentes continuam fail-closed com 409.
3. **Sem alteração por esta nota.** Nenhuma linha de código, gate, workflow, spec, validation/release, ADR ou constituição foi alterada aqui; a implementação cabe ao ciclo 097 em sua spec e testes.

## (d) Estado **[NV]** e gates esperados

**[NV] até deploy final:** estado real da instalação em produção, comportamento do link (visível em instalação vazia, oculto em instalada), E2E autenticado via bootstrap → sessão → `/api/v1/tasks`, e CI/proteção/Coolify/Forgejo/aprovações ao vivo. Nenhum deploy foi executado por esta nota.

Gates e verificações esperados, nesta ordem, antes de declarar concluído:

1. `python3 scripts/ci/check_gate_integrity.py` → **PASS** (registrado no diagnóstico do ciclo; reexecutar no checkout final) e `git diff --check` limpo. Não continuar se vermelho.
2. Testes/vitest da tela de login cobrindo **ambos** os estados: instalação vazia → link visível; instalação configurada → link oculto; e o fluxo bootstrap → sessão → `/tasks` (inclusive o caso 409 preservado).
3. `npm --prefix apps/frontend run check` → 0 erros; suíte frontend verde.
4. Build Docker de `flora-web`/API e medição local: `/`, `/config.js` e `/health` → **200**; fluxo de primeira configuração em instalação vazia → sessão e `GET /api/v1/tasks` com token → **200**; em instalação já configurada, link ausente e bootstrap direto → **409**.
5. Após o deploy final, medir em navegador real os dois estados da tela de login e o fluxo autenticado. Registrar data/hora, commit e resultado em nova nota append-only.
6. **Rollback:** reapontar o app Coolify para a imagem/commit anterior, sem force-push; revalidar `/`, `/health`, o link e o guardião 409.

## Histórico e escopo

Documento novo e append-only. Nenhuma linha de `STATE.md`, `PROGRESSO.md`, ADRs, constituição, `specs/`, `.specify/validations/` ou `.specify/releases/` foi reescrita. Nenhuma credencial foi copiada ou exposta. Afirmações sobre estado remoto/Coolify/DNS/CI não remediadas nesta nota permanecem **[NV]** até reconsulta ao vivo e deploy final.
