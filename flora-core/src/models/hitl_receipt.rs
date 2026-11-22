//! Envelope e regras puras do recibo HITL consumido pela MDMV-Autônoma.
//!
//! O contrato é bilateral: a Autônoma valida o envelope em
//! `src/contracts/flora_bridge.py`; aqui apenas se produz e valida o mesmo
//! payload de forma determinística, sem I/O e sem segredo.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Estações válidas no envelope (caixa alta, como no contrato Python).
pub const VALID_STATIONS: &[&str] = &[
    "S0_RADAR",
    "S1_INTENT",
    "S2_ASSEMBLY",
    "S3_VERIFICATION",
    "S4_MONETIZATION",
    "S5_DISTRIBUTION",
    "S6_TELEMETRY",
];

/// Estágios de retomada válidos em `metrics_payload` (caixa baixa).
pub const ALLOWED_RESUME_STAGES: &[&str] = &[
    "s0_radar",
    "s1_intent",
    "s2_assembly",
    "s3_verification",
    "s4_monetization",
    "s5_distribution",
    "s6_telemetry",
];

/// Decisão humana de Tier 4, sem segredo e sem efeito colateral.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HitlDecision {
    /// Identificador da ordem de trabalho recebida da Autônoma.
    pub work_order_id: String,
    /// Ativo/orachoamento associado.
    pub ativo_id: String,
    /// Estação que originou o escalonamento.
    pub station: String,
    /// Hash dos artefatos avaliados pelo operador.
    pub artifacts_hash: String,
    /// Veredito explícito do operador.
    pub operator_approved: bool,
    /// Identificador do operador autenticado.
    pub operator_id: String,
    /// Confirmação de redação de dados pessoais (LGPD).
    pub pii_redacted: bool,
    /// Nenhum envio externo real foi autorizado por esta decisão.
    pub real_external_send: bool,
    /// Estágio de retomada, sempre explícito e em caixa baixa.
    pub next_stage: String,
}

/// Recibo assinado com os oito campos exigidos pelo consumidor.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HitlReceipt {
    /// Ordem de trabalho correspondente.
    pub work_order_id: String,
    /// Ativo correspondente.
    pub ativo_id: String,
    /// Estação (caixa alta).
    pub station: String,
    /// Produtor literal: `flora`.
    pub producer_system: String,
    /// Status literal: `verified`.
    pub status: String,
    /// Hash dos artefatos.
    pub artifacts_hash: String,
    /// Métricas validadas pelo consumidor.
    pub metrics_payload: serde_json::Value,
    /// HMAC-SHA256 em hexadecimal minúsculo dos sete campos acima.
    pub signature_hmac: String,
}

fn invalid(field: &'static str, message: &str) -> Error {
    Error::Validation {
        field: field.to_owned(),
        message: message.to_owned(),
    }
}

/// Valida a decisão antes de qualquer assinatura (fail-closed).
///
/// # Errors
///
/// Retorna erro de validação para campo vazio, estação/estágio fora da
/// allowlist ou qualquer uma das quatro travas de privacidade/autorização.
pub fn validate_decision(decision: &HitlDecision) -> Result<()> {
    for (field, value) in [
        ("work_order_id", decision.work_order_id.as_str()),
        ("ativo_id", decision.ativo_id.as_str()),
        ("artifacts_hash", decision.artifacts_hash.as_str()),
        ("operator_id", decision.operator_id.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(invalid(field, "must be a non-empty string"));
        }
    }
    if !VALID_STATIONS.contains(&decision.station.as_str()) {
        return Err(invalid("station", "station is not in the allowlist"));
    }

    if !ALLOWED_RESUME_STAGES.contains(&decision.next_stage.as_str()) {
        return Err(invalid("next_stage", "next_stage is not in the allowlist"));
    }
    if !decision.pii_redacted {
        return Err(invalid("pii_redacted", "unredacted PII is rejected"));
    }
    if decision.real_external_send {
        return Err(invalid(
            "real_external_send",
            "external send requires separate approval",
        ));
    }
    Ok(())
}

/// Serializa os sete campos não assinados em JSON canônico.
///
/// `serde_json::Map` sem `preserve_order` é um `BTreeMap`: chaves ordenadas,
/// separadores compactos e UTF-8 preservado, equivalentes a
/// `json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=False)`.
///
/// # Errors
///
/// Retorna erro de serialização se `metrics_payload` não for objeto.
pub fn unsigned_payload(decision: &HitlDecision) -> Result<String> {
    let metrics = metrics_payload(decision);
    let fields = [
        ("artifacts_hash", serde_json::json!(decision.artifacts_hash)),
        ("ativo_id", serde_json::json!(decision.ativo_id)),
        ("metrics_payload", metrics),
        ("producer_system", serde_json::json!("flora")),
        ("station", serde_json::json!(decision.station)),
        ("status", serde_json::json!("verified")),
        ("work_order_id", serde_json::json!(decision.work_order_id)),
    ];
    let map: serde_json::Map<String, serde_json::Value> = fields
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect();
    serde_json::to_string(&map).map_err(|err| invalid("metrics_payload", &err.to_string()))
}

/// Constrói o `metrics_payload` validado pelo consumidor Python.
#[must_use]
pub fn metrics_payload(decision: &HitlDecision) -> serde_json::Value {
    serde_json::json!({
        "next_stage": decision.next_stage,
        "operator_approved": decision.operator_approved,
        "operator_id": decision.operator_id,
        "pii_redacted": decision.pii_redacted,
        "real_external_send": decision.real_external_send,
    })
}
