// #![warn(missing_docs)]
#![doc(
    html_logo_url = "https://avatars3.githubusercontent.com/u/15439811?v=3&s=200",
    html_favicon_url = "https://iorust.github.io/favicon.ico",
    html_root_url = "https://iorust.github.io",
    html_playground_url = "https://play.rust-lang.org",
    issue_tracker_base_url = "https://github.com/iorust/jsonrpc-lite/issues"
)]

//! JSON-RPC 2.0 Specification serialization for Rust.

pub mod error;
pub mod jsonrpc;

pub use error::{Error, ErrorCode, Result};
pub use jsonrpc::*;
