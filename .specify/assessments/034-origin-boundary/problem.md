# Problem Definition: Fronteira de origem do WebSocket no Seed

- **Slug**: 034-origin-boundary
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O critério #2 de reentrada de ADR-0002 permanece em aberto: o canal WebSocket do Seed não verifica a origem da conexão, o frontend não resolve URLs de API/WS a partir de configuração de runtime e não existe prova ao vivo não-browser da fronteira (intake.md; research.md:5-7). A pesquisa em sete linguagens mostra que o comportamento esperado é permitir `Origin` ausente e negar `null` ou origens não listadas com 403 antes de autenticação e acesso ao banco, usando correspondência exata de bytes contra a mesma allowlist da aplicação; ressalta também que `Origin` é uma fronteira de origem de navegador, não autenticação, e que literais divergentes ou proxy que remove o cabeçalho invalidam a garantia (research.md:7). A tentação de provar o critério contra o binário em execução na porta `:3000` esbarra em um fato: esse binário está desatualizado (`ACAO: *`, sem `/llms.txt`) e pertence a outro agente, portanto não pode ser tocado (research.md:9; decision.md, "Non-goals / non-claims"). O problema é fechar uma lacuna transversal de segurança de longa data com evidência determinística e honesta, sem reivindicar mais do que foi provado.

## Affected Users & Stakeholders

- **Users**: membro que utiliza o canal WebSocket pelo navegador; operador que publica o Seed atrás de proxy reverso e precisa que a origem seja preservada.
- **Stakeholders**: mantenedores e responsáveis pela segurança — precisam encerrar o critério de reentrada com prova; revisor do gate de ADR-0002 — precisa de evidência rastreável e de registro honesto do que ainda é NV.

## Goals

- Estabelecer uma fronteira de origem para o WebSocket que negue origens `null` e não listadas antes de qualquer checagem de autenticação ou acesso a banco, preservando o comportamento para conexões sem `Origin`.
- Permitir que o frontend resolva endpoints de API e WS a partir de configuração de runtime, sem cópia de literais no código.
- Dispor de testes puros e determinísticos para a regra de origem e de prova ao vivo isolada, sem interferir em processos ou dados de terceiros.
- Registrar no gate da ADR o que foi evidenciado e o que permanece fora de alcance.

## Non-Goals

- Não afirmar que o critério #2 está atendido antes de todas as verificações determinísticas, da prova ao vivo, da lane de regressão em navegador e da nota na ADR estarem verdes (decision.md).
- Não implementar Documentos FASE 7, não limpar affordances do frontend, não promover cargo-vet, não criar tag/release e não executar load/restore.
- Não tocar no processo em `:3000` nem no trabalho de `/metrics` de outro agente; não usar o binário desatualizado como evidência.
- Não afirmar que Tauri funciona em todos os sistemas operacionais, que proxy reverso preserva `Origin` ou que o vazamento por `?token=` foi resolvido — permanecem [NV].

## Success Metrics

- Predicado puro de origem cobre os casos de ausência, `null` e origem não listada com testes determinísticos (baseline: nenhuma checagem de origem no WebSocket).
- Testes oneshot sem banco cobrem o comportamento da conexão sob origens permitidas e negadas (baseline: nenhum teste DB-free).
- Provas ao vivo executam em instância isolada na porta 3100 com banco próprio, incluindo preflight, `select-organization` com Bearer e sondas de `Origin` (baseline: binário desatualizado em `:3000`, nenhuma prova).
- Nota na ADR registra evidência e limites, sem reivindicar o critério além do provado (baseline: critério em aberto).

## Cost of Inaction

A lacuna de origem no WebSocket persiste como uma não-verificação transversal antiga, permitindo que uma conexão seja aceita sem que sua origem tenha sido examinada; ao mesmo tempo, a ausência de prova impede encerrar o critério de reentrada e leva a afirmações de conformidade mais amplas do que o evidenciado. O custo é exposição de segurança e credibilidade do gate; efeito em produção não foi medido nesta sessão [NV].

## Open Questions

- [NEEDS CLARIFICATION: quais literais de origem são necessários para Tauri em cada sistema operacional, hoje não provados?]
- [NEEDS CLARIFICATION: o proxy reverso da instalação encaminha o cabeçalho `Origin` ao backend?]
- [NEEDS CLARIFICATION: como tratar o vazamento de token por `?token=` registrado como NV?]
- [NEEDS CLARIFICATION: quem executa e valida a lane de regressão em navegador necessária para considerar o critério fechado?]
- [NEEDS CLARIFICATION: o que acontece com a configuração de runtime após a inicialização, dado que ela é lida apenas no startup?]
