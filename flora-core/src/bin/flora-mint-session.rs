//! Dev-only helper: mint a deterministic Flora session JWT for browser e2e tests.
//!
//! Gated behind the `dev-mint` feature and excluded from default/release builds.
//! It emits a complete Playwright `storageState` JSON so the very same token is
//! injected on the frontend origin. Refuses `production` environments.

#![expect(
    clippy::print_stdout,
    reason = "a CLI helper must print the generated storageState path"
)]

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use flora_core::config::Config;
use flora_core::utils::jwt::{FloraClaims, decode_token, encode_token};
use serde_json::json;

/// Alice's stable user ID from `dev-seed.sql`.
const ALICE: &str = "22222222-2222-2222-2222-222222222222";
/// jti of the idempotent pre-selection session in `scripts/dev-smoke-session.sql`.
const JTI: &str = "018f0000-0000-7000-8000-000000000001";
/// The seeded System organization (nil UUID), used before `select-organization`.
const NIL_ORG: &str = "00000000-0000-0000-0000-000000000000";
const EMAIL: &str = "alice@acme.test";
/// localStorage key read by `AuthState.svelte.ts` on the frontend origin.
const AUTH_KEY: &str = "flora.auth.v1";

#[derive(Debug, Parser)]
#[command(
    name = "flora-mint-session",
    about = "Mint a seeded Flora session for e2e"
)]
struct Args {
    /// Absolute path of the Playwright storageState file to write.
    #[arg(long)]
    storage_state: PathBuf,
    /// Optional token TTL in seconds (defaults to the configured session TTL).
    #[arg(long)]
    ttl_secs: Option<u64>,
    /// Frontend origin on which the auth state will be injected.
    #[arg(long, default_value = "http://127.0.0.1:4173")]
    frontend_origin: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load().context("load Flora configuration")?;
    if config.app.env.eq_ignore_ascii_case("production") {
        return Err(anyhow::anyhow!(
            "refusing to mint session tokens in a production environment"
        ));
    }

    let ttl = args.ttl_secs.unwrap_or(config.app.session_ttl_secs);
    let now = chrono::Utc::now().timestamp();
    let exp = i64::try_from(ttl).context("session TTL exceeds i64")?;
    let claims = FloraClaims {
        sub: ALICE.to_owned(),
        jti: JTI.to_owned(),
        organization_id: NIL_ORG.to_owned(),
        email: EMAIL.to_owned(),
        iat: now,
        exp: now + exp,
        ver: 1,
    };

    // Guard against a claim-shape/secret mismatch before writing anything.
    let token = encode_token(&claims, &config.app.jwt_secret);
    let decoded = decode_token(&token, &config.app.jwt_secret)
        .context("minted token failed local validation")?;
    anyhow::ensure!(
        decoded.sub == ALICE && decoded.jti == JTI,
        "round-trip claim mismatch"
    );

    let value = json!({
        "cookies": [],
        "origins": [{
            "origin": args.frontend_origin,
            "localStorage": [{
                "name": AUTH_KEY,
                "value": json!({
                    "token": token,
                    "user": { "id": ALICE },
                    "organizationId": null,
                    "workspaceId": null
                }).to_string()
            }]
        }]
    });

    let path = args.storage_state;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create storageState directory {}", parent.display()))?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(&value)?)
        .with_context(|| format!("write storageState {}", path.display()))?;
    println!("wrote {}", path.display());
    Ok(())
}
