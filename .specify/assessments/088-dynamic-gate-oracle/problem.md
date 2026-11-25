# Problem Definition: Independência da fonte de verdade no gate de contratos

- **Slug**: 088-dynamic-gate-oracle
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

`scripts/ci/check_gate_integrity.py` era um guarda-espelho: comparava o mapa `FLOORS`, escrito à mão no script, com as linhas `run_leg <label> <filter> <floor>`, escritas à mão em `.github/workflows/rust.yml`. Se as duas cópias continuassem iguais, o guarda passava mesmo existindo no código um contrato `#[ignore]` ausente de ambas — os dois lados eram alimentados por humano, sem fonte de verdade independente (`decision.md:11-16`; `research.md:159-162`). Isso já custou dois ciclos consecutivos: 086 (três contratos de RBAC) e 087 (quatro de bootstrap), ambos descobertos por inspeção manual (`decision.md:18-24`). Um terceiro esquecimento passaria exatamente como os anteriores. A listagem apontada como confiável é `cargo test -p flora-tests -- --ignored --list`, pois filtros `cargo test <filtro> -- --list` não funcionaram de forma confiável naquele checkout (`decision.md:26-31`). No estado corrente, persistem limites: a mensagem de sucesso sai de `len(FLOORS)`/`sum(FLOORS.values())`, não da descoberta (`check_gate_integrity.py:174-177`), e a associação por `startswith` pode deixar órfãos ou sobrepor prefixos (`research.md:66-67`).

## Affected Users & Stakeholders

- **Users**: mantenedor do gate ADR-0003, que precisa que um contrato `#[ignore]` novo sem cobertura falhe o `rust-build` em vez de se tornar um terceiro *confidence gap* (`research.md:159`).
- **Stakeholders**: dono do processo de merge autônomo; autores de contratos de banco; consumidores da linha de sucesso `all N db contract floors intact (M total)` em registros e relatórios (`research.md:178`).

## Goals

- Fazer a validação dos pisos depender da própria fonte de verdade (os testes ignorados no código), e não da igualdade entre duas listas mantidas à mão.
- Garantir falha fechada quando a listagem falhar, estiver vazia ou qualquer contagem divergir (`decision.md:49-53`).
- Preservar as checagens estáticas existentes e o modo `--remote`, que hoje compara contextos required ao vivo (`decision.md:55-58`; `research.md:168-169`).
- Manter a mensagem de sucesso interpretável por quem a consome, distinguindo contagem descoberta de constantes copiadas (`research.md:178,186`).

## Non-Goals

- Criar novo required check ou alterar o conjunto `REQUIRED`/`ALLOWED_JOB_IF` (`decision.md:71-79`).
- Alterar `.github/workflows/rust.yml`, os próprios testes `#[ignore]` ou qualquer lega/piso específico (`decision.md` 088, "Fora").
- Promover scripts SDD da factory a gate ou evidência de verificação — decisão 089 (`research.md:175`).
- Afirmar medição de custo/tempo de compilação acrescentada ao gate: não foi medida nos artefatos (`research.md:188`); estado remoto **[NV]**.

## Success Metrics

- Contrato `#[ignore]` adicionado sem lega correspondente provoca falha do guarda (baseline: guarda passava com duas listas manuais iguais — `research.md:160`).
- Divergência controlada de piso ou lista vazia faz o guarda falhar fechado (baseline: não detectável pelo espelho — `decision.md:49-53`).
- Mensagem de sucesso coerente com a contagem descoberta (baseline: `all 10 db contract floors intact (58 total)` registrado; checkout com 11 pisos/87 — `research.md:187`).
- Cobertura global dos testes ignorados verificável, sem órfãos nem atribuições ambíguas (baseline: validação feita leg a leg, sem checagem global — `research.md:67`).

## Cost of Inaction

Um terceiro contrato sem lega passaria o guarda exatamente como 086 e 087 passaram, deixando contratos de banco fora do merge automático. A confiança no CI continuaria dependente de alguém notar a omissão manualmente, e a divergência entre o que a decisão descreve ("contagem derivada da descoberta") e o que o script exibe (`len(FLOORS)`) seguiria invisível para consumidores externos (`research.md:66,186`).

## Open Questions

- [NEEDS CLARIFICATION: `cargo test -p flora-tests -- --ignored --list` é determinístico e seguro em todos os ambientes do job `rust-build`, inclusive sem Postgres?] (`intake.md:39`)
- [NEEDS CLARIFICATION: como tratar nomes de testes ignorados que não pertencem a nenhuma lega ou que casam com mais de um prefixo de filtro?] (`intake.md:40`; `research.md:67`)
- [NEEDS CLARIFICATION: a associação por `startswith` é suficiente para submódulos futuros ou será preciso delimitar fronteiras entre prefixos?] (`intake.md:41`)
- [NEEDS CLARIFICATION: quais consumidores dependem da mensagem exata `all 10 db contract floors intact (58 total)` e como reagem a uma contagem derivada diferente (hoje 11/87)?] (`intake.md:42`; `research.md:187`)
- [NEEDS CLARIFICATION: a descoberta passa a depender da compilação do workspace de testes; quem garante que isso não acrescenta tempo ou custo ao gate?] (`intake.md:43`)
- [NEEDS CLARIFICATION: a mensagem de sucesso deve usar a contagem descoberta em vez de `len(FLOORS)`/`sum(FLOORS.values())`, ou a divergência registrada entre decisão e código é aceita?] (`research.md:186`)
