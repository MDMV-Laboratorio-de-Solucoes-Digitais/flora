# Release: 090 — Bump de segurança rustls 0.23.43 → 0.23.46

**Ciclo**: 090 · **Data**: 2026-11-20 · **Branch**: `feature/090-rustls-security-bump` · **Baseline**: `824ba00e9`
**Tipo**: atualização de segurança de dependência — `Cargo.lock` e isenção cargo-vet alinhados para `rustls 0.23.46`, sem alterar o requisito `rustls = "0.23"` do manifest. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `Cargo.lock` — entrada `name = "rustls"` (`Cargo.lock:2748`) atualizada de `0.23.43` para `0.23.46`; demais entradas preservadas.
- `supply-chain/config.toml` — `[[exemptions.rustls]]` (`1115-1117`) com `version = "0.23.46"` e `criteria = "safe-to-deploy"`.
- `Cargo.toml:189` permanece `rustls = "0.23"` — o bump é exclusivamente de lockfile, sem mudança de faixa de compatibilidade.
- Verificação: gate autoritativo **PASS** (`all 10 db contract floors intact (58 total)`, `exit=0`), `cargo test --workspace` `exit=0` (0 falhas), dependency-review limpo. Detalhe literal em `.specify/validations/090-rustls-security-bump/validate.md` (R4 **APROVADO**).

## SemVer

**Proposta:** patch `0.4.4` → `0.4.5`. Motivo: correção de segurança em dependência transitiva/direta já coberta pela faixa `0.23` do manifest — nenhuma alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto. Mesmo critério dos precedentes imediatos 087 (`0.4.1 → 0.4.2`), 088 (`0.4.2 → 0.4.3`) e 089 (`0.4.3 → 0.4.4`) para mudanças de governança/CI e dependências sem quebra de contrato. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` `Cargo.toml:189` mantém `rustls = "0.23"`; faixa de compatibilidade inalterada — o bump ocorre somente no lockfile.
- `FR-002` `Cargo.lock:2748` registra `rustls` `0.23.46` em vez de `0.23.43`, sem resíduo da versão anterior e sem tocar outras entradas.
- `FR-003` a isenção `[exemptions.rustls]` em `supply-chain/config.toml:1115-1117` passa a `version = "0.23.46"`, preservando `criteria = "safe-to-deploy"`.
- `FR-004` gate autoritativo (`scripts/ci/check_gate_integrity.py`) segue PASS com `all 10 db contract floors intact (58 total)` e `cargo test --workspace` fecha com `exit=0`, 0 falhas.
- `FR-005` dependency-review limpo e diff de código limitado a `Cargo.lock` + `supply-chain/config.toml`; manifest, workflow, ADRs, constituição, PR/merge/produção e DBs de produção intocados.
- `T001` âncora confirmada: `.specify/feature.json` = `specs/090-rustls-security-bump`, branch @ `824ba00e9`; `T002` diagnóstico da divergência entre `Cargo.lock:2748` (`0.23.43`) e o alvo `0.23.46`, com isenção já atualizada; `T003` bump do lockfile mantendo `Cargo.toml:189` em `"0.23"`; `T004` ajuste da isenção cargo-vet para `0.23.46` (`safe-to-deploy`); `T005` verificação do gate PASS, `cargo test --workspace` e dependency-review limpo (FR-004/FR-005); `T006` validação da fronteira do diff e fechamento dos artefatos 090 (SC-001/FR-005).

## Evidência

`.specify/validations/090-rustls-security-bump/validate.md` (R4 **APROVADO**, saídas literais esperadas/consolidadas do handoff) · `specs/090-rustls-security-bump/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=6/6`) · `specs/090-rustls-security-bump/converge.md` (**Converged**, sem novas tarefas) · baseline `824ba00e9` · diff do ciclo: `Cargo.lock` + `supply-chain/config.toml`, 2 arquivos. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar `Cargo.lock` e a isenção anteriores, sem force-push e sem reescrita de histórico:

1. em `Cargo.lock`, restaurar a entrada de `rustls` para `version = "0.23.43"` (se as alterações ainda estiverem não commitadas, `git checkout -- Cargo.lock`; se commitadas, novo commit de reversão ou `git revert` do commit do ciclo);
2. em `supply-chain/config.toml:1115-1117`, restaurar `[[exemptions.rustls]]` para `version = "0.23.43"`, mantendo `criteria = "safe-to-deploy"`;
3. manter `Cargo.toml:189` (`rustls = "0.23"`), workflow, gate, contratos em `flora-tests/`, ADRs e constituição intactos — nenhum deles foi alterado pelo ciclo;
4. após a reversão, repetir `python3 scripts/ci/check_gate_integrity.py`, `cargo test --workspace` e dependency-review: o esperado é gate **PASS** `all 10 db contract floors intact (58 total)`, testes sem falhas e dependency-review limpo também na versão `0.23.43`.

Não há arquivos novos de código, migrations, testes, config de produção nem artefatos de runtime para remover — apenas os documentos do ciclo 090 (`spec.md`, `analyze.md`, `converge.md`, este release e o R4), que são registros e não afetam o sistema. **Nenhum banco de produção é tocado, alterado ou removido.**

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS não substitui a CI remota.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **DBs de produção**: nenhum acesso, escrita ou remoção.
- **Escala**: mudança em 2 arquivos (lockfile + isenção cargo-vet); sem teste de carga, UI ou comportamento de runtime a verificar além da suíte do workspace.
