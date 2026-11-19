# Shape: limpeza das chaves OIDC em `.env.example`

- **Slug**: 081-env-oidc-cleanup · **Recomendado**: Opção A (substituir `:62-63`, remover `:65`, manter o resto) · **Data**: 2026-11-19T22:06Z

## Opção A — documentar a chave real e remover o ruído (recomendada)

- **Sketch**: em `.env.example`, trocar as linhas `:62-63` por um comentário curto apontando que o callback vem de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`) e que URIs extras usam `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`, allowlist em `auth.rs:105-117`), com o exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]`; deletar `:65` (`# OIDC_ISSUER_URL`); alinhar `:64` para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`". Appetite: trivial — 2 linhas substituídas, 1 removida, 1 redação, 1 arquivo.
- **Trade-offs**: elimina a chave inerte `__REDIRECT_URI` e o alias fantasma `OIDC_ISSUER_URL` sem tocar comportamento — nenhum leitor Rust existe para eles (`oidc.rs:7-22`, `config/mod.rs:43-49`, `oidc.rs:49-71`). Preserva `:59-61` (leitores Figment) e `:66-68` (`std::env::var` em `oidc.rs:27,29,31`), que são reais. Compat de config intacta: sem `deny_unknown_fields` (`config/mod.rs:17-31`), um deploy que ainda exporta `FLORA_OIDC__REDIRECT_URI` continua subindo com a chave ignorada — exatamente o comportamento de hoje. `[NV]` gates pós-edição e consumidores externos do template não verificados nesta avaliação.
- **Rabbit holes**: descomentar o exemplo (array Figment inválido quebra startup); trocar aspas duplas por simples; duplicar o callback derivado de `base_url` na allowlist (o callback já é sempre aceito, `oidc.rs:19`); editar `oidc.rs`, `config/mod.rs` ou `auth.rs` (são citados, não mudam); apagar `ZITADEL_*` (são lidas por `std::env::var`); renomear `__REDIRECT_URI` em vez de removê-lo (mantém a dívida); tocar docs/specs históricos (são registro, não alvo); mexer no exemplo de CORS em `:31-39` (controle separado).

## Opção B — dar suporte real a `FLORA_OIDC__REDIRECT_URI` no Rust

- **Sketch**: adicionar campo `redirect_uri` a `OidcConfig` com `#[serde(default)]` e usá-lo como fallback em `auth.rs:102-104`.
- **Trade-offs**: muda contrato de config e código de auth, cria segunda fonte de callback ao lado de `base_url`, exige decisão de produto/ADR e testes de allowlist. Fora do escopo 80/20 do ciclo e do pedido (limpeza de template, não de runtime). Rejeitada.

## Opção C — manter o template como está

- **Sketch**: não editar; aceitar drift e a chave inerte.
- **Trade-offs**: zero diff, mas preserva exatamente a confusão registrada em `docs/research/2026-11-16-016-macro-files-ops-parity.md:32` e deixa 080 pela metade (flag removida no Rust, template ainda desatualizado). É o estado que 081 existe para corrigir. Rejeitada.

## Fora de escopo

`flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, demais linhas de `.env.example` (`:1-61,66-87`, exceto `:64` opcional), docs/specs de research (inclui `docs/research/2026-11-16-016-macro-files-ops-parity.md:32` e `docs/research/2026-11-16-030-cors-allowlist-plan.md:24,137`), `specs/001-flora-seed-v0-1/contracts/OIDC.md`, `.specify/queue/worktrees/state-closure-072/*`, gate e `scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/081-* (exceto os 5 artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.

## Premissas

`FLORA_OIDC__REDIRECT_URI` e `OIDC_ISSUER_URL` não têm leitores em Rust (varredura 2026-11-19: `oidc.rs:7-22,49-71`, `config/mod.rs:43-49`); `ZITADEL_*` e `FLORA_OIDC__{ISSUER_URL,CLIENT_ID,CLIENT_SECRET}` têm (respectivamente `oidc.rs:27,29,31` e `config/mod.rs:43-46` → `oidc.rs:9,11,13`); o callback é derivado de `FLORA_APP__BASE_URL` (`auth.rs:101`) e extras passam pela allowlist exata (`auth.rs:105-117` com `oidc.rs:20-21`); figment 0.10.19 (`Cargo.lock:941-942`) analisa arrays com aspas duplas, padrão já em `.env.example:35`; o template não é carregado em runtime (`.env.example:33-34`); FR-001..FR-004 e SC-001..SC-004 de `define.md` regem a implementação. `[NV]` todos os comandos de verificação dependem da edição e não foram executados nesta avaliação somente leitura.
