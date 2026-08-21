//! Auto-save for drafts (messages, tasks) before session termination.

use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// A draft saved by a user.
#[derive(Debug, Clone)]
pub struct Draft {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: String,
    pub content: String,
}

/// Service for managing drafts.
#[derive(Debug, Clone, Default)]
pub struct DraftService {
    // In-memory store for now
    drafts: Arc<RwLock<HashMap<Uuid, Draft>>>,
}

impl DraftService {
    /// Creates a new `DraftService`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            drafts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Saves a draft for a user.
    ///
    /// # Errors
    ///
    /// Currently never returns an error as it relies on an in-memory store.
    pub async fn save_draft(&self, user_id: Uuid, draft_type: &str, content: &str) -> Result<Uuid> {
        let id = Uuid::now_v7();
        let draft = Draft {
            id,
            user_id,
            kind: draft_type.to_string(),
            content: content.to_string(),
        };

        let _ = self.drafts.write().await.insert(id, draft);
        Ok(id)
    }

    /// Gets all drafts for a user.
    ///
    /// # Errors
    ///
    /// Currently never returns an error as it relies on an in-memory store.
    pub async fn get_drafts(&self, user_id: Uuid) -> Result<Vec<Draft>> {
        let user_drafts = {
            let lock = self.drafts.read().await;
            lock.values()
                .filter(|draft| draft.user_id == user_id)
                .cloned()
                .collect()
        };
        Ok(user_drafts)
    }

    /// Deletes a draft.
    ///
    /// # Errors
    ///
    /// Currently never returns an error as it relies on an in-memory store.
    pub async fn delete_draft(&self, draft_id: Uuid) -> Result<()> {
        let _ = self.drafts.write().await.remove(&draft_id);
        Ok(())
    }
}
