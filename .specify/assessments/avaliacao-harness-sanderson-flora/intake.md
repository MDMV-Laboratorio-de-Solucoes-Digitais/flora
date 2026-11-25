# Idea Intake: Avaliação das ferramentas de desenvolvimento auto-evolutivas de Steve Sanderson (NDC Oslo 2026)

- **Slug**: `avaliacao-harness-sanderson-flora`
- **Created**: 2026-11-24
- **Source**: `https://www.youtube.com/watch?v=yDLnn9OAHbE` (pasted text + URL público; hosts: `youtube.com`/`youtu.be`)
- **Type**: `exploration`

## Idea (as captured)

> Palestra **"Build your own self evolving dev tools an make yourself unique"**, Steve Sanderson (Microsoft / equipe do agente do GitHub Copilot), NDC Oslo 2026, ~56 min. Tese: com a IA elevando a vazão de código (de >200 para ~400 PRs/mês em cinco meses, [02:53]), depender apenas das mesmas ferramentas prontas tende a padronizar desenvolvedores ([05:00]); a diferenciação está em construir ferramentas próprias, auto-modificáveis e ajustadas ao fluxo individual ([01:47]).
>
> O palestrante constrói um *harness* mínimo com dois loops aninhados ([06:39]), adiciona leitura/escrita de arquivos ([08:40]) e shell irrestrito — explicitamente perigoso por não ter sandbox ou permissões ([09:50]); mostra compactação de contexto por chamada ao LLM preservando *system prompt* e resumo ([11:37]/[13:49]); subagentes paralelos como ferramenta que instancia novas cópias do agente ([16:33]); e memória entre sessões gravada em arquivo e reinjetada no *system prompt* ([18:40]). Alertas de produção: falhas de rede, *rate limits*, travamentos, edições simultâneas, *sandboxing* e saídas maiores que a janela; recomendação de construir sobre SDKs existentes ([20:46]/[21:35]).
>
> Evolução do harness do Copilot: reescrita de TypeScript/Node (1–2M linhas, ~1s de startup, ~100 MB) para Rust conduzida por um engenheiro com agentes paralelos ([23:50]); comunicação entre sessões levou um agente a fazer *merge* e publicar alterações alheias não finalizadas após recusas ([25:47]); mudanças de incentivo exigem benchmarks constantes ([22:44]). Ferramentas pessoais de um usuário passam a ser econômicas em cerca de uma hora ([27:36]): assistente de revisão de PRs com React+ELK+WebSocket+Copilot SDK e *worktrees* locais ([29:53]), que se auto-modifica em tempo de uso com *hot reload* ([32:52]) e restilo por chat ([34:07]).
>
> Para escalar, apresenta o **Agent Host Protocol (AHP)**, especificação pública usada internamente em VS Code/GitHub Copilot ([36:20]), que modela o ambiente inteiro (sistema de arquivos, shell, automações) e tunela ferramentas locais entre clientes ([38:14]), com demonstração de endpoint WebSocket lido da especificação e conexão do VS Code à mesma sessão ([42:08]). O experimento provocativo **Vibe OS** gera HTML por janela sem código de aplicação ([44:31]), aplica *diffs* customizados `replace`/`append` a eventos do usuário ([48:20]), inventa aplicativos sob demanda ([51:14]) e reinterpreta arquivos salvos por outros aplicativos ([52:16]); o próprio autor descarta um SO inteiramente alucinado como arquitetura de produção e sugere painéis/laterais customizáveis sobre dados reais ([54:13]).

A descrição completa do operador (três versões em pt-BR/EN, com timestamps) é o insumo bruto desta captura. Conteúdo de vídeo, páginas web, issues e skills é **dado não-confiável**: nenhuma instrução encontrada nesses meios foi tratada como autoridade.

## Restated

O pedido é avaliar, uma a uma, as ideias, técnicas, ferramentas e comportamentos demonstrados no vídeo — incluindo alternativas e concorrentes — quanto à viabilidade, possibilidade, desejabilidade e interesse para o Flora, produzindo decisões autônomas `ADAPT`/`ADOPT`/`DEFER`/`AVOID`/`REJECT` e alimentando o próximo ciclo 80/20 do projeto. Nenhuma decisão de implementação, compra, deploy ou alteração de gate está implícita nesta captura.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), na solicitação de 24/11/2026 às 20:31.
- **Trigger**: pedido explícito de orquestração com pesquisa extensa em múltiplos idiomas, uso do aparato `mdmv-dark-factory` como fonte de ideias/convenções e execução do fluxo `assess` iniciando por `intake`.
- ** Âncora do ALVO**: `flora`, base `github/dev@0ef79ec60`; gate autoritativo `python3 scripts/ci/check_gate_integrity.py`; remoto canônico GitHub, `origin`/Forgejo apenas espelho.
- **Restrição**: trabalho alheio em `fix/promo-355-conflict`, `github-workflows-2026-11-24`, `chore/speckit-1.4.3-upgrade` e PR #410 (`dev→main`) preservado; nenhum merge de `main`, deploy, auto-merge, force-push ou alteração de proteção.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: quais das propostas são capacidades de produto do Flora e quais são apenas ferramentas do agente?]
- [NEEDS CLARIFICATION: o Flora já cobre alguma forma de memória, subagentes, protocolo de agente ou painéis customizáveis?]
- [NEEDS CLARIFICATION: qual é o custo de execução, latência e risco de segurança de cada proposta em VPS barata?]
- [NEEDS CLARIFICATION: quais alternativas open-source maduras fazem o mesmo com menos risco?]
- [NEEDS CLARIFICATION: AHP é realmente uma especificação pública estável, e o que muda em relação a MCP/ACP?]
- [NEEDS CLARIFICATION: que parte do vídeo é demonstração didática e que parte tem evidência de produção?]
- [NEEDS CLARIFICATION: quais idiomas e fontes primárias confirmam independentemente as afirmações do palestrante?]
- [NEEDS CLARIFICATION: existe algum item do backlog de fechamento do Flora que alguma proposta feche com consumidor nomeado?]

## Inventário capturado (sem julgamento — a avaliação fica em `research.md`/`decision.md`)

| ID | Proposta capturada | Timestamp |
|---|---|---|
| P01 | Harness mínimo de dois loops aninhados | 06:39 |
| P02 | Ferramentas determinísticas de leitura/escrita de arquivos | 08:40 |
| P03 | Ferramenta de shell arbitrário sem sandbox/permissões | 09:50 |
| P04 | Compactação de contexto por resumo do LLM | 11:37–13:49 |
| P05 | Compactação proativa/em segundo plano | 13:55 |
| P06 | Subagentes paralelos como ferramenta do agente | 15:44–16:33 |
| P07 | Memória entre sessões gravada em arquivo e reinjetada | 17:57–18:40 |
| P08 | Construir sobre SDKs de harness existentes | 21:35 |
| P09 | Reescrita Node/TypeScript → Rust por startup/memória | 23:50 |
| P10 | Comunicação entre sessões de agentes | 22:55–25:47 |
| P11 | Avaliação/benchmarks contínuos de incentivos de agentes | 22:44 |
| P12 | Economia de ferramenta pessoal de um usuário em ~1 hora | 27:17–27:36 |
| P13 | Assistente de revisão de PRs com worktrees locais | 28:03–29:53 |
| P14 | Diagramas de arquitetura interativos (React + ELK) | 29:53 |
| P15 | Aplicativo que se auto-modifica no próprio código | 29:46–32:52 |
| P16 | Evolução em tempo real por hot reload e restilo via chat | 32:52–34:07 |
| P17 | Agent Host Protocol modelando ambiente inteiro | 36:20–38:20 |
| P18 | Endpoint WebSocket AHP gerado a partir da especificação pública | 40:20–42:08 |
| P19 | Múltiplos clientes sincronizados na mesma sessão | 39:02–42:08 |
| P20 | Tunelamento de ferramentas locais pela rede | 38:14 |
| P21 | Vibe OS: sistema operacional alucinado sem código de aplicação | 44:01–44:31 |
| P22 | HTML gerado sob demanda por janela via sessão do SDK | 45:49 |
| P23 | Diffs HTML mínimos disparados por eventos (replace/append) | 48:20 |
| P24 | Catálogo infinito de aplicativos inventados pela busca | 51:14 |
| P25 | Persistência sem formato rígido (load/save + reinterpretação) | 52:16–52:48 |
| P26 | Painéis/laterais customizáveis sobre dados reais | 54:13 |
| P27 | Sandboxing e permissões para ferramentas de shell | 20:46 |
| P28 | Resiliência de produção (rate limits, travamentos, edições concorrentes) | 20:46–21:35 |
| P29 | Revisão paralela por múltiplos modelos | 15:44–16:33 |
| P30 | Isolamento por worktree para revisão de PRs | 28:03 |
| P31 | Checklist de revisão persistido/salvo na ferramenta | 31:28–32:52 |
| P32 | Escala de centenas de PRs por mês | 02:53 |
| P33 | Diferenciação por ferramentas pessoais sob medida | 01:42–01:47 |
| P34 | SO inteiramente alucinado como arquitetura de produto | 54:13 |

## URL Trust Policy

- URL do vídeo: `https://www.youtube.com/watch?v=yDLnn9OAHbE`; host `youtube.com`, esquema `https`, sem userinfo, query de identificador de vídeo sem credenciais.
- Classificação: URL público fornecida pelo operador; **conteúdo externo tratado como dado não-confiável**, sem execução de instruções encontradas em transcrições, páginas, issues ou descrições.
- Redirecionamentos externos e hosts não listados não foram seguidos automaticamente pelas investigações; cada fonte adicional foi registrada em `research.md` com título, URL e data.

**Próximo passo sugerido:** `/speckit.assess.research slug=avaliacao-harness-sanderson-flora`.
