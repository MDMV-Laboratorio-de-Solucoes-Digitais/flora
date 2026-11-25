# Problem Definition: Ciclo 80/20 sem baseline verificável

- **Slug**: 003-flora-80-20-cycle
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md, baseline.md

## Problem Statement

O ciclo 003 partiu de um pedido de decisão autônoma sobre o próximo incremento 80/20 da Flora (alternativa self-hosted e barata em VPS ao Macro, Rust + PostgreSQL e UI Svelte 5). O obstáculo não era escassez de código: as specs 001/002 e seus checklists/tasks marcavam ampla conclusão, mas o baseline de runtime não estava evidenciado neste ambiente, a T116 (OIDC PKCE) permanecia aberta e `apps/frontend` e `apps/web` coexistiam com caminho canônico e CI ambíguos. Intake e research registram drift entre README/specs/checklists e verificação real.

Ao mesmo tempo, a pergunta trazida ao ciclo — se REA (`morluto/rea`) e Craft-Anything (`storytold/craft-anything`) seriam viáveis/desejáveis para "reimplementar o Macro" — deslocava a atenção para uma via de alto risco (legal/ToS, contaminação de clean room, toolchain com permissões de host), quando o próprio Macro é AGPL-3.0 de código aberto e fonte documental autoritativa. O problema central é a falta de um critério verificável para priorizar o próximo passo e de um baseline confiável sobre o qual apoiar qualquer entrega — não a ausência de uma ferramenta de engenharia reversa.

## Affected Users & Stakeholders

- **Users**: mantenedores e contribuidores da Flora, que precisam saber o que realmente funciona antes de adicionar código; operadores de VPS de pequenas e médias empresas que dependem de um Seed confiável para adotar a alternativa.
- **Stakeholders**: operador do repositório que solicitou o ciclo e detém a autorização para ações fora do escopo autônomo; responsáveis pela governança SDD/dark-factory, que precisam de evidência determinística para aceitar gates; futuros usuários PME representados indiretamente pela direção do produto.

## Goals

- Estabelecer um critério explícito de priorização do próximo ciclo ancorado em evidência executável, e não em marcação de tarefas.
- Tornar visível o que do baseline (fmt/clippy/test/build e checks de frontend) é verificável no ambiente disponível e o que deve ser marcado como NV.
- Esclarecer o papel da T116 (PKCE) como lacuna de confiança e segurança a ser tratada antes de ampliar superfície.
- Reduzir a ambiguidade sobre caminho canônico de frontend/CI/documentação sem assumir verificações não executadas.
- Delimitar o risco associado a tratar ferramentas de engenharia reversa como via estratégica para compreender um projeto de código aberto.

## Non-Goals

- Não propor neste artefato a solução do ciclo (mudanças de código, API, modelo de dados, plano de tarefas ou veredito de implementação).
- Não perseguir paridade ampla com o Macro, reuso de código incompatível com AGPL ou engenharia reversa de alvo não autorizado.
- Não declarar builds, testes ou integrações como verdes sem execução observável; itens não executados permanecem NV.
- Não realizar deploys, mudanças de infra/produção, merges/releases ou manuseio de segredos.

## Success Metrics

- Comandos de verificação do baseline executados com evidência registrada (baseline: parcial/não consolidado — research.md relata apenas verificações locais pontuais e claim de convergência sem runtime evidenciado).
- Estado da T116/PKCE: aberta vs. concluída com evidência (baseline: aberta, conforme intake.md).
- Itens de checklist/spec marcados como concluídos sem evidência de execução de runtime (baseline: não quantificado; drift reconhecido em research.md).
- Caminho canônico de frontend entre `apps/frontend`/`apps/web` e alinhamento com CI: definido vs. ambíguo (baseline: ambíguo).

## Cost of Inaction

Sem baseline confiável, cada feature adicional é construída sobre suposições: regressões passam despercebidas, o tempo de iteração cresce e a documentação se afasta ainda mais do runtime. A lacuna de PKCE deixa o fluxo de autenticação sem uma garantia básica esperada em OIDC. Adotar REA/Craft-Anything como eixo estratégico acrescenta custo de toolchain e exposição legal sem ganhar acesso a informação que já está disponível no código aberto do Macro. O resultado é um Seed que "parece pronto" em artefatos, mas não pode ser defendido com evidência perante operadores de VPS.

## Open Questions

- [NEEDS CLARIFICATION: qual ambiente e autorização existem para executar verificações de CI remoto e integrações com serviços externos neste ciclo?]
- [NEEDS CLARIFICATION: o aceite da T116 (PKCE) inclui apenas o fluxo de sucesso ou também casos de falha/state/nonce com evidência?]
- [NEEDS CLARIFICATION: REA/Craft-Anything devem ser considerados apenas como referência tática para artefatos opacos autorizados, ou há interesse explícito em avaliação separada?]
