use std::error;
use std::fmt::{Formatter, Display, Result};
use std::string::ToString;
use serde_json::{Value, to_string};

/// Error Code
#[derive(Clone, PartialEq, Debug)]
enum ErrorCode {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError,
    /// The JSON sent is not a valid Request object.
    InvalidRequest,
    /// The method does not exist / is not available.
    MethodNotFound,
    /// Invalid method parameter(s).
    InvalidParams,
    /// Internal JSON-RPC error.
    InternalError,
    /// Reserved for implementation-defined server-errors.
    ServerError(i64),
}

use self::ErrorCode::*;

impl ErrorCode {
    fn code(&self) -> i64 {
        match *self {
            ParseError => -32700,
            InvalidRequest => -32600,
            MethodNotFound => -32601,
            InvalidParams => -32602,
            InternalError => -32603,
            ServerError(code) => code,
        }
    }

    fn as_str(&self) -> &'static str {
        match *self {
            ParseError => "Parse error",
            InvalidRequest => "Invalid request",
            MethodNotFound => "Method not found",
            InvalidParams => "Invalid params",
            InternalError => "Internal error",
            ServerError(_) => "Server error",
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self.as_str(), f)
    }
}

/// A Rust representation of a JSON-RPC Error Object
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    code: i64,
    message: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl Error {
    // Utility method for the no parameter
    // creation methods.
    fn new(code: ErrorCode) -> Self {
        Self::custom(code.code(), code.to_string())
    }

    /// Create a JSON-RPC 2.0 Parse Error, which is defined
    /// by the [JSON-RPC Specification][spec] as:
    ///
    /// * Invalid JSON was received by the server.
    /// * An error occurred on the server while parsing the JSON text.
    ///
    /// ## Examples
    /// Using `RpcError::parse_error` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning a Parse Error.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::parse_error(),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "code": -32700,
    ///         "message": "Parse error"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn parse_error() -> Self {
        Self::new(ParseError)
    }

    /// Create a JSON-RPC 2.0 Invalid Request Error, which is defined
    /// by the [JSON-RPC Specification][spec] as:
    ///
    /// * The JSON sent is not a valid Request object.
    ///
    /// ## Examples
    /// Using `RpcError::invalid_request` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning an Invalid Request Error.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::invalid_request(),
    /// # from_value(
    /// #   json!(
    ///     {
    ///         "code": -32600,
    ///         "message": "Invalid request"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn invalid_request() -> Self {
        Self::new(InvalidRequest)
    }

    /// Create a JSON-RPC 2.0 Method Not Found Error, which is defined
    /// by the [JSON-RPC Specification][spec] as:
    ///
    /// * The method does not exist / is not available.
    ///
    /// ## Examples
    /// Using `RpcError::method_not_found` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning a Method not Found Error.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::method_not_found(),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "code": -32601,
    ///         "message": "Method not found"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn method_not_found() -> Self {
        Self::new(MethodNotFound)
    }

    /// Create a JSON-RPC 2.0 Invalid Parameters Error, which is defined
    /// by the [JSON-RPC Specification][spec] as:
    ///
    /// * Invalid method parameter(s).
    ///
    /// ## Examples
    /// Using `RpcError::invalid_params` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning an Invalid Params Error.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::invalid_params(),
    /// # from_value(
    /// #   json!(
    ///     {
    ///         "code": -32602,
    ///         "message": "Invalid params"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn invalid_params() -> Self {
        Self::new(InvalidParams)
    }

    /// Create a JSON-RPC 2.0 Internal Error, which is defined
    /// by the [JSON-RPC Specification][spec] as:
    ///
    /// * Internal JSON-RPC error.
    ///
    /// ## Examples
    /// Using `RpcError::internal_error` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning an Internal Error.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::internal_error(),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "code": -32603,
    ///         "message": "Internal error"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn internal_error() -> Self {
        Self::new(InternalError)
    }

    /// Create a JSON-RPC 2.0 Server Error, with a specific code.
    /// This Error is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * Reserved for implementation-defined server-errors.
    ///
    /// ## Examples
    /// Using `RpcError::server_error` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning a Server Error
    /// with the provided error code.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::server_error(-32050),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "code": -32050,
    ///         "message": "Server error"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn server_error(code: i64) -> Self {
        Self::new(ServerError(code))
    }

    /// Create a JSON-RPC 2.0 Error that was not defined by
    /// the [JSON-RPC Specification][spec]
    ///
    /// ## Examples
    /// Using `RpcError::custom` you can quickly create a valid
    /// JSON-RPC Error Object set up for returning a custom Error.
    ///
    /// Which can be helpful when returning application Errors.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::custom(505, "Application Error"),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "code": 505,
    ///         "message": "Application Error"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification#error_object
    pub fn custom(code: i64, msg: String) -> Self {
        Error {
            code: code,
            message: msg,
            data: None,
        }
    }

    /// Add additional information about the error.
    /// ## Examples
    /// Most likely, Application Errors would have data to return to the
    /// Client/Server, this makes adding that data simpler.
    ///
    /// Due to taking anything that can be turned into a JSON [Value][value],
    /// it can handle something as simple as a String.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// assert_eq!(
    ///     RpcError::custom(505, "Application Error")
    ///         .with_data("Failed to obtain pudding"),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "code": 505,
    ///         "message": "Application Error",
    ///         "data": "Failed to obtain pudding"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    /// Or something more complex, like a full JSON Object.
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate serde_json;
    /// # use serde_json::from_value;
    /// extern crate jsonrpc_lite;
    /// use jsonrpc_lite::*;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         RpcError::custom(505, "Application Error")
    ///             .with_data(json!({
    ///                 "color": "blue",
    ///                 "hasPet": true,
    ///                 "numBunnies": 3
    ///             })
    ///         ),
    /// #   from_value(
    /// #        json!(
    ///         {
    ///             "code": 505,
    ///             "message": "Application Error",
    ///             "data": {
    ///                 "color": "blue",
    ///                 "hasPet": true,
    ///                 "numBunnies": 3
    ///             }
    ///         }
    /// #   )
    /// #   ).unwrap()
    ///     );
    /// }
    /// ```
    ///
    /// [value]: ../serde_json/value/enum.Value.html
    pub fn with_data<D: Into<Option<Value>>>(mut self, data: D) -> Self {
        self.data = data.into();
        self
    }

    /// An [`i64`][i64] that indicates the error type that occurred
    /// ## Examples
    ///
    /// ```rust
    /// use jsonrpc_lite::RpcError;
    ///
    /// let error = RpcError::parse_error();
    /// assert_eq!(error.code(), -32700);
    /// ```
    ///
    /// [i64]: https://doc.rust-lang.org/std/primitive.i64.html
    pub fn code(&self) -> i64 {
        self.code
    }

    /// A [`String`][string] providing a short description of the error.
    /// ## Examples
    ///
    /// ```rust
    /// use jsonrpc_lite::RpcError;
    ///
    /// let error = RpcError::parse_error();
    /// assert_eq!(&error.message(), "Parse error");
    /// ```
    ///
    /// [string]: https://doc.rust-lang.org/std/string/index.html
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// A JSON [`Value`][value] that contains additional information about the error.
    /// ## Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::RpcError;
    ///
    /// let error = RpcError::custom(505, "Application Error").with_data("pups");
    /// assert!(error.data().is_some());
    /// assert_eq!(
    ///     error.data(),
    ///     Some(
    /// # &json!(
    ///         "pups"
    /// # )
    ///     )
    /// );
    /// # }
    /// ```
    ///
    /// [value]: ../serde_json/value/enum.Value.html
    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&to_string(&self).unwrap(), f)
    }
}
