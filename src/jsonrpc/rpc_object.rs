use super::*;
use serde_json::{from_str, from_slice, from_reader, from_value, Result as SerdeResult};
use std::io::Read;

/// JSON-RPC 2.0 Request and Response object
/// [JSON-RPC 2.0 Specification](http://www.jsonrpc.org/specification).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpc {
    /// Request object
    Request(Request),
    /// Notification object
    Notification(Notification),
    /// Success Response
    Success(Success),
    /// Error Response
    Error(Error),
}

impl JsonRpc {
    /// Create a fully formed JSON-RPC 2.0 Request Object without Params
    /// ## Examples
    /// Using `JsonRpc::request` you can create a valid JSON-RPC
    /// Request Object ready for serialization.
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
    ///     JsonRpc::request(42, "getData"),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "method": "getData",
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    pub fn request<I: Into<Id>, S: Into<String>>(id: I, method: S) -> Self {
        JsonRpc::Request(Request {
            jsonrpc: String::from("2.0"),
            method: method.into(),
            params: None,
            id: id.into(),
        })
    }

    /// Create a fully formed JSON-RPC 2.0 Request Object with Params
    /// ## Examples
    /// Using `JsonRpc::request_with_params` you can create a valid JSON-RPC
    /// Request Object that contains parameters and is ready for serialization.
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
    ///     JsonRpc::request_with_params(42, "getData", vec!["key"]),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "method": "getData",
    ///         "params": ["key"],
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    pub fn request_with_params<I: Into<Id>, P: Into<Params>, S: Into<String>>(id: I, method: S, params: P) -> Self {
        JsonRpc::Request(Request {
            jsonrpc: String::from("2.0"),
            method: method.into(),
            params: Some(params.into()),
            id: id.into(),
        })
    }

    /// Create a fully formed JSON-RPC 2.0 Notification Object without Params
    /// ## Examples
    /// Using `JsonRpc::notification` you can create a valid JSON-RPC
    /// Notification Object ready for serialization.
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
    ///     JsonRpc::notification("update"),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "method": "update"
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    pub fn notification<S: Into<String>>(method: S) -> Self {
        JsonRpc::Notification(Notification {
            jsonrpc: String::from("2.0"),
            method: method.into(),
            params: None,
        })
    }

    /// Create a fully formed JSON-RPC 2.0 Notification Object with Params
    /// ## Examples
    /// Using `JsonRpc::notification_with_params` you can create a valid JSON-RPC
    /// Notification Object that contains parameters and is ready for serialization.
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
    ///     JsonRpc::notification_with_params("update", vec!["key"]),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "method": "update",
    ///         "params": ["key"]
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    pub fn notification_with_params<P: Into<Params>, S: Into<String>>(method: S, params: P) -> Self {
        JsonRpc::Notification(Notification {
            jsonrpc: String::from("2.0"),
            method: method.into(),
            params: Some(params.into()),
        })
    }

    /// Create a fully formed JSON-RPC 2.0 Successful Response Object
    /// ## Examples
    /// Using `JsonRpc::success` you can create a valid JSON-RPC Successful
    /// Response Object ready for serialization.
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
    ///     JsonRpc::success(42, vec![42, 92, 32]),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "result": [42, 92, 32],
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    pub fn success<I: Into<Id>>(id: I, result: &Value) -> Self {
        JsonRpc::Success(Success {
            jsonrpc: String::from("2.0"),
            result: result.clone(),
            id: id.into(),
        })
    }

    /// Create a fully formed JSON-RPC 2.0 Error Response Object
    /// by passing a valid [`RpcError`][rpc-error].
    /// ## Examples
    /// Using `JsonRpc::error` you can create a valid and highly
    /// customizable JSON-RPC Error Response Object ready for serialization.
    ///
    /// For more information on the customizability of the error, please read
    /// the documentation for [`RpcError`][rpc-error].
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// # use serde_json::from_value;
    /// # extern crate jsonrpc_lite;
    /// # use jsonrpc_lite::*;
    /// #
    /// # fn main() {
    /// let error = RpcError::custom(505, "Application Error").with_data("Failed to obtain pudding");
    ///
    /// assert_eq!(
    ///     JsonRpc::error(42, error),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": 505,
    ///             "message": "Application Error",
    ///             "data": "Failed to obtain pudding"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [rpc-error]: struct.RpcError.html
    pub fn error<I: Into<Id>>(id: I, error: RpcError) -> Self {
        JsonRpc::Error(Error {
            jsonrpc: String::from("2.0"),
            error,
            id: id.into(),
        })
    }

    /// Create a fully formed JSON-RPC 2.0 Parse Error Response Object.
    ///
    /// Which is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * Invalid JSON was received by the server.
    /// * An error occurred on the server while parsing the JSON text.
    ///
    /// ## Examples
    /// Using `JsonRpc::parse_error` you can quickly create a valid
    /// JSON-RPC Error Response Object set up for returning a Parse Error
    /// that is ready for serialization.
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
    ///     JsonRpc::parse_error(42),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": -32700,
    ///             "message": "Parse error"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn parse_error<I: Into<Id>>(id: I) -> Self {
        Self::error(id, RpcError::parse_error())
    }

    /// Create a fully formed JSON-RPC 2.0 Invalid Request Error Response Object.
    ///
    /// Which is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * The JSON sent is not a valid Request object.
    ///
    /// ## Examples
    /// Using `JsonRpc::invalid_request_error` you can quickly create a valid
    /// JSON-RPC Error Response Object set up for returning a Invalid Request Error
    /// that is ready for serialization.
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
    ///     JsonRpc::invalid_request_error(42),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": -32600,
    ///             "message": "Invalid request"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn invalid_request_error<I: Into<Id>>(id: I) -> Self {
        Self::error(id, RpcError::invalid_request())
    }

    /// Create a fully formed JSON-RPC 2.0 Method Not Found Error Response Object.
    ///
    /// Which is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * The method does not exist / is not available.
    ///
    /// ## Examples
    /// Using `JsonRpc::method_not_found_error` you can quickly create a valid
    /// JSON-RPC Error Response Object set up for returning a Method Not Found Error
    /// that is ready for serialization.
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
    ///     JsonRpc::method_not_found_error(42),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": -32601,
    ///             "message": "Method not found"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn method_not_found_error<I: Into<Id>>(id: I) -> Self {
        Self::error(id, RpcError::method_not_found())
    }

    /// Create a fully formed JSON-RPC 2.0 Invalid Params Error Response Object.
    ///
    /// Which is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * Invalid method parameter(s).
    ///
    /// ## Examples
    /// Using `JsonRpc::invalid_params_error` you can quickly create a valid
    /// JSON-RPC Error Response Object set up for returning a Invalid Params Error
    /// that is ready for serialization.
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
    ///     JsonRpc::invalid_params_error(42),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": -32602,
    ///             "message": "Invalid params"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn invalid_params_error<I: Into<Id>>(id: I) -> Self {
        Self::error(id, RpcError::invalid_params())
    }

    /// Create a fully formed JSON-RPC 2.0 Internal Error Response Object.
    ///
    /// Which is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * Internal JSON-RPC error.
    ///
    /// ## Examples
    /// Using `JsonRpc::internal_error` you can quickly create a valid
    /// JSON-RPC Error Response Object set up for returning a Internal Error
    /// that is ready for serialization.
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
    ///     JsonRpc::internal_error(42),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": -32603,
    ///             "message": "Internal error"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn internal_error<I: Into<Id>>(id: I) -> Self {
        Self::error(id, RpcError::internal_error())
    }

    /// Create a fully formed JSON-RPC 2.0 Server Error Response Object.
    ///
    /// Which is defined by the [JSON-RPC Specification][spec] as:
    ///
    /// * Reserved for implementation-defined server-errors.
    ///
    /// ## Examples
    /// Using `JsonRpc::server_error` you can quickly create a valid
    /// JSON-RPC Error Response Object set up for returning a Server Error
    /// that is ready for serialization.
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
    ///     JsonRpc::server_error(42, -32050),
    /// # from_value(
    /// #    json!(
    ///     {
    ///         "jsonrpc": "2.0",
    ///         "error": {
    ///             "code": -32050,
    ///             "message": "Server error"
    ///         },
    ///         "id": 42
    ///     }
    /// #   )
    /// # ).unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// [spec]: http://www.jsonrpc.org/specification
    pub fn server_error<I: Into<Id>>(id: I, code: i64) -> Self {
        Self::error(id, RpcError::server_error(code))
    }

    /// Returns the ID of this JSON-RPC Object, if it had one.
    ///
    /// Only the Request, Success, and Error variants have an Id.
    pub fn get_id(&self) -> Option<Id> {
        match *self {
            JsonRpc::Request(ref v) => Some(v.id.clone()),
            JsonRpc::Success(ref v) => Some(v.id.clone()),
            JsonRpc::Error(ref v) => Some(v.id.clone()),
            _ => None,
        }
    }

    /// Returns the method of this JSON-RPC Object, if it had one.
    ///
    /// Only the Request, and Notification variants have a method.
    pub fn get_method(&self) -> Option<&str> {
        match *self {
            JsonRpc::Notification(ref v) => Some(&v.method),
            JsonRpc::Request(ref v) => Some(&v.method),
            _ => None,
        }
    }

    /// Returns the parameters of this JSON-RPC Object, if it had one.
    ///
    /// Only the Request, and Notification variants have Params.
    pub fn get_params(&self) -> Option<Params> {
        match *self {
            JsonRpc::Notification(ref v) => v.params.as_ref().cloned(),
            JsonRpc::Request(ref v) => v.params.as_ref().cloned(),
            _ => None,
        }
    }

    /// Returns the result value of this JSON-RPC Object, if it had one.
    ///
    /// Only the Success variant has a result.
    pub fn get_result(&self) -> Option<&Value> {
        match *self {
            JsonRpc::Success(ref v) => Some(&v.result),
            _ => None,
        }
    }

    /// Returns the error object of this JSON-RPC Object, if it had one.
    ///
    /// Only the Error variant has an error.
    pub fn get_error(&self) -> Option<&RpcError> {
        match *self {
            JsonRpc::Error(ref v) => Some(&v.error),
            _ => None,
        }
    }

    /// Returns true if this JSON-RPC Object is a `JsonRpc::Request`
    pub fn is_request(&self) -> bool {
        if let JsonRpc::Request(_) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if this JSON-RPC Object is a `JsonRpc::Notification`
    pub fn is_notification(&self) -> bool {
        if let JsonRpc::Notification(_) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if this JSON-RPC Object is a `JsonRpc::Success`
    pub fn is_success(&self) -> bool {
        if let JsonRpc::Success(_) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if this JSON-RPC Object is a `JsonRpc::Error`
    pub fn is_error(&self) -> bool {
        if let JsonRpc::Error(_) = *self {
            true
        } else {
            false
        }
    }

    // Helper method for getting the JSON-RPC version
    fn get_version(&self) -> String {
        match *self {
            JsonRpc::Request(ref s) => s.jsonrpc.clone(),
            JsonRpc::Notification(ref s) => s.jsonrpc.clone(),
            JsonRpc::Success(ref s) => s.jsonrpc.clone(),
            JsonRpc::Error(ref s) => s.jsonrpc.clone(),
        }
    }

    // Helper method for making sure that the jsonrpc
    // field is set to "2.0"
    fn check_version(self) -> Result<Self> {
        let ver = self.get_version();
        if ver != "2.0" {
            Err(JsonRpcErr::InvalidVersionParsed(ver))
        } else {
            Ok(self)
        }
    }

    // Helper method for parsing Values into JsonRpc Objects
    fn parse(val: SerdeResult<Value>) -> Result<Self> {
        if let Ok(val) = val {
            if let Ok(val) = from_value::<Self>(val) {
                Self::check_version(val)
            } else {
                Ok(Self::error((), RpcError::invalid_request()))
            }
        } else {
            Ok(Self::error((), RpcError::parse_error()))
        }
    }

    // Helper method for parsing a batch of Values into JsonRpc Objects
    // whilst following JSON-RPC batch rules.
    fn parse_vec(vals: SerdeResult<Vec<Value>>) -> Vec<Result<Self>> {
        if let Ok(vals) = vals {
            if vals.is_empty() {
                vec![Ok(Self::error((), RpcError::invalid_request()))]
            } else {
                vals.into_iter().map(Ok).map(Self::parse).collect()
            }
        } else {
            vec![Ok(Self::error((), RpcError::parse_error()))]
        }
    }

    /// Parse a JSON-RPC Object from a String
    pub fn from_str(input: &str) -> Result<Self> {
        Self::parse(from_str::<Value>(input))
    }

    /// Parse a JSON-RPC Batch from a String
    pub fn from_str_vec(input: &str) -> Vec<Result<Self>> {
        Self::parse_vec(from_str::<Vec<Value>>(input))
    }

    /// Parse a JSON-RPC Object from an array of bytes
    pub fn from_slice(input: &[u8]) -> Result<Self> {
        Self::parse(from_slice::<Value>(input))
    }

    /// Parse a JSON-RPC Batch from an array of bytes
    pub fn from_slice_vec(input: &[u8]) -> Vec<Result<Self>> {
        Self::parse_vec(from_slice::<Vec<Value>>(input))
    }

    /// Parse a JSON-RPC Object from a `Reader`
    pub fn from_reader<R: Read>(input: R) -> Result<Self> {
        Self::parse(from_reader::<R, Value>(input))
    }

    /// Parse a JSON-RPC Batch from a `Reader`
    pub fn from_reader_vec<R: Read>(input: R) -> Vec<Result<Self>> {
        Self::parse_vec(from_reader::<R, Vec<Value>>(input))
    }
}
