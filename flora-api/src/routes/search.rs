//! Global search API routes.

use axum::{
    Router,
    extract::{Query, State},
    response::Json,
    routing::get,
};
use serde::{Deserialize, Serialize};

use flora_core::error::Result;

use crate::AppState;

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
/// Per T079, T080, T080.1.
async fn search(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>> {
    let org_id = headers
        .get("x-organization-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| uuid::Uuid::parse_str(v).ok())
        .ok_or(flora_core::error::Error::OrganizationContextRequired)?;

    if params.q.trim().is_empty() {
        return Ok(Json(SearchResponse {
            query: params.q,
            results: vec![],
        }));
    }

    let limit = params.limit.unwrap_or(20).min(50);

    let types = params.types.map(|t| {
        t.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>()
    });

    let results = state
        .search_service
        .search(&org_id, &params.q, types.as_deref(), limit)
        .await?;

    let api_results = results
        .into_iter()
        .map(|r| SearchResult {
            result_type: r.item_type,
            id: r.id,
            snippet: r.snippet,
        })
        .collect();

    tracing::debug!(query = %params.q, org_id = %org_id, "Search executed");
    Ok(Json(SearchResponse {
        query: params.q,
        results: api_results,
    }))
}
