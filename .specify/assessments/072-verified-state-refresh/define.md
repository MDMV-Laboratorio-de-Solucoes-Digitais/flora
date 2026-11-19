# Define: refresh verificado de estado (072)

- **Slug**: 072-verified-state-refresh
- **Definido**: 2026-11-19T04:22Z
- **Entradas**: `intake.md`, `research.md`

## Problema

Agentes e maintainers que leem `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md` e `docs/research/index.md` encontram como estado verificado uma nota de 01:58Z (ciclo 068), embora os ciclos 069 e 070 já estejam fechados e mesclados. As contagens nessas notas descrevem um checkout anterior (58 specs / 55 assessments / 54 validations / 54 releases), enquanto o checkout real tem 61/58/57/57. A ausência de nota datada para os ciclos mais recentes e a divergência numérica tornam esses documentos instrumentos de decisão desatualizados.

## Afetados e stakeholders

- **Agentes de sessão** que usam esses quatro arquivos como âncora inicial — leem números errados e podem reabrir dívidas já fechadas (evidência: `docs/research/index.md:169-170` vs `:183`).
- **Maintainers** que revisam PRs e precisam distinguir histórico append-only de estado atual.
- **Usuários finais**: não afetados diretamente; o defeito é de governança/informação, não de comportamento do produto.

## Metas

1. Registrar, por apêndice datado, o estado verificado em 2026-11-19 com âncora local `cdf50eaff` (início da sessão) e HEAD final da feature no fechamento.
2. Tornar explícitas as contagens reais e o intervalo de ciclos cobertos (001–070) nos quatro documentos, sem reescrever notas anteriores.
3. Preservar a correção do ciclo 069 (137 arquivos = 137 links, `missing=[]`) e a dívida da leg RBAC como `[NV]`/escalada, sem reabri-las por inferência.

## Não-metas

- Não alterar código Rust/Svelte, workflows, `scripts/ci/`, pisos, ADRs, constituição, migrations ou produção.
- Não tocar o worktree/branch `feature/071-dead-auth-service` nem renumerar trabalho alheio.
- Não regenerar `.generated.md` nem inventar fonte para índices.
- Não afirmar status remoto, CI ao vivo, merge ou aprovações sem verificação ao vivo.
- Não resolver a leg RBAC do CI (exige aprov maintainer) nem a emenda constitucional (exige maintainer).

## Métricas de sucesso

- **SC-001** — todos os quatro documentos contêm apêndice datado com contagens 61/58/57/57/4/137 e intervalo até 070.
- **SC-002** — `git diff --name-only dev..HEAD` contém apenas os quatro documentos, artefatos do ciclo 072 e `.specify/memory/STATE.md`.
- **SC-003** — nenhuma linha histórica foi removida ou modificada; apenas acréscimos no fim dos arquivos.
- **SC-004** — gate autoritativo permanece PASS com as mesmas 4 checks e 8 pisos/51; nenhum `[NV]` foi convertido em afirmação verificada.

## Baseline (se nada for feito)

Os documentos continuam desatualizados; agentes podem citar 58/55/54/54 e tratar a dívida 047+ como aberta, contrariando a evidência local de 137/137 do ciclo 069 e o fechamento 070. Custo: decisões futuras ancoradas em estado morto.

## Perguntas em aberto

- Nenhuma que bloqueie a especificação. A identidade da feature é `feature.json`/`check-prerequisites.sh` (051 no início, 072 após criação legítima); a divergência é registrada, não reescrita.
