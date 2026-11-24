# Briefing de Compreensão — 120 / #411..#413

**Data**: 2026-11-24 · **Domínio crítico**: auth / rbac / hitl
**Issues**: [#411](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/411)–[#413](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/413) · **Autor**: orquestrador

## Intenção

Expor a emissão autenticada e autorizada de recibos HITL assinados via `POST /api/v1/hitl/receipts` (`flora-api/src/routes/hitl.rs`), conectando o protocolo HTTP à porta `HitlReceiptPort` (`HitlReceiptAdapter`) já implementada no ciclo 106, garantindo derivação exclusiva de identidade/organização pelo servidor e falha segura sanitizada (`503`) quando o segredo HMAC não estiver configurado.

## Invariantes

- **Antes**:
  - `HitlReceiptAdapter` existia em `flora-api/src/adapters/hitl_receipt.rs` e `AppState::hitl_receipt_adapter()` em `flora-api/src/state/app_state.rs`, porém sem rota HTTP montada em `flora-api/src/routes/mod.rs` (`BF-03` aberto).
  - `Error::ServiceUnavailable(_)` em `flora-core/src/error.rs` retornava HTTP `500 INTERNAL_SERVER_ERROR` com `self.to_string()`, podendo expor detalhes internos de indisponibilidade.
- **Depois**:
  - **Identidade e tenant exclusivamente no servidor**: `require_hitl_operator` resolve `auth.require_org()`, exige membership ativa e `Permission::OrgAdmin` (`can_emit_hitl_receipt`), rejeitando não-autenticados (`401`), sem contexto org (`400`) e não-admins (`403`).
  - **Anti-spoofing de `operator_id`**: `EmitHitlReceiptRequest` usa `#[serde(deny_unknown_fields)]` e não aceita `operator_id` nem `organization_id`; `build_decision` preenche `operator_id = auth.user_id.to_string()` antes de validar e assinar.
  - **Fail-closed ordenado (`422` antes de `503`)**: `validate_decision(&decision)?` executa antes da emissão para rejeitar decisões inválidas com `422 Unprocessable Entity`, e qualquer `Error::Configuration(_)` na ausência de `FLORA_HITL__HMAC_SECRET` é convertido em `Error::ServiceUnavailable(...)` → HTTP `503` com corpo fixo sanitizado (`{"error":"service_unavailable","message":"Service temporarily unavailable"}`), jamais logando `hmac_secret`, `signature_hmac` ou `artifacts_hash`.

## Decisões implícitas

1. **Exigir `Permission::OrgAdmin` (incluindo `*`) em `can_emit_hitl_receipt`**: emissão de recibo Tier 4 é ato administrativo de organização sem permissão granular dedicada no banco legado; exigir `OrgAdmin` evita escalação por papéis operacionais comuns (`TaskWrite`, `ChannelWrite`).
2. **Validar `EmitHitlReceiptRequest` com `#[serde(deny_unknown_fields)]`**: qualquer cliente que tente forjar `operator_id`, `organization_id` ou campos extras recebe `422` no extrator JSON em vez de serem silenciosamente ignorados.
3. **Sem alteração de `Cargo.lock` ou schema SQL**: todas as dependências (`hmac`, `sha2`, `hex`, `serde`) e tabelas de sessão/membership/RBAC já existem no workspace.

## Comando de verificação

```bash
export CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target
export FLORA_DATABASE__POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:15433/flora'
export DATABASE_URL="$FLORA_DATABASE__POSTGRES_URL"
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets -- -D warnings && \
cargo test --workspace && \
python3 scripts/ci/check_gate_integrity.py && \
cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1
```

## Pergunta-teste ao SE

**Pergunta**: Se um membro comum (`OrgRead` + `TaskWrite`), ou mesmo um `OrgAdmin` enviando `"operator_id": "outro-operador"` no corpo JSON, tentar emitir um recibo HITL, ou se um administrador enviar decisão válida quando `FLORA_HITL__HMAC_SECRET` estiver vazio em staging/local, algum recibo assinado ou segredo vaza?
**Resposta esperada**: Não. O membro comum recebe `403 Forbidden`; a tentativa de forjar `operator_id` é rejeitada com `422` por `deny_unknown_fields`; e a ausência do segredo falha fechado com `503` e payload fixo `"Service temporarily unavailable"` sem expor `FLORA_HITL__HMAC_SECRET`.

## [NV]

- Transporte ao vivo até instância externa da `MDMV-Autonoma` e deploy manual em staging/produção no Coolify permanecem fora deste ciclo.
