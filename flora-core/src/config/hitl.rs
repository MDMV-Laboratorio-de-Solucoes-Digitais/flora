//! Configuração de assinatura de recibos HITL (carregada de `FLORA_HITL__*`).

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Configuração do emissor de recibos; feature desligada quando o segredo é vazio.
#[derive(Clone, Deserialize, Serialize, Default)]
pub struct HitlConfig {
    /// Segredo HMAC compartilhado com a Autônoma; nunca serializado ou logado.
    #[serde(default, skip_serializing)]
    pub hmac_secret: String,
}

impl std::fmt::Debug for HitlConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HitlConfig")
            .field("hmac_secret", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl HitlConfig {
    /// Valida o segredo quando presente; vazio desliga a feature (fail-closed na emissão).
    ///
    /// # Errors
    ///
    /// Retorna erro de configuração quando o segredo fornecido é curto.
    pub fn validate(&self) -> Result<()> {
        if self.hmac_secret.is_empty() {
            return Ok(());
        }
        if self.hmac_secret.len() < 32 {
            return Err(Error::Configuration(
                "FLORA_HITL__HMAC_SECRET must be at least 32 characters".to_owned(),
            ));
        }
        Ok(())
    }
}
