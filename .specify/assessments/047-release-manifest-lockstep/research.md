# Research: 047 — Release manifest lockstep

**Data:** 2026-11-17T13:44Z · **Modo:** evidência local; remoto **[NV]**.

## Evidência primária (local, executada)

- `.github/workflows/release.yml:53-61` — passo "Assert lockstep version across all 8 manifests":
  `jq -e` com `length == 8` e `all versions == $RELEASE_VERSION`.
- `cargo metadata --no-deps --format-version 1` → `6 ['flora', 'flora-api',
  'flora-core', 'flora-messaging', 'flora-search', 'flora-tests']`.
- `RELEASE_VERSION=0.0.0 jq -e ... /tmp/flora-metadata-047.json` → exit 1 (vermelho).
- `Cargo.toml:26-32` — 5 membros de workspace + pacote raiz `flora` = 6 manifests.
- `AGENTS.md:4` cita 7 crates e `flora-tasks`/`flora-files`, que não existem no checkout:
  a prosa é a origem provável do número 8.

## Triagem

- Não há outro consumidor local do literal `8` em workflows (`rg 'length == 8' .github` → apenas release.yml).
- `scripts/ci/check_gate_integrity.py` cobre os 4 required checks de dev/main, não o release;
  não precisa de alteração.
- Correlação com o déficit de fase: specs param em 042, validations/releases chegam a 046;
  o número 047 foi escolhido para não reutilizar identidade alheia.

## NV

Execução real do workflow de release em GitHub Actions, gatilho de tag e permissões
de publicação: **[NV]** — não verificados ao vivo e fora deste escopo.
