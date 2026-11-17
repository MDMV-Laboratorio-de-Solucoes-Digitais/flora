# Intake: 047 — Release manifest lockstep

**Data:** 2026-11-17T13:44Z · **Âncora:** `dev@04bf70bdd` (branch `feature/047-release-manifest-lockstep`)

## Sinal

Auditoria da realidade (BLOCO 2) encontrou `.github/workflows/release.yml:53-61`
exigindo `length == 8` para pacotes `flora*`, enquanto `cargo metadata --no-deps`
retorna 6 (`flora`, `flora-api`, `flora-core`, `flora-messaging`, `flora-search`,
`flora-tests`).

## Teste vermelho (executado)

Comando idêntico ao do workflow, com `RELEASE_VERSION=0.0.0`, contra
`/tmp/flora-metadata-047.json`: **`RED_TEST_EXIT=1`**.

## Ranqueamento 80/20 (BLOCO 5)

Critério: `(Impacto × Confiança) / (Esforço × Risco)`, cada eixo 1–5.
C1 = (4×5)/(1×1) = **20.0**, à frente de C2/C3 (10.0), C4 (3.75), C5 (2.08),
C7 (1.67) e C6 (0.80, além de exigir autorização SE).

## Fronteiras

- Escopo: apenas a asserção de contagem no workflow de release.
- Fora: tag, release, deploy, branch protection, constitution, ADR, outros workflows.
- Nenhum artefato da factory `aa7600d` importado (decisão A do BLOCO 0.a).
