//! # Flora API - Axum HTTP Server
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
pub mod extractors;
pub mod middleware;
pub mod routes;
/// Application state shared across all requests.
///
/// This module contains the application state structure and related functionality.
pub mod state;

pub use state::AppState;
