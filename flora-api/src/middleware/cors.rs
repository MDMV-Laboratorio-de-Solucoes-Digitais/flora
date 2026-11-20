//! CORS middleware configuration.

use std::time::Duration;

use axum::http::{HeaderName, HeaderValue, Method, header};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Creates a CORS layer allowing only the configured browser origins.
///
/// `Authorization: Bearer …` is a non-safelisted request header, so it is listed
/// in `allow_headers`; it is **not** a credentialed request. `allow_credentials`
/// is intentionally left off (cookies are never used), which also avoids
/// tower-http's credentials + wildcard panic path.
///
/// # Panics
///
/// `AllowOrigin::list` panics on a literal `*` entry. `AppConfig::validate()`
/// rejects such entries at startup, so this cannot happen through `Config::load()`.
///
/// # Examples
///
/// ```ignore
/// let cors = cors_layer(&["http://localhost:5173".to_owned()]);
/// ```
#[must_use = "CORS layer must be applied to the router"]
pub fn cors_layer(origins: &[String]) -> CorsLayer {
    // AppConfig::validate() guarantees every entry parses; filter_map only guards
    // a caller that bypassed validation. Invalid entries are never silently kept.
    let origins: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect();
    // Retained for legacy/test helpers; no route currently trusts this header.
    let organization_header = HeaderName::from_static("x-organization-id");

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            organization_header,
        ])
        .max_age(Duration::from_hours(24))
    // No allow_credentials(true), no expose_headers(Any), no manual vary:
    // tower-http derives `Vary: Origin` for origin lists.
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode, header};
    use axum::routing::get;
    use tower::ServiceExt;

    use super::cors_layer;

    const ALLOWED: &str = "http://127.0.0.1:4173";

    fn router() -> Router {
        let origins = vec![
            ALLOWED.to_owned(),
            "tauri://localhost".to_owned(),
            "http://tauri.localhost".to_owned(),
        ];
        Router::new()
            .route("/health", get(|| async { "ok" }))
            .layer(cors_layer(&origins))
    }

    fn preflight(origin: &str) -> Result<Request<Body>, axum::http::Error> {
        Request::builder()
            .method(Method::OPTIONS)
            .uri("/health")
            .header(header::ORIGIN, origin)
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
            .header(
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                "authorization,content-type",
            )
            .body(Body::empty())
    }

    #[tokio::test]
    async fn allowed_origin_preflight_returns_acao_authorization_and_no_credentials()
    -> Result<(), Box<dyn std::error::Error>> {
        let response = router().oneshot(preflight(ALLOWED)?).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .and_then(|value| value.to_str().ok()),
            Some(ALLOWED)
        );
        let methods = response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_METHODS)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        assert!(methods.contains("POST"), "ACAM was: {methods}");
        let headers = response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_HEADERS)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        assert!(headers.contains("authorization"), "ACAH was: {headers}");
        assert!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
                .is_none()
        );
        let vary = response
            .headers()
            .get(header::VARY)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        assert!(
            vary.to_ascii_lowercase().contains("origin"),
            "Vary was: {vary}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn disallowed_origin_preflight_has_no_acao() -> Result<(), Box<dyn std::error::Error>> {
        let response = router().oneshot(preflight("https://evil.example")?).await?;
        // tower-http answers OPTIONS itself even for a disallowed origin: status is
        // still 200 and ACAM/ACAH may be present. The security assertion is the
        // absence of Access-Control-Allow-Origin.
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .is_none()
        );
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Wildcard origin")]
    fn wildcard_origin_is_rejected_by_allow_origin_list() {
        let _ = cors_layer(&["*".to_owned()]);
    }

    #[tokio::test]
    async fn default_configuration_origins_are_accepted() -> Result<(), Box<dyn std::error::Error>>
    {
        let config = flora_core::config::Config::default();
        config.app.validate()?;
        let response = router()
            .oneshot(preflight(&config.app.cors_origins[3])?)
            .await?;
        assert_eq!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .and_then(|value| value.to_str().ok()),
            Some("http://127.0.0.1:4173")
        );
        Ok(())
    }
}
