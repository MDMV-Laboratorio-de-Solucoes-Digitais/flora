//! Flora API - Axum HTTP Server

pub mod extractors;
pub mod middleware;
pub mod routes;
pub mod state;

pub use state::AppState;
