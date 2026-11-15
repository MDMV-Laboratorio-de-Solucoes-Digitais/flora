//! Server-side transient storage for OIDC auth transactions (PKCE/state).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use flora_core::error::{Error, Result};

/// TTL for OIDC auth transactions (state/verifier/redirect).
const TRANSACTION_TTL_SECS: u64 = 600;
/// Redis key prefix for OIDC auth transactions.
const KEY_PREFIX: &str = "flora:oidc:tx:";

/// Persisted data for a single OIDC authorization transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcTransaction {
    /// PKCE code verifier associated with the authorization request.
    pub code_verifier: String,
    /// Redirect URI used for the authorization request and token exchange.
    pub redirect_uri: String,
}

/// In-memory fallback entry with expiration.
#[derive(Debug, Clone)]
struct MemoryEntry {
    transaction: OidcTransaction,
    expires_at: Instant,
}

/// Transient store for OIDC PKCE/state transactions.
///
/// Prefers Valkey/Redis when available for shared/atomic storage, and falls
/// back to an in-memory store for single-node or test environments.
#[derive(Clone, Default)]
pub struct OidcTransactionStore {
    redis: Option<ConnectionManager>,
    memory: Arc<RwLock<HashMap<String, MemoryEntry>>>,
}

impl std::fmt::Debug for OidcTransactionStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OidcTransactionStore")
            .field("has_redis", &self.redis.is_some())
            .finish_non_exhaustive()
    }
}

impl OidcTransactionStore {
    /// Creates a store with an optional Valkey/Redis connection manager.
    #[must_use]
    pub fn new(redis: Option<ConnectionManager>) -> Self {
        Self {
            redis,
            memory: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Persists a transaction for `state` with a fixed TTL.
    ///
    /// # Errors
    ///
    /// Returns an error if the backing store cannot persist the transaction.
    pub async fn store(&self, state: &str, transaction: OidcTransaction) -> Result<()> {
        if let Some(conn) = &self.redis {
            let mut conn = conn.clone();
            let key = format!("{KEY_PREFIX}{state}");
            let payload = serde_json::to_string(&transaction).map_err(|e| {
                Error::Internal(format!("failed to serialize oidc transaction: {e}"))
            })?;
            let result: redis::RedisResult<()> = redis::cmd("SET")
                .arg(&key)
                .arg(payload)
                .arg("EX")
                .arg(TRANSACTION_TTL_SECS)
                .arg("NX")
                .query_async(&mut conn)
                .await;
            result.map_err(|e| {
                Error::ServiceUnavailable(format!("oidc state store unavailable: {e}"))
            })?;
            return Ok(());
        }

        let mut map = self.memory.write().await;
        // Opportunistic cleanup of expired entries.
        let now = Instant::now();
        map.retain(|_, entry| entry.expires_at > now);
        let _previous = map.insert(
            state.to_owned(),
            MemoryEntry {
                transaction,
                expires_at: now + Duration::from_secs(TRANSACTION_TTL_SECS),
            },
        );
        drop(map);
        Ok(())
    }

    /// Consumes and returns the transaction for `state`.
    ///
    /// Returns `Ok(None)` when the state is missing, expired, or already used.
    ///
    /// # Errors
    ///
    /// Returns an error if the backing store cannot be accessed.
    pub async fn consume(&self, state: &str) -> Result<Option<OidcTransaction>> {
        if let Some(conn) = &self.redis {
            let mut conn = conn.clone();
            let key = format!("{KEY_PREFIX}{state}");
            let value: redis::RedisResult<Option<String>> =
                redis::cmd("GETDEL").arg(&key).query_async(&mut conn).await;
            let value = value.map_err(|e| {
                Error::ServiceUnavailable(format!("oidc state store unavailable: {e}"))
            })?;
            let Some(payload) = value else {
                return Ok(None);
            };
            let transaction = serde_json::from_str(&payload).map_err(|e| {
                Error::Internal(format!("failed to deserialize oidc transaction: {e}"))
            })?;
            return Ok(Some(transaction));
        }

        let mut map = self.memory.write().await;
        let now = Instant::now();
        let entry = map.remove(state);
        drop(map);
        let Some(entry) = entry else {
            return Ok(None);
        };
        if entry.expires_at <= now {
            return Ok(None);
        }
        Ok(Some(entry.transaction))
    }
}
