#![allow(unused_imports, reason = "tests use imports differently between lib and test targets")]
//! Integration tests — full-stack scenarios connecting multiple crates.
//! Created via /speckit-implement when TDD phases are reached.
//! Will be implemented when core features (Phase 3+) are ready.
mod messaging;
mod registration;
mod workspace;
mod rbac;
mod tasks;
mod files;
/// Search integration tests.
pub mod search;
/// Notification integration tests.
pub mod notifications;
mod cross_story;
