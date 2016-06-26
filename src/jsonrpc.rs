extern crate serde;
extern crate serde_json;

use serde_json::{Value, Map, from_str, to_value};
use serde_json::value::ToJson;

use super::error::{Result, Error};

/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included. If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null  and Numbers SHOULD NOT contain fractional parts
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
            &Value::I64(val) => Ok(Id::Num(val)),
            &Value::Null => Ok(Id::Null),
            _ => Err(Error::invalid_request()),
        }
    }
}

impl ToJson for Id {
    fn to_json(&self) -> Value {
        match self {
            &Id::Num(ref val) => to_value(val),
            &Id::Str(ref val) => to_value(val),
            _ => Value::Null,
        }
    }
}

/// A Structured value that holds the parameter values to be used during the invocation of the method. This member MAY be omitted.
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
    fn to_json(&self) -> Value {
        match self {
            &Params::Array(ref val) => to_value(val),
            &Params::Map(ref val) => to_value(val),
            _ => Value::Null,
        }
    }
}

/// JSON-RPC 2.0 Request object and Response object
/// [JSON-RPC 2.0 Specification](http://www.jsonrpc.org/specification).
#[derive(Clone, PartialEq, Debug)]
pub enum JsonRPC {
	/// Request object
    Request(Map<String, Value>),
	/// Notification object
    Notification(Map<String, Value>),
	/// Success Response
    Success(Map<String, Value>),
	/// Error Response
    Error(Map<String, Value>),
	/// Error Request
    ErrorRequst(Error),
}

impl JsonRPC {
	/// Creates a JSON-RPC 2.0 request object without params
    pub fn request(id: &Id, method: &str) -> Self {
        let mut map = Map::new();
        map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
        map.insert("method".to_string(), to_value(method));
        map.insert("id".to_string(), id.to_json());
        JsonRPC::Request(map)
    }

	/// Creates a JSON-RPC 2.0 request object with params
    pub fn request_with_params(id: &Id, method: &str, params: &Params) -> Self {
        let mut map = Map::new();
        map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
        map.insert("method".to_string(), to_value(method));
        let params = params.to_json();
        if params != Value::Null {
            map.insert("params".to_string(), params);
        }

        map.insert("id".to_string(), id.to_json());
        JsonRPC::Request(map)
    }

	/// Creates a JSON-RPC 2.0 notification object without params
    pub fn notification(method: &str) -> Self {
        let mut map = Map::new();
        map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
        map.insert("method".to_string(), to_value(method));
        JsonRPC::Notification(map)
    }

	/// Creates a JSON-RPC 2.0 notification object with params
    pub fn notification_with_params(method: &str, params: &Params) -> Self {
        let mut map = Map::new();
        map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
        map.insert("method".to_string(), to_value(method));
        let params = params.to_json();
        if params != Value::Null {
            map.insert("params".to_string(), params);
        }
        JsonRPC::Notification(map)
    }

	/// Creates a JSON-RPC 2.0 success response object
    pub fn success(id: &Id, result: &Value) -> Self {
        let mut map = Map::new();
        map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
        map.insert("result".to_string(), result.clone());
        map.insert("id".to_string(), id.to_json());
        JsonRPC::Success(map)
    }

	/// Creates a JSON-RPC 2.0 error response object
    pub fn error(id: &Id, error: &Error) -> Self {
        let mut map = Map::new();
        map.insert("jsonrpc".to_string(), Value::String("2.0".to_string()));
        map.insert("error".to_string(), error.to_json());
        map.insert("id".to_string(), id.to_json());
        JsonRPC::Error(map)
    }

    pub fn get_id(&self) -> Option<Id> {
		match self {
            &JsonRPC::Request(ref map) => {
				Some(Id::from_value(map.get("id").unwrap()).unwrap())
			}
            &JsonRPC::Success(ref map) => {
				Some(Id::from_value(map.get("id").unwrap()).unwrap())
			}
            &JsonRPC::Error(ref map) => {
				Some(Id::from_value(map.get("id").unwrap()).unwrap())
			}
            _ => None,
        }
	}

	pub fn get_method<'a>(&'a self) -> Option<&'a str> {
		match self {
            &JsonRPC::Request(ref map) => {
				Some(map.get("method").unwrap().as_string().unwrap())
			}
            &JsonRPC::Notification(ref map) => {
				Some(map.get("method").unwrap().as_string().unwrap())
			}
            _ => None,
        }
	}

	pub fn get_params(&self) -> Option<Params> {
		match self {
            &JsonRPC::Request(ref map) => {
				match map.get("params") {
					Some(params) => Some(Params::from_value(params).unwrap()),
					None => None,
				}
			}
            &JsonRPC::Notification(ref map) => {
				match map.get("params") {
					Some(params) => Some(Params::from_value(params).unwrap()),
					None => None,
				}
			}
            _ => None,
        }
	}

	pub fn get_result<'a>(&'a self) -> Option<&'a Value> {
		match self {
            &JsonRPC::Success(ref map) => Some(map.get("result").unwrap()),
            _ => None,
        }
	}

	pub fn get_error<'a>(&'a self) -> Option<&'a Value> {
		match self {
			&JsonRPC::Error(ref map) => Some(map.get("error").unwrap()),
            _ => None,
        }
	}

	/// Parses JSON string to a array of JSON-RPC 2.0 object
    pub fn parse(json: &str) -> Vec<Self> {
        let mut res: Vec<Self> = Vec::new();

        let json = from_str::<Value>(json);
        if json.is_err() {
            res.push(JsonRPC::ErrorRequst(Error::parse_error()));
            return res;
        }

        let json: Value = json.unwrap();
        if json.is_array() {
            let array: &Vec<Value> = json.as_array().unwrap();
            for val in array.iter() {
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
        let json = json.as_object();
        if json.is_none() {
            return JsonRPC::ErrorRequst(Error::invalid_request());
        }
        let json: &Map<String, Value> = json.unwrap();

        let version = json.get("jsonrpc").and_then(|val| {
            if val == &to_value("2.0") {
                Some(val)
            } else {
                None
            }
        });
        if version.is_none() {
            return JsonRPC::ErrorRequst(Error::invalid_request());
        }

        let id = json.get("id");

        if id.is_none() {
            let method = json.get("method").and_then(|val| {
                if val.is_string() {
                    Some(val.as_string().unwrap())
                } else {
                    None
                }
            });
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
            let params: &Params = &params.unwrap();
            return JsonRPC::notification_with_params(method.unwrap(), params);
        }

        let id = Id::from_value(id.unwrap());
        if id.is_err() {
            return JsonRPC::ErrorRequst(Error::invalid_request());
        }
        let id: &Id = &id.unwrap();

        if json.contains_key("method") {
            let method = json.get("method").and_then(|val| {
                if val.is_string() {
                    Some(val.as_string().unwrap())
                } else {
                    None
                }
            });
            if method.is_none() {
                return JsonRPC::ErrorRequst(Error::method_not_found());
            }

            let params = json.get("params");
            if params.is_none() {
                return JsonRPC::request(id, method.unwrap());
            }
            let params = Params::from_value(params.unwrap());
            if params.is_err() {
                return JsonRPC::ErrorRequst(Error::invalid_request());
            }
            let params: &Params = &params.unwrap();
            return JsonRPC::request_with_params(id, method.unwrap(), params);
        }

        if json.contains_key("result") {
            let result = json.get("result");
            return JsonRPC::success(id, result.unwrap());
        }

        if json.contains_key("error") {
            let error = json.get("error");
            let error = Error::from_value(error.unwrap());
            if error.is_err() {
                return JsonRPC::ErrorRequst(Error::invalid_request());
            }
            let error: &Error = &error.unwrap();
            return JsonRPC::error(id, error);
        }

        return JsonRPC::ErrorRequst(Error::invalid_request());
    }
}

impl ToJson for JsonRPC {
	/// Converts a JSON-RPC 2.0 object to a JSON value
    fn to_json(&self) -> Value {
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
                                                   &Params::Array(vec![Value::I64(0),
                                                                       Value::I64(1)]));
        println!("{:?}", jsonrpc);
    }

    #[test]
    fn jsonrpc_parse() {
        let jsonrpc = JsonRPC::parse("{\"id\": 0, \"jsonrpc\": \"2.0\", \"method\": \"test\", \
                                      \"params\": [0,1]}");
        println!("{:?}", jsonrpc);
    }
}
