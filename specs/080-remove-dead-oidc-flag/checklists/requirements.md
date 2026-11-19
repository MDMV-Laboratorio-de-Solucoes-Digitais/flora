# Requirements Quality Checklist — 080

**Feature**: [spec.md](../spec.md) · **Slug**: 080-remove-dead-oidc-flag
**Created**: 2026-11-19T21:35Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T21:35Z, contra `define.md`/`decision.md` (acessados 2026-11-19T21:35Z). A execução dos gates e a compatibilidade com consumidores externos permanecem `[NV]`.

## Completude

- [x] FR-001 especifica a remoção exata da doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e da linha `pub local_fallback_enabled: bool,` em `flora-core/src/config/oidc.rs:18-19`, deixando `code_challenge_method` seguido diretamente de `allowed_redirect_uris` (`:20-23`, com seu `#[serde(default)]` intacto).
- [x] FR-002 especifica a remoção de `local_fallback_enabled: true,` do `impl Default for OidcConfig` (`:40`), mantendo as demais 5 inicializações (`:29-41`) e o `Ok(())` final de `validate()` (`:73`) inalterados.
- [x] FR-003 proíbe `deny_unknown_fields`, `#[serde(default)]`, alias de env ou qualquer substituto: `Config` (`flora-core/src/config/mod.rs:17-31`) permanece sem `deny_unknown_fields` e `Config::load` (`:43-49`) continua ignorando a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` sem erro de startup.
- [x] FR-004 limita o diff rastreado a `flora-core/src/config/oidc.rs` (3 linhas removidas), além dos 5 artefatos `.specify/assessments/080-remove-dead-oidc-flag/`; nenhum outro arquivo de código, config, doc, gate, CI ou lockfile é modificado.

## Clareza e consistência

- [x] SC-001 é verificável: `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` retorna **zero** correspondências após as duas remoções; as únicas menções restantes no repo são 3 docs de research históricos e a cópia em worktree `state-closure-072`, fora de escopo.
- [x] SC-002 é verificável: `cargo check -p flora-core` e `cargo clippy -p flora-core --all-targets -D warnings` terminam sem erro nem novo warning; `cargo fmt --check` limpo (a edição é só remoção de linhas, sem reindentação). `[NV]` — não executado nesta avaliação.
- [x] SC-003 é verificável: `python3 scripts/ci/check_gate_integrity.py` passa e o gate do alvo segue PASS; `Config::load` aceita ambiente com a chave legada definida (compat por ausência de `deny_unknown_fields`). `[NV]` — depende da execução pós-edição.
- [x] SC-004 é verificável: `git diff --stat` mostra somente `flora-core/src/config/oidc.rs` com 3 linhas a menos e os 5 artefatos 080; zero mutação de `.env.example`, auth, docs, worktree, gate, CI, DB ou segredos; base `e3944e962` preservado em todo o resto.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7); a rastreabilidade por T001..T004 em `plan.md` e `tasks.md` usa cada ID nominalmente, com SC-001 em T001 e T002 (o `grep` só zera após ambas as remoções).
- [x] Evidência determinística (código colado em `flora-core/src/config/oidc.rs:18-19,40`, zero leitores em Rust via `grep`, `config/mod.rs:17-31,43-49` sem `deny_unknown_fields`, `.env.example:58-68` sem a chave, research G9/NV-7) distinguida de `[NV]` para gates pós-edição e consumidores externos de `OidcConfig`.

## Edge cases

- [x] E se as 3 linhas já estiverem ausentes? FR-001/FR-002 são idempotentes: nada a remover e SC-001 já satisfeito.
- [x] E se existir leitor da flag em Rust? O `grep` pré-edition retornou zero; se aparecesse, a remoção quebraria a compilação — SC-002 detecta; criar shim está fora de escopo (FR-003).
- [x] E se algum deploy definir `FLORA_OIDC__LOCAL_FALLBACK_ENABLED`? Sem `deny_unknown_fields`, a chave é ignorada com segurança (FR-003/SC-003); consumidores externos que lessem o campo via `Serialize` ficam `[NV]` — nenhum consumidor de serialização de config foi evidenciado.
- [x] E se `allowed_redirect_uris` ou `validate()` forem alterados por acidente? FR-001/FR-002 exigem byte-idênticos; SC-004 detecta qualquer diff além das 3 linhas.
- [x] E se houver risco de DB/rede/segredos/`.tokens/`? Nenhum comando de DB é executado; a fronteira de paths permanece explícita em FR-004/SC-004.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` (2026-11-19T21:32Z) antes da escrita dos artefatos, com E=(3×5)/(1×1)=15,0 e opção A (deletar 3 linhas de `oidc.rs`) eleita.
- [x] R3 zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 079→080 documentada via `--dry-run`, sem bloqueio); os `[NV]` não são perguntas bloqueantes.
- [x] R4 e R7 respeitados na rastreabilidade: FR/SC individuais em `spec.md`, `plan.md` e `tasks.md`, sem intervalos, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: auth (`auth.rs`, `services/auth_service.rs`), `.env.example`, docs de research, worktree `state-closure-072`, `config/mod.rs` e demais módulos, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/080-*` (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.
