//! Adaptador que assina o envelope HITL com HMAC-SHA256 canônico.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use flora_core::error::{Error, Result};
use flora_core::models::{HitlDecision, HitlReceipt, metrics_payload, unsigned_payload};
use flora_core::traits::HitlReceiptPort;

/// Emissor determinístico do `VerificationReceipt` consumido pela Autônoma.
pub struct HitlReceiptAdapter {
    secret: String,
}

impl std::fmt::Debug for HitlReceiptAdapter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HitlReceiptAdapter")
            .field("secret", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl HitlReceiptAdapter {
    /// Cria o adaptador; o segredo vazio desliga a emissão (fail-closed).
    #[must_use]
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_owned(),
        }
    }
}

impl HitlReceiptPort for HitlReceiptAdapter {
    fn emit(&self, decision: &HitlDecision) -> Result<HitlReceipt> {
        flora_core::models::validate_decision(decision)?;
        if self.secret.is_empty() {
            return Err(Error::Configuration(
                "FLORA_HITL__HMAC_SECRET is required to emit receipts".to_owned(),
            ));
        }
        let payload = unsigned_payload(decision)?;
        let mut signer = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes()).map_err(|_| {
            Error::Configuration("invalid HMAC key length for HITL receipts".to_owned())
        })?;
        signer.update(payload.as_bytes());
        let signature = hex::encode(signer.finalize().into_bytes());

        Ok(HitlReceipt {
            work_order_id: decision.work_order_id.clone(),
            ativo_id: decision.ativo_id.clone(),
            station: decision.station.clone(),
            producer_system: "flora".to_owned(),
            status: "verified".to_owned(),
            artifacts_hash: decision.artifacts_hash.clone(),
            metrics_payload: metrics_payload(decision),
            signature_hmac: signature,
        })
    }
}

#[cfg(test)]
#[expect(
    clippy::expect_used,
    reason = "test fixtures: failures are assertion failures"
)]
mod tests {
    use flora_core::models::{HitlDecision, unsigned_payload};
    use flora_core::traits::HitlReceiptPort;

    use super::HitlReceiptAdapter;

    const SECRET: &str = "d15-vector-only-secret-0123456789abcdef";
    const SIGNATURE: &str = "ba563acce4ed017e817584be22f6a70fe40748b2ae16a57b01398dfd8bb80dbb";
    const CANONICAL: &str = "{\"artifacts_hash\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"ativo_id\":\"ativo-d15-001\",\"metrics_payload\":{\"next_stage\":\"s4_monetization\",\"operator_approved\":true,\"operator_id\":\"op-42\",\"pii_redacted\":true,\"real_external_send\":false},\"producer_system\":\"flora\",\"station\":\"S3_VERIFICATION\",\"status\":\"verified\",\"work_order_id\":\"wo-d15-001\"}";

    fn decision() -> HitlDecision {
        HitlDecision {
            work_order_id: "wo-d15-001".to_owned(),
            ativo_id: "ativo-d15-001".to_owned(),
            station: "S3_VERIFICATION".to_owned(),
            artifacts_hash: "a".repeat(64),
            operator_approved: true,
            operator_id: "op-42".to_owned(),
            pii_redacted: true,
            real_external_send: false,
            next_stage: "s4_monetization".to_owned(),
        }
    }

    #[test]
    fn test_canonical_payload_and_signature_match_python_vector() {
        let adapter = HitlReceiptAdapter::new(SECRET);
        let decision = decision();
        assert_eq!(unsigned_payload(&decision).expect("payload"), CANONICAL);
        let receipt = adapter.emit(&decision).expect("receipt");
        assert_eq!(receipt.signature_hmac, SIGNATURE);
        assert_eq!(receipt.producer_system, "flora");
        assert_eq!(receipt.status, "verified");
    }

    #[test]
    fn test_rejects_invalid_decisions_and_missing_secret() {
        let adapter = HitlReceiptAdapter::new(SECRET);
        let mut bad = decision();
        bad.next_stage = "S4_MONETIZATION".to_owned();
        assert!(adapter.emit(&bad).is_err());
        let mut bad = decision();
        bad.real_external_send = true;
        assert!(adapter.emit(&bad).is_err());
        let mut bad = decision();
        bad.pii_redacted = false;
        assert!(adapter.emit(&bad).is_err());
        assert!(HitlReceiptAdapter::new("").emit(&decision()).is_err());
    }
}
