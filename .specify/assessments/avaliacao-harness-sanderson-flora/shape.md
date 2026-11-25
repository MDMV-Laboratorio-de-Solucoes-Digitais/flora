# Shape — avaliação harness Sanderson aplicada ao Flora

**Data**: 2026-11-24 · **Slug**: `avaliacao-harness-sanderson-flora` · **Fase**: 04 shape

## Opções em confronto (e por que o Flora não é um harness)

1. **Reimplementar o protótipo do palestrante dentro do Flora** — rejeitado liminarmente. O Flora é uma API multi-tenant (Axum/PostgreSQL) com Svelte 5 no frontend; não há consumidor para um loop de agente no produto. `[EV]` locally: zero identificadores de SDK no runtime. Reintroduziria retries, rate limits, stalls e sandbox que SDKs já resolvem.
2. **Adotar capacidades como convenções de engenharia** — usar padrões do vídeo (resumo com cauda verbatim, journals append-only, worktrees isolados, revisão independente) dentro da governança existente. É a via preferida quando não há consumidor de produto; não altera gate, CI, ADR ou migracões.
3. **Comprar/adotar prontos (CodeRabbit/Greptile/Graphite/Copilot; omp/OpenCode/Claude Agent SDK)** — preferível sempre que a necessidade é ferramenta de desenvolvedor, não feature de produto. Custo público, sem manutenção de harness próprio.
4. **Introduzir protocolo/UI generativa no produto (AHP/A2UI/Vibe OS)** — somente se houver demanda comprovada. Conflita diretamente com o norte `debloat`/VPS barata e com o backlog P2 (agentes/IA/memória/rotinas adiados até Seed estável).

## Alternativas mapeadas (detalhe e fontes em `research.md`)

| Família | Alternativa preferida ao Flora | Efeito |
|---|---|---|
| Loops/tools (P01–P05) | omp / Claude Agent SDK / OpenAI Agents SDK; tools tipadas com allowlist | transparência didática permanece no vídeo; nenhuma dependência nova no produto |
| Subagentes/memória (P06/P07/P11/P29) | omp subagents com paths disjuntos; `STATE.md`/journals append-only como memória `evidence-only`; graders determinísticos | paralelismo com custo controlado e sem autoridade de merge |
| Protocolos (P17–P20) | AHP quando o problema é ambiente compartilhado; MCP/ACP/A2A conforme o limite | evita protocolo novo; distingue sessão de agente de ambiente |
| Revisão (P13/P14/P31/P32) | CodeRabbit/Greptile/Graphite/Copilot ou revisor agêntico local; diagramas via codegraph/Mermaid | sem app próprio de revisão, sem ELK dedicado |
| UI generativa (P21–P26) | A2UI com catálogo fechado sobre dados reais; nunca HTML arbitrário | painéis tipados, acessíveis e versionáveis |
| Segurança/resiliência (P03/P27/P28) | nenhum shell no produto; E2B/Daytona+gVisor/Kata se isolamento for necessário; checkpointers maduros | mantém VPS e tenancy intactos |

## Trade-off central

O vídeo otimiza **autonomia e adaptabilidade individual**; o Flora otimiza **previsibilidade, tenancy e operação em VPS barata**. Onde os objetivos colidem (shell irrestrito, auto-modificação de paths protegidos, persistência sem esquema, catálogo infinito de apps), a decisão é `REJECT`/`AVOID` no produto e, no máximo, `ADAPT` em ferramenta local efêmera. Onde os objetivos convergem (resumos auditáveis, paralelismo com isolamento, revisão independente, fences, briefings de compreensão), a decisão é `ADOPT`/`ADAPT`.

## Restrições que moldam a decisão

- Constituição v1.4.0 e ADR-0003: gate `check_gate_integrity.py` e required checks são autoridade; merge é decisão de máquina sob gates verdes, mas promoção `dev→main`/deploy permanece humana.
- Assessment anterior (188 decisões): MCP read-only, `rmcp` pinned, memória opaca, Briefings de Compreensão, gate determinístico insubstituível; mudanças precisam de ADR, não deste artefato.
- `docs/research/2026-11-16-036-authorization-matrix.md:23`: auto-modificação de gates/harness/políticas sem PR versionado + aprovação humana é proibida.
- Fora de escopo: serviços pagos obrigatórios, dependências proprietary no Seed, credenciais, deploy, tag/release, alteração de proteção de branch.

## Como o veredito será expresso

`decision.md` trará 34 linhas `Pnn — VEREDITO — justificativa — alternativa`. Vereditos mistos são explícitos (`DEFER no Seed / ADAPT em ferramenta local`). O `decision brief` em `docs/research/` registrará o que muda, o que não muda, as fontes com datas e a cobertura idiomática real. Nenhuma linha autoriza implementação sem ciclo SDD próprio.
