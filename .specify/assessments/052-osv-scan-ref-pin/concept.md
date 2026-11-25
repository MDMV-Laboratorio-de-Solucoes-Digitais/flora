# Concept: Varredura OSV com ref que resolve

- **Slug**: 052-osv-scan-ref-pin
- **Created**: 2026-11-25
- **Recommended option**: Fixar no tag documentado

## Options

### Option A — Fixar no tag documentado
- **Sketch**: Apontar o workflow semanal de varredura para um tag existente e recomendado pela documentação oficial do action, mantendo todos os parâmetros atuais do job. O objetivo é apenas fazer o job voltar a produzir o sinal de vulnerabilidades de dependências.
- **Appetite**: small (incerteza: baixa — tag verificado em duas fontes independentes e inputs conferidos; execução real do job [NV]).
- **Trade-offs**: Restaura o sinal advisory com uma mudança de uma linha e nenhum efeito nos checks obrigatórios. Em troca, continua sendo um job não-obrigatório: o alinhamento com o tag é uma convenção do upstream, não uma garantia de execução observada.
- **Rabbit holes**: Promover a varredura a required check, mudando a governança e o conjunto de checks; adicionar tolerância a erro em um job que já é opcional; alterar agendamento, permissões ou dependências.

### Option B — Fixar por hash imutável
- **Sketch**: Substituir a referência por um hash completo do tag verificado, abrindo mão do nome legível.
- **Appetite**: small (incerteza: baixa quanto ao hash, alta quanto ao processo de atualização).
- **Trade-offs**: Mais rígido e imutável, mas foge da convenção documentada pelo action e cria dívida de atualização manual sem benefício para um job apenas advisory.
- **Rabbit holes**: Criar um processo de rotação manual de hash; confundir imutabilidade com segurança do supply chain.

### Option C — Não fazer / mudar a natureza do job
- **Sketch**: Manter a referência que não resolve, ou remover/promover o job em vez de corrigir o ref.
- **Appetite**: none para manter; large para promover (incerteza: alta — mudança de governança).
- **Trade-offs**: Manter não custa implementação, mas o sinal de vulnerabilidades continua sem ser produzido; remover perde o sinal; promover altera ADR e os checks obrigatórios, decisão fora deste ciclo.
- **Rabbit holes**: Tratar o job como ruído e eliminá-lo; mexer em `fail-on-vuln`, agendamento ou Dependabot para "compensar" a falha.

## Recommendation

Recomendo a **Option A**. Ela atende aos requisitos de `define.md` — ref existente e documentado, inputs inalterados, nenhum required check ou ADR alterado, diff de uma linha — e é a alternativa de menor risco entre as verificadas. A execução real do job no próximo agendamento ou via disparo manual segue como item a observar [NV].

## Out of Scope (for the recommended option)

- Promover a varredura a required check; alterar ADR-0003, constituição ou a integridade dos checks.
- Alterar `fail-on-vuln`, agendamento, permissões, inputs do job ou Dependabot.
- Outros workflows, RBAC/RLS, merge ou deploy.
- PRs, worktrees ou stashes alheios.

## Assumptions to Validate

- A referência configurada passa a ser o tag esperado.
- A verificação de integridade de gate continua aprovada, com os checks obrigatórios intactos.
- Os gates Rust e frontend permanecem verdes e o diff de código é de uma linha em um arquivo.
- Os inputs do job são compatíveis com os do tag escolhido.
- Execução real do job no GitHub Actions após o pin: [NV].
