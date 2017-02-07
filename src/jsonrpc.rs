use serde_json::{Value, Map, from_str, to_value};
use serde_json::Result as SerdeResult;
use serde_json::value::ToJson;

use {Result, Error};

/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included. If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null and Numbers SHOULD NOT contain fractional parts
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Id {
    Num(i64),
    Str(String),
    Null,
}

impl Id {
    fn from_value(val: &Value) -> Result<Id> {
        match val {
            &Value::String(ref val) => Ok(Id::Str(val.to_string())),
            &Value::Number(ref val) => {
                Ok(Id::Num({
                    if val.is_i64() {
                        val.as_i64().unwrap()
                    } else if val.is_u64() {
                        val.as_u64().unwrap() as i64
                    } else {
                        return Err(Error::invalid_request());
                    }
                }))
            }
            &Value::Null => Ok(Id::Null),
            _ => Err(Error::invalid_request()),
        }
    }
}

impl ToJson for Id {
    fn to_json(&self) -> SerdeResult<Value> {
        match self {
            &Id::Num(ref val) => to_value(val),
            &Id::Str(ref val) => to_value(val),
            _ => Ok(Value::Null),
        }
    }
}

/// A Structured value that holds the parameter values
/// to be used during the invocation of the method.
/// This member MAY be omitted.
#[derive(Clone, PartialEq, Debug)]
pub enum Params {
    Array(Vec<Value>),
    Map(Map<String, Value>),
    None,
}

impl Params {
    fn from_value(val: &Value) -> Result<Params> {
        match val {
            &Value::Array(ref v) => Ok(Params::Array(v.clone())),
            &Value::Object(ref v) => Ok(Params::Map(v.clone())),
            _ => Err(Error::invalid_request()),
        }
    }
}

impl ToJson for Params {
    fn to_json(&self) -> SerdeResult<Value> {
        match self {
            &Params::Array(ref val) => to_value(val),
            &Params::Map(ref val) => to_value(val),
            _ => Ok(Value::Null),
        }
    }
}

/// JSON-RPC 2.0 Request object and Response object
/// [JSON-RPC 2.0 Specification](http://www.jsonrpc.org/specification).
#[derive(Clone, PartialEq, Debug)]
pub enum JsonRPC {
    /// Request object
    Request(Value),
    /// Notification object
    Notification(Value),
    /// Success Response
    Success(Value),
    /// Error Response
    Error(Value),
    /// Error Request
    ErrorRequst(Error),
}

impl JsonRPC {
    /// Creates a JSON-RPC 2.0 request object without params
    pub fn request(id: &Id, method: &str) -> Self {
        JsonRPC::Request(json!({
            "jsonrpc": "2.0",
            "method": method,
            "id": id.to_json().unwrap(),
        }))
    }

    /// Creates a JSON-RPC 2.0 request object with params
    pub fn request_with_params(id: &Id, method: &str, params: &Params) -> Self {
        match params {
            &Params::None => Self::request(id, method),
            _ => {
                JsonRPC::Request(json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params.to_json().unwrap(),
                    "id": id.to_json().unwrap(),
                }))
            }
        }
    }

    /// Creates a JSON-RPC 2.0 notification object without params
    pub fn notification(method: &str) -> Self {
        JsonRPC::Notification(json!({
            "jsonrpc": "2.0",
            "method": method,
        }))
    }

    /// Creates a JSON-RPC 2.0 notification object with params
    pub fn notification_with_params(method: &str, params: &Params) -> Self {
        match params {
            &Params::None => Self::notification(method),
            _ => {
                JsonRPC::Notification(json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params.to_json().unwrap(),
                }))
            }
        }
    }

    /// Creates a JSON-RPC 2.0 success response object
    pub fn success(id: &Id, result: &Value) -> Self {
        JsonRPC::Success(json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id.to_json().unwrap(),
        }))
    }

    /// Creates a JSON-RPC 2.0 error response object
    pub fn error(id: &Id, error: &Error) -> Self {
        JsonRPC::Error(json!({
            "jsonrpc": "2.0",
            "error": error.to_json().unwrap(),
            "id": id.to_json().unwrap(),
        }))
    }

    pub fn get_id(&self) -> Option<Id> {
        match self {
            &JsonRPC::Request(ref v) |
            &JsonRPC::Success(ref v) |
            &JsonRPC::Error(ref v) => Some(Id::from_value(&v["id"]).unwrap()),
            _ => None,
        }
    }

    pub fn get_method<'a>(&'a self) -> Option<&'a str> {
        match self {
            &JsonRPC::Request(ref v) |
            &JsonRPC::Notification(ref v) => Some(v["method"].as_str().unwrap()),
            _ => None,
        }
    }

    pub fn get_params(&self) -> Option<Params> {
        match self {
            &JsonRPC::Request(ref v) |
            &JsonRPC::Notification(ref v) => Params::from_value(&v["params"]).ok(),
            _ => None,
        }
    }

    pub fn get_result<'a>(&'a self) -> Option<&'a Value> {
        match self {
            &JsonRPC::Success(ref v) => Some(&v["result"]),
            _ => None,
        }
    }

    pub fn get_error<'a>(&'a self) -> Option<&'a Value> {
        match self {
            &JsonRPC::Error(ref v) => Some(&v["error"]),
            _ => None,
        }
    }

    /// Parses JSON string to a array of JSON-RPC 2.0 object
    pub fn parse(json: &str) -> Vec<Self> {
        let mut res = Vec::new();

        let json = from_str::<Value>(json);
        if json.is_err() {
            res.push(JsonRPC::ErrorRequst(Error::parse_error()));
            return res;
        }
        let json = json.unwrap();

        if json.is_array() {
            for val in json.as_array().unwrap() {
                res.push(Self::parse_object(val));
            }
        } else {
            res.push(Self::parse_object(&json));
        }

        if res.len() == 0 {
            res.push(JsonRPC::ErrorRequst(Error::invalid_request()));
        }

        res
    }

    /// Parses JSON value object to a JSON-RPC 2.0 object
    pub fn parse_object(json: &Value) -> Self {
        if json["jsonrpc"] == json!("2.0") {
            if let Ok(id) = Id::from_value(&json["id"]) {
                if id == Id::Null {
                    let method = json.get("method").and_then(Value::as_str);
                    if method.is_none() {
                        return JsonRPC::ErrorRequst(Error::method_not_found());
                    }

                    let params = json.get("params");
                    if params.is_none() {
                        return JsonRPC::notification(method.unwrap());
                    }
                    let params = Params::from_value(params.unwrap());
                    if params.is_err() {
                        return JsonRPC::ErrorRequst(Error::invalid_request());
                    }
                    return JsonRPC::notification_with_params(method.unwrap(), &params.unwrap());
                } else {
                    if let Some(method) = json.get("method") {
                        let method = method.as_str();
                        if method.is_none() {
                            return JsonRPC::ErrorRequst(Error::method_not_found());
                        }

                        let params = json.get("params");
                        if params.is_none() {
                            return JsonRPC::request(&id, method.unwrap());
                        }
                        let params = Params::from_value(params.unwrap());
                        if params.is_err() {
                            return JsonRPC::ErrorRequst(Error::invalid_request());
                        }
                        return JsonRPC::request_with_params(&id, method.unwrap(), &params.unwrap());

                    } else if let Some(result) = json.get("result") {
                        return JsonRPC::success(&id, result);
                    } else if let Some(error) = json.get("error") {
                        let error = Error::from_value(error);
                        if error.is_err() {
                            return JsonRPC::ErrorRequst(Error::invalid_request());
                        }

                        return JsonRPC::error(&id, &error.unwrap());
                    }
                }
            }
        }

        JsonRPC::ErrorRequst(Error::invalid_request())
    }
}

impl ToJson for JsonRPC {
    /// Converts a JSON-RPC 2.0 object to a JSON value
    fn to_json(&self) -> SerdeResult<Value> {
        match self {
            &JsonRPC::Request(ref val) => to_value(val),
            &JsonRPC::Notification(ref val) => to_value(val),
            &JsonRPC::Success(ref val) => to_value(val),
            &JsonRPC::Error(ref val) => to_value(val),
            &JsonRPC::ErrorRequst(ref val) => val.to_json(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn jsonrpc_request() {
        let jsonrpc = JsonRPC::request(&Id::Num(0), "test");
        println!("{:?}", jsonrpc);
    }

    #[test]
    fn jsonrpc_request_with_params() {
        let jsonrpc = JsonRPC::request_with_params(&Id::Num(0),
                                                   "test",
                                                   &Params::Array(vec![Value::Number(0.into()),
                                                                       Value::Number(1.into())]));
        println!("{:?}", jsonrpc);
    }

    #[test]
    fn jsonrpc_parse() {
        let jsonrpc = JsonRPC::parse("{\"id\": 0, \"jsonrpc\": \"2.0\", \"method\": \"test\", \
                                      \"params\": [0,1]}");
        println!("{:?}", jsonrpc);
    }
}
