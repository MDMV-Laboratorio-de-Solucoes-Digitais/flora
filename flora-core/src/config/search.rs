//! Meilisearch configuration.

use serde::{Deserialize, Serialize};

/// Configuration for Meilisearch-powered full-text search.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchConfig {
    /// Meilisearch host URL.
    pub url: String,
    /// Meilisearch API key.
    pub api_key: Option<String>,
    /// Default number of search results per page.
    pub default_limit: usize,
    /// Maximum number of search results.
    pub max_limit: usize,
    /// Index naming template (e.g., "`flora_org`_{`org_id`}").
    pub index_template: String,
    /// Whether to enable search (can be disabled for minimal deployments).
    pub enabled: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("MEILISEARCH_URL")
                .unwrap_or_else(|_| "http://localhost:7700".to_owned()),
            api_key: std::env::var("MEILISEARCH_API_KEY").ok(),
            default_limit: 20,
            max_limit: 100,
            index_template: "flora_org_{@@org_id@@}".to_owned(),
            enabled: true,
        }
    }
}

impl SearchConfig {
    /// Validates the search configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.enabled && self.url.is_empty() {
            return Err(crate::Error::Configuration(
                "search.url is required when search is enabled".to_owned(),
            ));
        }
        if self.default_limit == 0 || self.default_limit > self.max_limit {
            return Err(crate::Error::Configuration(
                "search.default_limit must be > 0 and <= max_limit".to_owned(),
            ));
        }
        Ok(())
    }

    /// Generates the index name for a given organization.
    #[must_use]
    pub fn index_name(&self, org_id: &str) -> String {
        self.index_template.replace("{@@org_id@@}", org_id)
    }
}
