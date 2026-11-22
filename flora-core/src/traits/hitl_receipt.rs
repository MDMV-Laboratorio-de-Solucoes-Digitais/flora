//! Porta pura para emissão de recibos HITL verificáveis pela Autônoma.

use crate::error::Result;
use crate::models::{HitlDecision, HitlReceipt};

/// Emite o envelope `VerificationReceipt` de oito campos a partir de uma decisão humana.
pub trait HitlReceiptPort: Send + Sync + std::fmt::Debug {
    /// Valida a decisão e assina o payload canônico.
    ///
    /// # Errors
    ///
    /// Retorna erro quando a decisão viola uma trava ou o segredo está ausente.
    fn emit(&self, decision: &HitlDecision) -> Result<HitlReceipt>;
}
