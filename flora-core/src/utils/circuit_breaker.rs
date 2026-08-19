//! Circuit‑breaker utilities for external services (Zitadel, Meilisearch, Valkey).
//!
//! The implementation is a placeholder that currently forwards calls without
//! observing failures. A full implementation would track failure counts,
//! implement open/half‑open/closed states and exponential back‑off.

use crate::error::{Error, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Circuit‑breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Closed,
    Open,
    HalfOpen,
}

/// A simple circuit breaker.
#[derive(Debug)]
pub struct CircuitBreaker {
    state: Arc<Mutex<State>>,
    failures: u32,
    threshold: u32,
}

impl CircuitBreaker {
    /// Creates a new `CircuitBreaker` with the given failure threshold.
    pub fn new(threshold: u32) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::Closed)),
            failures: 0,
            threshold,
        }
    }

    /// Executes the provided async closure if the circuit is closed or half‑open.
    ///
    /// If the circuit is open, returns `Error::CircuitBreakerOpen`.
    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut state = self.state.lock().await;
        match *state {
            State::Closed => {
                drop(state);
                match f().await {
                    Ok(v) => Ok(v),
                    Err(_) => {
                        *state = State::Open;
                        Err(Error::CircuitBreakerOpen("service call failed".to_string()))
                    }
                }
            }
            State::HalfOpen => {
                drop(state);
                f().await
            }
            State::Open => Err(Error::CircuitBreakerOpen(
                "circuit breaker is open".to_string(),
            )),
        }
    }
}
