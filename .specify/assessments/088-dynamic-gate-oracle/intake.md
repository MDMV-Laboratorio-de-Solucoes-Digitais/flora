# Idea Intake: Oráculo dinâmico de pisos no gate

- **Slug**: 088-dynamic-gate-oracle
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/088-dynamic-gate-oracle/decision.md` (+ `specs/088-dynamic-gate-oracle/spec.md`)
- **Type**: improvement

## Idea (as captured)

`scripts/ci/check_gate_integrity.py` é descrito como guarda-espelho: compara o
mapa `FLOORS`, escrito à mão no script, com as linhas `run_leg <label>
<filter> <floor>` escritas à mão em `.github/workflows/rust.yml`; se as duas
cópias continuam iguais, o guarda passa mesmo existindo um contrato `#[ignore]`
fora de ambas (`decision.md:11-16`). Isso já custou dois ciclos seguidos: em 086
os três contratos de RBAC e em 087 os quatro de bootstrap, ambos descobertos só
manualmente; um terceiro esquecimento passaria o guarda exatamente como os
anteriores (`decision.md:18-24`). A lista apontada como confiável é
`cargo test -p flora-tests -- --ignored --list`, pois filtros
`cargo test <filtro> -- --list` não funcionaram de forma confiável neste checkout
(`decision.md:26-31`). O artefato registra como direção substituir a comparação
de espelho pela descoberta dinâmica dentro do próprio guarda, preservando as
checagens estáticas e o modo `--remote` (`decision.md:33-58`;
`specs/088-dynamic-gate-oracle/spec.md:9,13-17`).

## Restated

Propõe-se que o guarda de integridade descubra os contratos `#[ignore]` a partir
do próprio código e confira as legs e os pisos contra essa descoberta, em vez de
comparar duas listas mantidas manualmente. A motivação registrada é evitar que
um contrato novo sem lega se torne um terceiro *confidence gap*.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: dois esquecimentos consecutivos (086 RBAC, 087 bootstrap) descobertos apenas por inspeção humana, somados à natureza de espelho do guarda, que não vê o código-fonte dos testes (`decision.md:18-24`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: `cargo test -p flora-tests -- --ignored --list` é determinístico e seguro em todos os ambientes do job `rust-build`, inclusive sem Postgres?]
- [NEEDS CLARIFICATION: como tratar nomes de testes ignorados que não pertencem a nenhuma das 10 legs ou que casam com mais de um prefixo de filtro?]
- [NEEDS CLARIFICATION: a associação por `startswith` é suficiente para submódulos futuros ou será preciso delimitar fronteiras entre prefixos?]
- [NEEDS CLARIFICATION: quais consumidores dependem da mensagem exata `all 10 db contract floors intact (58 total)` e como reagem a uma contagem derivada diferente?]
- [NEEDS CLARIFICATION: a descoberta passa a depender da compilação do workspace de testes; quem garante que isso não acrescenta tempo ou custo ao gate?]
