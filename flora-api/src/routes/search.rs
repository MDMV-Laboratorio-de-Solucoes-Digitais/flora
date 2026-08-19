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
    State(_state): State<AppState>,
    _headers: axum::http::HeaderMap,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>> {
    if params.q.trim().is_empty() {
        return Ok(Json(SearchResponse {
            query: params.q,
            results: vec![],
        }));
    }

    let _limit = params.limit.unwrap_or(20).min(50);

    // TODO: T079 — Integrate Meilisearch indexing here.
    // Currently returns empty results as a stub.
    // Real implementation will:
    // 1. Build org-scoped index name: flora_org_{org_id}
    // 2. Filter all search by organization_id (T079.1)
    // 3. Apply performance targets (T080.1): past week <5s, past month <10s

    tracing::debug!(query = %params.q, "Search executed (Meilisearch stub)");
    Ok(Json(SearchResponse {
        query: params.q,
        results: vec![],
    }))
}
