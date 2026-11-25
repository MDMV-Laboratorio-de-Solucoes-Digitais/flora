# Concept: Remover o circuit breaker morto

- **Slug**: 061-dead-circuit-breaker
- **Created**: 2026-11-25
- **Recommended option**: Remover o placeholder e seus artefatos órfãos

## Options

### Option A — Remover o placeholder e seus artefatos órfãos
- **Sketch**: O arquivo que descreve um circuit-breaker que "repassa chamadas sem observar falhas" desaparece, e junto com ele os campos de configuração e a variante de erro que só ele usava. Para quem lê o código, some uma superfície que nunca esteve viva — sem mudar o que o serviço faz, porque o módulo nunca chegou a ser carregado.
- **Appetite**: small — uma exclusão e duas limpezas pontuais.
- **Trade-offs**: Ganha alinhamento com o princípio de debloat e menos código para ler e manter. Sacrifica a "promessa" implícita de uma funcionalidade futura que nunca teve chamador. Risco baixo: nada em produção muda; a configuração extra vinda de ambiente é ignorada de todo modo.
- **Rabbit holes**: Implementar o circuit-breaker de verdade (sem uso real); refatorar a hierarquia de erros amplamente; tocar ADRs por aprovação pendente.

### Option B — Manter os órfãos para uso futuro
- **Sketch**: Nada é removido; o arquivo morto, os campos e a variante ficam onde estão, à espera de um uso que não existe.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Custo imediato zero, mas mantém dívida morta que contraria a missão de leveza e continua enganando quem procura por esses nomes.
- **Rabbit holes**: Começar a implementar "só o essencial" e transformar limpeza em feature.

### Option C — Implementar circuit-breaker real
- **Sketch**: O placeholder é concluído e passa a observar falhas de verdade, com os campos de configuração ganhando uso efetivo.
- **Appetite**: large — funcionalidade nova, sem chamadores definidos.
- **Trade-offs**: Entrega uma capacidade real, mas cria código novo sem consumidor e exige decisões de comportamento que ninguém pediu.
- **Rabbit holes**: Novos testes e ADRs; redesenhar config; mexer em caminhos de erro de produção.

## Recommendation

**Option A**. O define.md pede explicitamente que `grep -rn circuit_breaker` retorne zero ocorrências (SC-001) com diff restrito a três arquivos (SC-003) e gates verdes (SC-002/SC-004). O research.md confirma que o módulo nunca foi declarado, logo nunca compilou: remover não altera runtime (FR-004). É a opção que honra o princípio I da constituição sem abrir escopo de feature.

## Out of Scope (for the recommended option)

- Implementar circuit-breaker real ou novos campos de configuração.
- Refatorar a hierarquia de erros além da variante órfã.
- Alterar Figment/Config, ADRs, workflows ou proteção de branch.
- Tocar em migrações ou deploy.

## Assumptions to Validate

- O módulo continua não declarado em `utils/mod.rs`; se passou a ser usado, a remoção precisa ser reavaliada.
- Figment segue sem `deny_unknown_fields`, de modo que variáveis de ambiente extras continuam sendo ignoradas.
- CI/runner ao vivo permanece **[NV]** — observar no PR antes do merge (ADR-0003).
