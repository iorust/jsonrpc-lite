use std::error;
use std::result;
use std::fmt;

/// A specialized [`Result`][std-result] type for JSON-RPC parsing operations.
///
/// This type is broadly used across `jsonrpc-lite` for any operation which may
/// produce an error.
///
/// This typedef is generally used to avoid writing out `jsonrpc_lite::Error`
/// directly and is otherwise a direct mapping to `Result`.
///
/// While usual Rust style is to import types directly, aliases of `Result` often
/// are not, to make it easier to distinguish between them. `Result` is generally
/// assumed to be `std::result::Result`, and so users of this alias will generally
/// use `jsonrpc_lite::Result` instead of shadowing
/// the prelude's import of `std::result::Result`.
///
/// [std-result]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type Result<T> = result::Result<T, Error>;

/// The error type for JSON-RPC parsing operations.
///
/// As of version `0.6.0` of `jsonrpc-lite` the only possible error is an invalid
/// version of the JSON-RPC protocol being in the parsed data.
#[derive(Debug)]
pub enum Error {
    /// The version that was parsed from the input of one
    /// of the parsing functions in the [JsonRpc][rpc-struct]
    /// struct was not "2.0" as required by the
    /// [JSON-RPC 2.0 Specification][json-spec]
    ///
    /// [json-spec]: http://www.jsonrpc.org/specification
    /// [rpc-struct]: enum.JsonRpc.html
    InvalidVersionParsed(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            InvalidVersionParsed(_) => "Serde Parsed something other than 2.0 as the JsonRpc version",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            InvalidVersionParsed(ref v) => write!(f, "Serde Parsed {} as the version instead of 2.0", v),
        }
    }
}
