# Problem Definition: Gate de verificação do Seed não executável

- **Slug**: 026-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

A promessa de "Seed v0.1 verificado" do ADR-0001 é hoje sustentada principalmente por prosa. A auditoria registra que a CI não executa `fmt`, `clippy` restrito nem `cargo deny`, que os artefatos do ciclo 025 não estão finalizados e que a constituição ainda se contradiz com o ADR-0001 — embora as substituições centrais (PG FTS, sistema de arquivos local, notificações in-app e OIDC verificado) estejam verificadas em código e verdes (`research.md:3`).

O controle de dependências não fecha: `deny.toml` permite `AGPL-3.0`, mas as crates declaram `AGPL-3.0-or-later`; uma execução local retorna código 4 em licenças, enquanto `bans` e `sources` passam (`research.md:4`). O `cargo-vet` acumula 430 isenções `safe-to-deploy` e 12 `safe-to-run`, mas `supply-chain/` está ignorado pelo Git — logo não há gate em checkout limpo — e a auditoria completa (~8,7 milhões de linhas) é inviável (`research.md:5`). Há ainda uma armadilha de dependência: `opentelemetry-otlp 0.22` é declarada, mas não existe no crates.io `[NV: disponibilidade remota não verificada localmente]`, e as declarações fantasma não aparecem no `Cargo.lock` (`research.md:6`).

Por fim, a documentação normativa não coincide: o ADR, constituição, README, ROADMAP e AGENTS carregam afirmações divergentes sobre o que é Seed e sobre o que já foi resolvido (`research.md:3,7-8`). O problema é de verificabilidade e coerência, não de ausência das capacidades em si.

## Affected Users & Stakeholders

- **Users**: desenvolvedores que dependem da CI para saber se uma mudança quebra o contrato Seed; operadores que leem a documentação para decidir infraestrutura. [ASSUMPTION: perfis inferidos de `research.md:3` e `intake.md:4`; não há pesquisa com usuários registrada.]
- **Stakeholders**: mantenedores do ADR-0001 e da constituição, equipe de CI/supply-chain, responsáveis pelos artefatos do ciclo 025 e pelos documentos de governança.

## Goals

- Tornar o gate do ADR-0001 executável na CI, de modo que a afirmação "verificado" deixe de depender de interpretação manual (`research.md:3-4`).
- Eliminar a falha de licenciamento que impede a checagem de dependências de passar (`research.md:4`).
- Alinhar os documentos de autoridade ao estado real das capacidades e remover declarações de dependências que não existem (`research.md:3,6`).
- Finalizar a evidência do ciclo 025 pendente (`research.md:3`).

## Non-Goals

- Fora do enquadramento: store/imports/exemptions do `cargo-vet`, job de advisories com rede RustSec e SBOM (`decision.md:20`).
- Também não compõem o problema a aposentadoria do scaffolding PGMQ/Valkey, e2e de browser (que depende de JWT fresco), Tauri, exporter OTLP completo, validação de carga/restauração em 2vCPU/4GB e paridade com Meilisearch (`decision.md:21`). Novos serviços, mudanças no frontend e reversão de trabalho paralelo não são parte deste enquadramento (`intake.md:17-18`).

## Success Metrics

- `fmt`, `clippy` restrito e checagem de dependências executam na CI (baseline: nenhum dos três presente — `research.md:3`).
- A checagem de licenças termina sem erro (baseline: código 4 em licenças — `research.md:4`).
- Documentos de autoridade não contradizem o ADR-0001 (baseline: constituição em conflito — `research.md:3`).
- Nenhuma dependência declarada aponta para versão inexistente (baseline: `opentelemetry-otlp 0.22` declarada — `research.md:6`).
- Artefatos do ciclo 025 estão finalizados com resultado de PR real (baseline: não finalizados — `research.md:3`).

## Cost of Inaction

Sem gate executável, o rótulo "verificado" pode ser reivindicado por mudanças que não passaram por formatação, lint restrito ou checagem de licenças. A falha de licenciamento impede a checagem de passar e o store do `cargo-vet` fora de versão deixa o supply-chain sem evidência em checkout limpo. A dependência fantasma é uma armadilha para quem tentar habilitar observabilidade, e as contradições documentais forçam cada leitor a decidir qual fonte é normativa.

## Open Questions

- [NEEDS CLARIFICATION: o job de advisories do `cargo deny` deve ser informativo ou impeditivo? `research.md:4` o separa por depender de rede RustSec, sem definir política.]
- [NEEDS CLARIFICATION: qual versão da constituição deve ser considerada normativa após a emenda e como versions anteriores são referenciadas? `decision.md:12` registra a emenda, sem detalhar política de versão.]
- [NEEDS CLARIFICATION: a isenção `AGPL-3.0-or-later` deve ser registrada como allow-list global ou por crate? `research.md:4` descreve a correção de uma linha, sem critério de escopo.]
- [NEEDS CLARIFICATION: qual é o destino do `TraceLayer` e do OTLP após a remoção das declarações fantasma? `research.md:6` os trata como opcionais/futuros, sem decisão registrada.]
