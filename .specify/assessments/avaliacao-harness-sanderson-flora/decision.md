# Decision — avaliação harness Sanderson aplicada ao Flora

**Data**: 2026-11-24 · **Slug**: `avaliacao-harness-sanderson-flora` · **Fase**: 05 decide

**Veredito: go**

Assessment documental, sem implementação. Base do ALVO: `github/dev@0ef79ec60`; oráculo: `python3 scripts/ci/check_gate_integrity.py`. F/D/I/V = 1–5 (viabilidade em Flora / benefício a consumidor nomeado / interesse estratégico / segurança-custo-ops), todos `[INF]` calibrados pela precedência (1) evidência local/governança do ALVO, (2) fonte primária oficial, (3) reportagem/opinião. `[EV]` = comando+saída dos relatórios 01–18 (nada reexecutado); `[NV]` = não verificado ao vivo. Fundamentação, fontes com título/URL/data, cobertura idiomática, correções factuais e injeções em `research.md`.

## Matriz final (P01–P34)

| ID | Proposta | Veredito | F | D | I | V | Justificativa (Flora) | Alternativa preferida | Risco / mitigação |
|---|---|---|---|---|---|---|---|---|---|
| P01 | minimal two-loop harness | DEFER | 2 | 2 | 3 | 4 | Sem loop de agente em `flora-core`/`flora-api`; omp já é o harness [EV]. Reimplementar traria retries/stalls que o SDK resolve. | SDK (Copilot/OpenAI/omp) + `health.rs` | Confundir demo com produção / manter fora do gate |
| P02 | file read/write determinísticos | ADAPT | 3 | 3 | 4 | 4 | `files.rs:81-89,127,247` já cobre upload/download [EV]; se agente surgir, handles validados + `tenant.rs`, nunca path livre. | Read/Edit/Write + allowlist do SDK | Path traversal/cross-org / handles e append-only |
| P03 | shell arbitrário sem sandbox | REJECT | 2 | 1 | 2 | 1 | `AGENTS.md` nega unsafe/panic e o gate é insubstituível; shell livre em API multi-tenant é RCE. | Ferramentas granulares; shell só em E2B/Daytona | Exfiltração com credenciais / nenhuma tool shell em produção |
| P04 | compactação por LLM | ADAPT | 3 | 3 | 4 | 4 | Só `context-budget-adoption-rationale.md:21-22` [EV]; vale como formato de checkpoint de `analyze.md`/`converge.md`, não runtime. | auto-compact do SDK + cauda verbatim | Perda de estado / resumo + referências append-only |
| P05 | compactação proativa | DEFER | 2 | 2 | 3 | 3 | Sem loop longo em Flora [EV], sem consumidor; a factory não implementa compactação em runtime. | Token budgeting + checkpoint por turno | Custo/beta de provider / gatilho por limite de token |
| P06 | subagentes paralelos | ADOPT | 4 | 4 | 3 | 3 | omp já faz fan-out file-disjoint; 3–5 subagentes somente leitura, artefatos em `/tmp`, nunca git compartilhado. | Agente único + `codegraph_explore` | ~15× tokens/escrita duplicada / teto + saída em arquivo |
| P07 | memória entre sessões em arquivo | ADAPT | 4 | 4 | 3 | 2 | `STATE.md`/`PROGRESSO.md` já são memória documental [EV]; reaproveitar journals append-only, evidence-only e repo-scoped. | RAG sobre `specs/`; tabela com ACL | Envenenamento persistente / diff, proveniência, sem realimentar gate |
| P08 | construir sobre SDK existente | ADAPT | 4 | 4 | 3 | 4 | Direção certa, mas alvo é ACP/MCP (já `LlmGatewayPort`+`rmcp` read-only [18]), não AHP; sem dependência nova. | SDK ACP/MCP pinned; não harness próprio | Dependência especulativa / porta de reentrada no gateway |
| P09 | rewrite Node/TS→Rust | AVOID | 2 | 1 | 2 | 1 | `find flora-core flora-api -name '*.rs'` = 489 [EV]: back-end já Rust; os 8.823 TS são bundle do browser. Sem alvo. | Perfilagem + SDK in-process (P08) | Importar ~US$120k/14,5 semanas sem problema / não portar |
| P10 | comunicação entre sessões | REJECT | 2 | 2 | 3 | 1 | Constituição v1.4.0 proíbe agente mesclar worktree alheio; admitir só status-only. "Milhares de agentes" é [NV]. | A2A/ACP arbitrado por servidor; status em `STATE.md` | Merge de trabalho alheio / zero autoridade de escrita |
| P11 | evals contínuos de incentivos | ADAPT | 4 | 5 | 4 | 4 | Falta eval de incentivos [11]; registrar assess em `specs/110-mutation-pilot/` com casos-ouro, custo e IC — observação, não 5º check. | CI determinístico + eval ad hoc | Goodhart/juiz viciado / holdout rotativo + graders determinísticos |
| P12 | ferramenta de uma hora | ADAPT | 5 | 3 | 3 | 4 | Audiência é múltipla e ADR-0003 torna merge decisão de máquina; protótipo descartável em `/tmp`, fora do gate. | Claude Code/Codex CLI + script local | Protótipo virar autoridade / expirar com a sessão |
| P13 | PR review com worktrees | DEFER | 4 | 3 | 3 | 3 | Outras sessões são donas de worktrees e arquivos sujos; comprar revisão (Copilot/CodeRabbit/Greptile) em vez de app local. | Copilot review + diagramas Archify | Worktree virar base de merge / nunca mesclar sem PR+checks |
| P14 | diagramas React + ELK | ADAPT | 5 | 4 | 3 | 5 | Frontend é Svelte 5/Vite sem react/elkjs [EV]; convenção instalada é Archify + codegraph; ELK é dual-license e +21–46% tempo de revisão. | Archify PR-pack + Mermaid (já presentes) | Bundle/licença e sobrecarga / SVG estático, uma view por camada |
| P15 | app edita o próprio código | AVOID | 4 | 1 | 3 | 1 | Constituição v1.4.0 proíbe auto-modificar gates/harness/políticas sem PR+aprovação; PR hostil reescreveria o gate. | Self-edit só em `/tmp` + PR revisado | RCE de prompt / paths protegidos intocáveis |
| P16 | evolução por hot reload | DEFER | 4 | 2 | 2 | 4 | HMR do Vite já existe em dev, mas não é versão/auditoria; o back-end do demo exigia restart manual. | Skill/checklist versionada via git | Drift de estado efêmero / recarregar na próxima sessão |
| P17 | protocolo AHP completo | DEFER | 2 | 1 | 2 | 2 | `grep AHP` em Flora = 0 [EV]; Flora já sincroniza domínio por WS próprio; expor FS/shell a browser fura RBAC/tenancy. | MCP escopo restrito + WS atual | SSRF/escalada / DEFER até consumidor real |
| P18 | endpoint AHP gerado da spec | AVOID | 2 | 1 | 2 | 2 | A spec gera JSON Schemas/tipos, não servidor; duplicaria `websocket_handler` sem consumidor. | Consumir com crates `ahp`/`ahp-ws` se surgir host externo | Churn 1.0→1.1→1.2-draft / não implementar |
| P19 | sessões multi-cliente | DEFER | 2 | 2 | 3 | 2 | Multi-usuário já via WS+Postgres [EV], mas a divergência VS Code/`copilotd` mostra que conectar≠sincronizar. | Estender `WsEnvelope` com eventos tipados | Interoperabilidade prometida / estado autoritativo no servidor |
| P20 | tunelar ferramentas locais | AVOID | 2 | 1 | 1 | 1 | Clientes Flora são browsers; o canal TCP é experimental, privado ao criador e cai na reconexão. | MCP sobre TLS / REST+WS | Pivot/SSRF na rede do host / nenhum túnel |
| P21 | Vibe OS zero-app-code | REJECT | 1 | 1 | 2 | 1 | Workspace multitenant com OIDC e `ensure_workspace_in_org`: "zero app code" = zero autorização executável. | A2UI/MCP Apps com host retendo autorização | Ação alucinada furar tenant / demo fica em `/tmp` |
| P22 | HTML por janela via LLM | AVOID | 3 | 3 | 4 | 2 | HTML cru com dado não confiável é XSS (OWASP); aceitar só JSON validado contra catálogo shadcn-svelte e strings de `pt-BR.ts`. | json-render/A2UI `createSurface` + zod | XSS/injeção e custo / nunca `innerHTML` |
| P23 | diffs mínimos event-driven | ADAPT | 4 | 4 | 3 | 5 | `WsEnvelope` tem fallback `{type:string; payload?:unknown}` [EV]; estreitar o transporte e publicar patches tipados em runes, sem DOM patcher caseiro. | Svelte `$derived` sobre dados do servidor | Desync/perda de foco (WCAG 2.4.11) / `surfaceId`+seq idempotente |
| P24 | catálogo infinito de apps | AVOID | 2 | 1 | 2 | 1 | Catálogo "infinito" é o oposto do catálogo travado; `specs/110` é a âncora real do Flora. | Enum finito de capacidades da API Flora | App fantasma sem backend / validação server-side + `tenant.rs` |
| P25 | load/save agnóstico | DEFER | 3 | 2 | 3 | 2 | PostgreSQL 18 + 10 migrations append-only são canônicos [EV]; Markdown é só fronteira de intercâmbio, não verdade. | `pg_dump`/JSON Schema versionado; SQLite só local | Corrupção silenciosa / checksum + round-trip |
| P26 | painéis sobre dados reais | ADOPT | 4 | 5 | 5 | 4 | Existe sidebar real em `components/files/*` [EV], sem personalização; definir painel em JSON via migration + store rune. | Widgets fixos (menos flexíveis) | Drag-only/i18n/AA / alternativa a drag + alvos ≥24px |
| P27 | sandbox/permissões | ADOPT | 4 | 5 | 4 | 4 | Factory já tem bwrap `--unshare-all/net`, `--ro-bind / /`, `REQUIRES_HUMAN` [EV]; patch obrigatório bwrap ≥0.12.0 (CVE-2026-87766). | gVisor/Kata com imagem imutável | Escape se bwrap antigo / versionar + fail-closed |
| P28 | resiliência em produção | ADOPT | 3 | 5 | 4 | 3 | `rate_limit.rs`, `flock -n` e fallback 429/5xx existem [EV], mas `budget_caps_manifest.py` é `evidence_only`, não enforcement. | Fila com cotas centralizadas | Cap não aplicado/last-wins / cotas executadas + CAS + watchdog |
| P29 | revisão multi-modelo | ADAPT | 4 | 4 | 4 | 3 | `code_review_engine.py` mantém `never_gates=True` [EV]; dois modelos com rubrica e dedupe, veredito apenas informativo. | Revisor único + gate determinístico | Rubber-stamp/ruído / gate continua insubstituível |
| P30 | isolamento por worktree | DEFER | 3 | 3 | 3 | 3 | Factory já tem `worktree_hygiene.py`/pool [EV] e omp isola por worktree — mas não é sandbox de segurança. | Graphite stacked PRs; revisão comprada | Head stale / refazer build; não tocar em worktrees alheios |
| P31 | checklist de revisão salva | ADOPT | 5 | 4 | 4 | 4 | `.omp/commands/speckit.checklist.md` e o contrato `pr-review-checklist.md` já dão o formato; versionar enxuta, não sidebar auto-editada. | CodeRabbit review instructions/Learnings | Checklist auto-editado omitir checks / gate verde + revisão humana |
| P32 | escala centenas de PRs/mês | DEFER | 4 | 4 | 3 | 2 | `health.rs::metrics` expõe só `flora_api_up` [EV]; 377→1.072 commits não são PRs nem valor. Não definir "400 PRs" como KPI. | Painel somente leitura: lead time, 1ª aprovação, revert, LOC/PR | Goodhart/revisão rasa / volume+estabilidade em conjunto |
| P33 | ferramentas próprias | ADAPT | 4 | 3 | 4 | 3 | Diferenciador real é o domínio (`HitlReceipt`/`metrics_payload`, LGPD/HITL) que Copilot/Cursor não conhecem; adaptar o ACI no harness existente. | Copilot/Claude Code + MCP + gates compartilhados | Ferramenta órfã/churn / dry-run + custo por resolução |
| P34 | OS alucinado como arquitetura | REJECT | 1 | 1 | 2 | 1 | `OBJETIVO.md`/`AGENTS.md` são non-goals; demo provocativa, nenhum fornecedor oferece esse "OS". | Manter A2UI como UI generativa delimitada | UI não determinística / REJECT como arquitetura |

## Contagem

ADOPT 5 (P06, P26, P27, P28, P31) · ADAPT 10 (P02, P04, P07, P08, P11, P12, P14, P23, P29, P33) · DEFER 9 (P01, P05, P13, P16, P17, P19, P25, P30, P32) · AVOID 6 (P09, P15, P18, P20, P22, P24) · REJECT 4 (P03, P10, P21, P34). Total 34. Vereditos utilizados: ADAPT, ADOPT, DEFER, AVOID, REJECT.

## Divergências resolvidas

Precedência aplicada: (1) evidência local/governança do ALVO, (2) fonte primária oficial, (3) reportagem/opinião.

- P01: 14 ADOPT vs 02/11 DEFER → **DEFER**; sem loop em Flora e omp já é o harness (local).
- P04/P05: 14 ADOPT vs 02 ADAPT/DEFER e 11 DEFER → **ADAPT** (handoff) / **DEFER** (sem consumidor).
- P08: 14/11 ADOPT vs 04 ADAPT → **ADAPT**; já coberto por omp/`rmcp`, sem nova dependência (local).
- P10: 03 REJECT vs 14 DEFER vs 11 AVOID → **REJECT** irrestrito, ADOPT só status-only (constituição v1.4.0).
- P11: 03/14 ADOPT vs 15/11 ADAPT → **ADAPT**; eval é observação, não novo porteiro (local).
- P13: 06/11 DEFER vs 16 ADAPT(diagramas) → **DEFER**; worktrees alheios, diagramas via Archify.
- P15: 14 ADAPT vs 06/11 AVOID → **AVOID**; auto-modificação proibida sem PR+aprovação (local).
- P22: 11 REJECT vs 07 AVOID(cru)/ADOPT(JSON) → **AVOID** do HTML cru; JSON validado é a via aceita (OWASP).
- P23: 07 ADOPT vs 11 DEFER → **ADAPT**; transporte tipado e runes já existem (local).
- P26/P29: 07/14 ADOPT vs 11 ADAPT/DEFER → **ADOPT** (painel sobre dado real) / **ADAPT** (`never_gates`).
- P32: 11 ADOPT(cadência) vs 15 DEFER → **DEFER** como meta; medir como observação (DORA/METR + `health.rs`).
- 80/20: 11 propõe BUG-111 → **inválido**: já corrigido em `6288a8569`, ancestor de `github/dev` (local).

## Candidato 80/20

**Não há candidato válido que feche um gap real do backlog Flora.** (a) O único com gap nomeado veio de 11 (BUG-111-01/02/03), mas foi corrigido em `6288a8569`, ancestor de `github/dev` — descartado por autoridade local. (b) 04 e 05 concluem explicitamente "nenhum". (c) Os demais estão já cobertos (P02/P19/P27/P28/P32), são documentais/sem consumidor (P01/P05/P14/P17/P19/P30) ou exigem dependência/churn/auto-modificação proibida. (d) As lacunas reais restantes — DMs placeholder (`STATE.md:924,933`), zero-state de staging (`PROGRESSO.md:462`), remoção de `FLORA_BOOTSTRAP__*` até 29/11, E2E autenticado [NV], canário C6 RLS e mutação workspace-wide/CI (`specs/110/analyze.md:19`) — não são fechadas por nenhuma ideia P01–P34. **Quase-candidato (não contabilizado):** P11 como assess evidence-only em `specs/110-mutation-pilot/` (consumidor Maintainer; prova `python3 scripts/ci/check_gate_integrity.py` → PASS [EV] + dry-run do `eval_stealth_etapa1.py` **[NV]**) — falta item de backlog formal e prova ao vivo, e eval não pode virar 5º check.

## O que não muda

- Nenhum arquivo de `flora-core`/`flora-api`/`flora-tests`/frontend/migrations/gate/workflow/ADR/spec alterado; assessment é documental (precedente: 188 decisões, zero implementação).
- `python3 scripts/ci/check_gate_integrity.py` continua o oráculo único (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`); eval é observação, nunca porteiro.
- ADR-0003 (merge por máquina sob checks verdes) e constituição v1.4.0: proibida auto-modificação de gates/harness/políticas sem PR+aprovação.
- PR #410 (promoção `dev`→`main` @`0ef79ec`) não é mesclado; `origin` é espelho, `github` é canônico.
- Worktrees e arquivos sujos alheios intactos; nenhuma branch própria criada; nenhuma skill instalada; nenhuma credencial ou URL inventada.
- PostgreSQL 18 com migrations append-only, pt-BR canônico + runes e lints Rust estritos permanecem; MCP read-only/`rmcp` pinned, memória opaca e Briefings de Compreensão (precedentes) seguem vigentes.

## Autorizações não concedidas

- Nenhuma autorização para implementar runtime de agente, loop próprio, tools de arquivo/shell novas, endpoint AHP, renderizador de HTML, catálogo de apps ou persistência alternativa no Flora.
- Nenhuma autorização para alterar `scripts/ci/check_gate_integrity.py`, required checks, proteção de branch, ADR-0003, constituição v1.4.0, migrations, workflows ou paths protegidos; nenhum eval vira 5º check.
- Nenhuma autorização para mesclar PR #410 ou qualquer PR, promover `dev`→`main`, fazer deploy, force-push, tag/release, criar branch própria ou tocar worktrees/branches/arquivos sujos alheios.
- Nenhuma autorização para instalar skills, Dependências ou serviços pagos obrigatórios; nenhuma credencial, segredo ou URL inventada foi incluída.
- Nenhuma autorização para expor FS/shell ao browser, aceitar HTML cru/`innerHTML` com dado não confiável ou registrar veredito automático de revisão como gate.
- Escopo de escrita deste assessment: apenas os cinco artefatos do diretório do slug, o roteiro/evidência de validação, o `decision brief` em `docs/research/`, o espelho em `docs/governance/` e as três notas append-only (`PROGRESSO`, `STATE`, `index`). Nenhum path de código/produto/gate/workflow é alterado. Commit/push da branch de assessment segue ADR-0003 somente após gates locais e ACR; não autoriza merge de `main`, deploy ou tocar trabalho alheio.
- Os vereditos `ADAPT`/`ADOPT` são direção de projeto, não ordem de implementação: toda mudança exige ciclo SDD próprio, PR versionado, gates verdes e revisão humana quando aplicável.
