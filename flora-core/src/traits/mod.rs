//! Trait definitions for Flora Workspace.

pub mod auth_provider;
pub mod repository;
pub mod storage_provider;

pub use auth_provider::{AuthProvider, UserInfo};
pub use repository::*;
pub use storage_provider::StorageProvider;
