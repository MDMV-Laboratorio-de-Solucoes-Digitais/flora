# Concept: Logger local sem transporte fantasma

- **Slug**: 049-local-logger
- **Created**: 2026-11-25
- **Recommended option**: Coletor local com teto e diagnóstico

## Options

### Option A — Coletor local com teto e diagnóstico
- **Sketch**: Transformar o logger em um coletor apenas local: ele continua oferecendo os mesmos níveis de registro para quem o usa, para de tentar enviar dados para um destino que não existe, guarda um número limitado de entradas e expõe uma cópia para diagnóstico local. O contrato documentado passa a descrever esse comportamento.
- **Appetite**: small (incerteza: baixa — API pública preservada e nenhuma superfície HTTP nova).
- **Trade-offs**: Elimina os 404 silenciosos sem introduzir `console.log` e sem exigir mudança nos pontos de uso. Em troca, abre mão de telemetria centralizada — que já não existia neste checkout — e mantém o diagnóstico restrito ao processo local.
- **Rabbit holes**: Criar rota/serviço backend ou exporter para "aproveitar" o envio; usar armazenamento persistente e deixar o buffer crescer sem limite; engolir o 404 e manter o drift documental.

### Option B — Só parar de enviar (menor coisa que poderia funcionar)
- **Sketch**: Remover o envio e o agendamento de envio, mantendo o registro em memória da forma mais simples, sem teto explícito nem cópia de diagnóstico.
- **Appetite**: small (incerteza: baixa).
- **Trade-offs**: Acaba com o 404 com a menor mudança possível, mas sem teto o buffer pode crescer sem limite e sem a cópia de diagnóstico o contrato segue desatualizado.
- **Rabbit holes**: Alterar a assinatura pública e obrigar a mudança nos pontos de uso; reintroduzir `console.log` para "ver" os registros.

### Option C — Comprar / implementar ingestão no backend
- **Sketch**: Criar o endpoint de ingestão que o logger promete, com transporte e algum destino real para as entradas.
- **Appetite**: large (incerteza: alta — não há coletor definido neste checkout).
- **Trade-offs**: Tornaria a telemetria real, mas exige uma superfície de ingestão não autenticada e nenhum coletor existe para receber os dados; os artefatos registraram essa alternativa como descartada.
- **Rabbit holes**: Nova superfície HTTP sem autenticação; dependências e exporter fora do escopo; mexer em constituição ou ADRs.

## Recommendation

Recomendo a **Option A**, coerente com a decisão registrada. Ela responde ao problema em `define.md` — fim do 404 silencioso e do drift no contrato — preservando a API pública e a regra que proíbe `console.log`, com um teste unitário que prova a ausência de requisições e o limite do buffer.

## Out of Scope (for the recommended option)

- Rota ou serviço backend; exporter OTel; dependências novas.
- Introduzir `console.log` ou enfraquecer a regra no-console.
- Alterar os pontos de uso existentes, constituição, ADRs ou PR #262.
- Armazenamento persistente das entradas.

## Assumptions to Validate

- Não resta nenhuma chamada ao endpoint de telemetria no frontend.
- O teste unitário no projeto `server` prova a ausência de requisição de rede e o limite do buffer.
- Os gates frontend (check/lint/test/build) e Rust permanecem verdes.
- O comportamento real do envio em produção e a existência de coletor externo: [NV].
- Execução em browser real e CI ao vivo: [NV].
