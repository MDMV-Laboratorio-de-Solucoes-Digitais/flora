//! Meilisearch integration for global search across organization entities.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
use flora_core::{Error, Result};
use meilisearch_sdk::{client::Client, search::SearchResults};
use serde_json::Value;
use std::fmt::{self, Write};
use tracing::{debug, instrument};
use uuid::Uuid;

/// Search service for global search across entities in an organization.
pub struct SearchService {
    client: Client,
    index_template: String,
}

impl fmt::Debug for SearchService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchService")
            .field("client", &"<Client>")
            .field("index_template", &self.index_template)
            .finish()
    }
}

impl SearchService {
    /// Creates a new `SearchService`.
    ///
    /// # Errors
    ///
    /// Returns an error if the client fails to connect.
    pub fn new(url: &str, api_key: Option<&str>, index_template: &str) -> Result<Self> {
        let client = api_key
            .map_or_else(
                || Client::new(url, None::<&str>),
                |key| Client::new(url, Some(key)),
            )
            .map_err(|e| Error::Search(e.to_string()))?;

        Ok(Self {
            client,
            index_template: index_template.to_owned(),
        })
    }

    /// Suppresses the false-positive nursery lint: `{org_id}` is a literal search pattern
    /// for `str::replace`, not a format argument.
    #[allow(
        clippy::allow_attributes,
        clippy::literal_string_with_formatting_args,
        reason = "The '{org_id}' argument is a literal search-and-replace pattern for str::replace(), \
not a formatting argument. This is a false positive from the nursery lint."
    )]
    fn index_name(&self, org_id: &Uuid) -> String {
        self.index_template.replace("{org_id}", &org_id.to_string())
    }

    /// Sets up the index with filterable attributes.
    ///
    /// # Errors
    /// Returns an error if the meilisearch client fails to execute the request.
    #[instrument(skip(self))]
    pub async fn setup_index(&self, org_id: &Uuid) -> Result<()> {
        let index = self.client.index(self.index_name(org_id));
        let _task = index
            .set_filterable_attributes(["type", "created_at_timestamp"])
            .await
            .map_err(|e| Error::Search(e.to_string()))?;
        Ok(())
    }

    /// Indexes a message for full-text search.
    ///
    /// # Errors
    /// Returns an error if the meilisearch client fails to add the document.
    #[instrument(skip(self, message))]
    pub async fn index_message(
        &self,
        org_id: &Uuid,
        message: &flora_core::models::Message,
    ) -> Result<()> {
        debug!("Indexing message {}", message.id);
        let index = self.client.index(self.index_name(org_id));
        let doc = serde_json::json!({
            "id": message.id.to_string(),
            "type": "message",
            "content": message.content,
            "channel_id": message.channel_id.to_string(),
            "sender_id": message.sender_id.to_string(),
            "created_at": message.created_at.to_rfc3339(),
            "created_at_timestamp": message.created_at.timestamp(),
        });
        let _task = index
            .add_documents(&[doc], Some("id"))
            .await
            .map_err(|e| Error::Search(e.to_string()))?;
        Ok(())
    }

    /// Indexes a task for full-text search.
    ///
    /// # Errors
    /// Returns an error if the meilisearch client fails to add the document.
    #[instrument(skip(self, task))]
    pub async fn index_task(&self, org_id: &Uuid, task: &flora_core::models::Task) -> Result<()> {
        debug!("Indexing task {}", task.id);
        let index = self.client.index(self.index_name(org_id));
        let doc = serde_json::json!({
            "id": task.id.to_string(),
            "type": "task",
            "content": format!("{} {}", task.title, task.description.as_deref().unwrap_or("")),
            "assignee_id": task.assignee_id.map(|id| id.to_string()),
            "status": task.status.to_string(),
            "created_at": task.created_at.to_rfc3339(),
            "created_at_timestamp": task.created_at.timestamp(),
        });
        let _task = index
            .add_documents(&[doc], Some("id"))
            .await
            .map_err(|e| Error::Search(e.to_string()))?;
        Ok(())
    }

    /// Indexes a file for full-text search.
    ///
    /// # Errors
    /// Returns an error if the meilisearch client fails to add the document.
    #[instrument(skip(self, file))]
    pub async fn index_file(&self, org_id: &Uuid, file: &flora_core::models::File) -> Result<()> {
        debug!("Indexing file {}", file.id);
        let index = self.client.index(self.index_name(org_id));
        let doc = serde_json::json!({
            "id": file.id.to_string(),
            "type": "file",
            "content": file.name, // Name and maybe metadata
            "size": file.size_bytes,
            "created_at": file.created_at.to_rfc3339(),
            "created_at_timestamp": file.created_at.timestamp(),
        });
        let _task = index
            .add_documents(&[doc], Some("id"))
            .await
            .map_err(|e| Error::Search(e.to_string()))?;
        Ok(())
    }

    /// Deletes a document from the index.
    ///
    /// # Errors
    /// Returns an error if the meilisearch client fails to delete the document.
    #[instrument(skip(self))]
    pub async fn delete_document(&self, org_id: &Uuid, id: &Uuid) -> Result<()> {
        debug!("Deleting document {} from index", id);
        let index = self.client.index(self.index_name(org_id));
        let _task = index
            .delete_document(id.to_string())
            .await
            .map_err(|e| Error::Search(e.to_string()))?;
        Ok(())
    }

    /// Searches for documents matching a query within an organization.
    ///
    /// # Errors
    /// Returns an error if the meilisearch client fails to execute the search query.
    #[instrument(skip(self))]
    pub async fn search(
        &self,
        org_id: &Uuid,
        query: &str,
        types: Option<&[String]>,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        debug!("Searching in org {}: query={}", org_id, query);
        let index = self.client.index(self.index_name(org_id));

        let mut search_query = index.search();
        let _ = search_query.with_query(query).with_limit(limit);

        let mut filter = String::new();
        if let Some(types) = types.filter(|t| !t.is_empty()) {
            let types_str = types
                .iter()
                .map(|t| format!("'{t}'"))
                .collect::<Vec<_>>()
                .join(", ");
            let _ = write!(filter, "type IN [{types_str}]");
        }

        if !filter.is_empty() {
            let _ = search_query.with_filter(&filter);
        }

        let results: SearchResults<Value> = search_query
            .execute()
            .await
            .map_err(|e| Error::Search(e.to_string()))?;

        let mut out = Vec::with_capacity(results.estimated_total_hits.unwrap_or(0));
        for hit in results.hits {
            let doc = hit.result;
            let id = doc
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let doc_type = doc
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let snippet = doc
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            out.push(SearchResult {
                id,
                item_type: doc_type,
                snippet,
            });
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
    pub item_type: String,
    /// Text snippet for preview.
    pub snippet: String,
}
