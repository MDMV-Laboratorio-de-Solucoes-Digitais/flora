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

#![forbid(clippy::allow_attributes)]
#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod config;
pub mod error;
pub mod migrations;
pub mod models;
pub mod repositories;
pub mod services;
pub mod traits;

pub use error::{Error, Result};
