//! JSON-RPC 2.0 implementation
//!
//! This module provides types and utilities for working with JSON-RPC 2.0 protocol.
//! It implements the core JSON-RPC 2.0 objects as defined in the specification,
//! including requests, notifications, and responses.
//!
//! The main type is `JsonRpc` which represents all possible JSON-RPC message types.
//! Helper methods are provided for creating and parsing JSON-RPC messages.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Result as SerdeResult, Value};

use crate::Error as RpcError;

/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included. If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null and Numbers SHOULD NOT contain fractional parts
///
/// As per the JSON-RPC 2.0 specification, this identifier is used to correlate
/// requests with their corresponding responses.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum Id {
    /// Numeric identifier
    Num(i64),
    /// String identifier
    Str(String),
    /// Null identifier (represented as unit type in Rust)
    None(()),
}

impl From<()> for Id {
    /// Converts unit type to Id::None
    ///
    /// # Arguments
    ///
    /// * `val` - The unit value to convert
    ///
    /// # Returns
    ///
    /// A new Id::None variant
    fn from(val: ()) -> Self {
        Id::None(val)
    }
}

impl From<i64> for Id {
    /// Converts an i64 to Id::Num
    ///
    /// # Arguments
    ///
    /// * `val` - The i64 value to convert
    ///
    /// # Returns
    ///
    /// A new Id::Num variant containing the provided value
    fn from(val: i64) -> Self {
        Id::Num(val)
    }
}

impl From<String> for Id {
    /// Converts a String to Id::Str
    ///
    /// # Arguments
    ///
    /// * `val` - The String value to convert
    ///
    /// # Returns
    ///
    /// A new Id::Str variant containing the provided value
    fn from(val: String) -> Self {
        Id::Str(val)
    }
}

/// A Structured value that holds the parameter values
/// to be used during the invocation of the method.
/// This member MAY be omitted.
///
/// Parameters can be provided as either an ordered array or a named map,
/// as per the JSON-RPC 2.0 specification.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    /// Parameters as an ordered array of values
    Array(Vec<Value>),
    /// Parameters as a map of named values
    Map(Map<String, Value>),
    /// No parameters (represented as unit type in Rust)
    None(()),
}

impl From<Value> for Params {
    /// Converts a serde_json::Value to Params
    ///
    /// # Arguments
    ///
    /// * `val` - The Value to convert
    ///
    /// # Returns
    ///
    /// - Params::Array if the value is an array
    /// - Params::Map if the value is an object
    /// - Params::None otherwise
    fn from(val: Value) -> Self {
        match val {
            Value::Array(v) => Params::Array(v),
            Value::Object(v) => Params::Map(v),
            _ => Params::None(()),
        }
    }
}

impl From<Vec<Value>> for Params {
    /// Converts a Vec<Value> to Params::Array
    ///
    /// # Arguments
    ///
    /// * `val` - The vector to convert
    ///
    /// # Returns
    ///
    /// A new Params::Array variant containing the provided vector
    fn from(val: Vec<Value>) -> Self {
        Params::Array(val)
    }
}

impl From<Map<String, Value>> for Params {
    /// Converts a Map<String, Value> to Params::Map
    ///
    /// # Arguments
    ///
    /// * `val` - The map to convert
    ///
    /// # Returns
    ///
    /// A new Params::Map variant containing the provided map
    fn from(val: Map<String, Value>) -> Self {
        Params::Map(val)
    }
}

/// JSON-RPC 2.0 Request object
///
/// A request object represents a call to a method on the server.
/// It contains a method name, optional parameters, and an identifier
/// that will be used to match the response to this request.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Request {
    /// JSON-RPC protocol version (always "2.0")
    jsonrpc: String,
    /// Name of the method to be invoked
    method: String,
    /// Parameters to be used during the invocation of the method
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
    /// Client-established identifier for this request
    id: Id,
}

/// JSON-RPC 2.0 Notification object
///
/// A notification is similar to a request but does not require a response.
/// It contains a method name and optional parameters, but no identifier.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// JSON-RPC protocol version (always "2.0")
    jsonrpc: String,
    /// Name of the method to be invoked
    method: String,
    /// Parameters to be used during the invocation of the method
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
}

/// JSON-RPC 2.0 Success Response object
///
/// A success response is sent when a request has been processed successfully.
/// It contains the result of the method call and the identifier from the request.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Success {
    /// JSON-RPC protocol version (always "2.0")
    jsonrpc: String,
    /// The result of the method call
    result: Value,
    /// Client-established identifier matching the request
    id: Id,
}

/// JSON-RPC 2.0 Error Response object
///
/// An error response is sent when a request could not be processed successfully.
/// It contains an error object and the identifier from the request.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    /// JSON-RPC protocol version (always "2.0")
    jsonrpc: String,
    /// The error that occurred
    error: RpcError,
    /// Client-established identifier matching the request
    id: Id,
}

/// JSON-RPC 2.0 Request object and Response object
/// [JSON-RPC 2.0 Specification](http://www.jsonrpc.org/specification).
///
/// This enum represents all possible JSON-RPC message types:
/// - Request: A method call with an identifier
/// - Notification: A method call without an identifier (no response expected)
/// - Success: A successful response to a request
/// - Error: An error response to a request
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
    /// Creates a JSON-RPC 2.0 request object without params
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier for the request
    /// * `method` - The name of the method to call
    ///
    /// # Returns
    ///
    /// A new JsonRpc::Request variant
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    ///
    /// let request = JsonRpc::request(1, "echo");
    /// ```
    pub fn request<I: Into<Id>>(id: I, method: &str) -> Self {
        JsonRpc::Request(Request {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: None,
            id: id.into(),
        })
    }

    /// Creates a JSON-RPC 2.0 request object with params
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier for the request
    /// * `method` - The name of the method to call
    /// * `params` - The parameters to pass to the method
    ///
    /// # Returns
    ///
    /// A new JsonRpc::Request variant with parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    /// use serde_json::json;
    ///
    /// let request = JsonRpc::request_with_params(1, "add", json!([1, 2]));
    /// ```
    pub fn request_with_params<I: Into<Id>, P: Into<Params>>(
        id: I,
        method: &str,
        params: P,
    ) -> Self {
        JsonRpc::Request(Request {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: Some(params.into()),
            id: id.into(),
        })
    }

    /// Creates a JSON-RPC 2.0 notification object without params
    ///
    /// # Arguments
    ///
    /// * `method` - The name of the method to call
    ///
    /// # Returns
    ///
    /// A new JsonRpc::Notification variant
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    ///
    /// let notification = JsonRpc::notification("ping");
    /// ```
    pub fn notification(method: &str) -> Self {
        JsonRpc::Notification(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: None,
        })
    }

    /// Creates a JSON-RPC 2.0 notification object with params
    ///
    /// # Arguments
    ///
    /// * `method` - The name of the method to call
    /// * `params` - The parameters to pass to the method
    ///
    /// # Returns
    ///
    /// A new JsonRpc::Notification variant with parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    /// use serde_json::json;
    ///
    /// let notification = JsonRpc::notification_with_params("log", json!({"level": "info", "message": "Hello"}));
    /// ```
    pub fn notification_with_params<P: Into<Params>>(method: &str, params: P) -> Self {
        JsonRpc::Notification(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: Some(params.into()),
        })
    }

    /// Creates a JSON-RPC 2.0 success response object
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier matching the request
    /// * `result` - The result of the method call
    ///
    /// # Returns
    ///
    /// A new JsonRpc::Success variant
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    /// use serde_json::json;
    ///
    /// let response = JsonRpc::success(1, &json!(42));
    /// ```
    pub fn success<I: Into<Id>>(id: I, result: &Value) -> Self {
        JsonRpc::Success(Success {
            jsonrpc: String::from("2.0"),
            result: result.clone(),
            id: id.into(),
        })
    }

    /// Creates a JSON-RPC 2.0 error response object
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier matching the request
    /// * `error` - The error that occurred
    ///
    /// # Returns
    ///
    /// A new JsonRpc::Error variant
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::{JsonRpc, Error};
    ///
    /// let response = JsonRpc::error(1, Error::method_not_found());
    /// ```
    pub fn error<I: Into<Id>>(id: I, error: RpcError) -> Self {
        JsonRpc::Error(Error {
            jsonrpc: String::from("2.0"),
            error,
            id: id.into(),
        })
    }

    /// Gets the JSON-RPC protocol version
    ///
    /// # Returns
    ///
    /// The protocol version string ("2.0") or None if not available
    pub fn get_version(&self) -> Option<&str> {
        match self {
            JsonRpc::Notification(ref v) => Some(&v.jsonrpc),
            JsonRpc::Request(ref v) => Some(&v.jsonrpc),
            JsonRpc::Success(ref v) => Some(&v.jsonrpc),
            JsonRpc::Error(ref v) => Some(&v.jsonrpc),
        }
    }

    /// Gets the identifier from the JSON-RPC message
    ///
    /// # Returns
    ///
    /// The identifier if present (for requests and responses), or None for notifications
    pub fn get_id(&self) -> Option<Id> {
        match *self {
            JsonRpc::Request(ref v) => Some(v.id.clone()),
            JsonRpc::Success(ref v) => Some(v.id.clone()),
            JsonRpc::Error(ref v) => Some(v.id.clone()),
            _ => None,
        }
    }

    /// Gets the method name from the JSON-RPC message
    ///
    /// # Returns
    ///
    /// The method name if present (for requests and notifications), or None for responses
    pub fn get_method(&self) -> Option<&str> {
        match *self {
            JsonRpc::Notification(ref v) => Some(&v.method),
            JsonRpc::Request(ref v) => Some(&v.method),
            _ => None,
        }
    }

    /// Gets the parameters from the JSON-RPC message
    ///
    /// # Returns
    ///
    /// The parameters if present (for requests and notifications), or None for responses
    pub fn get_params(&self) -> Option<Params> {
        match *self {
            JsonRpc::Notification(ref v) => v.params.as_ref().cloned(),
            JsonRpc::Request(ref v) => v.params.as_ref().cloned(),
            _ => None,
        }
    }

    /// Gets the result from a successful JSON-RPC response
    ///
    /// # Returns
    ///
    /// The result if this is a success response, or None otherwise
    pub fn get_result(&self) -> Option<&Value> {
        match *self {
            JsonRpc::Success(ref v) => Some(&v.result),
            _ => None,
        }
    }

    /// Gets the error from an error JSON-RPC response
    ///
    /// # Returns
    ///
    /// The error if this is an error response, or None otherwise
    pub fn get_error(&self) -> Option<&RpcError> {
        match *self {
            JsonRpc::Error(ref v) => Some(&v.error),
            _ => None,
        }
    }

    /// Parses a JSON string into a JSON-RPC message
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to parse
    ///
    /// # Returns
    ///
    /// A Result containing either the parsed JsonRpc or a serde_json error
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    ///
    /// let input = r#"{"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1}"#;
    /// let request = JsonRpc::parse(input).unwrap();
    /// ```
    pub fn parse(input: &str) -> SerdeResult<Self> {
        use serde_json::from_str;
        from_str(input)
    }

    /// Parses a JSON string into a vector of JSON-RPC messages
    ///
    /// This is useful for batch requests and responses.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to parse
    ///
    /// # Returns
    ///
    /// A Result containing either a vector of parsed JsonRpc objects or a serde_json error
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::JsonRpc;
    ///
    /// let input = r#"[{"jsonrpc":"2.0","method":"sum","params":[1,2,4],"id":"1"},{"jsonrpc":"2.0","method":"notify_hello","params":[7]}]"#;
    /// let batch = JsonRpc::parse_vec(input).unwrap();
    /// assert_eq!(batch.len(), 2);
    /// ```
    pub fn parse_vec(input: &str) -> SerdeResult<Vec<Self>> {
        use serde_json::from_str;
        from_str(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};

    #[test]
    fn request() {
        let jsonrpc = to_value(JsonRpc::request((), "test"))
            .expect("Unable to turn request into a Json Value");
        assert_eq!(
            jsonrpc,
            json!({
                "id": null,
                "jsonrpc": "2.0",
                "method": "test"
            })
        );
    }

    #[test]
    fn request_with_params_vec() {
        let jsonrpc = to_value(JsonRpc::request_with_params(
            46714,
            "test",
            json!([true, false, false, true]),
        ))
        .expect("Unable to turn request_with_params_vec into a Json Value");
        assert_eq!(
            jsonrpc,
            json!({
                "id": 46714,
                "jsonrpc": "2.0",
                "method": "test",
                "params": [true, false, false, true]
            })
        );
    }

    #[test]
    fn request_with_params_map() {
        let jsonrpc = to_value(JsonRpc::request_with_params(
            String::from("alpha-gamma-06714"),
            "test",
            json!({
                "key": "94151351-5651651658-56151351351",
                "n": 5158,
                "mean": 454.54
            }),
        ))
        .expect("Unable to turn request_with_params_map into a Json Value");
        assert_eq!(
            jsonrpc,
            json!({
                "id": "alpha-gamma-06714",
                "jsonrpc": "2.0",
                "method": "test",
                "params": {
                    "key": "94151351-5651651658-56151351351",
                    "n": 5158,
                    "mean": 454.54
                }
            })
        );
    }
}
