//! Meilisearch integration for global search across organization entities.

use flora_core::{Error, Result};
use meilisearch_sdk::{client::Client, search::SearchResults};
use serde_json::Value;
use uuid::Uuid;

/// Search service for global search across entities in an organization.
pub struct SearchService {
    client: Client,
    index_template: String,
}

impl SearchService {
    /// Creates a new SearchService.
    pub fn new(url: &str, api_key: Option<&str>, index_template: &str) -> Result<Self> {
        let client = if let Some(key) = api_key {
            Client::new(url, Some(key))
        } else {
            Client::new(url, None::<&str>)
        }
        .map_err(|e| Error::Search(e.to_string()))?;

        Ok(Self {
            client,
            index_template: index_template.to_owned(),
        })
    }

    fn index_name(&self, org_id: &Uuid) -> String {
        self.index_template.replace("{org_id}", &org_id.to_string())
    }

    /// Indexes a message for full-text search.
    pub async fn index_message(
        &self,
        org_id: &Uuid,
        message: &flora_core::models::Message,
    ) -> Result<()> {
        let index = self.client.index(self.index_name(org_id));
        let doc = serde_json::json!({
            "id": message.id.to_string(),
            "type": "message",
            "content": message.content,
            "channel_id": message.channel_id.to_string(),
            "sender_id": message.sender_id.to_string(),
            "created_at": message.created_at.to_rfc3339(),
        });
        index
            .add_documents(&[doc], Some("id"))
            .await
            .map_err(|e| Error::Search(e.to_string()))?;
        Ok(())
    }

    /// Searches for documents matching a query within an organization.
    pub async fn search(
        &self,
        org_id: &Uuid,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let index = self.client.index(self.index_name(org_id));
        let results: SearchResults<Value> = index
            .search()
            .with_query(query)
            .with_limit(limit)
            .execute()
            .await
            .map_err(|e| Error::Search(e.to_string()))?;

        let mut out = Vec::with_capacity(results.estimated_total_hits.unwrap_or(0));
        for hit in results.hits {
            // SearchResults<T> has hits: Vec<SearchResult<T>>, each with .result field
            let doc = hit.result;
            let id = doc
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let type_ = doc
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let snippet = doc
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            out.push(SearchResult { id, type_, snippet });
        }
        Ok(out)
    }
}

/// A search result entry.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchResult {
    /// Entity ID.
    pub id: String,
    /// Entity type (e.g., "message", "task").
    #[serde(rename = "type")]
    pub type_: String,
    /// Text snippet for preview.
    pub snippet: String,
}
