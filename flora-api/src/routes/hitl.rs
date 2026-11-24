//! Human-in-the-loop (HITL) Tier 4 signed receipt HTTP endpoint (`BF-03` / cycle 120).

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use uuid::Uuid;

use crate::extractors::auth::AuthExtractor;
use crate::state::AppState;
use flora_core::error::{Error, Result};
use flora_core::models::{HitlDecision, HitlReceipt, Permission, Role, validate_decision};
use flora_core::repositories::{PgMembershipRepository, PgRoleRepository};
use flora_core::traits::{HitlReceiptPort, MembershipRepository, RoleRepository};

/// Request body to emit a signed Tier 4 HITL receipt.
///
/// `operator_id` and `organization_id` are never accepted from the client;
/// both come exclusively from the verified session (`AuthExtractor`).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmitHitlReceiptRequest {
    /// Work order identifier received from the upstream orchestrator.
    pub work_order_id: String,
    /// Asset identifier associated with the decision.
    pub ativo_id: String,
    /// Uppercase station identifier (e.g. `S3_VERIFICATION`).
    pub station: String,
    /// Digest of the artifacts reviewed by the operator.
    pub artifacts_hash: String,
    /// Explicit human operator verdict.
    pub operator_approved: bool,
    /// Confirmation that personal data was redacted (LGPD).
    pub pii_redacted: bool,
    /// Confirmation that no external send was authorized.
    pub real_external_send: bool,
    /// Lowercase resume stage allowlisted for downstream ingestion.
    pub next_stage: String,
}

/// Creates the HITL router mounted at `/api/v1/hitl`.
pub fn create_hitl_router() -> Router<AppState> {
    Router::new().route("/receipts", post(emit_hitl_receipt))
}

/// Pure authorization guard for HITL receipt emission (`OrgAdmin` or `*`).
fn can_emit_hitl_receipt(role: &Role) -> bool {
    role.has_permission(Permission::OrgAdmin)
}

/// Binds the client payload to the server-authenticated `operator_id`.
fn build_decision(req: EmitHitlReceiptRequest, operator_id: Uuid) -> HitlDecision {
    HitlDecision {
        work_order_id: req.work_order_id,
        ativo_id: req.ativo_id,
        station: req.station,
        artifacts_hash: req.artifacts_hash,
        operator_approved: req.operator_approved,
        operator_id: operator_id.to_string(),
        pii_redacted: req.pii_redacted,
        real_external_send: req.real_external_send,
        next_stage: req.next_stage,
    }
}

async fn require_hitl_operator(state: &AppState, auth: &AuthExtractor) -> Result<Uuid> {
    let org_id = auth
        .require_org()
        .map_err(|_| Error::OrganizationContextRequired)?;
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(auth.user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_owned()))?;
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let role = role_repo
        .find_by_id_and_organization(membership.role_id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !can_emit_hitl_receipt(&role) {
        return Err(Error::Forbidden("Insufficient permissions".to_owned()));
    }
    Ok(org_id)
}

/// `POST /api/v1/hitl/receipts` — Validate and sign a Tier 4 HITL receipt.
async fn emit_hitl_receipt(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<EmitHitlReceiptRequest>,
) -> Result<(StatusCode, Json<HitlReceipt>)> {
    let _org_id = require_hitl_operator(&state, &auth).await?;
    let decision = build_decision(req, auth.user_id);
    validate_decision(&decision)?;
    let receipt = match state.hitl_receipt_adapter().emit(&decision) {
        Ok(receipt) => receipt,
        Err(Error::Configuration(_)) => {
            return Err(Error::ServiceUnavailable(
                "HITL receipt signing is not configured".to_owned(),
            ));
        }
        Err(err) => return Err(err),
    };
    tracing::info!(
        work_order_id = %receipt.work_order_id,
        station = %receipt.station,
        operator_id = %auth.user_id,
        operator_approved = decision.operator_approved,
        "Emitted signed HITL receipt"
    );
    Ok((StatusCode::CREATED, Json(receipt)))
}

#[cfg(test)]
#[expect(
    clippy::expect_used,
    reason = "unit test assertions for HITL request binding and guards"
)]
mod tests {
    use flora_core::models::{Permission, Role};
    use flora_core::traits::HitlReceiptPort;
    use uuid::Uuid;

    use crate::adapters::HitlReceiptAdapter;

    use super::{EmitHitlReceiptRequest, build_decision, can_emit_hitl_receipt};

    fn role_with(permissions: &[&str]) -> Role {
        let mut role = Role::new(Uuid::now_v7(), "HitlTestRole");
        role.permissions = serde_json::json!(permissions);
        role
    }

    #[test]
    fn can_emit_hitl_receipt_requires_org_admin_or_wildcard() {
        assert!(can_emit_hitl_receipt(&role_with(&["org:admin"])));
        assert!(can_emit_hitl_receipt(&role_with(&["*"])));
        assert!(!can_emit_hitl_receipt(&role_with(&["org:read"])));
        assert!(!can_emit_hitl_receipt(&role_with(&[
            "task:write",
            "role:write"
        ])));
        assert!(!can_emit_hitl_receipt(&role_with(&[])));
        assert!(!role_with(&["org:read"]).has_permission(Permission::OrgAdmin));
    }

    #[test]
    fn emit_request_rejects_client_supplied_operator_id() {
        let payload = serde_json::json!({
            "work_order_id": "wo-1",
            "ativo_id": "ativo-1",
            "station": "S3_VERIFICATION",
            "artifacts_hash": "a".repeat(64),
            "operator_approved": true,
            "operator_id": "forged-operator",
            "pii_redacted": true,
            "real_external_send": false,
            "next_stage": "s4_monetization"
        });
        let parsed = serde_json::from_value::<EmitHitlReceiptRequest>(payload);
        assert!(
            parsed.is_err(),
            "client-supplied operator_id must be rejected"
        );
    }

    #[test]
    fn build_decision_binds_authenticated_operator_id_and_emits_canonical_signature() {
        let operator_id = Uuid::now_v7();
        let payload = serde_json::json!({
            "work_order_id": "wo-1",
            "ativo_id": "ativo-1",
            "station": "S3_VERIFICATION",
            "artifacts_hash": "a".repeat(64),
            "operator_approved": true,
            "pii_redacted": true,
            "real_external_send": false,
            "next_stage": "s4_monetization"
        });
        let req: EmitHitlReceiptRequest = serde_json::from_value(payload).expect("valid request");
        let decision = build_decision(req, operator_id);
        assert_eq!(decision.operator_id, operator_id.to_string());

        let receipt = HitlReceiptAdapter::new("d15-vector-only-secret-0123456789abcdef")
            .emit(&decision)
            .expect("signed receipt");
        assert_eq!(receipt.producer_system, "flora");
        assert_eq!(receipt.status, "verified");
        assert_eq!(
            receipt.metrics_payload["operator_id"],
            operator_id.to_string()
        );
    }
}
