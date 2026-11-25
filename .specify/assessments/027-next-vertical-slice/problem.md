# Problem Definition: Base de código anuncia componentes que não executam

- **Slug**: 027-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

A base ainda carrega, como parte do Seed, uma crate de transporte de notificações sem consumo: `flora-notifications` não tem chamadores em Rust e os campos de `NotificationsConfig` são validados, mas nunca lidos (`research.md:3`). O caminho XADD, caso fosse ativado, colidiria com o índice de dedup do ciclo 022 (`research.md:3`). O perigo não é apenas código morto: é a aparência de que PGMQ/Valkey são mecanismos de entrega suportados, quando a entrega real é feita por notificações em Postgres.

A observabilidade está no mesmo padrão: a feature `trace` do `tower-http` está habilitada, mas `TraceLayer::new_for_http()` nunca é anexada ao roteador; um exporter OTLP dependeria de nove condições (receptor, versões alinhadas, desligamento, orçamento) e permanece fora de alcance (`research.md:4`).

A documentação de governança também descreve um estado superado: o ADR-0001 ainda classifica como NV itens já resolvidos, README está desatualizado, a versão da constituição em ROADMAP/OBJETIVO continua v1.2.0 e tarefas de `specs/001` marcam como concluído trabalho de stream Valkey que nunca foi conectado (`research.md:5`). Ao mesmo tempo, o gate `cargo-vet` é viável como controle de dependências novas, mas o store está ignorado e os imports exigem rede (`research.md:6`), e o e2e de browser depende de JWT com linha de sessão semeada — `jti` é a autoridade — estando o token antigo expirado (`research.md:7`). Não há ainda oráculo independente para advisories (`research.md:8`).

## Affected Users & Stakeholders

- **Users**: operadores que escolhem transporte de notificações com base na documentação; desenvolvedores que dependem de tracing por requisição para diagnosticar a API. [ASSUMPTION: perfis inferidos de `research.md:3-5`; sem entrevistas registradas.]
- **Stakeholders**: mantenedores do ADR-0001, README, ROADMAP/OBJETIVO e specs, equipe de CI e de observabilidade.

## Goals

- Fazer com que o código e os documentos anunciem apenas mecanismos que realmente executam no Seed (`research.md:3-5`).
- Preservar os caminhos que funcionam — serviço central de notificações, repositório Postgres com dedup e perfil `realtime` com Valkey — enquanto se remove o que nunca é usado (`research.md:3`).
- Disponibilizar rastreamento por requisição sem coletor ou serviço adicional (`research.md:4`).
- Reconciliar as afirmações de ADR, README, planejamento e specs com o estado verificado (`research.md:5`).

## Non-Goals

- Fora deste enquadramento: gate `cargo-vet` com store e imports, e2e de browser com JWT fresco, job de advisories OSV/Dependabot, SBOM de release, exporter OTLP, Tauri e validação de 2vCPU/4GB (`decision.md:15`).
- Também não fazem parte do problema novos serviços obrigatórios, mudanças no frontend e a reversão de trabalho paralelo (`intake.md:16-17`). A remoção do transporte não pode atingir `NotificationService`, a dedup em Postgres nem o perfil `realtime` (`research.md:3`).

## Success Metrics

- Nenhuma referência a `pgmq`, `XADD` ou `flora_notifications` permanece em arquivos `.rs`/`.toml` (baseline: crate e config presentes — `research.md:3`; `decision.md:12`).
- O roteador da API passa a anexar tracing por requisição (baseline: feature habilitada, sem anexação — `research.md:4`).
- ADR, README, ROADMAP/OBJETIVO e specs não contêm afirmações superadas (baseline: quatro frentes de desvio — `research.md:5`; `decision.md:11`).
- `Cargo.lock` difere apenas da remoção da crate (baseline: não há diff controlado — `decision.md:9`).

## Cost of Inaction

Documentação e código que anunciam mecanismos inexistentes induzem operadores a depender de entrega que não acontece e a dimensionar infraestrutura desnecessária. O caminho XADD inutilizado é uma armadilha latente contra a dedup já implementada. Sem tracing anexado, a API não oferece correlação mínima por requisição. A divergência entre ADR e specs faz com que itens resolvidos sejam reabertos continuamente, desperdiçando ciclos de governança.

## Open Questions

- [NEEDS CLARIFICATION: o perfil `realtime` com Valkey deve continuar documentado como parte do Seed ou explicitamente rotulado como opcional? `research.md:3` o preserva, sem definir rótulo.]
- [NEEDS CLARIFICATION: quem valida a redação final do ADR-0001 após a reconciliação? `research.md:5` aponta o desvio, sem indicar proprietário.]
- [NEEDS CLARIFICATION: o `TraceLayer` deve ser condicionado a configuração ou sempre anexado? `research.md:4` descreve a anexação, sem registrar política de ativação.]
- [NEEDS CLARIFICATION: como tratar tarefas de specs que marcam work não conectado como concluído? `research.md:5` registra o problema, sem definir convenção de correção.]
