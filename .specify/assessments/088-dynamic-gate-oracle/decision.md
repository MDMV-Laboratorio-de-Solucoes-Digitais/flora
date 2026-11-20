# Decision: trocar o espelho de pisos por oráculo dinâmico de contratos ignorados (ciclo 088)

- **Slug**: 088-dynamic-gate-oracle
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/088-dynamic-gate-oracle` @ `cbc885383`
- **Artefatos revisados**: evidência local (guarda, workflow, contratos `#[ignore]`) | spec | plan | tasks

## Contexto / Problema

`scripts/ci/check_gate_integrity.py` é hoje um **guarda espelho**: compara o
mapa `FLOORS`, escrito à mão no script, com as linhas `run_leg <label>
<filter> <floor>`, escritas à mão em `.github/workflows/rust.yml`. Se as duas
cópias continuam iguais, o guarda passa — mesmo que exista no código um
contrato `#[ignore]` que não esteja em nenhuma das duas. Os dois lados são
alimentados por humano; não há fonte de verdade independente.

Isso já custou dois ciclos seguidos: em 086 os 3 contratos `#[ignore]` de
`flora-tests/src/contract/rbac.rs` estavam fora do CI e em 087 os 4 de
`flora-tests/src/contract/bootstrap.rs`. Em ambos a descoberta foi manual —
foi preciso notar o *confidence gap*, abrir o ciclo, adicionar a lega no
workflow e o piso no guarda. Um terceiro esquecimento passaria o guarda
exatamente como os dois anteriores passaram: o espelho não tem como ver o
código-fonte dos testes.

A lista confiável é `cargo test -p flora-tests -- --ignored --list`: ela
enumera TODOS os testes ignorados (por exemplo as 4 linhas
`contract::bootstrap::tests::db_bootstrap_*`). Filtros do tipo `cargo test
<filtro> -- --list` não funcionaram de forma confiável neste checkout — por
isso o oráculo consome a lista completa e aplica os filtros das legs
localmente, por prefixo, em vez de depender de uma listagem filtrada.

## Decisão

Transformar a validação de db floors em **oráculo dinâmico** dentro do próprio
`scripts/ci/check_gate_integrity.py`, sem tocar no workflow:

1. Executar `cargo test -p flora-tests -- --ignored --list` com `cwd=ROOT`,
   `capture_output=True, text=True`; usar somente `stdout` (cargo pode emitir
   warnings na `stderr`).
2. Extrair os nomes de testes das linhas terminadas em `: test`.
3. Extrair de `rust.yml` cada `run_leg <label> <filter> <floor>` com regex
   ancorada em linha.
4. Para cada leg: exigir que `label` esteja em `FLOORS`, que o `floor` do
   workflow seja igual a `FLOORS[label]` e que a quantidade de nomes
   ignorados que começam com `filter` (`startswith`, para cobrir submódulos)
   seja **exatamente** igual ao piso. Exigir também uma única `run_leg` por
   rótulo registrado.
5. Falhar fechado se o cargo falhar, se a lista estiver vazia, se não houver
   legs ou se qualquer contagem divergir. Manter a mensagem de sucesso
   `all N db contract floors intact (M total)` — hoje
   `all 10 db contract floors intact (58 total)` — agora derivada da
   descoberta, não de constantes copiadas.

As verificações estáticas existentes (`REQUIRED`, jobs, `continue-on-error`,
path filters, comandos `cargo build/test --workspace`) e o modo `--remote`
permanecem intactos. Stdlib apenas: nenhuma dependência, rede ou API nova. O
diff fica limitado ao guarda e aos quatro artefatos mínimos do ciclo 088.

## Scorecard

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | O guarda compara duas cópias manuais; 086 (3 contratos RBAC) e 087 (4 bootstrap) foram esquecimentos descobertos só humano |
| Força da evidência | strong | `cargo test -p flora-tests -- --ignored --list` lista os ignorados de forma determinística; filtros `--list` falharam, então a origem é a lista completa; FLOORS atual = 10 legs / 58 |
| Valor vs. inação | strong | Inação mantém a possibilidade de um 3º `#[ignore]` órfão passar o gate; com o oráculo, contrato novo ou piso defasado falha o `rust-build` |
| Viabilidade | strong | Uma função no script existente, stdlib (`subprocess`/`re`), sem novo check obrigatório, sem workflow, sem segredo |
| Fit estratégico | strong | Reforça ADR-0003 sem criar outro required check: o mesmo guarda determinístico passa a consultar a fonte de verdade (os testes) |
| Risco | strong | Fail-closed em cargo/list vazia/divergência; checagens estáticas e `--remote` preservadas; diff reversível e confinado ao guarda + 4 artefatos |

## Handoff

**Dentro**: `scripts/ci/check_gate_integrity.py` (oráculo dinâmico) e os
quatro artefatos 088 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`).

**Fora**: `.github/workflows/rust.yml`, os próprios testes `#[ignore]`,
`validate/`, `release/`, `.specify/memory/STATE.md`, constituição, ADRs,
outros pisos/legs, `REQUIRED`/`ALLOWED_JOB_IF`, código de aplicação, PRs e
worktree espelho.

**Métricas de sucesso**: gate PASS com
`all 10 db contract floors intact (58 total)` derivado de 58 nomes
`contract::…: test` descobertos; divergência controlada de piso ou lista vazia
faz o guarda falhar; `git diff --name-only cbc885383` limitado ao guarda e aos
4 artefatos do ciclo.

**Verificação (evidência local registrada, gates/formatters não reexecutados
nesta avaliação de docs)**: oráculo executado sobre os 10 contratos
`#[ignore]` do workspace (`auth 13, messaging 4, websocket 8, search 8,
notifications 5, files 6, tasks 4, workspace 3, rbac 3, bootstrap 4 = 58`);
`python3 scripts/ci/check_gate_integrity.py` = PASS com as 10 floors (58).
