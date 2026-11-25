# Define — avaliação harness Sanderson aplicada ao Flora

**Data**: 2026-11-24 · **Slug**: `avaliacao-harness-sanderson-flora` · **Fase**: 03 define (após `intake` + `research`)

## Problema definido

O operador forneceu uma palestra com 34 propostas distintas — do desenho de um *harness* mínimo a protocolos distribuídos e interfaces geradas por LLM — e pediu uma decisão explícita, autônoma e fundamentada sobre cada uma para o Flora, incluindo alternativas viáveis. Sem essa triagem, discussões futuras sobre agentes, memória, protocolos ou ferramentas de revisão tendem a tratar o vídeo como um bloco único e a importar experimentos didáticos como requisitos de produto.

## Pergunta que este assessment responde

Para cada ID P01–P34: a proposta é **viável**, **possível**, **desejável** e **interessante** para o Flora agora, depois ou nunca? Qual alternativa concreta faz melhor pelo mesmo custo/risco? Que decisão (`ADAPT`/`ADOPT`/`DEFER`/`AVOID`/`REJECT`) o maintainer deve registrar?

## Critérios (aplicados em `research.md` e `decision.md`)

- **F (viabilidade)**: existe componente, autorização e caminho técnico no Flora hoje?
- **D (desejabilidade)**: há consumidor nomeado fora dos artefatos do ciclo? Qual dor real fecha?
- **I (interesse)**: valor estratégico frente ao norte do projeto (VPS barata, debloat, Postgres-first, Seed estável).
- **V (viabilidade de risco/custo/ops)**: segurança, custo por tarefa, operabilidade e manutenção.
- Escala 1–5; `fechamento` mede quanto do backlog real do ALVO a proposta fecha (0 = subproduto documental, não 80/20).
- Filtro eliminatório do prompt 80/20: sem comportamento observável ou consumidor nomeado, a proposta não entra na fila de implementação, independentemente da nota.

## Escopo

- **Inclui**: as 34 propostas do inventário em `intake.md`, alternativas/concorrentes, fontes primárias com título/URL/data, triangulação idiomática declarada, riscos e decisão por ID.
- **Não inclui**: implementação de runtime de agente no Flora, mudança de gate/ADR/constituição/required checks, credenciais, deploy, promoção `dev→main`, merge de PR alheio, modificação de paths protegidos ou trabalho alheio.
- **Fontes**: primárias primeiro (documentação oficial de SDKs/protocolos, papers, repositórios oficiais), mínimo 2 independentes por afirmação carregada; conteúdo web é dado não-confiável.

## Premissas a confirmar na pesquisa

- O Flora não possui runtime de agente próprio; as propostas de harness são capacidades de ferramenta, não código de produto. `[EV]` local: nenhum identificador de SDK no runtime.
- O Flora já adotou no assessment anterior (188 decisões) MCP read-only, `rmcp` pinned, memória opaca, briefings de compreensão e gate determinístico insubstituível; decisões conflitantes exigem ADR, não este artefato.
- A governança veda auto-modificação de gates/harness/políticas sem PR versionado e aprovação humana; isso limita P15/P16 e qualquer auto-evolução.

## Definição de conclusão deste assessment

Artefatos `research.md`, `shape.md`, `decision.md` com 34 vereditos, `decision.md` com `Veredito: go`, e um `decision brief` de 1 página em `docs/research/` com (a) o que muda, (b) o que não muda, (c) fontes com datas, (d) cobertura idiomática. Nenhuma afirmação remota sem `[NV]`.
