//! Trait definitions for Flora Workspace.

pub mod auth_provider;
pub mod hitl_receipt;
pub mod repository;
pub mod storage_provider;

pub use auth_provider::{AuthProvider, UserInfo};
pub use hitl_receipt::HitlReceiptPort;
pub use repository::*;
pub use storage_provider::StorageProvider;
