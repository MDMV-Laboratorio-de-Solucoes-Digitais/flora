# Decision: Identidade real de autores no chat

- **Slug**: 116-chat-author-identity
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | REST e realtime serializam apenas `sender_id` (UUID) e o frontend renderiza `Usuário <prefixo>` com letra hexadecimal; usuários não reconhecem autores e leitores de tela recebem identidade sem sentido (intake.md:5-9; problem.md:9). |
| Evidence strength | strong | Linhas verificadas em `flora-api/src/routes/channels.rs`, `realtime.rs`/`websocket.rs` e `ChatLayout.svelte:124,130,161,166`; lacuna #3 do núcleo Seed priorizada no backlog em `STATE.md` (intake.md:7-9; decision.md anterior:9). |
| Value vs. inaction | strong | Próximo item autônomo de maior retorno (240/625): sem a mudança, a colaboração no fluxo `workspace → canal → mensagem` permanece degradada e a correção fica mais custosa após mais mensagens dependerem do formato (problem.md:38). |
| Feasibility / appetite | adequate | Appetite medium (semanas): a mudança atravessa API, eventos, contrato DB e cliente, exigindo resolução em lote e coordenação entre listagem, envio, edição e exclusão (concept.md:11-12). |
| Strategic fit | strong | Resolve a lacuna sem serviço novo, migration ou required check; depende do Briefing de Compreensão `tenant` (ciclo 109) e do lockstep de piso (concept.md:29; decision.md anterior:13-16). |
| Risk posture | adequate | Risco de enumeração cross-org e exposição de campos sensíveis se a resolução sair do canal autorizado; compatibilidade retroativa e campos aditivos mitigam, mas comportamento remoto permanece **[NV]** (problem.md:20; concept.md:12). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: a lacuna é verificada no código, o retorno é o próximo autônomo do núcleo Seed e a opção recomendada não cria serviço, migration ou required check. Preservam-se o cálculo e os fatos anteriores — score **240/625** (`impacto=4 × confiança=5 × (6−esforço=3)=3 × (6−risco=2)=4`), próximo item autônomo de maior retorno no núcleo Seed conforme backlog priorizado em `STATE.md` (decision.md anterior:9) — e as restrições: Briefing de Compreensão obrigatório (`tenant`, `.specify/templates/briefing-compreensao.md`, ciclo 109) comprovando impossibilidade de enumeração arbitrária de usuários fora dos canais da organização; lockstep obrigatório de `FLOORS["messaging"] = 5` em `check_gate_integrity.py` e `.github/workflows/rust.yml` (total `79`); passo `11b taskstoissues` após `analyze` e antes de `implement`; sem `.unwrap()`/`.expect()` em produção; SemVer `MINOR` (`0.8.0 → 0.9.0`) (decision.md anterior:11-16). Nenhuma contradição nos artefatos justifica mudança de veredito.

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; as perguntas sobre avatar e enumeração são validações conduzidas no handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Tornar a autoria de mensagens REST e realtime identificável por nome/inicial legível, sem enumeração arbitrária de usuários fora do canal/organização.
- **Chosen approach**: Opção A — `UserRepository::find_by_ids` em lote em `flora-core`, campos aditivos retrocompatíveis `sender_name` e `sender_avatar_url` em `MessageResponse` e `MessageEventPayload`, preenchidos após validação de pertencimento, com utilitário `chatAuthor.ts` em TDD no cliente.
- **In scope / out of scope**: Inclui envio/listagem/catch-up/edição/exclusão, `models.ts`, `mappers.ts`, `MessageStore.svelte.ts`, `ChatLayout.svelte`, Briefing `tenant`, lockstep e release MINOR; exclui diretório público de usuários, DMs reais, migrations/schema e novos required checks.
- **Success metrics**: `# general` mostra `Alice Admin` e `Bob Builder` em vez de `Usuário <id>`; 0 campos sensíveis em DTOs/eventos; piso `messaging: 5` (total `79`) em lockstep; nomes e iniciais anunciados por `aria-label`.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: qual canal/canais exatos servem como evidência final do E2E?]; [NEEDS CLARIFICATION: `sender_avatar_url` tem fonte autorizada ou segue `None` até política de avatar?]; [NEEDS CLARIFICATION: o briefing 109 cobre integralmente o risco de enumeração cross-org neste caso?]; [NEEDS CLARIFICATION: como medir ausência de campos sensíveis sem criar required check?].
