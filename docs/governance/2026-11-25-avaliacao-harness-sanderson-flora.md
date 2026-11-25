# Governança — avaliação harness Sanderson (NDC Oslo 2026)

**Data**: 2026-11-25 · **Slug**: `avaliacao-harness-sanderson-flora` · **Base**: `github/dev@0ef79ec60`

## Decisão registrada

Assessment documental concluído com `Veredito: go` e 34 vereditos: ADOPT 5, ADAPT 10, DEFER 9, AVOID 6, REJECT 4. O artefato canônico é `.specify/assessments/avaliacao-harness-sanderson-flora/decision.md`; a pesquisa com fontes está em `research.md`; o brief executivo em `docs/research/2026-11-24-sanderson-selfevolving-tools.md`.

## Limites de autoridade

- Nenhuma proposta autoriza implementação, mudança de gate, ADR, constituição, CI, migration, credencial ou deploy.
- Constituição v1.4.0 e ADR-0003 permanecem: merge em `dev` é decisão de máquina sob checks verdes; promoção `dev→main` e deploy são competência humana.
- Auto-modificação de paths protegidos (gates, harness, políticas, contratos) sem PR versionado e aprovação humana continua proibida — base do `AVOID` em P15 e do `REJECT` em P10.
- PR #410 (`dev→main`) não foi mesclado por esta sessão; nenhum auto-merge, force-push ou alteração de proteção.
- `origin`/Forgejo é espelho; `github` é o remoto canônico para PRs.

## Consequência operacional

Não há candidato 80/20 elegível no inventário P01–P34: o único gap nomeado pela pesquisa (BUG-111-01/02/03) já foi fechado em `6288a8569`, ancestral de `github/dev` `[EV]`; os demais itens são já cobertos, documentais ou exigem dependência/churn incompatível com o norte `debloat`/VPS barata. Lacunas reais remanescentes devem ser tratadas em ciclos próprios, não reclassificadas como "progresso" deste assessment. Pesquisa externa foi tratada como dado não-confiável; injeções em E2B docs, PR #410, A2UI e site comunitário zh foram registradas e ignoradas.

## Evidência

Ver `.specify/validations/avaliacao-harness-sanderson-flora/{test-roadmap,evidence}.md`. Gate e status remoto são marcados conforme observação ao vivo em `evidence.md`; o que não foi observado permanece `[NV]`.
