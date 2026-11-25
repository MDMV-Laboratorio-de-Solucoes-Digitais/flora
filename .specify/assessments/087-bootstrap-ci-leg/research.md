# Idea Research: Lega bootstrap no CI

- **Slug**: 087-bootstrap-ci-leg
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; nomes de testes, workflow e scripts em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. Números de linha citados são do checkout da worktree isolada (base `github/dev@e88dc8e4d`); estados remotos ao vivo não foram reconsultados e são marcados `[NV]`.

## Users & Demand

- A demanda é do mantenedor/guardião do gate: os contratos DB de bootstrap devem rodar no job obrigatório `db-tests`, para que falhas de provisionamento atômico, idempotência e concorrência falhem o merge automático [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:25-29; specs/087-bootstrap-ci-leg/spec.md:13-20] (confidence: high).
- O gap identificado eram quatro contratos `#[ignore]` em `flora-tests/src/contract/bootstrap.rs`, enumerados no artefato como `db_bootstrap_creates_org_admin_role_membership_workspace_and_session`, `db_bootstrap_rejects_second_call_without_writes`, `db_bootstrap_rejects_when_organization_exists` e `db_bootstrap_concurrent_calls_create_single_organization` [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:11-18; flora-tests/src/contract/bootstrap.rs:210-211,272-273,295-296,313-314] (confidence: high).
- O gatilho foi a repetição do padrão descoberto em 086 (RBAC), também por inspeção manual, com o mesmo risco de filtro vazio que retorna exit 0 [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:34; .specify/assessments/087-bootstrap-ci-leg/decision.md:21-27] (confidence: high).
- A spec define o teste independente: executar a lega com Postgres local, observar `4 passed`, remover a linha e observar zero [source: specs/087-bootstrap-ci-leg/spec.md:22-23,116-118] (confidence: high).
- Os quatro cenários cobrem criação de organização/admin/papel/membership/workspace/sessão, rejeição de segunda chamada e concorrência — registrados como relevantes ao merge [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:17-19; .specify/assessments/087-bootstrap-ci-leg/decision.md:24-27] (confidence: high).

## Prior Art

- O mesmo helper `run_leg` do ciclo 086 foi reutilizado, com o piso `bootstrap` acrescentado ao mapa `FLOORS` e `"rbac": 3` preservado naquele snapshot [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:29-39; .github/workflows/rust.yml:94-108] (confidence: high).
- A armadilha do filtro vazio voltou a ser citada no workflow (comment do ciclo 037) e no artefato do ciclo [source: .github/workflows/rust.yml:87-89; .specify/assessments/087-bootstrap-ci-leg/decision.md:21-24] (confidence: high).
- O filtro longo `...::db_bootstrap_` casava zero testes e não satisfazia a regex do guarda; o filtro `contract::bootstrap::tests::db_` cobria os mesmos quatro testes e era aceito [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:33-36; specs/087-bootstrap-ci-leg/spec.md:80-82; .specify/memory/STATE.md:735] (confidence: high).
- A regex atual `LEG_RE` em `scripts/ci/check_gate_integrity.py:60` exige linhas `run_leg <label> <filter> <floor>`; a referência "linha 118" no artefato é do snapshot do ciclo e não coincide com a numeração atual do arquivo [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:35-36; scripts/ci/check_gate_integrity.py:60] (confidence: high).
- O guarda de integridade é o mesmo mecanismo de ADR-0003; nenhum novo required check foi introduzido [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:41-50; specs/087-bootstrap-ci-leg/spec.md:34-41] (confidence: high).

## Market & Context

- O padrão de "diff cirúrgico" dos ciclos 086/087 limita a mudança a duas linhas de CI e quatro artefatos, sem tocar testes, código de aplicação, `validate/`, `release/` ou `STATE.md` [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:52-59; specs/087-bootstrap-ci-leg/spec.md:99-101] (confidence: high).
- O banco é Postgres 18 provisionado como serviço do job `db-tests`, com as duas variáveis de URL no `env` do job [source: .github/workflows/rust.yml:51-68] (confidence: high).
- A contagem de pisos passou de 9/54 (086) para 10/58 (087) naquele snapshot; o ciclo 088 substituiu o espelho por oráculo dinâmico logo em seguida [source: .specify/memory/STATE.md:721-747] (confidence: high).
- A evidência local (4 passed; gate PASS com 10/58) foi registrada no handoff e não reexecutada por agentes de documentação [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:61-68] (confidence: high).

## Data & Constraints

- Estado atual do checkout: `.github/workflows/rust.yml:120` contém `run_leg bootstrap contract::bootstrap::tests::db_ 13` e `scripts/ci/check_gate_integrity.py:52` contém `"bootstrap": 13` — o piso corrente é 13, não os 4 do ciclo 087 [source: .github/workflows/rust.yml:120; scripts/ci/check_gate_integrity.py:52] (confidence: high).
- `flora-tests/src/contract/bootstrap.rs` hoje contém 13 ocorrências de `#[ignore]`: além dos quatro `db_bootstrap_*` originais, há testes `db_startup_password_*` e `db_bootstrap_status_*` que também casam com o prefixo `db_` [source: flora-tests/src/contract/bootstrap.rs:210-647; .github/workflows/rust.yml:120] (confidence: high).
- O `FLOORS` atual tem 11 entradas somando 87 (`auth 25, messaging 5, websocket 8, search 8, notifications 6, files 7, tasks 4, workspace 3, rbac 5, bootstrap 13, hitl 3`) [source: scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- As linhas originais citadas no artefato (`:138`, `:188`, `:211`, `:229`) não correspondem às linhas atuais do arquivo (`:210`, `:272`, `:295`, `:313`); o conteúdo dos nomes é o mesmo, mas qualquer citação por linha deve usar o snapshot corrente [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:15-18; flora-tests/src/contract/bootstrap.rs:210-314] (confidence: high).
- O helper falha cedo se `FLORA_DATABASE__POSTGRES_URL` ou `DATABASE_URL` estiverem ausentes e conta `N passed` com `awk`; o guarda usa correspondência exata do piso e uma `run_leg` por rótulo [source: .github/workflows/rust.yml:96-107; scripts/ci/check_gate_integrity.py:150-160] (confidence: high).
- Estado ao vivo de CI/PR/merge e do Postgres do CI: **[NV]** — não reconsultado nesta avaliação documental [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:41] (confidence: high).

## Evidence Against the Idea

- A evidência registrada (4 passed, `all 10 db contract floors intact (58 total)`) é de snapshot 2026-11-20 e diverge do estado atual do checkout (13, 11 pisos/87). Tomar os números do artefato como vigentes geraria expectativa errada de cobertura [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:61-68; .github/workflows/rust.yml:120; scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- O filtro `...::db_bootstrap_` já havia falhado uma vez (zero testes) antes da correção para `db_`; isso mostra que a escolha de prefixo é frágil e dependente da regex, não apenas do nome do teste [source: .specify/memory/STATE.md:735; specs/087-bootstrap-ci-leg/spec.md:80-82] (confidence: high).
- O guarda da época era um espelho de duas listas escritas à mão; acrescentar `bootstrap` a ambas protegia contra remoção da lega, mas não contra um novo contrato `#[ignore]` esquecido — o que de fato aconteceu com os testes adicionados depois [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:11-24; flora-tests/src/contract/bootstrap.rs:344-647] (confidence: high).
- O determinismo do cenário de chamadas concorrentes no Postgres descartável do CI não foi medido aqui; o artefato apenas o listou como risco coberto [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:40] (confidence: medium).
- Não há evidência local de que a adição do décimo piso tenha exigido atualização de documentação de estado ou outros artefatos além do diff de seis arquivos; o próprio `STATE.md` foi explicitamente mantido fora do diff [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:41; .specify/assessments/087-bootstrap-ci-leg/decision.md:57-59] (confidence: high).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: o piso 4 acompanhava exatamente os quatro testes `db_bootstrap_*`; quem decide e em qual ciclo ele muda quando novos contratos de bootstrap aparecem — como os `db_startup_password_*`/`db_bootstrap_status_*` hoje contados no piso 13?] [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:38; flora-tests/src/contract/bootstrap.rs:344-647] (confidence: high).
- [NEEDS CLARIFICATION: a escolha do filtro `contract::bootstrap::tests::db_` em vez de `...::db_bootstrap_` foi ditada pela regex do guarda; essa regex e seus limites estão documentados fora do próprio script?] [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:39; scripts/ci/check_gate_integrity.py:60] (confidence: high).
- [NEEDS CLARIFICATION: os quatro cenários originais, inclusive o de chamadas concorrentes, são determinísticos no Postgres descartável do CI?] [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:40] (confidence: high).
- [NEEDS CLARIFICATION: a adição do décimo piso (à época 10 entradas/58) exige atualização de documentação de estado ou de outros artefatos além do diff de seis arquivos?] [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:41] (confidence: high).
- [NEEDS CLARIFICATION: se o job `db-tests` rodar em outra branch que não `dev`, a lega `bootstrap` deve manter o mesmo piso ou pode divergir?] [source: .specify/assessments/087-bootstrap-ci-leg/intake.md:42] (confidence: high).
- [NEEDS CLARIFICATION: como reconciliar as linhas citadas no artefato (`:138`, `:188`, `:211`, `:229`) com as linhas correntes (`:210`, `:272`, `:295`, `:313`) sem reescrever o registro histórico?] [source: .specify/assessments/087-bootstrap-ci-leg/decision.md:15-18; flora-tests/src/contract/bootstrap.rs:210-314] (confidence: high).
- [ASSUMPTION] O piso 13 atual continua representando "todos os contratos `#[ignore]` de bootstrap"; não há artefato que vincule explicitamente os nove testes adicionais ao ciclo que os introduziu. (confidence: low).

## Sources

- `system` | `.specify/assessments/087-bootstrap-ci-leg/intake.md:1-42` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/087-bootstrap-ci-leg/decision.md:1-68` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/087-bootstrap-ci-leg/spec.md:9-132` e `specs/087-bootstrap-ci-leg/plan.md:14-60` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.github/workflows/rust.yml:51-121` (job `db-tests`, helper `run_leg`, leg `bootstrap` com piso 13) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `scripts/ci/check_gate_integrity.py:42-60,150-176` (`FLOORS`, `LEG_RE`) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `flora-tests/src/contract/bootstrap.rs:210-647` (contratos `#[ignore]`) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:729-737` e `docs/PROGRESSO.md:204-211` (registro do ciclo 087) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/088-dynamic-gate-oracle/decision.md:11-24` (limite do guarda espelho) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: nenhum host buscado. Nenhuma URL externa é necessária para os fatos locais acima; nenhum redirect seguido, nenhuma credencial incluída.
