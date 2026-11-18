# Intake: 055 — Auth floor thirteen

**Data:** 2026-11-18T01:33Z · **Âncora:** `github/dev@d5d7ef5d` · branch `feature/055-auth-floor-thirteen`

## Sinal

O leg `auth` do job `db-tests` usa piso 12, mas o filtro
`contract::auth::db_` descobre **13** testes `#[ignore]`. Um teste pode
desaparecer ou falhar sem que o gate fique vermelho — folga de 1 no check
obrigatório de merge. Os outros sete legs usam piso exatamente igual à contagem.

## Evidência (verificada)

- `cargo test -q -p flora-tests 'contract::auth::db_' -- --list` → **13 testes**
  (executado nesta sessão, 01:31Z).
- `.github/workflows/rust.yml:107` → `run_leg auth … 12`;
  `scripts/ci/check_gate_integrity.py:43` → `"auth": 12`.
- `scripts/ci/check_gate_integrity.py:120` → texto "49 total" (soma dos pisos).
- No ciclo 053, os 8 legs rodaram com Postgres 18.6 real: auth **13 passed**
  (piso 12) — os 13 executam e passam, sem teste condicional observado.
- `docs/adr/0003:19` ainda cita "12/4/8/8/5/6 — total 43" — dívida documental
  de governança, fora de escopo (não reescrita).

## 80/20

`(I×C)/(E×R)` = (4×5)/(2×1) = **10.0** — candidato nº 1, registrado em
`STATE.md` antes do BLOCO 6. ADR drift 5.0, índice de docs 3.33, N3 1.0,
dívidas de fase 0.67; C5/C6 não autônomos.

## Fronteiras

Escopo: 3 linhas em 2 arquivos (piso 12→13 em dois locais, texto 49→50)
mais artefatos do ciclo e notas datadas. Fora: ADR-0003, constituição,
testes Rust, outros pisos, C5/C6, deploy. Factory lida em
`d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7`; nada materializado no alvo.
