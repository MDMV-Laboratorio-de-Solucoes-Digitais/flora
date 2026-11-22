# Shape — avaliação da arquitetura multiagente aplicada ao Flora

**Data**: 2026-11-22 · **Slug**: `avaliacao-arquitetura-multiagente-flora`

## Forma escolhida

Assessment documental, sem mudança de código. Cada proposta do PDF que toca o Flora recebe um ID estável dentro do seu eixo e uma das cinco decisões:

- **ADOPT**: incorpore como restrição/artefato agora, sem novo serviço obrigatório.
- **ADAPT**: incorpore a ideia com forma própria do Flora (trait, perfil, gate ou documento), não copiando a implementação do plano.
- **DEFER**: registre porta de reentrada e pré-condição; não implemente agora.
- **AVOID**: alternativa compatível existe ou a opção viola norma/arquitetura.
- **REJECT**: premissa falsa, dependência sem manutenção ou obrigação já absorvida por outro padrão.

## Estrutura

1. `intake.md` — captura da ideia (sem julgamento).
2. `research.md` — eixos entregues, eixos **[NV]**, correções factuais, cobertura idiomática e segurança da pesquisa.
3. `define.md` — problema, critérios e fronteira.
4. `decision.md` — veredito e registro consolidado das 152 decisões, com ponteiro para os relatórios.
5. Relatórios `docs/research/mdmv-multiagent/*.md` — matriz completa com justificativa, evidência (título+URL+data) e `[NV]`.

## Riscos e mitigação

- **Rate-limit de busca**: dois eixos não foram entregues; registrados **[NV]**, não deduzidos.
- **Fontes de fornecedor**: benchmarks de fabricante tratados como direcionais, não como promessa.
- **Injeção em conteúdo web**: instruções incorporadas em páginas foram registradas e ignoradas.
- **Deriva normativa**: datas fiscais/IA são versionadas em documento, nunca em constantes de código.

## Entrega

A decisão é apenas documental. Implementações futuras devem nascer de ciclos SDD separados, com gate do alvo e PR para `dev/`.
