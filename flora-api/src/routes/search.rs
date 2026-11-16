//! Global search API routes.

use axum::{
    Router,
    extract::{Query, State},
    response::Json,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::PgMembershipRepository;
use flora_core::traits::MembershipRepository;

use crate::AppState;
use crate::extractors::AuthExtractor;

/// Query parameters for search.
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// The search query string.
    pub q: String,
    /// Filter by result types.
    #[serde(rename = "type")]
    pub types: Option<String>,
    /// Maximum number of results to return.
    pub limit: Option<usize>,
}

/// A single search result.
#[derive(Debug, Serialize)]
pub struct SearchResult {
    /// The type of the result.
    #[serde(rename = "type")]
    pub result_type: String,
    /// The result ID.
    pub id: String,
    /// A snippet of the result content.
    pub snippet: String,
}

/// Response for a search query.
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    /// The original search query.
    pub query: String,
    /// The search results.
    pub results: Vec<SearchResult>,
}

/// Creates the search router.
pub fn create_search_router() -> Router<AppState> {
    Router::new().route("/", get(search))
}

/// `GET /api/v1/search` — Global search across messages, tasks, and files.
/// 019: PG FTS fallback for messages when Meilisearch is unavailable/disabled.
async fn search(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _ = membership_repo
        .find_by_user_and_organization(auth.user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    let q = params.q.trim().to_string();
    if q.is_empty() {
        return Ok(Json(SearchResponse {
            query: params.q,
            results: vec![],
        }));
    }

    let limit = params.limit.unwrap_or(20).min(50);
    let raw_types: Option<Vec<String>> = params.types.as_deref().map(|t| {
        t.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    });

    if let Some(search_service) = state.search_service.as_ref() {
        let results = search_service
            .search(&org_id, &q, raw_types.as_deref(), limit)
            .await?;
        let api_results = results
            .into_iter()
            .map(|r| SearchResult {
                result_type: r.item_type,
                id: r.id,
                snippet: r.snippet,
            })
            .collect();
        tracing::debug!(query = %q, org_id = %org_id, backend = "meilisearch", "Search executed");
        return Ok(Json(SearchResponse {
            query: params.q,
            results: api_results,
        }));
    }

    let normalized_types: Option<Vec<String>> = raw_types.as_ref().map(|ts| {
        ts.iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>()
    });
    let results =
        search_messages_pg(&state, org_id, &q, normalized_types.as_deref(), limit).await?;
    tracing::debug!(query = %q, org_id = %org_id, backend = "postgres", "Search executed");
    Ok(Json(SearchResponse {
        query: params.q,
        results,
    }))
}

async fn search_messages_pg(
    state: &AppState,
    org_id: Uuid,
    q: &str,
    types: Option<&[String]>,
    limit: usize,
) -> Result<Vec<SearchResult>> {
    let include_messages =
        types.is_none_or(|ts| ts.iter().any(|t| t == "message" || t == "messages"));
    if !include_messages {
        return Ok(vec![]);
    }

    let limit_i64 = i64::try_from(limit).unwrap_or(20);

    let rows = sqlx::query(
        r"
        WITH q AS (
            SELECT
                COALESCE(
                    websearch_to_tsquery('simple', $2),
                    plainto_tsquery('simple', $2)
                ) AS tsq
        )
        SELECT
            m.id,
            left(regexp_replace(m.content, '\s+', ' ', 'g'), 200) AS snippet,
            ts_rank_cd(m.search_tsv, q.tsq) AS rank
        FROM messages m, q
        WHERE m.organization_id = $1
          AND m.is_deleted = false
          AND q.tsq IS NOT NULL
          AND q.tsq <> ''::tsquery
          AND m.search_tsv @@ q.tsq
        ORDER BY rank DESC, m.created_at DESC, m.id DESC
        LIMIT $3
        ",
    )
    .bind(org_id)
    .bind(q)
    .bind(limit_i64)
    .fetch_all(&*state.db_pool)
    .await
    .map_err(|e| Error::Search(e.to_string()))?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let id: Uuid = row
            .try_get("id")
            .map_err(|e| Error::Search(e.to_string()))?;
        let snippet: Option<String> = row
            .try_get("snippet")
            .map_err(|e| Error::Search(e.to_string()))?;
        results.push(SearchResult {
            result_type: "message".to_string(),
            id: id.to_string(),
            snippet: snippet.unwrap_or_default(),
        });
    }
    Ok(results)
}
