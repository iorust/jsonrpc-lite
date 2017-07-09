#![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars3.githubusercontent.com/u/15439811?v=3&s=200",
       html_favicon_url = "https://iorust.github.io/favicon.ico",
       html_root_url = "https://iorust.github.io",
       html_playground_url = "https://play.rust-lang.org",
       issue_tracker_base_url = "https://github.com/iorust/jsonrpc-lite/issues")]
//! `jsonrpc-lite` is a lightweight library for translating _stringly_-typed
//! Json RPC calls that follow the [JSON-RPC 2.0 Specification][json-spec]
//! into Rust types.
//! ## Quick Examples
//! ```rust
//! extern crate jsonrpc_lite;
//! use jsonrpc_lite::JsonRpc;
//!
//! fn main() {
//!     // Making a JsonRpc Struct from a JSON string
//!     let str_update_notification = JsonRpc::from_str(r#"{
//!             "jsonrpc": "2.0",
//!             "method": "update"
//!     }"#).unwrap();
//!     // Is equivalent to making them using the methods provided by
//!     // the JsonRpc struct.
//!     let struct_update_notification = JsonRpc::notification("update");
//!
//!     assert_eq!(str_update_notification, struct_update_notification);
//! }
//! ```
//!
//! [json-spec]: http://www.jsonrpc.org/specification
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

pub use self::jsonrpc::{Id, Params, JsonRpc, RpcError};
pub use self::error::{Result, Error};
#[doc(hidden)]
pub use serde_json::{Value, Map};

mod jsonrpc;
mod error;
