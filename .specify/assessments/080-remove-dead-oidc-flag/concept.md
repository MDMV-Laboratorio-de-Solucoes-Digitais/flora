# Shape: remover flag OIDC morta `local_fallback_enabled`

- **Slug**: 080-remove-dead-oidc-flag · **Recomendado**: Opção A (remoção direta das 3 linhas) · **Data**: 2026-11-19T21:32Z

## Opção A — remover doc+campo e a inicialização (recomendada)

- **Sketch**: em `flora-core/src/config/oidc.rs`, deletar `:18-19` (doc + `pub local_fallback_enabled: bool,`) e `:40` (`local_fallback_enabled: true,`). Appetite: trivial — 3 linhas, 1 arquivo.
- **Trade-offs**: elimina a falsa garantia de fallback (G9) sem mudar comportamento, pois a flag nunca foi lida (`validate()` em `:52-74` não a consulta; único uso do struct em `mod.rs:24,59`). Compat de config preservada: sem `deny_unknown_fields` (`mod.rs:17-31`), `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` legado é ignorado pelo Figment (`mod.rs:43-49`) sem erro de startup. `[NV]` gates pós-edição não executados nesta avaliação.
- **Rabbit holes**: adicionar `#[serde(default)]` "para não quebrar" (desnecessário e contradiz a remoção); editar `auth.rs`/`auth_service.rs` (a flag nunca cruzou essa fronteira); tocar `.env.example` (não documenta a chave); renomear em vez de remover (mantém dívida); editar docs de research (são registro histórico de G9/NV-7).

## Opção B — manter o campo e implementar o fallback

- **Sketch**: dar uso real à flag em `validate()`/auth (fail-open quando Zitadel indisponível).
- **Trade-offs**: é exatamente o caminho que G9 classifica como risco de segurança (fallback local com issuer HTTP, segredo vazio); exige decisão de produto/ADR e toca código de auth, fora do escopo 80/20 do ciclo. Rejeitada.

## Opção C — manter a flag morta

- **Sketch**: não editar; aceitar o default `true` enganoso.
- **Trade-offs**: zero diff, mas preserva a dívida e o sinal de segurança inexistente que motivou G9/NV-7. É o estado que a iniciativa 080 existe para corrigir. Rejeitada.

## Fora de escopo

Código de auth (`auth.rs`, `flora-core/src/services/auth_service.rs`), `.env.example`, `docs/research/*` (inclui os 3 arquivos que citam a flag), `.specify/queue/worktrees/state-closure-072/*`, `flora-core/src/config/mod.rs` e todos os demais módulos de config, gate e `scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/080-* (exceto os 5 artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.

## Premissas

A flag é write-only (zero leitores em Rust, varredura 2026-11-19); `Config` não usa `deny_unknown_fields`, logo a remoção não quebra parsing de env legado; `Config::load` usa somente `Serialized::defaults` + `Env` (sem arquivo YAML presente), então não há arquivo de config a migrar; os docs de research são dados, não alvo de edição; FR-001..FR-004 e SC-001..SC-004 de `define.md` regem a implementação. `[NV]` todos os comandos de verificação dependem da edição e não foram executados nesta avaliação somente leitura.
