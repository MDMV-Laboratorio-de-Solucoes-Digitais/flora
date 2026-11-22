# Research — avaliação da arquitetura multiagente aplicada ao Flora

**Data**: 2026-11-22 · **Slug**: `avaliacao-arquitetura-multiagente-flora`
**Fonte do plano**: PDF local `MDMV-Autonoma/docs/Arquitetura Multiagente Autônoma da MDMV.pdf`
**Factory (somente leitura)**: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`

## Eixos entregues (9)

| Eixo | Relatório | Decisões |
|---|---|---|
| Runtime durável | `docs/research/mdmv-multiagent/runtime-duravel-flora.md` | R1–R14 |
| Segurança/autorização | `docs/research/mdmv-multiagent/seguranca-agentes-flora.md` | S1–S19 |
| Interoperabilidade/controle | `docs/research/mdmv-multiagent/interoperabilidade-controle-flora.md` | I-01–I-18 |
| Infra/observabilidade/storage | `docs/research/mdmv-multiagent/infra-observabilidade-flora.md` | INF-01–INF-24 |
| Fábrica determinística | `docs/research/mdmv-multiagent/dark-factory-pipeline-flora.md` | F1–F12 |
| Gateway LLM/FinOps | `docs/research/mdmv-multiagent/gateway-llm-finops-flora.md` | G1–G14 |
| Canais/marketing | `docs/research/mdmv-multiagent/canais-marketing-flora.md` | C1–C24 |
| Compliance fiscal/jurídica | `docs/research/mdmv-multiagent/compliance-fiscal-flora.md` | C1–C13 (eixo próprio) |
| Frontend/acessibilidade | `docs/research/mdmv-multiagent/frontend-acessibilidade-flora.md` | FR-01–FR-14 |

**Total: 152 decisões** consolidadas em `decision.md`; justificativas e evidências (título+URL+data) em cada relatório.

## Eixos NÃO entregues — [NV]

- **Auditoria/imutabilidade e RLS runtime** — **[NV]**: a busca de fontes primárias externas foi limitada por rate-limit; o agente foi cancelado antes de escrever `auditoria-imutabilidade-flora.md`. Cobertura parcial existe nos eixos de segurança (S17) e infra (INF-20).
- **Alternativas de orquestração e premissas fracas do PDF** — **[NV]**: mesmo motivo; o agente foi cancelado. Parte das premissas foi corrigida de passagem nos eixos runtime/infra/canais/compliance, mas não há relatório dedicado.

Não há pretensão de cobertura integral do PDF: estes dois eixos permanecem em aberto e devem ser retomados quando a busca externa estiver disponível.

## Correções factuais ao plano (evidência primária nos relatórios)

1. **SQLx**: o Flora não usa macros `query!`, não tem `.sqlx/` nem gate `SQLX_OFFLINE` — checagem é runtime + contratos DB-gated (R4).
2. **Absurd SDK Rust**: `absurd-sdk` não existe no crates.io (404), repositório com 4 estrelas e único push em 2026-05-12 — rejeitado como dependência; motor upstream é viável (R7).
3. **Hardware Oracle**: Always Free é **2 OCPU/12 GB por tenancy** (não 4/24), com recuperação por ociosidade e sem SLA (INF-01/03/04).
4. **MCP**: especificação vigente de 2026-07-28; Tasks movidas para extensão; `rmcp` 3.11.0 (I-01/I-02/I-03).
5. **A2A**: Signed Agent Card é MAY (JWS RFC 7515 + payload RFC 8785), não obrigatório; `a2a-rs` em estágio inicial (I-06/I-07).
6. **CaMeL**: garantia de fluxo de controle, não autorização; não-goals explícitos (S8/S9).
7. **RFC 10070 (nov/2026)** atualiza RFC 7523: `aud` deve ser o issuer identifier do authorization server (S2).
8. **YouTube/Hashnode/GSC/Lei 15.325**: cotas, custos e tipificação no PDF estão desatualizados ou incorretos (C5/C6/C8/C16 do eixo canais).
9. **IBS/CBS e EU AI Act**: datas do PDF invertidas ou incompletas (C5/C10 do eixo compliance).
10. **oxlint/Biome**: não há plugin Svelte nativo estável; manter `eslint-plugin-svelte` + `svelte-check` (F8/FR-05).

## Cobertura idiomática real

- **pt-BR**: verificada ao vivo (PDF, constituição/ADRs do Flora, Oracle BR, Receita RS, Planalto/CONAR, WCAG pt-BR).
- **en**: verificada ao vivo (docs oficiais, RFCs, repositórios, papers, metadados de crates).
- **zh-Hans**: Secondary, via comentários em issue pública (RustFS), registrado no eixo infra.
- **es/fr/de/ja**: **[NV]** — não localizadas fontes primárias adicionais; alguns eixos marcaram busca sem resultado.


### Atualização — 2026-11-22T11:15Z (append-only)

Os dois eixos anteriormente **[NV]** foram retomados com leitura direta das fontes primárias (rate-limit contornado sem novas buscas):

- `docs/research/mdmv-multiagent/auditoria-imutabilidade-flora.md` — AUD-01…AUD-17 (RLS runtime, hash-chain, âncora WORM, PITR, papéis PG, retenção fiscal, gate advisory).
- `docs/research/mdmv-multiagent/alternativas-premissas-flora.md` — O1–O7 (orquestradores), P1–P6 (premissas financeiras), J1–J6 (premissas jurídicas/contradições).

**Total atualizado: 11 eixos, 188 decisões.** Os dois eixos não são mais **[NV]**; permanecem **[NV]** apenas os detalhes registrados nos próprios relatórios (RustFS WORM, restore drill, cotação, es/fr/de/zh/ja).

Correções adicionais ao PDF: compressão 99,77% depende de mês de 30 dias e o "piso US$ 1.000/dia" é burn de tokens, não custo de humano; US$ 68,69 irreproduzível (nota 8 `unknown_url`); X US$ 0,015 vs US$ 0,200 com URL = 13,3×; RLS está `ENABLE` com zero `CREATE POLICY` e a app conecta como `postgres`.

## Segurança da pesquisa

Conteúdo web foi tratado como dado não-confiável. Registradas tentativas de instrução incorporada em páginas de documentação (macro.com/docs/concepts/mentions sobre cadastro de agente; `SKILL.md` do Hashnode; avisos "ask AI" em docs) — nenhuma foi executada. Nenhuma credencial foi copiada ou registrada.
