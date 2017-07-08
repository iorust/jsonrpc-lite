use super::*;
use serde_json::Result as SerdeResult;

/// JSON-RPC 2.0 Request object and Response object
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
    /// Creates a JSON-RPC 2.0 request object without params
    pub fn request<I: Into<Id>>(id: I, method: &str) -> Self {
        JsonRpc::Request(Request {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: None,
            id: id.into(),
        })
    }

    /// Creates a JSON-RPC 2.0 request object with params
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
    pub fn notification(method: &str) -> Self {
        JsonRpc::Notification(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: None,
        })
    }

    /// Creates a JSON-RPC 2.0 notification object with params
    pub fn notification_with_params<P: Into<Params>>(method: &str, params: P) -> Self {
        JsonRpc::Notification(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from(method),
            params: Some(params.into()),
        })
    }

    /// Creates a JSON-RPC 2.0 success response object
    pub fn success<I: Into<Id>>(id: I, result: &Value) -> Self {
        JsonRpc::Success(Success {
            jsonrpc: String::from("2.0"),
            result: result.clone(),
            id: id.into(),
        })
    }

    /// Creates a JSON-RPC 2.0 error response object
    pub fn error<I: Into<Id>>(id: I, error: RpcError) -> Self {
        JsonRpc::Error(Error {
            jsonrpc: String::from("2.0"),
            error,
            id: id.into(),
        })
    }

    pub fn get_id(&self) -> Option<Id> {
        match *self {
            JsonRpc::Request(ref v) => Some(v.id.clone()),
            JsonRpc::Success(ref v) => Some(v.id.clone()),
            JsonRpc::Error(ref v) => Some(v.id.clone()),
            _ => None,
        }
    }

    pub fn get_method(&self) -> Option<&str> {
        match *self {
            JsonRpc::Notification(ref v) => Some(&v.method),
            JsonRpc::Request(ref v) => Some(&v.method),
            _ => None,
        }
    }

    pub fn get_params(&self) -> Option<Params> {
        match *self {
            JsonRpc::Notification(ref v) => v.params.as_ref().cloned(),
            JsonRpc::Request(ref v) => v.params.as_ref().cloned(),
            _ => None,
        }
    }

    pub fn get_result(&self) -> Option<&Value> {
        match *self {
            JsonRpc::Success(ref v) => Some(&v.result),
            _ => None,
        }
    }

    pub fn get_error(&self) -> Option<&RpcError> {
        match *self {
            JsonRpc::Error(ref v) => Some(&v.error),
            _ => None,
        }
    }

    pub fn parse(input: &str) -> SerdeResult<Self> {
        use serde_json::from_str;
        from_str(input)
    }

    pub fn parse_vec(input: &str) -> SerdeResult<Vec<Self>> {
        use serde_json::from_str;
        from_str(input)
    }
}
