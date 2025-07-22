//! [![github]](https://github.com/tamaskis/easy_cmd)&ensp;[![crates-io]](https://crates.io/crates/easy-cmd)&ensp;[![docs-rs]](https://docs.rs/easy-cmd)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Simple interfaces for running CLI commands from Rust.

// Linter setup.
#![warn(missing_docs)]

// Module declarations.
pub(crate) mod functions;
pub(crate) mod macros;
pub(crate) mod traits;

// Re-exports.
pub use crate::functions::run_command;
pub use crate::traits::AsCommand;
