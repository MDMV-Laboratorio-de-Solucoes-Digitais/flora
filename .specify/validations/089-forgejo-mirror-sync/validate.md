# Validation: 089 — Espelho Forgejo com push fast-forward e job advisory

**Data**: 2026-11-20 · **Branch**: `feature/089-forgejo-mirror-sync` · **Baseline/Âncora**: `15203ee78`
**Tipo**: infraestrutura de publicação — um workflow novo (`.github/workflows/mirror-to-forgejo.yml`) e os artefatos documentais do ciclo 089. Nenhuma mudança de API, runtime, schema, migration, gate, required check, proteção de branch, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| Parse YAML de `.github/workflows/mirror-to-forgejo.yml` | **YAML parse OK**, `jobs=['mirror']`, `on push dev/main + tags v*` — gatilho restrito a `dev`, `main` e `v*`, um único job `mirror` (FR-002, FR-004, SC-003) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — `all 10 db contract floors intact (58 total)`: 10 legs / 58 contratos, gate autoritativo inalterado pelo ciclo 088 (FR-004) |
| `git push origin refs/heads/dev:refs/heads/dev` | `f12a328c4..15203ee78 dev -> dev`, `exit=0` — fast-forward puro, sem `--force`, sem `--mirror` e sem deleção de refs; `origin/dev` (`f12a328c4`) é ancestral de `dev` (FR-003, SC-001) |
| Fronteira do diff e segredos | apenas 1 workflow novo + artefatos do ciclo; zero arquivos em `validate/`, `release/` ou envolvendo `STATE.md` na implementação, nenhum valor de `FORGEJO_USER`/`FORGEJO_TOKEN` registrado (FR-001, FR-005, SC-003) |

### Saídas coladas

Parse do workflow (saída literal consolidada do handoff):

```
$ parse YAML de .github/workflows/mirror-to-forgejo.yml
YAML parse OK
jobs=['mirror']
on push dev/main + tags v*
exit=0
```

Gate autoritativo (saída literal consolidada do handoff):

```
$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS
all 10 db contract floors intact (58 total)
exit=0
```

Push de verificação da ancestralidade (saída literal consolidada do handoff):

```
$ git push origin refs/heads/dev:refs/heads/dev
f12a328c4..15203ee78 dev -> dev
exit=0
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, comandos git, CI ao vivo nem inspeção do estado do Forgejo. As saídas acima são a evidência literal consolidada do handoff do main agent: parse YAML com `jobs=['mirror']` e gatilho `push` em `dev`/`main` + tags `v*`; gate PASS com `all 10 db contract floors intact (58 total)` (`exit=0`); e `git push origin refs/heads/dev:refs/heads/dev` com `f12a328c4..15203ee78 dev -> dev` (`exit=0`, fast-forward, sem force). Nenhuma inferência além dessa evidência.

## Fronteira

Contra a âncora `15203ee78`, a implementação do ciclo 089 consistiu em **um workflow novo e seus artefatos documentais**:

- `.github/workflows/mirror-to-forgejo.yml` (novo) — job `mirror`, event-driven, one-way e advisory sob ADR-0003: `on.push` em `dev`/`main` e tags `v*`; `permissions: {}`; `concurrency` por `github.ref` com `cancel-in-progress: false`; checkout com `fetch-depth: 0` e `persist-credentials: false`. `FORGEJO_USER`/`FORGEJO_TOKEN` vêm de `secrets.*` via `env` do step, falham fechado se vazios, são aplicados como header Basic em `http.<host>/.extraHeader` (URL sem credenciais) e descartados com `unset`. Antes de cada push de branch, `git fetch forgejo` atualiza `refs/remotes/forgejo/{dev,main}` e `git merge-base --is-ancestor forgejo/<branch> HEAD` autoriza `git push forgejo HEAD:refs/heads/<branch>` — sem `--force`, `--mirror` ou deleção; tags `v*` são enviadas pela própria ref; qualquer outro ref é recusado (FR-001..FR-004).
- Artefatos do ciclo — `.specify/assessments/089-forgejo-mirror-sync/decision.md` (verdict go, 2026-11-20) e `specs/089-forgejo-mirror-sync/{spec,plan,tasks}.md` — registros documentais que não afetam o sistema (FR-005).

Nada mais foi alterado pela implementação: sem mudança em workflows de CI existentes, `REQUIRED`/`ALLOWED_JOB_IF`, proteção de branch, gate autoritativo, runtime, config de produção, `STATE.md`, `validate/` ou `release/`. Este R4 e o R7 são os únicos arquivos acrescentados agora, como registros de governança; os artefatos 089 não contêm nenhum valor de secret.

## [NV]

- **Secrets**: os nomes `FORGEJO_USER` e `FORGEJO_TOKEN` são conhecidos, mas nenhum valor foi lido, copiado, gravado ou exposto nesta passagem. O provisionamento real no repositório e o comportamento do job com secrets válidos/ausentes não foram verificados ao vivo; o teste de ausência é fail-closed por inspeção do código, não por execução do job.
- **Estado vivo do Forgejo**: a condição atual do remoto `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`, do tip de `dev`/`main`, das tags e do job `mirror` não foi reconsultada neste snapshot. A premissa de fast-forward apoia-se apenas na evidência `f12a328c4..15203ee78 dev -> dev` (`exit=0`) do handoff; reconsultar antes de qualquer promoção.
- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado/verificado neste snapshot; execução ao vivo do GitHub Actions/Forgejo, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS e push de verificação não substituem a CI remota.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido; o ciclo não envolve migrations nem contratos DB.

## Portarias

R1 `go` antes da implementação (confidence gap do espelho e do limite de gate registrado em `.specify/assessments/089-forgejo-mirror-sync/decision.md`, verdict go, 2026-11-20); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md` (R7); R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.3 → 0.4.4`, changelog FR-001..FR-005 / T001..T006 e rollback igual à remoção de `.github/workflows/mirror-to-forgejo.yml`, **sem force-push no espelho** e sem reescrita de histórico, sem DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos escritos diretamente. Nenhuma tag, deploy ou merge executados; PR a critério do maintainer.
