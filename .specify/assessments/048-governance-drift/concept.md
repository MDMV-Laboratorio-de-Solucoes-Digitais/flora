# Concept: Governança alinhada ao que é executável

- **Slug**: 048-governance-drift
- **Created**: 2026-11-25
- **Recommended option**: Correção factual mínima com notas datadas

## Options

### Option A — Correção factual mínima com notas datadas
- **Sketch**: Ajustar o texto que orienta os agentes para descrever o workspace que realmente existe no checkout — os membros reais e a versão da toolchain efetivamente fixada — e anexar notas datadas de estado verificado aos documentos de governança, sem apagar o histórico. Nenhuma regra, permissão ou gate é alterado.
- **Appetite**: small (incerteza: baixa — apenas duas linhas e notas append-only, sem código executável).
- **Trade-offs**: Elimina a divergência na fonte que orienta backlog, verificação e decisões, com diff pequeno e revisável. Em troca, exige revisão deliberada justamente porque esse texto governa agentes; o risco é de governança, não operacional.
- **Rabbit holes**: Reescrever o arquivo inteiro e perder regras úteis; enfraquecer regras, permissões ou gates "de passagem"; transformar a nota datada em reescrita de histórico.

### Option B — Só corrigir o texto, sem notas (menor coisa que poderia funcionar)
- **Sketch**: Ajustar apenas as duas afirmações desatualizadas, deixando o registro de estado verificado para uma mudança posterior.
- **Appetite**: small (incerteza: baixa).
- **Trade-offs**: Ataca a causa raiz com a menor superfície possível, mas sem a nota datada o estado verificado não fica rastreável para quem lê os documentos de governança depois.
- **Rabbit holes**: Tratar a edição como puramente cosmética e pular a revisão deliberada; reintroduzir os nomes antigos em outra seção.

### Option C — Fazer o código corresponder ao texto
- **Sketch**: Em vez de corrigir o texto, reintroduzir os membros descritos e voltar a fixar a toolchain antiga para que tudo passe a coincidir com o documento.
- **Appetite**: large (incerteza: alta — mudaria arquitetura e dependências sem evidência).
- **Trade-offs**: Eliminaria a divergência, mas inverte a autoridade (o código é a fonte executável), reintroduz a versão associada a UB documentada e muda o workspace sem justificativa funcional.
- **Rabbit holes**: recriar crates órfãos; alterar dependências, workflows ou gates; perseguir "consistência" à custa do comportamento correto.

## Recommendation

Recomendo a **Option A**. Ela atende aos requisitos de `define.md` — lista real de membros, toolchain correta, correção factual sem enfraquecer regras e notas append-only — e respeita os delimitadores de `decision.md`. Mantém o diff revisável, que é o ponto sensível por se tratar do arquivo que orienta agentes.

## Out of Scope (for the recommended option)

- Constituição, ADRs, workflows, gates, código Rust e frontend.
- Reintroduzir `flora-tasks`/`flora-files` como crates.
- Alterar qualquer regra, permissão ou gate.
- PR #262, branches alheias e trabalho em voo da factory.
- Merge, auto-merge, tag, deploy ou force-push.

## Assumptions to Validate

- `grep` por `flora-tasks|flora-files|1.98.0` em `AGENTS.md` retorna zero ocorrências.
- O inventário por `cargo metadata` confere com a lista escrita.
- Os gates do alvo permanecem verdes.
- A edição passa por revisão deliberada, dado que `AGENTS.md` orienta agentes.
- Execução de CI e branch protection ao vivo: [NV].
