use Error as RpcError;
use serde_json::{Map, Value, Result as SerdeResult};

/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included. If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null and Numbers SHOULD NOT contain fractional parts
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum Id {
    Num(i64),
    Str(String),
    None(()),
}

impl From<()> for Id {
    fn from(val: ()) -> Self {
        Id::None(val)
    }
}

impl From<i64> for Id {
    fn from(val: i64) -> Self {
        Id::Num(val)
    }
}

impl From<String> for Id {
    fn from(val: String) -> Self {
        Id::Str(val)
    }
}

/// A Structured value that holds the parameter values
/// to be used during the invocation of the method.
/// This member MAY be omitted.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    Array(Vec<Value>),
    Map(Map<String, Value>),
    None(()),
}

impl From<Value> for Params {
    fn from(val: Value) -> Self {
        match val {
            Value::Array(v) => Params::Array(v),
            Value::Object(v) => Params::Map(v),
            _ => Params::None(()),
        }
    }
}

impl From<Vec<Value>> for Params {
    fn from(val: Vec<Value>) -> Self {
        Params::Array(val)
    }
}

impl From<Map<String, Value>> for Params {
    fn from(val: Map<String, Value>) -> Self {
        Params::Map(val)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Request {
    jsonrpc: String,
    method: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
    id: Id,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Notification {
    jsonrpc: String,
    method: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Success {
    jsonrpc: String,
    result: Value,
    id: Id,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    jsonrpc: String,
    error: RpcError,
    id: Id,
}

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

    pub fn get_version(&self) -> Option<&str> {
        match self {
            JsonRpc::Notification(ref v) => Some(&v.jsonrpc),
            JsonRpc::Request(ref v) => Some(&v.jsonrpc),
            JsonRpc::Success(ref v) => Some(&v.jsonrpc),
            JsonRpc::Error(ref v) => Some(&v.jsonrpc),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_value;

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
        )).expect("Unable to turn request_with_params_vec into a Json Value");
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
        )).expect("Unable to turn request_with_params_map into a Json Value");
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
