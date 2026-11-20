# Nota de governança — 2026-11-20 (ciclo 089)

**Assunto:** decisões de governança do ciclo 089 (espelho Forgejo).

**Âncora:** branch `feature/089-forgejo-mirror-sync` @ `15203ee78`; `.specify/feature.json` → `specs/089-forgejo-mirror-sync`. Registro append-only; nenhum validation/release existente tocado.

## (a) Gate autoritativo do alvo — decisão explícita do SE

Decisão explícita do SE: os **4 scripts SDD da factory** — `check_sdd_completeness.py`, `check_converge_append_only.py`, `check_task_id_uniqueness.py`, `check_work_order_dag.py` — **NÃO** são gate autoritativo do alvo. Eles falham por dívida legada de baseline da hub (75 specs) e não se aplicam ao estado verificado deste repositório; seu `FAIL` não é regressão e não bloqueia merge.

O gate autoritativo do alvo é, e continua sendo, `python3 scripts/ci/check_gate_integrity.py` (com o oráculo dinâmico do ciclo 088, 10 legs / 58 pisos). Nenhum dos 4 scripts SDD deve ser promovido a required check nem substituir o guardião. Isto prevalece sobre qualquer leitura que trate a suíte SDD da factory como fonte de verdade de gate.

## (b) Escopo mínimo de branch protection

Para verificar/monitorar a proteção de branch via API é necessário um **PAT fine-grained com permissão `Administration: read`, restrito somente ao repositório `flora`**. O `GITHUB_TOKEN` padrão **não serve** para ler configuração de branch protection (exige escopo Admin). Nenhuma permissão adicional (write, outros repos, organizations) é concedida.

Secret sugerido para o workflow que consulta a proteção: **`FLORA_BRANCH_PROTECTION_TOKEN`**. O secret deve ser provisionado fora do repositório; nenhuma credencial foi copiada, gravada ou exposta nesta sessão. **[NV]** provisionamento real do secret e leitura ao vivo da proteção.

## (c) Advisories de dependência — recomendação, sem promoção de gate

Recomendação: o bump de `rustls` **0.23.43 → 0.23.46** resolve **4 advisories canônicos**. `Cargo.lock` neste checkout ainda registra `0.23.43`.

Os advisories permanecem **advisory/não obrigatórios**: **não promover gate ainda** — ou seja, nenhum check de advisory entra no conjunto de required checks neste ciclo. O bump e a reavaliação de promoção ficam como recomendação registrada, não como alteração executada. **[NV]** aplicação do bump e recontagem de advisories.

## (d) Deploy Flora → Coolify (ligação MDMV-Autonoma)

Deploy executado pela ligação **MDMV-Autonoma** (`CoolifyDeployLiaison`):

- **App:** `Flora-Staging`
- **Deployment:** `44djh8p8owth5buhmcvgpilb`
- **Commit:** `15203ee78`
- **Health:** `200`
- **Rollback image:** `b1dafae3…` (imagem anterior preservada para reversão)

O deploy fica explicitamente fora do gate de merge (ADR-0003). O relatório chegou por **mensageria cross-session — [NV]**: reconsultar o estado real da app/deployment antes de qualquer decisão de promoção, rollback ou novo deploy. Nenhuma credencial de Coolify foi exposta.

## (e) Ciclo 089 — espelho Forgejo

Ciclo 089 registra o espelho do repositório para Forgejo:

- **Branch:** `feature/089-forgejo-mirror-sync` @ `15203ee78`; `.specify/feature.json` → `specs/089-forgejo-mirror-sync`.
- **Origem do espelho:** `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`.
- **Ancestralidade:** `origin/dev` = `f12a328c` é ancestral de `dev` (fast-forward); o espelho não diverge do histórico local.
- **Regra ADR-0003:** o job de espelho **NÃO** deve ser configurado como required check — permanece informativo/advisory, fora do conjunto de gates obrigatórios.

**[NV]** estado ao vivo do remoto Forgejo, do job de espelho e da proteção de branch: reconsultar antes de qualquer merge ou promoção. Nenhuma credencial de Forgejo copiada.

## Histórico e escopo

Este documento é um apêndice datado e append-only. Nenhuma linha de registros anteriores (STATE.md, PROGRESSO.md, ADRs, validations/releases) foi reescrita. Nenhum código, workflow, gate, migration ou constituição foi alterado por esta nota.
