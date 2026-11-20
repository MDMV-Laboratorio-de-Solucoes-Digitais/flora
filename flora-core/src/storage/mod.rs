//! Storage module for file handling.
//!
//! Provides abstractions for different storage backends (local filesystem, S3, etc.).

pub mod local;

pub use local::LocalFileSystem;
