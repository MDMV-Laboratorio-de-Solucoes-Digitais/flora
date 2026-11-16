//! Search API routes (019/020: Postgres FTS fallback for messages, tasks, files).

use axum::{
    Router,
    extract::{Query, State},
    response::Json,
    routing::get,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::PgMembershipRepository;
use flora_core::traits::MembershipRepository;

use crate::AppState;
use crate::extractors::AuthExtractor;

/// Search query parameters.
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// The search query string.
    pub q: String,
    /// Optional type filter (comma-separated: messages, tasks, files).
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Maximum number of results.
    pub limit: Option<u32>,
}

/// A single search result.
#[derive(Debug, Serialize)]
pub struct SearchResult {
    /// The type of result (message, task, file).
    #[serde(rename = "type")]
    pub result_type: String,
    /// The ID of the result.
    pub id: String,
    /// A snippet of the matched content.
    pub snippet: String,
}

/// Search response.
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    /// The original query.
    pub query: String,
    /// The search results.
    pub results: Vec<SearchResult>,
}

/// Creates the search router.
pub fn create_search_router() -> Router<AppState> {
    Router::new().route("/", get(search))
}

/// `GET /api/v1/search?q=...&type=...&limit=...` — Search messages, tasks, and files.
async fn search(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

    // Verify membership
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _ = membership_repo
        .find_by_user_and_organization(auth.user_id, org_id)
        .await?;

    let query = params.q.trim().to_string();
    if query.is_empty() {
        return Ok(Json(SearchResponse {
            query,
            results: vec![],
        }));
    }

    let limit = params.limit.unwrap_or(20).min(50);
    let types: Option<Vec<String>> = params.result_type.as_ref().map(|t| {
        t.split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    });

    if let Some(search_service) = &state.search_service {
        match search_service
            .search(&org_id, &query, types.as_deref(), limit as usize)
            .await
        {
            Ok(hits) => {
                let results = hits
                    .into_iter()
                    .map(|hit| SearchResult {
                        result_type: hit.item_type,
                        id: hit.id,
                        snippet: hit.snippet,
                    })
                    .collect();

                return Ok(Json(SearchResponse { query, results }));
            }
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    %org_id,
                    "Meilisearch query failed; falling back to Postgres FTS"
                );
            }
        }
    }

    // 019/020/022: Postgres FTS fallback (messages/tasks/files). Authoritative
    // correctness baseline and fail-open path when Meilisearch is unavailable/errors.
    let results = search_pg(&state, org_id, &query, types.as_deref(), limit).await?;
    Ok(Json(SearchResponse { query, results }))
}

#[derive(sqlx::FromRow)]
struct PgSearchRow {
    id: Uuid,
    snippet: Option<String>,
    rank: f32,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[expect(
    clippy::too_many_lines,
    reason = "020 PG fallback keeps per-entity query branches explicit and deterministic"
)]
async fn search_pg(
    state: &AppState,
    org_id: Uuid,
    query: &str,
    types: Option<&[String]>,
    limit: u32,
) -> Result<Vec<SearchResult>> {
    let limit_i64 = i64::from(limit);

    let include_messages = include_type(types, &["message", "messages"]);
    let include_tasks = include_type(types, &["task", "tasks"]);
    let include_files = include_type(types, &["file", "files"]);

    let mut merged: Vec<(SearchResult, f32, chrono::DateTime<chrono::Utc>, Uuid)> = Vec::new();

    if include_messages {
        let rows = sqlx::query_as::<_, PgSearchRow>(
            r"
            WITH q AS (
                SELECT COALESCE(
                    websearch_to_tsquery('simple', $2),
                    plainto_tsquery('simple', $2)
                ) AS tsq
            )
            SELECT
                m.id,
                left(regexp_replace(m.content, '\s+', ' ', 'g'), 200) AS snippet,
                ts_rank_cd(m.search_tsv, q.tsq)::real AS rank,
                m.created_at
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
        .bind(query)
        .bind(limit_i64)
        .fetch_all(state.db_pool.as_ref())
        .await?;

        for row in rows {
            merged.push((
                SearchResult {
                    result_type: "message".to_string(),
                    id: row.id.to_string(),
                    snippet: row.snippet.unwrap_or_default(),
                },
                row.rank,
                row.created_at,
                row.id,
            ));
        }
    }

    if include_tasks {
        let rows = sqlx::query_as::<_, PgSearchRow>(
            r"
            WITH q AS (
                SELECT COALESCE(
                    websearch_to_tsquery('simple', $2),
                    plainto_tsquery('simple', $2)
                ) AS tsq
            )
            SELECT
                t.id,
                left(
                    regexp_replace(
                        coalesce(t.title, '') || ' ' || coalesce(t.description, ''),
                        '\s+',
                        ' ',
                        'g'
                    ),
                    200
                ) AS snippet,
                ts_rank_cd(t.search_tsv, q.tsq)::real AS rank,
                t.created_at
            FROM tasks t, q
            WHERE t.organization_id = $1
              AND t.is_deleted = false
              AND q.tsq IS NOT NULL
              AND q.tsq <> ''::tsquery
              AND t.search_tsv @@ q.tsq
            ORDER BY rank DESC, t.created_at DESC, t.id DESC
            LIMIT $3
            ",
        )
        .bind(org_id)
        .bind(query)
        .bind(limit_i64)
        .fetch_all(state.db_pool.as_ref())
        .await?;

        for row in rows {
            merged.push((
                SearchResult {
                    result_type: "task".to_string(),
                    id: row.id.to_string(),
                    snippet: row.snippet.unwrap_or_default(),
                },
                row.rank,
                row.created_at,
                row.id,
            ));
        }
    }

    if include_files {
        let rows = sqlx::query_as::<_, PgSearchRow>(
            r"
            WITH q AS (
                SELECT COALESCE(
                    websearch_to_tsquery('simple', $2),
                    plainto_tsquery('simple', $2)
                ) AS tsq
            )
            SELECT
                f.id,
                left(
                    regexp_replace(
                        coalesce(f.name, '') || ' ' || coalesce(f.file_type, ''),
                        '\s+',
                        ' ',
                        'g'
                    ),
                    200
                ) AS snippet,
                ts_rank_cd(f.search_tsv, q.tsq)::real AS rank,
                f.created_at
            FROM files f, q
            WHERE f.organization_id = $1
              AND f.is_deleted = false
              AND q.tsq IS NOT NULL
              AND q.tsq <> ''::tsquery
              AND f.search_tsv @@ q.tsq
            ORDER BY rank DESC, f.created_at DESC, f.id DESC
            LIMIT $3
            ",
        )
        .bind(org_id)
        .bind(query)
        .bind(limit_i64)
        .fetch_all(state.db_pool.as_ref())
        .await?;

        for row in rows {
            merged.push((
                SearchResult {
                    result_type: "file".to_string(),
                    id: row.id.to_string(),
                    snippet: row.snippet.unwrap_or_default(),
                },
                row.rank,
                row.created_at,
                row.id,
            ));
        }
    }

    merged.sort_by(|a, b| {
        b.1.total_cmp(&a.1)
            .then_with(|| b.2.cmp(&a.2))
            .then_with(|| b.3.cmp(&a.3))
    });
    merged.truncate(limit as usize);
    Ok(merged.into_iter().map(|(r, _, _, _)| r).collect())
}

fn include_type(types: Option<&[String]>, accepted: &[&str]) -> bool {
    types.is_none_or(|list| list.is_empty() || list.iter().any(|t| accepted.contains(&t.as_str())))
}
