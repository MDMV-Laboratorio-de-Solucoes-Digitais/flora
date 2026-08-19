//! Authentication routes (OIDC login, logout, token refresh).
//!
//! Full implementation per T019.

use axum::{routing::post, Router};

pub fn create_auth_router() -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/refresh", post(refresh))
}

async fn login() -> &'static str {
    "auth/login - TODO: implement per T019"
}

async fn logout() -> &'static str {
    "auth/logout - TODO: implement per T019"
}

async fn refresh() -> &'static str {
    "auth/refresh - TODO: implement per T019"
}
