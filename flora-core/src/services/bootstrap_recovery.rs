//! Seleção administrativa e consumo one-shot do opt-in de recuperação `*`.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::User;

/// Lock distinto do `BOOTSTRAP_LOCK_KEY` usado em `POST /bootstrap`.
const WILDCARD_LOCK_KEY: i64 = 8_816_421_707;

/// Consulta canônica que restringe o wildcard a papéis administrativos.
pub const WILDCARD_ADMIN_SQL: &str = concat!(
    "SELECT u.id, u.email, u.oidc_subject, u.password_hash, ",
    "u.password_updated_at, u.display_name, u.avatar_url, u.profile, u.is_active, ",
    "u.created_at, u.updated_at ",
    "FROM users u ",
    "JOIN memberships m ON m.user_id = u.id ",
    "JOIN roles r ON r.id = m.role_id AND r.organization_id = m.organization_id ",
    "WHERE u.is_active = TRUE ",
    "AND m.organization_id <> '00000000-0000-0000-0000-000000000000'::uuid ",
    "AND r.name IN ('Admin', 'Owner') ",
    "ORDER BY u.created_at ASC, u.id ASC ",
    "LIMIT 1"
);
/// Seleciona o administrador mais antigo com papel `Admin`/`Owner`.
///
/// # Errors
///
/// Retorna erro de SQL quando a consulta falha.
pub async fn find_wildcard_admin(pool: &PgPool) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(WILDCARD_ADMIN_SQL)
        .fetch_optional(pool)
        .await
        .map_err(Error::from_sqlx)
}

/// Indica se o opt-in já foi consumido (persistente entre restarts).
///
/// # Errors
///
/// Retorna erro de SQL quando a leitura falha.
pub async fn wildcard_consumed(pool: &PgPool) -> Result<bool> {
    sqlx::query_scalar::<_, bool>(
        "SELECT consumed_at IS NOT NULL FROM bootstrap_wildcard_state WHERE singleton",
    )
    .fetch_one(pool)
    .await
    .map_err(Error::from_sqlx)
}

/// Consome o opt-in atomicamente; `true` apenas na primeira chamada.
///
/// # Errors
///
/// Retorna erro de SQL quando a transação falha.
pub async fn try_consume_wildcard(pool: &PgPool, user_id: Uuid) -> Result<bool> {
    let mut tx = pool.begin().await.map_err(Error::from_sqlx)?;
    let _lock = sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(WILDCARD_LOCK_KEY)
        .execute(&mut *tx)
        .await
        .map_err(Error::from_sqlx)?;
    let consumed = sqlx::query_scalar::<_, Uuid>(
        "UPDATE bootstrap_wildcard_state
            SET consumed_at = NOW(), consumed_user_id = $1
          WHERE singleton AND consumed_at IS NULL
         RETURNING consumed_user_id",
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(Error::from_sqlx)?;
    tx.commit().await.map_err(Error::from_sqlx)?;
    Ok(consumed.is_some())
}

#[cfg(test)]
mod tests {
    use super::WILDCARD_ADMIN_SQL;

    #[test]
    fn test_wildcard_sql_restricts_to_admin_roles_and_non_system_orgs() {
        assert!(WILDCARD_ADMIN_SQL.contains("r.name IN ('Admin', 'Owner')"));
        assert!(WILDCARD_ADMIN_SQL.contains("m.organization_id <>"));
        assert!(WILDCARD_ADMIN_SQL.contains("ORDER BY u.created_at ASC, u.id ASC"));
    }
}
