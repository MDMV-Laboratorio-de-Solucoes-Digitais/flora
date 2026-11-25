# Decision brief — ferramentas auto-evolutivas (Sanderson, NDC Oslo 2026)

**Data**: 2026-11-24 · **Slug**: `avaliacao-harness-sanderson-flora` · **Base**: `github/dev@0ef79ec60`

## O que muda

Registro documental de **34 decisões** sobre o vídeo `yDLnn9OAHbE`: **ADOPT 5** (P06 subagentes limitados, P26 painéis sobre dados reais, P27 sandbox/permissões, P28 resiliência, P31 checklist salva), **ADAPT 10** (P02, P04, P07, P08, P11, P12, P14, P23, P29, P33), **DEFER 9** (P01, P05, P13, P16, P17, P19, P25, P30, P32), **AVOID 6** (P09, P15, P18, P20, P22, P24), **REJECT 4** (P03, P10, P21, P34). A regra prática: capacidades de harness viram convenção **fora do produto**; nada que exponha shell, auto-modifique gates, persista sem esquema ou gere HTML cru entra no Flora.

Correções factuais ao material do operador ficam registradas: "~400 PRs/mês" é inferência a partir de "over 200" [02:53–03:04]; as "500k linhas" são o último PR da migração Rust, não o total (fonte primária de Toub: 832.378 linhas de produção, 128 PRs, 14,5 semanas, ~US$120 mil); AHP **compõe** ACP (camada de coordenação, não substituto); a spec gera schemas/tipos, não um servidor; e o canal TCP AHP é experimental.

## O que não muda

Nenhum `flora-core`/`flora-api`/frontend/migration/gate/workflow/ADR/spec é alterado. `python3 scripts/ci/check_gate_integrity.py` segue o oráculo único; ADR-0003 e constituição v1.4.0 continuam vedando auto-modificação de gates/harness/políticas sem PR+aprovação. PR #410 (`dev→main`) permanece escalado ao SWE; nenhum merge de `main`, deploy, auto-merge, force-push, credencial ou instalação de skill. Worktrees e arquivos sujos alheios preservados.

## Backlog / próximo 80/20

**Não há candidato 80/20 elegante neste inventário.** O único gap com consumidor nomeado sugerido pela pesquisa (BUG-111-01/02/03) já foi corrigido em `6288a8569`, ancestral de `github/dev` `[EV]`. As demais propostas ou já estão cobertas (P02/P19/P27/P28/P32), ou são documentais/sem consumidor, ou exigem dependência/churn proibido pelo norte `debloat`/VPS barata. Lacunas restantes do projeto (DMs placeholder, zero-state de staging, expiração de `FLORA_BOOTSTRAP__*`, E2E autenticado, canário RLS, mutação workspace-wide) não são fechadas por P01–P34. Registro fiel: **DEFER/AVOID/REJECT não vira ciclo de implementação**; escalonamento ao SWE na ausência de candidato com `fechamento > 0`.

## Fontes (primárias; acesso em 2026-11-24 salvo indicação)

- Microsoft Copilot SDK — *The agent loop* — https://github.com/microsoft/copilot-sdk/blob/main/docs/features/agent-loop.md
- Anthropic — *Compaction overview* — https://platform.claude.com/docs/en/build-with-claude/compaction
- Anthropic — *How we built our multi-agent research system* — https://www.anthropic.com/research/multi-agent-research-system — 2025-06-13
- Anthropic — *Demystifying evals for AI agents* — https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents — 2026-01-09
- Microsoft — *AHP overview* — https://microsoft.github.io/agent-host-protocol/guide/what-is-ahp.html ; *AHP and ACP* — https://microsoft.github.io/agent-host-protocol/guide/ahp-and-acp.html
- GitHub — *Building the Agent Host Protocol* — https://github.blog/engineering/engineering-principles/building-the-agent-host-protocol-for-sessions-that-continue-across-devices/ — 2026-11-17
- S. Toub — *Migrating the GitHub Copilot runtime to Rust* — https://github.blog/ai-and-ml/generative-ai/migrating-the-github-copilot-runtime-to-rust-using-copilot/ — 2026-09-16
- A2UI — *Protocol v1.0* — https://a2ui.org/specification/v1.0-a2ui/ — 2025-11-20/2026-06-08
- OWASP — *LLM01:2025 Prompt Injection* — https://genai.owasp.org/llmrisk/llm01-prompt-injection/ — 2025-04-17
- METR — *Early-2025 AI on experienced developers* — https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/ — 2025-07-10

**Cobertura idiomática real**: vídeo em inglês (legendas automáticas VTT obtidas, 00:10–56:17); documentação oficial consultada em en + es-es/fr-fr/de-de/zh-cn/ja-jp; repos e síntese em pt-BR. Legendas pt-BR/es do vídeo não obtidas (HTTP 429) **[NV]**; spec AHP somente em inglês. Conteúdo externo tratado como dado; injeções em docs E2B, corpo do PR #410, CTA A2UI e site comunitário zh foram registradas e ignoradas.
