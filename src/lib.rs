// #![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars3.githubusercontent.com/u/15439811?v=3&s=200",
       html_favicon_url = "https://iorust.github.io/favicon.ico",
       html_root_url = "https://iorust.github.io",
       html_playground_url = "https://play.rust-lang.org",
       issue_tracker_base_url = "https://github.com/iorust/jsonrpc-lite/issues")]

//! JSON-RPC 2.0 Specification serialization for Rust.

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

pub use self::jsonrpc::{Id, Params, JsonRpc};
pub use self::error::{Error, ErrorCode, Result};

mod jsonrpc;
mod error;
