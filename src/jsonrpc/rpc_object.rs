use super::*;
use serde_json::{from_str, from_slice, from_reader, from_value, Result as SerdeResult};
use std::io::Read;

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
    pub fn request_with_params<I: Into<Id>, P: Into<Params>>(id: I, method: &str, params: P) -> Self {
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

    pub fn is_request(&self) -> bool {
        if let JsonRpc::Request(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_notification(&self) -> bool {
        if let JsonRpc::Notification(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_success(&self) -> bool {
        if let JsonRpc::Success(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_error(&self) -> bool {
        if let JsonRpc::Error(_) = *self {
            true
        } else {
            false
        }
    }

    fn get_version(&self) -> String {
        match *self {
            JsonRpc::Request(ref s) => s.jsonrpc.clone(),
            JsonRpc::Notification(ref s) => s.jsonrpc.clone(),
            JsonRpc::Success(ref s) => s.jsonrpc.clone(),
            JsonRpc::Error(ref s) => s.jsonrpc.clone(),
        }
    }

    fn check_version(self) -> Result<Self> {
        let ver = self.get_version();
        if ver != "2.0" {
            Err(JsonRpcErr::InvalidVersionParsed(ver))
        } else {
            Ok(self)
        }
    }

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

    pub fn from_str(input: &str) -> Result<Self> {
        Self::parse(from_str::<Value>(input))
    }

    pub fn from_str_vec(input: &str) -> Vec<Result<Self>> {
        Self::parse_vec(from_str::<Vec<Value>>(input))
    }

    pub fn from_slice(input: &[u8]) -> Result<Self> {
        Self::parse(from_slice::<Value>(input))
    }

    pub fn from_slice_vec(input: &[u8]) -> Vec<Result<Self>> {
        Self::parse_vec(from_slice::<Vec<Value>>(input))
    }

    pub fn from_reader<R: Read>(input: R) -> Result<Self> {
        Self::parse(from_reader::<R, Value>(input))
    }

    pub fn from_reader_vec<R: Read>(input: R) -> Vec<Result<Self>> {
        Self::parse_vec(from_reader::<R, Vec<Value>>(input))
    }
}
