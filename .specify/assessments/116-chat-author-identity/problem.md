# Problem Definition: Identidade legível de autores no chat

- **Slug**: 116-chat-author-identity
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline)

## Problem Statement

No Seed do Flora, mensagens REST e eventos realtime expõem apenas `sender_id` (UUID), e o frontend renderiza `Usuário <prefixo do UUID>` com a primeira letra hexadecimal como avatar. Usuários finais não conseguem reconhecer quem escreveu em canais ou threads, e leitores de tela recebem uma identidade sem sentido; o problema importa agora porque o núcleo Seed já tem canais, mensagens e catch-up funcionando e a lacuna foi registrada como prioridade do MVP (`intake.md:5-11`; `decision.md:5-9`).

## Affected Users & Stakeholders

- **Users**: participantes de canais e threads que precisam distinguir autores por nome/inicial legível, inclusive com tecnologia assistiva (`intake.md:7-9`).
- **Stakeholders**: maintainer do Seed, que precisa fechar a lacuna #3 do núcleo sem quebrar tenancy nem criar um diretório público de usuários (`intake.md:13-15`; `decision.md:11-17`).

## Goals

- Tornar a autoria das mensagens identificável em REST, realtime, listagem e catch-up sem exigir que o cliente adivinhe a partir do UUID (`intake.md:9-11`).
- Preservar a compatibilidade retroativa dos contratos consumidos pelo frontend (`define.md:9`).
- Evitar exposição de atributos sensíveis ou enumeração arbitrária de usuários fora do canal/organização (`define.md:8`; `decision.md:11-13`).
- Manter a verificação determinística do contrato de mensagens alinhada ao gate (`decision.md:15`).

## Non-Goals

- Criar endpoint público de diretório de usuários ou DMs reais (`intake.md:13-15`).
- Alterar migrations SQL, schema ou required checks além do lockstep explicitamente registrado (`intake.md:15`; `decision.md:15`).
- Introduzir N+1 consultas ou carregar perfis completos no frontend (`define.md:7-8`).

## Success Metrics

- Mensagens em `/tasks` → `# general` mostram nomes reais (`Alice Admin`, `Bob Builder`) em vez de `Usuário <id>` (baseline: `Usuário {authorId.slice(0,8)}` — `intake.md:7-9`).
- Nenhum campo sensível (`password_hash`, `oidc_subject`, `profile`, `email` quando `display_name` existe) aparece em DTOs/eventos (baseline: não medido; `[NV]`).
- Contrato DB de mensagens cobre autoria em listagem e catch-up com piso atualizado (baseline: `messaging: 4`, total `79` — `decision.md:15`).
- Nomes e iniciais são anunciados por `aria-label` no frontend (baseline: não verificado; `[NV]`).

## Cost of Inaction

A experiência de chat do Seed continua com identificadores hexadecimal ilegíveis, o que degrada a colaboração básica e a acessibilidade justamente no fluxo núcleo `workspace → canal → mensagem`. A lacuna permanece no backlog e exige uma correção posterior mais custosa, após mais mensagens e eventos dependerem do formato atual (`intake.md:5-11`; `decision.md:7-9`).
## Open Questions

- [NEEDS CLARIFICATION: qual é a página/canal exata usada como evidência final do E2E com nomes reais?] (`intake.md:11`)
- [NEEDS CLARIFICATION: `sender_avatar_url` tem fonte autorizada no modelo de usuário atual ou deve seguir `None` até existir política de avatar?] (`define.md:9`)
- [NEEDS CLARIFICATION: o briefing de compreensão 109 já cobre integralmente o risco de enumeração cross-org para este caso?] (`decision.md:11-13`)
- [NEEDS CLARIFICATION: como medir a ausência de campos sensíveis nos DTOs sem transformar isso em novo required check?] (`decision.md:15-17`)
