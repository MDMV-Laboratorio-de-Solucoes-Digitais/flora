//! # Flora Core
//!
//! Shared traits, models, repositories, and utilities for Flora Workspace.
//!
//! ## Architecture
//!
//! - **Models**: Domain entities with validation
//! - **Traits**: Abstract interfaces for services and repositories
//! - **Repositories**: Data access layer abstractions
//! - **Services**: Business logic implementations
//!
//! ## Multi-Tenancy
//!
//! Every tenant-scoped entity MUST include `organization_id` for isolation.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]

// Re-export database types for use by dependent crates
pub use sqlx::PgPool;

pub mod config;
pub mod database;
pub mod error;
pub mod migrations;
pub mod models;
pub mod repositories;
pub mod services;
pub mod storage;
pub mod traits;
pub mod utils;

pub use error::{Error, Result};
