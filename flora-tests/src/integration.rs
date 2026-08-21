#![allow(
    unused_imports,
    reason = "tests use imports differently between lib and test targets"
)]
//! Integration tests module.
//!
//! Full-stack scenarios connecting multiple crates.
mod cross_story;
mod files;
mod messaging;
/// Notification integration tests.
pub mod notifications;
mod rbac;
mod registration;
/// Search integration tests.
pub mod search;
mod tasks;
mod workspace;
