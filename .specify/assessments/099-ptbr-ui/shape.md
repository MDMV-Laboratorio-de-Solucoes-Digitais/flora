# Shape: 099 — interface 100% pt-BR

**Data:** 2026-11-21 · **Branch**: `feature/099-ptbr-ui@7183a33fc`

## Forma escolhida

Um dicionário único, tipado e achatado, com um helper de formatação e um
mapeador de erros do backend. Cada literal visível é substituído por uma chave;
nada de biblioteca de i18n, detecção de navegador ou seletor de idioma — o
produto tem um único idioma de interface (pt-BR) e os termos técnicos são
exceções explícitas.

1. **Dicionário (T001)**: `lib/i18n/pt-BR.ts` exporta `t` (`as const`) com
   chaves como `nav.channels`, `chat.send`, `error.invalidCredentials` e
   `TranslationKey`; `lib/i18n/index.ts` reexporta `t`/`TranslationKey`, expõe
   `format()` para `{id}`, `{query}`, `{name}` e `{title}` e concentra
   `backendErrorMessage()`.
2. **Documento (T002)**: `app.html:2` já traz `lang="pt-BR"`; a tarefa é só
   confirmar e não regressir o atributo (FR-002).
3. **Autenticação (T003)**: `(auth)/+page.svelte` troca seus literais por chaves
   sem alterar os textos (para não quebrar os selectors) e
   `(auth)/callback/+page.svelte:69` passa a exibir `Autenticando...`.
4. **Shell (T004)**: `(app)/+layout.svelte` troca menu, conta, ações e painel de
   IA por chaves, incluindo `title`/`aria-label`.
5. **Páginas (T005)**: `tasks`, `channels`, `dms`, `select-org`, `settings` e
   `demo/**` usam o dicionário para cabeçalhos, formulários, estados de
   carregamento/vazio e mensagens.
6. **Componentes (T006–T007)**: chat/thread, arquivos, organização, busca e
   workspace passam a usar `t`; os textos derivados em `kanban.svelte.ts`
   (`A fazer`, `Em andamento`, `Concluído`) e `utils/search.ts`
   (`Mensagem`, `Tarefa`, `Arquivo`) também entram no dicionário.
7. **Erros (T008)**: `backendErrorMessage` substitui os `switch` espalhados:
   código de erro primeiro (`validation` + `admin_password` tem mensagem
   própria), depois status HTTP (401/409/422/429) e por fim o fallback da
   operação.
8. **Testes e fecho (T009–T010)**: selectors pt-BR preservados, varredura de
   zero literais, exceções técnicas conferidas e gates/lint/testes verdes;
   rastreabilidade T001..T010 em decision/spec/plan/tasks.

## Riscos e mitigação

- **Quebra de selectors**: mitigada mantendo as mensagens do ciclo 096
  exatamente iguais (`Entrar`, `voce@exemplo.com`, `Mínimo de 15 caracteres`,
  `/e-mail ou senha incorretos/i`); o spec existente é a rede de segurança
  (FR-008/T009).
- **Tradução parcial**: mitigada por varredura final sobre todas as páginas e
  componentes, incluindo módulos `.ts` que geram texto visível (SC-001).
- **Sobradinha técnica indevida**: apenas a lista explícita
  (OIDC, OAuth, API, JWT, REST, AI, SSO, slug) mais nomes próprios, rotas,
  identificadores e status canônicos; `AI` vira `IA` quando é texto corrente
  (`Painel de insights de IA`, `Pergunte à IA...`) (SC-002).
- **Mudança de contrato**: nenhum código de erro, status HTTP ou campo do body
  é alterado; FR-007 apenas mapeia o que o backend já devolve.
- **Tipagem**: `as const` + `TranslationKey` fazem o compilador rejeitar chaves
  inexistentes; `format()` preserva os marcadores quando o valor não é passado.
- **Escopo**: nenhum toque em `validate/`, `release/`, `STATE.md`, `PROGRESSO*`,
  `analyze/` ou `converge/`; nenhuma alteração de gate/workflow/ADR.

## Entrega

Diff restrito a `apps/frontend/src/lib/i18n/**`, componentes, páginas, helpers
de domínio e ao spec de login, além dos artefatos do ciclo 099. Os oito FR e três
SC são verificáveis por inspeção e pelos testes locais; gates/lint/testes estão
registrados como PASS no handoff consolidado e não foram reexecutados neste
artefato. IdP externo, deploy remoto e produção permanecem **[NV]**.
