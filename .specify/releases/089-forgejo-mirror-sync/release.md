# Release: 089 — Espelho Forgejo com push fast-forward e job advisory

**Ciclo**: 089 · **Data**: 2026-11-20 · **Branch**: `feature/089-forgejo-mirror-sync` · **Baseline/Âncora**: `15203ee78`
**Tipo**: infraestrutura de publicação — um workflow novo, event-driven, one-way e advisory sob ADR-0003. **Nenhuma tag, release, deploy, merge, produção, force-push ou reescrita de histórico.**

## Entregue

- `.github/workflows/mirror-to-forgejo.yml` (novo) — job `mirror` com `on.push` em `dev`, `main` e tags `v*`, `permissions: {}`, `concurrency` por `github.ref` com `cancel-in-progress: false`, checkout com `fetch-depth: 0` e `persist-credentials: false`. Credenciais `FORGEJO_USER`/`FORGEJO_TOKEN` lidas de `secrets.*` via `env` do step, fail-closed se vazias, aplicadas como header Basic em `http.<host>/.extraHeader` (URL do remoto sem credenciais) e descartadas com `unset`. Guarda `git merge-base --is-ancestor forgejo/<branch> HEAD` autoriza `git push forgejo HEAD:refs/heads/<branch>` — nunca `--force`, `--mirror` ou deleção; tags `v*` seguem pela própria ref e refs fora do escopo são recusadas.
- Verificação: **YAML parse OK** (`jobs=['mirror']`, `on push dev/main + tags v*`); **gate-integrity PASS** com `all 10 db contract floors intact (58 total)` (`exit=0`, 10 legs / 58 contratos); `git push origin refs/heads/dev:refs/heads/dev` → `f12a328c4..15203ee78 dev -> dev` (`exit=0`, fast-forward, sem force). Detalhe literal em `.specify/validations/089-forgejo-mirror-sync/validate.md` (R4 **APROVADO**).

## SemVer

**Proposta:** patch `0.4.3` → `0.4.4`. Motivo: nenhuma alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto — apenas um workflow de publicação advisory que espelha refs já existentes por fast-forward, sem criar required check e sem tocar o gate autoritativo (10 legs / 58 contratos, ciclo 088). Mesmo critério dos precedentes imediatos 087 (`0.4.1 → 0.4.2`) e 088 (`0.4.2 → 0.4.3`) para mudanças de governança/CI. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` o job lê `FORGEJO_USER` e `FORGEJO_TOKEN` de `secrets.*` para o `env` do step, falha fechado se qualquer um estiver vazio e autentica via `http.<host>/.extraHeader` com Basic sem embutir credenciais na URL do remoto, em argumentos ou em logs; o valor é descartado com `unset` após a configuração.
- `FR-002` o job espelha somente pushes em `dev`, `main` e tags `v*` para `FORGEJO_REMOTE=https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` (URL não secreta), com checkout de histórico completo (`fetch-depth: 0`, `persist-credentials: false`), `permissions: {}` e serialização por ref (`cancel-in-progress: false`); nome do job `mirror`, fora dos required checks.
- `FR-003` antes de cada push de branch, o job atualiza `refs/remotes/forgejo/dev` e `refs/remotes/forgejo/main` e exige `git merge-base --is-ancestor forgejo/<branch> HEAD`; o envio é `git push forgejo HEAD:refs/heads/<branch>`, sem `--force`, sem `--mirror` e sem deleção de refs. Falha do guarda, divergência ou push rejeitado encerram o job sem tentativa de reescrita. Premissa verificada: `origin/dev` = `f12a328c` é ancestral de `dev`, com push `f12a328c4..15203ee78 dev -> dev`, `exit=0`.
- `FR-004` o job permanece informativo/advisory sob ADR-0003: gatilho somente `push`, nome `mirror` ausente do conjunto de required checks e nenhuma alteração em proteção de branch, `REQUIRED`, `ALLOWED_JOB_IF` ou jobs de CI existentes. O gate autoritativo continua `scripts/ci/check_gate_integrity.py` (10 legs / 58 contratos).
- `FR-005` a mudança fica limitada a `.github/workflows/mirror-to-forgejo.yml` e aos 4 artefatos mínimos do ciclo 089 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`); nenhum arquivo de implementação em `validate/`, `release/` ou `.specify/memory/STATE.md` foi tocado e nenhum segredo foi registrado nos artefatos. R4 e R7 são os registros de governança escritos após a implementação.
- `T001` âncora confirmada: `.specify/feature.json` = `specs/089-forgejo-mirror-sync`, branch `feature/089-forgejo-mirror-sync` @ `15203ee78`, `origin` = `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`; `T002` diagnóstico do espelho e do limite de gate (`origin/dev` `f12a328c` ancestral de `dev` → fast-forward; ADR-0003 mantém o job fora dos required checks; secrets `FORGEJO_USER`/`FORGEJO_TOKEN` identificados); `T003` isolamento de credenciais e fail-closed no workflow (FR-001/FR-002); `T004` guarda de fast-forward e push sem force, com recusa de refs fora de `dev`/`main`/`v*` (FR-003); `T005` job advisory e diff cirúrgico, com `concurrency` por ref e `cancel-in-progress: false` (FR-004/FR-005); `T006` verificação de SC-001/SC-002/SC-003 e fechamento dos 4 artefatos 089, com estado vivo do Forgejo/job/proteção de branch marcado `[NV]`.

## Evidência

`.specify/validations/089-forgejo-mirror-sync/validate.md` (R4 **APROVADO**, saídas literais) · baseline `15203ee78` · diff da implementação: `.github/workflows/mirror-to-forgejo.yml` (novo) + 4 artefatos 089 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`). Parse YAML `jobs=['mirror']` com gatilho `push` em `dev`/`main` + tags `v*`; gate PASS 10 legs/58; `git push origin refs/heads/dev:refs/heads/dev` → `f12a328c4..15203ee78 dev -> dev` (`exit=0`).

## Rollback

Remover **exatamente um arquivo**: `.github/workflows/mirror-to-forgejo.yml`. Isso encerra os futuros disparos do espelho sem tocar no histórico do Forgejo.

- **Sem force-push no espelho**: não executar `git push --force`, `--mirror` ou deleção de refs em `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`; o espelho one-way permanece como estava no último push fast-forward (`f12a328c4..15203ee78 dev -> dev`). Reverter o workflow não implica reverter commits já espelhados.
- Se a mudança ainda estiver não commitada, basta remover o arquivo do working tree; se já commitada, usar um novo commit de reversão (ou `git revert` do commit do ciclo). Não há migrations, código de aplicação, gate, required check nem outros artefatos de runtime para remover — apenas os artefatos documentais 089 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`), além deste release e do R4, que são registros do ciclo e não afetam o sistema.
- Após a remoção, o gate autoritativo permanece inalterado: repetir `python3 scripts/ci/check_gate_integrity.py` deve continuar com `all 10 db contract floors intact (58 total)`. **Nenhum banco de produção é tocado, alterado ou removido.**

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions/Forgejo, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Parse YAML local, gate PASS e push de verificação não substituem a CI remota.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **Secrets e estado vivo do Forgejo**: nenhum valor de `FORGEJO_USER`/`FORGEJO_TOKEN` foi lido ou registrado, e a condição real do remoto, do job `mirror` e da proteção de branch não foi reconsultada — reconsultar antes de qualquer merge ou promoção.
- **DBs de produção**: nenhum acesso, escrita ou remoção; o ciclo não envolve migrations nem contratos DB.
- **Escala**: um workflow de publicação novo; sem teste de carga, UI, mudança de runtime ou comportamento de produto a verificar.
