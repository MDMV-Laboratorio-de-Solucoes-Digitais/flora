# Problem Definition: Circuit-breaker morto no repositório

- **Slug**: 061-dead-circuit-breaker
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`flora-core/src/utils/circuit_breaker.rs` tem 68 linhas e sua própria documentação admite ser um placeholder que "forwards calls without observing failures"; o arquivo não é declarado em `utils/mod.rs` (`pub mod jwt; pub mod pkce;`), portanto nunca é compilado nem testado (intake.md:9-12; research.md:5-7). Seus dois campos em `config/messaging.rs:19,21` (`circuit_breaker_threshold`, `circuit_breaker_recovery_secs`) e os defaults em `:33,34` não têm outro uso; a variante `Error::CircuitBreakerOpen` em `error.rs:179-181` e seu braço em `:365` (grupo mapeado para 500) são construídos apenas pelo arquivo morto (intake.md:14-17; research.md:9-11). `config/mod.rs:44-46` usa Figment com `Env::prefixed("FLORA_").split("__")` e sem `deny_unknown_fields`, logo remover os campos não quebra env extras (intake.md:19). Isso contraria o princípio I da constituição — debloat e leveza (intake.md:21). Ranqueamento 80/20 = 1.67 (intake.md:25-27).

## Affected Users & Stakeholders

- **Users**: desenvolvedores que leem o código e a configuração e podem supor que existe proteção de circuit-breaker onde há apenas um placeholder nunca carregado (intake.md:9-15).
- **Stakeholders**: mantenedores responsáveis pela leveza do repositório e pela superfície de leitura, que precisam remover dívida morta sem alterar comportamento de runtime (`define.md:16`; `decision.md:9-13`).

## Goals

- Excluir `utils/circuit_breaker.rs` (`define.md:7`; `shape.md:5-6`).
- Remover os dois campos e seus defaults de `config/messaging.rs` (`define.md:8`).
- Remover `Error::CircuitBreakerOpen` e seu braço de match em `error.rs` (`define.md:9`).
- Garantir que nenhum comportamento de runtime, CI, gate ou ADR mude (`define.md:10`; `research.md:18`).

## Non-Goals

- Não implementar circuit-breaker real — não há chamadores (`define.md:13`; `shape.md:10`).
- Não alterar Figment/Config, ADRs, workflows ou proteção de branch (`define.md:14`; `decision.md:12`).
- Não manter campos "para o futuro": env é ignorada e não há consumidor (`research.md:19`; `shape.md:11`).
- Não refatorar `Error` amplamente (`shape.md:12`).

## Success Metrics

- `grep -rn circuit_breaker` → 0 ocorrências no código (baseline: arquivo morto + referências em `config/messaging.rs:19,21,33,34` e `error.rs:181,365` — define.md:14; intake.md:14-17).
- Diff = 3 arquivos (1 exclusão, 2 edições) + artefatos do ciclo (baseline: nenhuma mudança de código — define.md:16; decision.md:11).
- `cargo check/build/test`, clippy, deny e fmt verdes (baseline: verdes; módulo nunca compilava — define.md:15; research.md:7).
- Gates do alvo permanecem verdes (baseline: verdes — define.md:17).
- CI/runner ao vivo: **[NV]** — observar no PR antes do merge (decision.md:17).

## Cost of Inaction

O placeholder morto e suas referências órfãs continuam ocupando superfície de leitura e sugerindo uma proteção que nunca existiu em runtime. A remoção é reversível e de risco baixo; o comportamento em produção não muda porque o módulo nunca entrou na compilação (intake.md:21; research.md:18; shape.md:15-17).

## Open Questions

- [NEEDS CLARIFICATION: algum env externo ainda define `FLORA_*` com os campos removidos?] (`intake.md:19`; `research.md:11`; **[NV]**)
- [NEEDS CLARIFICATION: algum ADR ou documento cita o circuit-breaker além do código?] (`define.md:14`; `decision.md:12`)
- [NEEDS CLARIFICATION: em qual PR a CI ao vivo será observada?] (`decision.md:17`; **[NV]**)
